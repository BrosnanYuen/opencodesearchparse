use anyhow::{anyhow, Result};
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
    Python,
    JavaScript,
    Rust,
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

pub fn parse_str(source: &str, lang: CodeLanguage, _thread_num: u16) -> Result<Vec<String>> {
    let mut parser = Parser::new();
    let language = match lang {
        CodeLanguage::C => language_c.into(),
        CodeLanguage::Cpp => language_cpp.into(),
        CodeLanguage::Python => language_python.into(),
        CodeLanguage::JavaScript => language_javascript.into(),
        CodeLanguage::Rust => language_rust.into(),
    };
    parser
        .set_language(&language)
        .map_err(|e| anyhow!("Error loading language: {}", e))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow!("Failed to parse source"))?;
    let root_node = tree.root_node();
    let mut segments: Vec<String> = Vec::new();
    for i in 0..root_node.child_count() {
        if let Some(node) = root_node.child(i as u32) {
            let kind = node.kind();
            let should_keep = match lang {
                CodeLanguage::C => matches!(
                    kind,
                    "preproc_include"
                        | "preproc_def"
                        | "preproc_function_def"
                        | "function_definition"
                        | "declaration"
                        | "struct_specifier"
                        | "class_specifier"
                        | "enum_specifier"
                        | "union_specifier"
                        | "type_definition"
                        | "expression_statement"
                        | "ERROR"
                        | ";"
                ),
                CodeLanguage::Cpp => matches!(
                    kind,
                    "preproc_include"
                        | "preproc_def"
                        | "preproc_function_def"
                        | "function_definition"
                        | "declaration"
                        | "struct_specifier"
                        | "class_specifier"
                        | "enum_specifier"
                        | "union_specifier"
                        | "type_definition"
                        | "expression_statement"
                        | "ERROR"
                ),
                CodeLanguage::Python => matches!(
                    kind,
                    "function_definition"
                        | "class_definition"
                        | "expression_statement"
                        | "assignment"
                        | "global_statement"
                ),
                CodeLanguage::JavaScript => matches!(
                    kind,
                    "function_declaration"
                        | "class_declaration"
                        | "lexical_declaration"
                        | "variable_declaration"
                        | "expression_statement"
                ),
                CodeLanguage::Rust => kind.ends_with("_item") || kind.ends_with("_definition"),
            };
            if should_keep
                && !kind.contains("comment")
                && kind != "translation_unit"
                && node.byte_range().len() > 0
            {
                let start = node.start_byte();
                let end = node.end_byte();
                if end <= source.len() {
                    let text = source[start..end].to_string();
                    if !text.trim().is_empty() {
                        segments.push(text);
                    }
                }
            }
        }
    }
    Ok(segments)
}

pub fn parse_file(file_path: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    parse_str(&content, lang, thread_num)
}

pub fn parse_dir(dir_path: &str, lang: CodeLanguage, thread_num: u16) -> Result<Vec<String>> {
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

    let mut all_segments: Vec<String> = Vec::new();
    pool.install(|| {
        let segments: Vec<Vec<String>> = files
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
mod tests;
