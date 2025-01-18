use std::path::Path;
use grep::regex::RegexMatcherBuilder;
use grep::searcher::{Searcher, SearcherBuilder, Sink, SinkMatch};
use tokio::fs;
use tokio::io;
use std::sync::{Arc, Mutex};
use regex::Regex;

use crate::types::{GrepMatch, GrepFileMatch, GrepOptions};

struct GrepSink {
    matches: Arc<Mutex<Vec<GrepMatch>>>,
    lines: Arc<Vec<String>>,
    context_lines: usize,
}

impl GrepSink {
    fn new(content: String, context_lines: usize) -> Self {
        let lines: Vec<String> = content.lines().map(String::from).collect();
        Self {
            matches: Arc::new(Mutex::new(Vec::new())),
            lines: Arc::new(lines),
            context_lines,
        }
    }
}

impl Sink for GrepSink {
    type Error = io::Error;

    fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch) -> Result<bool, Self::Error> {
        let lines = &self.lines;
        // line_number() returns a 1-based line number, or None on error
        let line_num = match mat.line_number() {
            Some(num) => num as usize,
            None => return Ok(true), // Skip invalid lines
        };
        
        let context_start = (line_num - 1).saturating_sub(self.context_lines);
        let context_end = usize::min(line_num - 1 + self.context_lines + 1, lines.len());
        
        let context = lines[context_start..context_end].join("\n");

        let mut matches = self.matches.lock().unwrap();
        matches.push(GrepMatch {
            line_number: line_num,
            context,
        });
        
        Ok(true)
    }
}

pub async fn grep_file(path: impl AsRef<Path>, pattern: &Regex, options: &GrepOptions) -> io::Result<Option<GrepFileMatch>> {
    let content = fs::read_to_string(path.as_ref()).await?;

    let mut searcher = SearcherBuilder::new()
        .line_number(true)
        .build();

    let sink = GrepSink::new(content.clone(), options.context_lines);
    let matches = Arc::clone(&sink.matches);

    let mut builder = RegexMatcherBuilder::new();
builder.case_insensitive(!options.case_sensitive);
let matcher = builder.build(pattern.as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    #[cfg(test)]
    eprintln!("Searching {} with pattern {:?}", path.as_ref().display(), pattern);    

    searcher.search_reader(&matcher, content.as_bytes(), sink)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
    #[cfg(test)]
    eprintln!("Found {} matches", matches.lock().unwrap().len());

    let matches = matches.lock().unwrap();
    
    Ok(if matches.is_empty() {
        None
    } else {
        Some(GrepFileMatch {
            path: path.as_ref().to_path_buf(),
            matches: matches.to_vec(),
        })
    })
}