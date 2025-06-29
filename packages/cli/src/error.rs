use std::fmt::Debug;
use thiserror::Error as ThisError;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(ThisError, Debug)]
pub(crate) enum Error {
    /// Used when errors need to propagate but are too unique to be typed
    #[error("{0}")]
    Unique(String),

    #[error("I/O Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Format Error: {0}")]
    Format(#[from] std::fmt::Error),

    #[error("Format failed: {0}")]
    Parse(String),

    #[error("Runtime Error: {0}")]
    Runtime(String),

    #[error("Cargo Error: {0}")]
    Cargo(String),

    #[error("Invalid proxy URL: {0}")]
    InvalidProxy(#[from] hyper::http::uri::InvalidUri),

    #[error("Establishing proxy: {0}")]
    ProxySetup(String),

    #[error("Bundling project: {0}")]
    BundleFailed(#[from] tauri_bundler::Error),

    #[error("Performing hotpatch: {0}")]
    PatchingFailed(#[from] crate::build::PatchError),

    #[error("Reading object file: {0}")]
    ObjectReadFailed(#[from] object::Error),

    #[error("{0}")]
    CapturedPanic(String),

    #[error("Rendering template error: {0}")]
    TemplateParse(#[from] handlebars::RenderError),

    #[error("Network connectivity error: {0}")]
    Network(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Unique(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Unique(s)
    }
}

impl From<html_parser::Error> for Error {
    fn from(e: html_parser::Error) -> Self {
        Self::Parse(e.to_string())
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::Runtime(e.to_string())
    }
}
