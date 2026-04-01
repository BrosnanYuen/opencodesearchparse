use super::*;

#[test]
fn test_parse_str_lines() {
    let source = "fn main() {\n    println!(\"hello\");\n}\n";
    let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
    assert!(!segments.is_empty());
}

#[test]
fn test_c_segmentation() {
    let source = r#"
#include <stdio.h>

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
    let expected = vec![
        "#include <stdio.h>\n".to_string(),
        "int global_var = 42;".to_string(),
        r#"struct Point {
    int x;
    int y;
};"#
            .to_string(),
        r#"int add(int a, int b) {
    return a + b;
}"#
        .to_string(),
        r#"void print_hello() {
    printf("Hello, World!\n");
}"#
        .to_string(),
        "#define MAX 100\n".to_string(),
    ];
    assert_eq!(segments, expected);
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
    let expected = vec![
        "#include <iostream>\n".to_string(),
        "int globalVar = 10;".to_string(),
        r#"class Car {
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
};"#
        .to_string(),
        r#"int multiply(int a, int b) {
    return a * b;
}"#
        .to_string(),
        r#"#define LONG_STRING "This is a very long string that " \
                    "spans multiple lines using " \
                    "backslashes for continuation."
"#
            .to_string(),
    ];
    assert_eq!(segments, expected);
}

#[test]
fn test_parse_str_c_and_cpp_capture_conditional_and_pragma_directives() {
    let source = r#"#if VALUE > 0
int positive = 1;
#elif VALUE == 0
int zero = 1;
#else
int negative = 1;
#endif

#ifndef FEATURE_FLAG
#define FEATURE_FLAG 1
#endif

#pragma once
"#;

    for lang in [CodeLanguage::C, CodeLanguage::Cpp] {
        let segments = parse_str(source, lang, 1).expect("parse_str failed for C/C++");
        assert!(segments.iter().any(|s| s.contains("#if VALUE > 0")));
        assert!(segments.iter().any(|s| s.contains("#elif VALUE == 0")));
        assert!(segments.iter().any(|s| s.contains("#else")));
        assert!(segments.iter().any(|s| s.contains("#endif")));
        assert!(segments.iter().any(|s| s.contains("#ifndef FEATURE_FLAG")));
        assert!(segments.iter().any(|s| s.contains("#pragma once")));
    }
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
    let segments = parse_str(source, CodeLanguage::Python, 1).expect("parse_str failed for Python");
    let expected = vec![
        "global_var = 42".to_string(),
        r#"def add(a, b):
    return a + b"#
            .to_string(),
        r#"class Car:
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
            print("Engine stopped")"#
            .to_string(),
        "PI = 3.14".to_string(),
    ];
    assert_eq!(segments, expected);
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
    let expected = vec![
        "static GLOBAL_VAR: i32 = 42;".to_string(),
        r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}"#
        .to_string(),
        r#"struct Point {
    x: i32,
    y: i32,
}"#
        .to_string(),
        r#"impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}"#
        .to_string(),
        "const PI: f64 = 3.14;".to_string(),
    ];
    assert_eq!(segments, expected);
}

#[test]
fn test_parse_str_functions() {
    let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn sub(a: i32, b: i32) -> i32 {\n    a - b\n}";
    let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
    let expected = vec![
        "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}".to_string(),
        "fn sub(a: i32, b: i32) -> i32 {\n    a - b\n}".to_string(),
    ];
    assert_eq!(segments, expected);
}

#[test]
fn test_parse_file() {
    let segments =
        parse_file("tests/data/rust/example.rs", CodeLanguage::Rust, 1).expect("parse_file failed");
    assert!(!segments.is_empty());
}

#[test]
fn test_parse_dir() {
    let segments = parse_dir("tests/data", CodeLanguage::Rust, 1).expect("parse_dir failed");
    assert!(!segments.is_empty());
}

#[test]
fn test_parse_dir_reports_walk_errors() {
    let result = parse_dir("tests/data/does-not-exist", CodeLanguage::Rust, 1);
    assert!(result.is_err());
    let message = format!("{}", result.expect_err("parse_dir should fail for missing directory"));
    assert!(message.contains("Failed to walk directory"));
}

#[test]
fn test_parse_dir_reports_file_parse_errors() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("opencodesearchparser-test-{}", unique));
    std::fs::create_dir_all(&dir).expect("failed to create temp directory");
    let bad_file = dir.join("invalid.rs");
    std::fs::write(&bad_file, [0xff, 0xfe, 0xfd]).expect("failed to write invalid utf-8 file");

    let dir_str = dir.to_string_lossy().to_string();
    let result = parse_dir(&dir_str, CodeLanguage::Rust, 1);

    // Best-effort cleanup for temp test artifacts.
    let _ = std::fs::remove_dir_all(&dir);

    assert!(result.is_err());
    let message = format!("{}", result.expect_err("parse_dir should surface parse/read errors"));
    assert!(message.contains("Failed to parse file"));
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
        assert!(!segments.is_empty());
    }
}

