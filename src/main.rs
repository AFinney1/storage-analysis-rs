extern crate walkdir;
extern crate rayon;

use std::env;
use std::fs;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;

#[derive(Debug)]
struct FileInfo {
    path: String,
    size: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: topn_files_and_dirs <path> <N>");
        return;
    }

    let path = &args[1];
    let n: usize = args[2].parse().unwrap();

    let file_sizes: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                let metadata = fs::metadata(entry.path()).unwrap();
                Some(FileInfo {
                    path: String::from(entry.path().to_str().unwrap()),
                    size: metadata.len(),
                })
            } else {
                None
            }
        })
        .collect();

    let mut file_sizes = file_sizes;
    file_sizes.par_sort_unstable_by_key(|k| std::cmp::Reverse(k.size));

    println!("Top {} files by size:", n);
    for file in file_sizes.into_iter().take(n) {
        let size_in_gb = file.size as f64 / (1024.0 * 1024.0 * 1024.0);
        println!("{}: {} bytes ({:.3} GB)", file.path, file.size, size_in_gb);
    }

    let dir_sizes: HashMap<_, _> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let size: u64 = WalkDir::new(entry.path())
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        if e.path().is_file() {
                            Some(fs::metadata(e.path()).unwrap().len())
                        } else {
                            None
                        }
                    })
                    .sum();
                Some((entry.path().to_str().unwrap().to_string(), size))
            } else {
                None
            }
        })
        .collect();

    let mut dir_sizes: Vec<_> = dir_sizes.into_iter().collect();
    dir_sizes.par_sort_unstable_by_key(|k| std::cmp::Reverse(k.1));

    println!("Top {} directories by size:", n);
    for dir in dir_sizes.into_iter().take(n) {
        let size_in_gb = dir.1 as f64 / (1024.0 * 1024.0 * 1024.0);
        println!("{}: {} bytes ({:.3} GB)", dir.0, dir.1, size_in_gb);
    }
}