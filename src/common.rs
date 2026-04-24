use crate::extractor::Extractor;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Wrapper {
    pub start: String,
    pub end: String,
}

#[derive(Copy, Clone)]
pub struct Chars {
    pub copied_char: &'static str,
    pub deleted_char: &'static str,
    pub delimeter: &'static str,
    pub modified_char: &'static str,
    pub new_char: &'static str,
    pub renamed_char: &'static str,
    pub type_changed_char: &'static str,
    pub unmerged_char: &'static str,
    pub untracked_char: &'static str,
}

impl Chars {
    pub fn new() -> Chars {
        Chars {
            delimeter: " ",
            modified_char: "%",
            deleted_char: "-",
            type_changed_char: "~",
            untracked_char: "*",
            renamed_char: "^",
            new_char: "+",
            copied_char: "=",
            unmerged_char: "!",
        }
    }
}

pub struct ShellFormatter {
    pub branch: Wrapper,
    pub ahead: Wrapper,
    pub behind: Wrapper,
    pub unmerged: Wrapper,
    pub untracked: Wrapper,
    pub unstaged: Wrapper,
    pub staged: Wrapper,
    pub chars: Chars,
}

impl ShellFormatter {
    pub fn format_output(&self, extractor: &Extractor) -> String {
        let mut out = String::new();
        let unmerged_counts = extractor.get_unmerged(&self.chars.unmerged_char);
        let untracked_counts = extractor.get_untracked(&self.chars.untracked_char);
        let unstaged_counts = extractor.get_unstaged(
            &self.chars.modified_char,
            &self.chars.deleted_char,
            &self.chars.type_changed_char,
        );
        let staged_counts = extractor.get_staged(
            &self.chars.modified_char,
            &self.chars.deleted_char,
            &self.chars.renamed_char,
            &self.chars.new_char,
            &self.chars.type_changed_char,
            &self.chars.copied_char,
        );

        write!(
            out,
            "{}{}{}",
            self.branch.start, extractor.branch, self.branch.end
        )
        .expect("Error");
        if !staged_counts.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.staged.start, staged_counts, self.staged.end
            )
            .expect("Error");
        }
        if !unstaged_counts.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.unstaged.start, unstaged_counts, self.unstaged.end
            )
            .expect("Error");
        }
        if !unmerged_counts.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.unmerged.start, unmerged_counts, self.unmerged.end
            )
            .expect("Error");
        }
        if !untracked_counts.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.untracked.start, untracked_counts, self.untracked.end
            )
            .expect("Error");
        }
        if !extractor.ahead.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.ahead.start, extractor.ahead, self.ahead.end
            )
            .expect("Error");
        }
        if !extractor.behind.is_empty() {
            write!(
                out,
                "{}{}{}{}",
                self.chars.delimeter, self.behind.start, extractor.behind, self.behind.end
            )
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
    use crate::common::{Chars, OutputFormatter};
    use crate::extractor::Extractor;

    fn fmt(status: &str) -> String {
        let chars: Chars = Chars::new();
        AnsiFormatter::new(chars).get_output(&Extractor::new(status))
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
        assert!(out.contains("\x1b[1;33m"));
        assert!(out.contains("%1"));
    }

    #[test]
    fn untracked_section_present() {
        let out = fmt("## main\n?? file.txt\n");
        assert!(out.contains("\x1b[0;31m"));
        assert!(out.contains("*1"));
    }

    #[test]
    fn staged_section_present() {
        let out = fmt("## main\nA  file.txt\n");
        assert!(out.contains("\x1b[38;5;66m"));
        assert!(out.contains("+1"));
    }

    #[test]
    fn unmerged_section_present() {
        let out = fmt("## main\nUU file.txt\n");
        assert!(out.contains("\x1b[0;35m"));
        assert!(out.contains("!1"));
    }

    #[test]
    fn empty_sections_omitted() {
        let out = fmt("## main\n");
        assert!(!out.contains('|'));
    }

    #[test]
    fn unstaged_and_untracked_are_separate_sections() {
        let out = fmt("## main\n M file\n?? other\n");
        assert!(out.contains("\x1b[1;33m"));
        assert!(out.contains("\x1b[0;31m"));
        assert!(out.contains("%1"));
        assert!(out.contains("*1"));
    }

    #[test]
    fn section_order() {
        let status =
            "## main...origin/main [ahead 1][behind 2]\nUU conflict\n M f1\n?? f2\nA  f3\n";
        let out = fmt(status);
        let pos_staged = out.find("\x1b[38;5;66m").unwrap();
        let pos_unstaged = out.find("\x1b[1;33m").unwrap();
        let pos_unmerged = out.find("\x1b[0;35m").unwrap();
        let pos_untrack = out.find("\x1b[0;31m").unwrap();
        let pos_ahead = out.find("{>1}").unwrap();
        let pos_behind = out.find("{<2}").unwrap();
        assert!(pos_staged < pos_unstaged);
        assert!(pos_unstaged < pos_unmerged);
        assert!(pos_unmerged < pos_untrack);
        assert!(pos_untrack < pos_ahead);
        assert!(pos_ahead < pos_behind);
    }
}
