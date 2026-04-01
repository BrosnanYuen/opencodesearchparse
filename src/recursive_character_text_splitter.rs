use crate::CodeLanguage;

#[derive(Debug)]
pub struct RecursiveCharacterTextSplitter {
    separators: Vec<String>,
    chunk_size: usize,
    chunk_overlap: usize,
    keep_separator: bool,
    _is_separator_regex: bool,
}

impl RecursiveCharacterTextSplitter {
    pub fn new(separators: Option<Vec<String>>, chunk_size: usize, chunk_overlap: usize) -> Self {
        let separators = separators.unwrap_or_else(|| {
            vec![
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ]
        });
        Self {
            separators,
            chunk_size: chunk_size.max(1),
            chunk_overlap: chunk_overlap.min(chunk_size / 2),
            keep_separator: true,
            _is_separator_regex: false,
        }
    }

    pub fn split_text(&self, text: &str) -> Vec<String> {
        self._split_text(text, &self.separators)
    }

    fn split_with_separator(&self, text: &str, separator: &str) -> Vec<String> {
        if separator.is_empty() {
            return text.chars().map(|c| c.to_string()).collect();
        }

        if !self.keep_separator {
            return text.split(separator).map(|s| s.to_string()).collect();
        }

        let mut splits: Vec<String> = Vec::new();
        let mut start = 0;

        for (idx, _) in text.match_indices(separator) {
            let end = idx + separator.len();
            if end > start {
                splits.push(text[start..end].to_string());
            }
            start = end;
        }

        if start < text.len() {
            splits.push(text[start..].to_string());
        }

        splits
    }

    fn _split_text(&self, text: &str, separators: &[String]) -> Vec<String> {
        if text.is_empty() {
            return vec![];
        }
        let mut final_chunks = vec![];
        let mut separator = separators.last().cloned().unwrap_or_default();
        let mut new_separators = vec![];
        for (i, s) in separators.iter().enumerate() {
            if s.is_empty() {
                separator = s.clone();
                break;
            }
            if text.contains(s) {
                separator = s.clone();
                new_separators = separators[i + 1..].to_vec();
                break;
            }
        }
        let splits = self.split_with_separator(text, &separator);
        let mut good_splits = vec![];
        let sep = if self.keep_separator { "" } else { &separator };
        for s in splits {
            if s.trim().is_empty() {
                continue;
            }
            if s.len() < self.chunk_size {
                good_splits.push(s);
            } else {
                if !good_splits.is_empty() {
                    let merged = self.merge_splits(&good_splits, sep);
                    final_chunks.extend(merged);
                    good_splits.clear();
                }
                if new_separators.is_empty() {
                    final_chunks.push(s);
                } else {
                    let other = self._split_text(&s, &new_separators);
                    final_chunks.extend(other);
                }
            }
        }
        if !good_splits.is_empty() {
            let merged = self.merge_splits(&good_splits, sep);
            final_chunks.extend(merged);
        }
        final_chunks
    }

    fn merge_splits(&self, splits: &[String], separator: &str) -> Vec<String> {
        let mut merged = vec![];
        let mut current_doc: Vec<String> = Vec::new();
        let mut current_len: usize = 0;
        let separator_len = separator.len();

        for split in splits {
            let split_len = split.len();
            let additional_sep_len = if current_doc.is_empty() { 0 } else { separator_len };
            let next_len = current_len + additional_sep_len + split_len;

            if next_len > self.chunk_size && !current_doc.is_empty() {
                merged.push(current_doc.join(separator));

                while !current_doc.is_empty()
                    && (current_len > self.chunk_overlap
                        || (current_len + additional_sep_len + split_len > self.chunk_size))
                {
                    let removed_len = current_doc[0].len();
                    current_doc.remove(0);
                    current_len = current_len.saturating_sub(removed_len);
                    if !current_doc.is_empty() {
                        current_len = current_len.saturating_sub(separator_len);
                    }
                }
            }

            if !current_doc.is_empty() {
                current_len += separator_len;
            }
            current_doc.push(split.clone());
            current_len += split_len;
        }

        if !current_doc.is_empty() {
            merged.push(current_doc.join(separator));
        }

        merged
    }

    pub fn from_language(language: CodeLanguage) -> Self {
        let separators = Self::get_separators_for_language(language);
        Self::new(Some(separators), 1000, 200)
    }

    fn get_separators_for_language(language: CodeLanguage) -> Vec<String> {
        match language {
            CodeLanguage::Rust => vec![
                "\nfn ".to_string(),
                "\nconst ".to_string(),
                "\nlet ".to_string(),
                "\nif ".to_string(),
                "\nwhile ".to_string(),
                "\nfor ".to_string(),
                "\nloop ".to_string(),
                "\nmatch ".to_string(),
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ],
            CodeLanguage::C | CodeLanguage::Cpp => vec![
                "\nclass ".to_string(),
                "\nvoid ".to_string(),
                "\nint ".to_string(),
                "\nif ".to_string(),
                "\nfor ".to_string(),
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ],
            CodeLanguage::Python => vec![
                "\nclass ".to_string(),
                "\ndef ".to_string(),
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ],
            _ => vec![
                "\n\n".to_string(),
                "\n".to_string(),
                " ".to_string(),
                "".to_string(),
            ],
        }
    }
}
