use std::path::PathBuf;

pub struct Document {
    pub content: String,
    pub file_location: PathBuf, // source_uri
    // consider adding file hash for incremental indexing.
}

pub struct Chunk {
    pub text: String,
    pub source
}
