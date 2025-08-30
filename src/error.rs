use std::io::Error as IoError;
use thiserror::Error as ThisError;
use windows::core::Error as WindowsError;

#[derive(Debug, ThisError)]
pub enum ShellcodeRunnerError {
    #[error("WindowsError::{0}")]
    WindowsError(#[from] WindowsError),
    #[error("IoError::{0}")]
    IoError(#[from] IoError),

    #[error("BufferTooSmall\n> needed : {needed:#X} ({needed}), got : {got:#X} ({got})")]
    BufferTooSmall { needed: usize, got: usize },
    #[error("InvalidOffset\n> offset : {offset:#X} ({offset}), len : {len:#X} ({len})")]
    InvalidOffset { offset: usize, len: usize },
    #[error(
        "InsufficientCapacity\n> got : {got:#X} ({got}), required : {required:#X} ({required})"
    )]
    InsufficientCapacity { got: usize, required: usize },

    #[error("InvalidFilePath\n> path : {path}")]
    InvalidFilePath { path: String },
    #[error("InvalidFilePath\n> path : {path}")]
    NotAFile { path: String },
}
