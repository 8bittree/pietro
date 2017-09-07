use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello, world!");
    let files = list_files("/Applications");
    println!("{:?}", files);
}

fn list_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.as_ref().is_dir() {
        if let Ok(mut dirEntries) = fs::read_dir(dir) {
            while let Some(Ok(entry)) = dirEntries.next() {
                files.push(entry.path());
            }
        }
    }
    files
}
