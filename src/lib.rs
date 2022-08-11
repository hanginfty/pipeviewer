pub mod args;
pub mod read;
pub mod stats;
pub mod write;

// block size: 16KB.
const CHUNK_SIZE: usize = 16 * 1024;
