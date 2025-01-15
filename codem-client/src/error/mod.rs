pub mod grep_error;
pub use grep_error::GrepError;

// Re-export common types
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;