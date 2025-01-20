use codem_core::types::TreeEntry;

pub fn format_tree_entry(entry: &TreeEntry, include_stats: bool) -> String {
    let mut output = String::new();
    let path = entry.path().file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| entry.path().to_string_lossy().to_string());

    let entry_type = if entry.is_dir() { "📁" } else { "📄" };

    // Add stats info if requested and available
    let stats = if include_stats {
        if let Some(stats) = entry.stats() {
            let mut parts = Vec::new();
            if let Some(lines) = stats.line_count {
                parts.push(format!("{} lines", lines));
            }
            if let Some(size) = stats.size {
                parts.push(format!("{} bytes", size));
            }
            if !parts.is_empty() {
                format!(" ({})", parts.join(", "))
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    output.push_str(&format!("{} {}{}", entry_type, path, stats));

    // Add children if it's a directory and has children
    if entry.is_dir() && !entry.is_empty() {
        for (i, child) in entry.iter().enumerate() {
            let is_last = i == entry.len() - 1;
            let prefix = if is_last { "└─" } else { "├─" };
            
            // Format child entry
            let child_str = format_tree_entry(child, include_stats);
            
            // Split child lines and add proper prefixes
            for (j, line) in child_str.lines().enumerate() {
                if j == 0 {
                    output.push_str(&format!("\n{}{}", prefix, line));
                } else {
                    let cont_prefix = if is_last { "  " } else { "│ " };
                    output.push_str(&format!("\n{}{}", cont_prefix, line));
                }
            }
        }
    }

    output
}