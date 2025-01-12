pub mod types;
pub mod error;
mod client;

pub use client::Client;
pub use error::FileError;
pub use types::{WriteMode, WriteOperation, WriteResult, FileMatch, CheckOptions};

#[cfg(test)]
mod tests;