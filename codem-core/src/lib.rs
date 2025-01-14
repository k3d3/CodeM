mod command;
mod directory;
mod error;
mod fs_ops;
mod grep;
mod path_utils;
pub mod types;

pub use command::*;
pub use directory::*;
pub use error::*;
pub use fs_ops::*;
pub use grep::*;
use path_absolutize::Absolutize;
pub use path_utils::*;

pub fn within_base_path(base_path: &std::path::Path, path: &std::path::Path) -> bool {
    let abs_path = path.absolutize_from(base_path).unwrap();
    abs_path.starts_with(base_path)
}

#[cfg(test)]
mod tests;
