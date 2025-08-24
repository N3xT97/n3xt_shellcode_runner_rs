use std::io::Error as IoError;
use thiserror::Error as ThisError;
use windows::core::Error as WindowsError;

#[derive(Debug, ThisError)]
pub enum ShellcodeRunnerError {
    #[error("WindowsError::{0}")]
    WindowsError(#[from] WindowsError),
    #[error("IoError::{0}")]
    IoError(#[from] IoError),

    #[error("BufferTooSmall\n> needed: {needed}, got: {got}")]
    BufferTooSmall { needed: usize, got: usize },
    #[error("InvalidOffset\n> offset: {offset}, len: {len}")]
    InvalidOffset { offset: usize, len: usize },
    #[error("InsufficientCapacity\n> capacity: {capacity}, required: {required}")]
    InsufficientCapacity { capacity: usize, required: usize },

    #[error("InvalidFilePath\n> path: {path}")]
    InvalidFilePath { path: String },
    #[error("InvalidFilePath\n> path: {path}")]
    NotAFile { path: String },
}
