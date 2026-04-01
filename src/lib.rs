use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use std::fs;
use tree_sitter::Parser;
use tree_sitter_c::LANGUAGE as language_c;
use tree_sitter_cpp::LANGUAGE as language_cpp;
use tree_sitter_javascript::LANGUAGE as language_javascript;
use tree_sitter_python::LANGUAGE as language_python;
use tree_sitter_rust::LANGUAGE as language_rust;
use walkdir::WalkDir;

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

fn get_file_extension(lang: CodeLanguage) -> Result<&'static str> {
    match lang {
        CodeLanguage::C => Ok("c"),
        CodeLanguage::Cpp => Ok("cpp"),
        CodeLanguage::Python => Ok("py"),
        CodeLanguage::JavaScript | CodeLanguage::Js => Ok("js"),
        CodeLanguage::Rust => Ok("rs"),
        unsupported => Err(anyhow!(
            "Language {:?} is not supported by parse_dir file-extension mapping",
            unsupported
        )),
    }
}

fn is_c_cpp_preprocessor_kind(kind: &str) -> bool {
    matches!(
        kind,
        "preproc_include"
            | "preproc_def"
            | "preproc_function_def"
            | "preproc_call"
            | "preproc_if"
            | "preproc_ifdef"
            | "preproc_elif"
            | "preproc_elifdef"
            | "preproc_else"
    )
}

pub fn parse_str(source: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<String>> {
    let mut parser = Parser::new();
    let language = match lang {
        CodeLanguage::C => language_c.into(),
        CodeLanguage::Cpp => language_cpp.into(),
        CodeLanguage::Python => language_python.into(),
        CodeLanguage::JavaScript | CodeLanguage::Js => language_javascript.into(),
        CodeLanguage::Rust => language_rust.into(),
        unsupported => {
            return Err(anyhow!(
                "Language {:?} is not supported by parse_str",
                unsupported
            ));
        }
    };
    parser
        .set_language(&language)
        .map_err(|e| anyhow!("Error loading language: {}", e))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("Failed to parse source"))?;
    let root_node = tree.root_node();
    let num_threads = thread_num as usize;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads.max(1))
        .build()?;

    let mut segment_ranges: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < root_node.child_count() {
        if let Some(node) = root_node.child(i as u32) {
            let kind = node.kind();
            let should_keep = match lang {
                CodeLanguage::C => matches!(
                    kind,
                    "function_definition"
                        | "declaration"
                        | "struct_specifier"
                        | "class_specifier"
                        | "enum_specifier"
                        | "union_specifier"
                        | "type_definition"
                ) || is_c_cpp_preprocessor_kind(kind),
                CodeLanguage::Cpp => matches!(
                    kind,
                    "function_definition"
                        | "declaration"
                        | "struct_specifier"
                        | "class_specifier"
                        | "enum_specifier"
                        | "union_specifier"
                        | "type_definition"
                ) || is_c_cpp_preprocessor_kind(kind),
                CodeLanguage::Python => matches!(
                    kind,
                    "function_definition"
                        | "class_definition"
                        | "expression_statement"
                        | "assignment"
                        | "global_statement"
                ),
                CodeLanguage::JavaScript | CodeLanguage::Js => matches!(
                    kind,
                    "function_declaration"
                        | "class_declaration"
                        | "lexical_declaration"
                        | "variable_declaration"
                        | "expression_statement"
                ),
                CodeLanguage::Rust => kind.ends_with("_item") || kind.ends_with("_definition"),
                _ => false,
            };

            if should_keep
                && !kind.contains("comment")
                && kind != "translation_unit"
                && node.byte_range().len() > 0
            {
                let start = node.start_byte();
                let mut end = node.end_byte();
                let mut consumed_next_semicolon = false;

                let should_merge_trailing_semicolon = matches!(
                    kind,
                    "struct_specifier" | "class_specifier" | "enum_specifier" | "union_specifier"
                ) && matches!(lang, CodeLanguage::C | CodeLanguage::Cpp);

                if should_merge_trailing_semicolon {
                    if let Some(next_node) = root_node.child((i + 1) as u32) {
                        if next_node.kind() == ";" {
                            end = next_node.end_byte();
                            consumed_next_semicolon = true;
                        }
                    }
                }

                if end <= source.len() {
                    segment_ranges.push((start, end));
                }

                if consumed_next_semicolon {
                    i += 1;
                }
            }
        }
        i += 1;
    }

    let segments: Vec<String> = pool.install(|| {
        segment_ranges
            .par_iter()
            .filter_map(|(start, end)| {
                let text = source[*start..*end].to_string();
                if text.trim().is_empty() {
                    None
                } else {
                    Some(text)
                }
            })
            .collect()
    });

    Ok(segments)
}

pub fn parse_file(file_path: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    parse_str(&content, lang, thread_num)
}

pub fn parse_dir(dir_path: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<String>> {
    let extension = get_file_extension(lang)?;
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter() {
        let entry = entry
            .map_err(|e| anyhow!("Failed to walk directory '{}': {}", dir_path, e))?;
        if entry.path().extension().map_or(false, |ext| ext == extension) {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    let num_threads = thread_num as usize;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads.max(1))
        .build()?;

    let parsed_segments: Result<Vec<Vec<String>>> = pool.install(|| {
        files
            .par_iter()
            .map(|file| {
                parse_file(file, lang, thread_num)
                    .with_context(|| format!("Failed to parse file '{}'", file))
            })
            .collect()
    });

    Ok(parsed_segments?.into_iter().flatten().collect())
}

pub mod recursive_character_text_splitter;

#[cfg(test)]
mod tests;
