use thiserror::Error;

#[derive(Debug)]
pub struct LineError {
    pub headers: Vec<String>,
    pub values: Vec<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("File not found {0}")]
    FileNotFound(String),

    #[error("Failed to read '{filename}'")]
    FileReadError {
        filename: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to read csv file '{filename}'")]
    CSVError {
        filename: String,
        #[source]
        source: csv::Error,
        line_in_error: Option<LineError>,
    },
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error("Failed to read file")]
    IO(#[from] std::io::Error),
    #[error("GTFS {0} is not a file or a directory")]
    NotFileOrDirectory(String),
    #[error("'{0}' is not a valid color")]
    InvalidColor(String),
    #[error("'{0}' is not a valid time")]
    InvalidTime(String),
    #[error("The id {0} is not known")]
    ReferenceError(String),
    #[cfg(feature = "read-url")]
    #[error("Failed to download file")]
    Fetch(#[from] reqwest::Error),
}
