use crate::types::{
    FileMetadata, PartialWriteResult, PartialWriteResultContent, WriteOperation, WriteResult,
};
use crate::WriteError;
use aho_corasick::AhoCorasick;
use std::cmp::min;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs;
use tokio::io;

#[derive(Default)]
pub struct ReadOptions {
    pub count_lines: bool,
}

#[derive(Debug)]
pub struct PatternInfo {
    /// Byte size difference between old_str and new_str
    pub size_diff: isize,
    /// Number of lines in new_str
    pub line_count: usize,
}

#[derive(Debug)]
pub struct MatchInfo {
    pub pattern_index: usize,
    /// Byte start of match relative to previous match end (or start of file)
    pub relative_match_start: usize,
}


pub async fn read_file(
    path: impl AsRef<Path>,
    options: ReadOptions,
) -> io::Result<(String, FileMetadata)> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).await?;
    let metadata = path.metadata()?;

    let line_count = if options.count_lines {
        Some(content.lines().count())
    } else {
        None
    };

    Ok((
        content,
        FileMetadata {
            modified: metadata.modified()?,
            size: metadata.len(),
            line_count,
        },
    ))
}

pub fn get_metadata(path: impl AsRef<Path>, options: ReadOptions) -> io::Result<FileMetadata> {
    let path = path.as_ref();
    let metadata = path.metadata()?;

    let line_count = if options.count_lines {
        Some(std::fs::read_to_string(path)?.lines().count())
    } else {
        None
    };

    Ok(FileMetadata {
        modified: metadata.modified()?,
        size: metadata.len(),
        line_count,
    })
}

pub async fn write_file(
    path: &Path,
    operation: WriteOperation,
    match_timestamp: Option<SystemTime>,
) -> Result<WriteResult, WriteError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if let Some(timestamp) = match_timestamp {
        let metadata = path.metadata()?;
        if metadata.modified()? != timestamp {
            return Err(WriteError::TimestampMismatch);
        }
    }

    match operation {
        WriteOperation::Full(contents) => {
            fs::write(path, &contents).await?;
            let result = WriteResult {
                line_count: contents.lines().count(),
                size: contents.len(),
                partial_write_result: None,
            };
            return Ok(result);
        }
        WriteOperation::Partial(partial_writes) => {
            // Keep track of matches for PartialWrites that have allow_multiple_matches set to false
            // Load all partial writes into an ahocorasick automaton
            // This is important, to make sure we modify in one pass, and thus, keep line numbers consistent
            let ac =
                AhoCorasick::new(partial_writes.writes.iter().map(|pw| pw.old_str.as_bytes()))?;

            // Read the file in its entirety
            let contents = fs::read_to_string(path).await?;

            // Strategy:
            // - Get the size difference between old_str and new_str for each partial write, as well as
            //   the new_str line count
            // - Find all matches using find_iter, and store the pattern and the beginning of the match,
            //   relative to the beginning of the previous match (or the start of the file)
            // - Replace each old_str with each new_str (based on match location). As each match
            //   is found and replaced, store the absolute beginning of each match, as well as which line
            //   the match starts on. This is probably best done by keeping track of the "current" line.
            // - Finally, once all replacements are done, split the output into lines, and find the
            //   line number of each match. Then, get the context lines around each match, and store
            //   the context lines, as well as the line number of the match.

            // First, get the size difference between old_str and new_str for each partial write, as well as
            // the new_str line count
            let mut size_diffs: Vec<PatternInfo> = Vec::new();
            for pw in &partial_writes.writes {
                let size_diff = pw.new_str.len() as isize - pw.old_str.len() as isize;
                let line_count = pw.new_str.lines().count();
                size_diffs.push(PatternInfo {
                    size_diff,
                    line_count,
                });
            }

            println!("{:#?}", size_diffs);

            // Find all matches using find_iter, and store the pattern and the beginning of the match,
            // relative to the beginning of the previous match (or the start of the file)
            let mut matches: Vec<MatchInfo> = Vec::new();
            // let mut last_match_start = 0;
            let mut last_match_end = 0;

            for mat in ac.find_iter(&contents) {
                let pattern_index = mat.pattern().as_usize();
                let relative_match_start = mat.start() - last_match_end;
                last_match_end = mat.end();

                matches.push(MatchInfo {
                    pattern_index,
                    relative_match_start,
                });
            }

            let mut output = String::new();
            let mut last_match_end_position = 0;

            // For each match, write out the preceding text, then the new_str, then update the current position
            for match_info in &matches {
                let pattern_index = match_info.pattern_index;
                let relative_match_start = match_info.relative_match_start;

                // Write out the text before the match
                output.push_str(&contents[last_match_end_position..last_match_end_position + relative_match_start]);

                // Write out the new string
                output.push_str(&partial_writes.writes[pattern_index].new_str);

                // Update the current position
                let old_len = partial_writes.writes[pattern_index].old_str.len();
                last_match_end_position += relative_match_start + old_len;
                if last_match_end_position == contents.len() {
                    break;
                }
            }

            // Write out the final text
            output.push_str(&contents[last_match_end_position..]);

            // Split output into lines
            let output_lines = output.lines().collect::<Vec<_>>();

            // So we now have:
            // - size_diff for each pattern given
            // - matches for each match found in the file
            // - output_lines for the new file contents, split into lines

            // Now, we need to find the start and end line numbers of each match, and get the context lines around each match
            let mut matches_out: Vec<PartialWriteResultContent> = Vec::new();
            let mut last_match_end = 0;

            for match_info in &matches {
                let pattern_index = match_info.pattern_index;
                let relative_match_start = match_info.relative_match_start;
                let old_str_len = partial_writes.writes[pattern_index].old_str.len();
                let size_diff = size_diffs[pattern_index].size_diff as usize;

                // Get the line number of the match
                let line_number_start = output[..last_match_end + relative_match_start].lines().count();
                let line_number_end = line_number_start + size_diffs[pattern_index].line_count;
                last_match_end += relative_match_start + old_str_len + size_diff;

                // Get the context lines around the match
                let context_start = line_number_start.saturating_sub(partial_writes.context_lines);
                // make sure context_end doesn't go past the end
                let context_end = min(line_number_end + partial_writes.context_lines, output_lines.len());

                // get context as string
                let context = output_lines[context_start..context_end].join("\n");

                let result = PartialWriteResultContent {
                    partial_write_index: pattern_index,
                    line_number_start,
                    line_number_end,
                    context,
                };

                matches_out.push(result);
            }

            // Write the new contents to the file
            tokio::fs::write(path, &output).await?;

            let full_content = if partial_writes.return_full_content {
                Some(output.clone())
            } else {
                None
            };

            let result = WriteResult {
                line_count: output_lines.len(),
                size: output.len(),
                partial_write_result: Some(PartialWriteResult {
                    content: matches_out,
                    full_content,
                }),
            };

            return Ok(result);
        }
    }
}
