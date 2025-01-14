pub mod client;
pub mod error;
pub mod project;
pub mod session;
pub mod types;

pub use client::Client;
pub use error::FileError;
pub use types::{CheckOptions, FileMatch, WriteMode, WriteOperation, WriteResult};

#[cfg(test)]
mod tests;
