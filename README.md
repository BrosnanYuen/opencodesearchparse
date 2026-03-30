# Open Code Search Parser

A Rust library designed to parse, extract, and chunk source code into manageable segments. This tool is ideal for processing large codebases, preparing code for LLM context windows, or extracting logical blocks (like functions, structs, and classes) from source files.

## Features

* **Multi-Language Support:** Natively targets and filters extensions for C, C++, Python, JavaScript, and Rust.
* **Smart Segmentation:** Splits code blocks automatically based on blank line delimiters (`\n\n`), keeping logically cohesive blocks (like complete function definitions or classes) intact.
* **Directory Walking:** Recursively scans directories to find and parse all files matching a specific target language.
* **Parallel Processing:** Utilizes `rayon` to process multiple files across directories concurrently, configurable by thread count.

## Dependencies

This library relies on the following crates:
* [`anyhow`](https://crates.io/crates/anyhow) - For idiomatic error handling.
* [`rayon`](https://crates.io/crates/rayon) - For parallel processing over directories.
* [`walkdir`](https://crates.io/crates/walkdir) - For recursive directory traversal.

## Usage

### 1. Parsing a Raw String
You can parse a raw string directly into a `Vec<String>`.

```rust
use opencodesearchparse::{parse_str, CodeLanguage};

let num_threads = 4;
let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn sub(a: i32, b: i32) -> i32 {\n    a - b\n}";
let segments = parse_str(source, CodeLanguage::Rust, num_threads).unwrap();

assert_eq!(segments.len(), 2);
println!("{}", segments[0]); // Prints the `add` function
```


### 2. Parsing a Single File
Read and chunk a specific source file.
```rust
use opencodesearchparse::{parse_file, CodeLanguage};

let num_threads = 4;
let file_path = "src/main.rs";
let segments = parse_file(file_path, CodeLanguage::Rust, num_threads).unwrap();

for segment in segments {
    println!("--- Segment ---\n{}\n", segment);
}
```

### 3. Parsing an Entire Directory

Recursively scan a directory, filter by a specific language, and process the files in parallel.
```rust
use opencodesearchparse::{parse_dir, CodeLanguage};

let dir_path = "./src";
let thread_count = 4; // Adjust based on your CPU cores

// Recursively finds all `.rs` files and parses them using 4 threads
let all_rust_segments = parse_dir(dir_path, CodeLanguage::Rust, thread_count).unwrap();

println!("Extracted {} total segments across the directory.", all_rust_segments.len());
```

### 4. Data Structures

Enum representing the supported target languages. Determines which file extensions are picked up during directory traversal.
```rust
pub enum CodeLanguage {
    C,
    Cpp,
    Python,
    JavaScript,
    Rust,
}
```
