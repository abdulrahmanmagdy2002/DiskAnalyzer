extern crate walkdir;

use std::fs;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let start_path = "/";  // Change this to the directory you want to analyze
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let metadata = fs::metadata(entry.path())?;
            total_size += metadata.len();
        }
    }

    println!("Total disk usage: {} Mega Bytes", total_size/1048576);

    Ok(())
}