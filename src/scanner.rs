use std::path::PathBuf;
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "file tool")]
#[command(about = "Performs deletion or search for duplicated files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    ///Delete files
    Delete{
        #[arg(long)]
        path: String,

        #[arg(long, default_value_t = 1)]
        threads: u8,

        #[arg(long, default_value_t = 1)]
        depth: usize,

        #[arg(long, default_value_t = true)]
        ignore_hidden: bool,
    },

    ///Inspect files
    Inspect{
        #[arg(long)]
        path: String,

        #[arg(long, default_value_t = 1)]
        threads: u8,

        #[arg(long, default_value_t = 1)]
        depth: usize,

        #[arg(long, default_value_t = true)]
        ignore_hidden: bool,
    },
}

pub fn find_files(path: &str, depth: usize, ignore_hidden: bool) -> Vec<PathBuf> {
    WalkDir::new(path)
        .max_depth(depth)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && (!ignore_hidden || !is_hidden(e)))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}