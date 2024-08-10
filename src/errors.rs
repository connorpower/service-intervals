use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid activity file format")]
    ParserError(#[source] csv::Error),

    #[error("IO error for path: \"{file_path}\"")]
    IOError {
        #[source]
        source: std::io::Error,
        file_path: String,
    },

    #[error("Unknown error")]
    Unknown,
}
