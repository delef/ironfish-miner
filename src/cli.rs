use std::path::PathBuf;
use clap::Parser;

const IPC_FILENAME: &'static str = "ironfish.ipc";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
pub struct Cli {
    /// Path to data
    #[clap(short, long, default_value = ".ironfish")]
    pub data_dir: PathBuf,

    /// Number of threads
    #[clap(short, long,  default_value_t = -1)]
    pub threads: isize,

    /// Batch size
    #[clap(short, long, default_value_t = 1_000_000)]
    pub batch_size: usize,
}

impl Cli {
    pub fn ipc_path(&self) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(IPC_FILENAME);
        path
    }
}