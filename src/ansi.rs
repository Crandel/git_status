use crate::common::{OutputFormatter, ShellFormatter, Wrapper};
use crate::extractor::Extractor;

pub struct AnsiFormatter {
    pub shell: ShellFormatter,
}

impl AnsiFormatter {
    pub fn new() -> AnsiFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                // Cyan (256-color)
                start: String::from("\x1b[38;5;37m"),
                end: String::from("\x1b[0m"),
            },
            ahead: Wrapper {
                // Bright green
                start: String::from("\x1b[1;32m{>"),
                end: String::from("}\x1b[0m"),
            },
            behind: Wrapper {
                // Bright red
                start: String::from("\x1b[1;31m{<"),
                end: String::from("}\x1b[0m"),
            },
            unstaged: Wrapper {
                // Bright yellow
                start: String::from("|\x1b[1;33m"),
                end: String::from("\x1b[0m"),
            },
            staged: Wrapper {
                // Teal (256-color)
                start: String::from("|\x1b[38;5;66m"),
                end: String::from("\x1b[0m"),
            },
            modified_char: String::from("%"),
            deleted_char: String::from("-"),
            untracked_char: String::from("*"),
            renamed_char: String::from("^"),
            new_char: String::from("+"),
        };
        AnsiFormatter { shell }
    }
}

impl Default for AnsiFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputFormatter for AnsiFormatter {
    fn get_output(&self, extractor: &Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
