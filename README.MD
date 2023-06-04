# TopN Files and Directories

A simple command-line Rust application that identifies and displays the top N largest files and directories in a specified directory.

## Features

* Traverse a directory to find the top N largest files.
* Traverse a directory to find the top N largest directories.
* Output the size of files and directories in bytes and gigabytes with up to 3 decimal places.

## Requirements

* Rust (edition 2018 or later)
* The `walkdir` and `rayon` crates

## Usage

To run this program, ensure you have Rust installed on your system. Then, clone this repository and navigate to the directory:

```bash
git clone https://github.com/your-username/storage-analysis-rs.git
cd storage-analysis-rs
To compile and run the program, use the cargo run command with the directory you want to analyze and the number of top files/directories you want to display as arguments:

bash
Copy code
cargo run <path> <N>
Here <path> is the directory you want to analyze and <N> is the number of top files/directories you want to display. For example:

bash
Copy code
cargo run /home/username 5