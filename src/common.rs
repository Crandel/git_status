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

#[cfg(test)]
mod tests {
    use crate::ansi::AnsiFormatter;
    use crate::common::OutputFormatter;
    use crate::extractor::Extractor;

    fn fmt(status: &str) -> String {
        AnsiFormatter::new().get_output(&Extractor::new(status))
    }

    #[test]
    fn clean_branch_only() {
        let out = fmt("## main\n");
        assert!(out.contains("main"));
        assert!(!out.contains('|'));
    }

    #[test]
    fn ahead_appears() {
        let out = fmt("## main...origin/main [ahead 3]\n");
        assert!(out.contains("{>3}"));
        assert!(!out.contains("{<"));
    }

    #[test]
    fn behind_appears() {
        let out = fmt("## main...origin/main [behind 5]\n");
        assert!(out.contains("{<5}"));
        assert!(!out.contains("{>"));
    }

    #[test]
    fn unstaged_section_present() {
        let out = fmt("## main\n M file.txt\n");
        assert!(out.contains("|\x1b[1;33m"));
        assert!(out.contains("%1"));
    }

    #[test]
    fn untracked_section_present() {
        let out = fmt("## main\n?? file.txt\n");
        assert!(out.contains("|\x1b[0;31m"));
        assert!(out.contains("*1"));
    }

    #[test]
    fn staged_section_present() {
        let out = fmt("## main\nA  file.txt\n");
        assert!(out.contains("|\x1b[38;5;66m"));
        assert!(out.contains("+1"));
    }

    #[test]
    fn empty_sections_omitted() {
        let out = fmt("## main\n");
        assert!(!out.contains('|'));
    }

    #[test]
    fn unstaged_and_untracked_are_separate_sections() {
        let out = fmt("## main\n M file\n?? other\n");
        assert!(out.contains("|\x1b[1;33m"));
        assert!(out.contains("|\x1b[0;31m"));
        assert!(out.contains("%1"));
        assert!(out.contains("*1"));
    }

    #[test]
    fn section_order() {
        let status = "## main...origin/main [ahead 1][behind 2]\n M f1\n?? f2\nA  f3\n";
        let out = fmt(status);
        let pos_ahead    = out.find("{>1}").unwrap();
        let pos_behind   = out.find("{<2}").unwrap();
        let pos_unstaged = out.find("|\x1b[1;33m").unwrap();
        let pos_untrack  = out.find("|\x1b[0;31m").unwrap();
        let pos_staged   = out.find("|\x1b[38;5;66m").unwrap();
        assert!(pos_ahead < pos_behind);
        assert!(pos_behind < pos_unstaged);
        assert!(pos_unstaged < pos_untrack);
        assert!(pos_untrack < pos_staged);
    }
}
