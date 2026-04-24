use crate::extractor::Extractor;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Wrapper {
    pub start: String,
    pub end: String,
}

pub struct ShellFormatter {
    pub branch: Wrapper,
    pub ahead: Wrapper,
    pub behind: Wrapper,
    pub unstaged: Wrapper,
    pub untracked: Wrapper,
    pub staged: Wrapper,
    pub modified_char: String,
    pub deleted_char: String,
    pub untracked_char: String,
    pub renamed_char: String,
    pub new_char: String,
}

impl ShellFormatter {
    pub fn format_output(&self, extractor: &Extractor) -> String {
        let mut out = String::new();
        let unstaged_counts = extractor.get_unstaged(
            &self.modified_char,
            &self.deleted_char,
        );
        let untracked_counts = extractor.get_untracked(&self.untracked_char);
        let staged_counts = extractor.get_staged(
            &self.modified_char,
            &self.deleted_char,
            &self.renamed_char,
            &self.new_char,
        );

        write!(out, "{}{}{}", self.branch.start, extractor.branch, self.branch.end)
            .expect("Error");
        if !extractor.ahead.is_empty() {
            write!(out, "{}{}{}", self.ahead.start, extractor.ahead, self.ahead.end)
                .expect("Error");
        }
        if !extractor.behind.is_empty() {
            write!(out, "{}{}{}", self.behind.start, extractor.behind, self.behind.end)
                .expect("Error");
        }
        if !unstaged_counts.is_empty() {
            write!(out, "{}{}{}", self.unstaged.start, unstaged_counts, self.unstaged.end)
                .expect("Error");
        }
        if !untracked_counts.is_empty() {
            write!(out, "{}{}{}", self.untracked.start, untracked_counts, self.untracked.end)
                .expect("Error");
        }
        if !staged_counts.is_empty() {
            write!(out, "{}{}{}", self.staged.start, staged_counts, self.staged.end)
                .expect("Error");
        }
        out
    }
}

pub trait OutputFormatter {
    fn get_output(&self, extractor: &Extractor) -> String;
}
