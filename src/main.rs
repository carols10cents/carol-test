use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, DocumentServiceError>;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    IoError(io::Error),
}

impl Error for DocumentServiceError {}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match self {
            RateLimitExceeded => write!(f, "You have exceeded the allowed number of documents per minute."),
            IoError(io) => write!(f, "I/O error: {}", io),
        }
    }
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::IoError(other)
    }
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}
//
// if rate limit, wait and try again
// if name is taken, add "(1)" and try again

fn main() {
    let file = create_document("my-document.txt");
    println!("file = {:?}", file);
}
