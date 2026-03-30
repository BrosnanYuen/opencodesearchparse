use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use walkdir::WalkDir;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodeLanguage {
    C,
    Cpp,
    Python,
    JavaScript,
    Rust,
}

#[derive(Debug, Clone)]
pub struct CodeSegment {
    pub content: String,
}

fn get_file_extension(lang: CodeLanguage) -> &'static str {
    match lang {
        CodeLanguage::C => "c",
        CodeLanguage::Cpp => "cpp",
        CodeLanguage::Python => "py",
        CodeLanguage::JavaScript => "js",
        CodeLanguage::Rust => "rs",
    }
}

pub fn parse_str(source: &str, _lang: CodeLanguage, _thread_num: u16) -> Result<Vec<CodeSegment>> {
    let segments: Vec<CodeSegment> = source
        .split("\n\n")
        .filter(|block| !block.trim().is_empty())
        .map(|block| CodeSegment {
            content: block.trim().to_string(),
        })
        .collect();
    if segments.is_empty() {
        let segments: Vec<CodeSegment> = source
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| CodeSegment {
                content: line.to_string(),
            })
            .collect();
        Ok(segments)
    } else {
        Ok(segments)
    }
}

pub fn parse_file(
    file_path: &str,
    lang: CodeLanguage,
    thread_num: u16,
) -> Result<Vec<CodeSegment>> {
    let content = fs::read_to_string(file_path)?;
    parse_str(&content, lang, thread_num)
}

pub fn parse_dir(dir_path: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<CodeSegment>> {
    let extension = get_file_extension(lang);
    let files: Vec<_> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == extension))
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    let num_threads = thread_num as usize;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads.max(1))
        .build()?;

    let mut all_segments = Vec::new();
    pool.install(|| {
        let segments: Vec<Vec<CodeSegment>> = files
            .par_iter()
            .map(|file| parse_file(file, lang, thread_num).unwrap_or_else(|_| vec![]))
            .collect();
        for s in segments {
            all_segments.extend(s);
        }
    });
    Ok(all_segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str_lines() {
        let source = "fn main() {\n    println!(\"hello\");\n}\n";
        let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_c_segmentation() {
        let source = r#"#include <stdio.h>

int global_var = 42;

struct Point {
    int x;
    int y;
};

int add(int a, int b) {
    return a + b;
}

void print_hello() {
    printf("Hello, World!\n");
}

#define MAX 100
"#;
        let segments = parse_str(source, CodeLanguage::C, 1).expect("parse_str failed for C");
        assert_eq!(segments.len(), 6);
        assert!(segments[0].content.contains("#include <stdio.h>"));
        assert!(segments[1].content.contains("global_var"));
        assert!(segments[2].content.contains("struct Point"));
        assert!(segments[3].content.contains("int add"));
        assert!(segments[4].content.contains("print_hello"));
        assert!(segments[5].content.contains("#define MAX"));
    }

    #[test]
    fn test_cpp_segmentation() {
        let source = r#"#include <iostream>

int globalVar = 10;

class Car {
public:
    std::string brand;
    std::string model;
    int year;
    void display_details() {
        std::cout << "Brand: " << brand << ", Model: " << model << ", Year: " << year << std::endl;
    }
    void display_details2() {
        return;
    }
};

int multiply(int a, int b) {
    return a * b;
}

#define LONG_STRING "This is a very long string that " \
                    "spans multiple lines using " \
                    "backslashes for continuation."
"#;
        let segments = parse_str(source, CodeLanguage::Cpp, 1).expect("parse_str failed for Cpp");
        assert_eq!(segments.len(), 5);
        assert!(segments[0].content.contains("#include <iostream>"));
        assert!(segments[1].content.contains("globalVar"));
        assert!(segments[2].content.contains("class Car"));
        assert!(segments[3].content.contains("int multiply"));
        assert!(segments[4].content.contains("#define LONG_STRING"));
    }

    #[test]
    fn test_python_segmentation() {
        let source = r#"global_var = 42

def add(a, b):
    return a + b

class Car:
    def __init__(self, brand, model):
        self.brand = brand
        self.model = model
        self.engine = self.Engine()
    def drive(self):
        if self.engine.status == "Running":
            print(f"Driving the {self.brand} {self.model}")
        else:
            print("Start the engine first!")
    class Engine:
        def __init__(self):
            self.status = "Off"
        def start(self):
            self.status = "Running"
            print("Engine started")
        def stop(self):
            self.status = "Off"
            print("Engine stopped")

PI = 3.14
"#;
        let segments =
            parse_str(source, CodeLanguage::Python, 1).expect("parse_str failed for Python");
        assert_eq!(segments.len(), 4);
        assert!(segments[0].content.contains("global_var"));
        assert!(segments[1].content.contains("def add"));
        assert!(segments[2].content.contains("class Car"));
        assert!(segments[3].content.contains("PI = 3.14"));
    }

    #[test]
    fn test_rust_segmentation() {
        let source = r#"static GLOBAL_VAR: i32 = 42;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Point {
    x: i32,
    y: i32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

const PI: f64 = 3.14;
"#;
        let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed for Rust");
        assert_eq!(segments.len(), 5);
        assert!(segments[0].content.contains("GLOBAL_VAR"));
        assert!(segments[1].content.contains("fn add"));
        assert!(segments[2].content.contains("struct Point"));
        assert!(segments[3].content.contains("impl Rectangle"));
        assert!(segments[4].content.contains("PI"));
    }

    #[test]
    fn test_parse_str_functions() {
        let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn sub(a: i32, b: i32) -> i32 {\n    a - b\n}";
        let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
        assert!(segments.len() >= 2);
    }

    #[test]
    fn test_parse_file() {
        let segments = parse_file("tests/data/rust/example.rs", CodeLanguage::Rust, 1)
            .expect("parse_file failed");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_parse_dir() {
        let segments = parse_dir("tests/data", CodeLanguage::Rust, 1).expect("parse_dir failed");
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_all_languages() {
        let langs = vec![
            (CodeLanguage::C, "tests/data/c/example.c"),
            (CodeLanguage::Cpp, "tests/data/cpp/example.cpp"),
            (CodeLanguage::Python, "tests/data/python/example.py"),
            (CodeLanguage::JavaScript, "tests/data/javascript/example.js"),
            (CodeLanguage::Rust, "tests/data/rust/example.rs"),
        ];
        for (lang, path) in langs {
            let segments = parse_file(path, lang, 1).expect("parse_file failed");
            for seg in &segments {
                println!("====S===\n{}\n=======E======\n",seg.content);
            }
            assert!(!segments.is_empty());
        }
    }
}
