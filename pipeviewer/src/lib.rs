pub use args::Args;
pub use io_worker::IoWorker;

mod args;
mod io_worker;
mod timer;

pub const CHUNK_SIZE: usize = 16 * 1024;
