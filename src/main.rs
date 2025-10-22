mod scanner;

use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "file tool")]
#[command(about = "Performs deletion or search for duplicated files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

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

fn inspect(path: &str, depth: usize, ignore_hidden: bool) {
    let paths = find_files(path, depth, ignore_hidden);

    println!("Found {} files", paths.len());
    println!("{:?}", paths);
}

fn delete(path: &str, depth: usize) {

}

fn find_files(path: &str, depth: usize, ignore_hidden: bool) -> Vec<PathBuf> {
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


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { path, threads, depth, ignore_hidden } => {
            println!("Inspecting {} files using {} threads", path, threads);
            inspect(&path, depth, ignore_hidden);
        }
        Commands::Delete{path, threads, depth, ignore_hidden} => {
            println!("Deleting {} files using {} threads", path, threads);
        }
    }

}