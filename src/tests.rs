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
include <stdio.h>

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
    assert!(segments.len() == 6);
    assert!(segments.iter().any(|s| s.contains("include <stdio.h>")));
    assert!(segments.iter().any(|s| s.contains("global_var")));
    assert!(segments.iter().any(|s| s.contains("struct Point")));
    assert!(segments.iter().any(|s| s.contains("int add")));
    assert!(segments.iter().any(|s| s.contains("print_hello")));
    assert!(segments.iter().any(|s| s.contains("#define MAX")));
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
    assert!(segments[0].contains("#include <iostream>"));
    assert!(segments[1].contains("globalVar"));
    assert!(segments[2].contains("class Car"));
    assert!(segments[3].contains("int multiply"));
    assert!(segments[4].contains("#define LONG_STRING"));
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
    assert_eq!(segments.len(), 4);
    assert!(segments[0].contains("global_var"));
    assert!(segments[1].contains("def add"));
    assert!(segments[2].contains("class Car"));
    assert!(segments[3].contains("PI = 3.14"));
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
    assert!(segments[0].contains("GLOBAL_VAR"));
    assert!(segments[1].contains("fn add"));
    assert!(segments[2].contains("struct Point"));
    assert!(segments[3].contains("impl Rectangle"));
    assert!(segments[4].contains("PI"));
}

#[test]
fn test_parse_str_functions() {
    let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn sub(a: i32, b: i32) -> i32 {\n    a - b\n}";
    let segments = parse_str(source, CodeLanguage::Rust, 1).expect("parse_str failed");
    assert!(segments.len() >= 2);
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
                println!("====S===\n{}\n=======E======\n",seg);
            }
        assert!(!segments.is_empty());
    }
}
