use crate::common::{Chars, OutputFormatter, ShellFormatter, Wrapper};
use crate::extractor::Extractor;

pub struct ZshFormatter {
    pub shell: ShellFormatter,
}

impl ZshFormatter {
    pub fn new(chars: Chars) -> ZshFormatter {
        let mut zsh_chars: Chars = chars;
        zsh_chars.modified_char = "%%";
        let shell = ShellFormatter {
            branch: Wrapper {
                start: String::from("%F{green}"),
                end: String::from("%f"),
            },
            ahead: Wrapper {
                start: String::from("%F{cyan}{>"),
                end: String::from("}%f"),
            },
            behind: Wrapper {
                start: String::from("%F{red}{<"),
                end: String::from("}%f"),
            },
            unmerged: Wrapper {
                // Magenta
                start: String::from("%F{magenta}"),
                end: String::from("%f"),
            },
            untracked: Wrapper {
                // Red
                start: String::from("%F{red}"),
                end: String::from("%f"),
            },
            unstaged: Wrapper {
                // Bright yellow
                start: String::from("%F{yellow}"),
                end: String::from("%f"),
            },
            staged: Wrapper {
                start: String::from("%F{teal}"),
                end: String::from("%f"),
            },
            chars: zsh_chars,
        };
        ZshFormatter { shell }
    }
}

impl OutputFormatter for ZshFormatter {
    fn get_output(&self, extractor: &Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
