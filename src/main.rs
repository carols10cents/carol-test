use std::error::Error;
use std::fs::File;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn create_document(filename: &str) -> Result<File, Box<Error>> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err("You have exceeded the allowed number of documents per minute".into());
    }

    Ok(File::create(filename)?)
}

fn main() {
}
