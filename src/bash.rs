use crate::common::{OutputFormatter, ShellFormatter, Wrapper};
use crate::extractor::Extractor;

pub struct BashFormatter {
    pub shell: ShellFormatter,
}

impl BashFormatter {
    pub fn new() -> BashFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                // Cyan (256-color)
                start: String::from("\\[\\033[38;5;37m\\]"),
                end: String::from("\\[\\033[0m\\]"),
            },
            ahead: Wrapper {
                // Bright green
                start: String::from("\\[\\033[1;32m\\]{>"),
                end: String::from("}\\[\\033[0m\\]"),
            },
            behind: Wrapper {
                // Bright red
                start: String::from("\\[\\033[1;31m\\]{<"),
                end: String::from("}\\[\\033[0m\\]"),
            },
            unstaged: Wrapper {
                // Bright yellow
                start: String::from("|\\[\\033[1;33m\\]"),
                end: String::from("\\[\\033[0m\\]"),
            },
            untracked: Wrapper {
                // Red
                start: String::from("|\\[\\033[0;31m\\]"),
                end: String::from("\\[\\033[0m\\]"),
            },
            staged: Wrapper {
                // Teal (256-color)
                start: String::from("|\\[\\033[38;5;66m\\]"),
                end: String::from("\\[\\033[0m\\]"),
            },
            modified_char: String::from("%"),
            deleted_char: String::from("-"),
            untracked_char: String::from("*"),
            renamed_char: String::from("^"),
            new_char: String::from("+"),
        };
        BashFormatter { shell }
    }
}

impl Default for BashFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for BashFormatter {
    fn get_output(&self, extractor: &Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
