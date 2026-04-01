# Open Code Search Parser

`opencodesearchparser` is a Rust library for parsing source files into top-level code segments using Tree-sitter.

## Public API

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodeLanguage {
    C,
    Cpp,
    Go,
    Java,
    Kotlin,
    Python,
    JavaScript,
    Js,
    Ts,
    Php,
    Proto,
    R,
    Rust,
}

pub fn parse_str(source: &str, lang: CodeLanguage, thread_num: u16) -> anyhow::Result<Vec<String>>;
pub fn parse_file(file_path: &str, lang: CodeLanguage, thread_num: u16) -> anyhow::Result<Vec<String>>;
pub fn parse_dir(dir_path: &str, lang: CodeLanguage, thread_num: u16) -> anyhow::Result<Vec<String>>;

pub mod recursive_character_text_splitter;
```

`thread_num == 0` is treated as `1` thread internally.

## Current Language Support

| Area | Supported now |
|---|---|
| `parse_str` / `parse_file` parsing | `C`, `Cpp`, `Python`, `JavaScript`, `Js`, `Rust` |
| `parse_dir` extension filtering | `C` (`.c`), `Cpp` (`.cpp`), `Python` (`.py`), `JavaScript`/`Js` (`.js`), `Rust` (`.rs`) |
| Other `CodeLanguage` variants | Present in the enum, but currently return an error in parsing and/or directory mapping |

## Segmentation Behavior (Current)

- C/C++: keeps top-level functions, declarations, struct/class/enum/union/type definitions, and preprocessor nodes (`include`, `define`, macro function define, conditional directives, and preprocessor calls like `#pragma`).
- C/C++ struct/class/enum/union declarations are emitted with trailing `;` when it is a separate sibling node.
- Python: keeps top-level function definitions, class definitions, expression statements, assignments, and global statements.
- JavaScript/Js: keeps top-level function declarations, class declarations, lexical/variable declarations, and expression statements.
- Rust: keeps top-level nodes whose kinds end with `_item` or `_definition`.
- Top-level comment nodes and empty/whitespace-only segments are skipped.

## Parallelism

- `parse_str` uses a Rayon thread pool (`thread_num`) for segment materialization.
- `parse_file` reads one file, then calls `parse_str` with the same `thread_num`.
- `parse_dir` walks directories with `walkdir`, filters by extension, then parses matching files in parallel with Rayon.

## RecursiveCharacterTextSplitter

`recursive_character_text_splitter::RecursiveCharacterTextSplitter` provides recursive chunking with configurable separators, chunk size, and overlap.

Key constructors:

```rust
pub fn new(separators: Option<Vec<String>>, chunk_size: usize, chunk_overlap: usize) -> Self;
pub fn from_language(language: CodeLanguage) -> Self;
pub fn split_text(&self, text: &str) -> Vec<String>;
```

## Usage Examples

### Parse a string

```rust
use anyhow::Result;
use opencodesearchparser::{parse_str, CodeLanguage};

fn main() -> Result<()> {
    let source = r#"
static GLOBAL_VAR: i32 = 42;
fn add(a: i32, b: i32) -> i32 { a + b }
"#;

    let segments = parse_str(source, CodeLanguage::Rust, 4)?;
    println!("segments: {}", segments.len());
    Ok(())
}
```

### Parse a file

```rust
use anyhow::Result;
use opencodesearchparser::{parse_file, CodeLanguage};

fn main() -> Result<()> {
    let segments = parse_file("tests/data/rust/example.rs", CodeLanguage::Rust, 2)?;
    println!("segments: {}", segments.len());
    Ok(())
}
```

### Parse a directory

```rust
use anyhow::Result;
use opencodesearchparser::{parse_dir, CodeLanguage};

fn main() -> Result<()> {
    let segments = parse_dir("tests/data", CodeLanguage::Rust, 8)?;
    println!("segments: {}", segments.len());
    Ok(())
}
```

### Use the recursive splitter

```rust
use opencodesearchparser::recursive_character_text_splitter::RecursiveCharacterTextSplitter;
use opencodesearchparser::CodeLanguage;

let splitter = RecursiveCharacterTextSplitter::from_language(CodeLanguage::Rust);
let chunks = splitter.split_text("fn a() {}\n\nfn b() {}");
assert!(!chunks.is_empty());
```