#[test]
fn test_recursive_character_text_splitter() {
    let splitter =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::from_language(
            CodeLanguage::Rust,
        );
let text = r#"static GLOBAL_VAR: i32 = 42;

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
    let chunks = splitter.split_text(text);
    assert!(!chunks.is_empty());
}

#[test]
fn test_recursive_splitter_various_chunk_sizes() {
    let text = r#"static GLOBAL_VAR: i32 = 42;

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
    let splitter1 =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 60, 10);
    let chunks1 = splitter1.split_text(text);
    for seg in &chunks1 {
        println!("====S===\n{}\n=======E======\n", seg);
    }
    assert!(chunks1.len() >= 2);
    let splitter2 =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 100, 20);
    let chunks2 = splitter2.split_text(text);
    for seg in &chunks2 {
        println!("====S===\n{}\n=======E======\n", seg);
    }
    assert!(chunks2.len() >= 1);
}

#[test]
fn test_recursive_splitter_empty_input_returns_empty() {
    let splitter =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 40, 10);
    let chunks = splitter.split_text("");
    assert!(chunks.is_empty());
}

#[test]
fn test_recursive_splitter_whitespace_only_input_returns_empty() {
    let splitter =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 40, 10);
    let chunks = splitter.split_text(" \n\t  \n");
    assert!(chunks.is_empty());
}

#[test]
fn test_recursive_splitter_character_fallback_respects_chunk_size() {
    let splitter =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 5, 2);
    let text = "abcdefghijklmnopqrstuvwxyz";
    let chunks = splitter.split_text(text);
    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|c| c.len() <= 5));
    assert_eq!(chunks[0], "abcde");

    for pair in chunks.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        assert_eq!(&left[left.len() - 2..], &right[..2]);
    }

    let mut rebuilt = chunks[0].clone();
    for chunk in chunks.iter().skip(1) {
        rebuilt.push_str(&chunk[2..]);
    }
    assert_eq!(rebuilt, text);
}

#[test]
fn test_recursive_splitter_custom_separator_splits_and_merges_deterministically() {
    let splitter = crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(
        Some(vec![",".to_string(), "".to_string()]),
        3,
        0,
    );
    let chunks = splitter.split_text("a,b,c,d");
    assert_eq!(
        chunks,
        vec!["a,".to_string(), "b,".to_string(), "c,d".to_string()]
    );
}

#[test]
fn test_recursive_splitter_preserves_token_order_in_output() {
    let splitter =
        crate::recursive_character_text_splitter::RecursiveCharacterTextSplitter::new(None, 9, 2);
    let chunks = splitter.split_text("alpha\n\nbeta\ngamma\ndelta");
    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|c| !c.trim().is_empty()));
    assert!(chunks.iter().all(|c| c.len() <= 9));

    let joined = chunks.concat();
    let tokens = ["alpha", "beta", "gamma", "delta"];
    let mut start = 0;
    for token in tokens {
        let pos = joined[start..]
            .find(token)
            .expect("token should appear in joined output in order");
        start += pos + token.len();
    }
}

#[test]
fn test_parse_str_empty_source_returns_empty_for_all_languages() {
    let langs = [
        CodeLanguage::C,
        CodeLanguage::Cpp,
        CodeLanguage::Python,
        CodeLanguage::JavaScript,
        CodeLanguage::Rust,
    ];
    for lang in langs {
        let segments = parse_str("", lang, 1).expect("parse_str should not fail on empty source");
        assert!(segments.is_empty());
    }
}

#[test]
fn test_parse_str_javascript_extracts_variable_function_and_class() {
    let source = r#"const version = 1;

function add(a, b) {
    return a + b;
}

class Car {
    drive() {
        return "go";
    }
}
"#;
    let segments = parse_str(source, CodeLanguage::JavaScript, 1)
        .expect("parse_str failed for JavaScript");
    let expected = vec![
        "const version = 1;".to_string(),
        r#"function add(a, b) {
    return a + b;
}"#
        .to_string(),
        r#"class Car {
    drive() {
        return "go";
    }
}"#
        .to_string(),
    ];
    assert_eq!(segments, expected);
}

#[test]
fn test_parse_str_skips_top_level_comment_nodes_for_rust() {
    let source = r#"// top-level comment
/* another comment */
static GLOBAL: i32 = 1;
// comment before function
fn run() {
    // inside-function comment stays part of function text
}
"#;
    let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed for Rust");
    assert_eq!(segments.len(), 2);
    assert!(segments.iter().any(|s| s.contains("static GLOBAL")));
    assert!(segments.iter().any(|s| s.contains("fn run")));
    assert!(!segments
        .iter()
        .any(|s| s.trim_start().starts_with("//") || s.trim_start().starts_with("/*")));
}

#[test]
fn test_parse_str_is_consistent_across_thread_num_values() {
    let source = r#"static GLOBAL_VAR: i32 = 42;

fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    let single_thread = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
    let multi_thread = parse_str(source, CodeLanguage::Rust, 8).expect("parse_str failed");
    assert_eq!(single_thread, multi_thread);
}
