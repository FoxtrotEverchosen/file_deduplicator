mod scanner;
mod hasher;

use clap::Parser;
use scanner::{Cli, Commands};
use hasher::*;

fn inspect(path: &str, depth: usize, ignore_hidden: bool) {
    let paths = scanner::find_files(path, depth, ignore_hidden);

    println!("Found {} files", paths.len());
    println!("{:?}", paths);
}

fn delete(path: &str, depth: usize) {

}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { path, threads, depth, ignore_hidden } => {
            println!("Inspecting {} files using {} threads", path, threads);
            inspect(&path, depth, ignore_hidden);
        }
        Commands::Delete { path, threads, depth, ignore_hidden } => {
            println!("Deleting {} files using {} threads", path, threads);
        }
    }

}