pub use args::Args;
pub use io_worker::IoWorker;
pub use stats::stats;

mod args;
mod io_worker;
mod stats;

pub const CHUNK_SIZE: usize = 16 * 1024;
