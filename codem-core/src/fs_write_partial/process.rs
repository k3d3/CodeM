use crate::types::{PartialWrite, WriteResult, PartialWriteResult, WriteResultDetails};
use crate::WriteError;
use std::path::Path;
use tokio::fs;

use super::line_map::build_line_map;
use super::find::find_matches;
use super::process_matches::process_matches;

pub async fn process_partial_write(
    path: &Path,
    partial_writes: PartialWrite,
) -> Result<WriteResult, WriteError> {
    let contents = fs::read_to_string(path).await?;
    let lines: Vec<&str> = contents.lines().collect();
    let line_map = build_line_map(&contents);

    let matches = find_matches(&contents, &partial_writes)?;
    let (output, matches_out) = process_matches(
        &contents,
        &lines,
        &line_map,
        &matches,
        &partial_writes,
    );

    fs::write(path, &output).await?;

    Ok(WriteResult {
        line_count: output.lines().count(),
        size: output.len(),
        details: WriteResultDetails::Partial(PartialWriteResult {
            change_results: matches_out,
            full_content: if partial_writes.return_full_content {
                Some(output)
            } else {
                None
            },
        }),
    })
}