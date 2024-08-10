use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid activity file format or contents")]
    ActivityFormatError(#[source] csv::Error),

    #[error("Invalid DB file format or contents")]
    DBFormatError(#[source] anyhow::Error),

    #[error("IO error for path: \"{file_path}\"")]
    IOError {
        #[source]
        source: std::io::Error,
        file_path: String,
    },

    #[error("Unknown error")]
    Unknown,
}
