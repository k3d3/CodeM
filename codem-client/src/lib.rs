pub mod client;
pub mod config;
pub mod error;
pub mod project;
mod session;
pub mod types;

// Re-export main types
pub use client::Client;
pub use config::ClientConfig;
pub use project::Project;
pub use error::ClientError;
pub use session::{SessionId, SessionInfo};
pub use session::manager::SessionManager;