use crate::common::{Chars, OutputFormatter, ShellFormatter, Wrapper};
use crate::extractor::Extractor;

pub struct AnsiFormatter {
    pub shell: ShellFormatter,
}

impl AnsiFormatter {
    pub fn new(chars: Chars) -> AnsiFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                // Bright green
                start: String::from("\x1b[1;32m"),
                end: String::from("\x1b[0m"),
            },
            ahead: Wrapper {
                // Cyan (256-color)
                start: String::from("\x1b[38;5;37m{>"),
                end: String::from("}\x1b[0m"),
            },
            behind: Wrapper {
                // Bright red
                start: String::from("\x1b[1;31m{<"),
                end: String::from("}\x1b[0m"),
            },
            unmerged: Wrapper {
                // Magenta
                start: String::from("\x1b[0;35m"),
                end: String::from("\x1b[0m"),
            },
            untracked: Wrapper {
                // Red
                start: String::from("\x1b[0;31m"),
                end: String::from("\x1b[0m"),
            },
            unstaged: Wrapper {
                // Bright yellow
                start: String::from("\x1b[1;33m"),
                end: String::from("\x1b[0m"),
            },
            staged: Wrapper {
                // Teal (256-color)
                start: String::from("\x1b[38;5;66m"),
                end: String::from("\x1b[0m"),
            },
            chars: chars,
        };
        AnsiFormatter { shell }
    }
}

impl OutputFormatter for AnsiFormatter {
    fn get_output(&self, extractor: &Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
