use crate::common::{OutputFormatter, ShellFormatter, Wrapper};
use crate::extractor::Extractor;

pub struct ZshFormatter {
    pub shell: ShellFormatter,
}

impl ZshFormatter {
    pub fn new() -> ZshFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                start: String::from("%F{cyan}"),
                end: String::from("%f"),
            },
            ahead: Wrapper {
                start: String::from("%F{green}{>"),
                end: String::from("}%f"),
            },
            behind: Wrapper {
                start: String::from("%F{red}{<"),
                end: String::from("}%f"),
            },
            unstaged: Wrapper {
                start: String::from("|%F{yellow}"),
                end: String::from("%f"),
            },
            staged: Wrapper {
                start: String::from("|%F{green}"),
                end: String::from("%f"),
            },
            modified_char: String::from("%%"),
            deleted_char: String::from("-"),
            untracked_char: String::from("*"),
            renamed_char: String::from("^"),
            new_char: String::from("+"),
        };
        ZshFormatter { shell: shell }
    }
}

impl OutputFormatter for ZshFormatter {
    fn get_output(&self, extractor: Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
