use common::{OutputFormatter, ShellFormatter, Wrapper};
use extractor::Extractor;

pub struct BashFormatter {
    pub shell: ShellFormatter,
}

impl BashFormatter {
    pub fn new() -> BashFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                start: String::from(""),
                end: String::from("\\[\\e[0m\\]"),
            },
            ahead: Wrapper {
                start: String::from("\\[\\033[1;32m\\]{>"),
                end: String::from("}\\[\\e[0m\\]"),
            },
            behind: Wrapper {
                start: String::from("\\[\\033[1;31m\\]{<"),
                end: String::from("}\\[\\e[0m\\]"),
            },
            unstaged: Wrapper {
                start: String::from("|\\[\\033[1;33m\\]"),
                end: String::from("\\[\\e[0m\\]"),
            },
            staged: Wrapper {
                start: String::from("|\\[\\033[0;32m\\]"),
                end: String::from("\\[\\e[0m\\]"),
            },
            modified_char: String::from("%"),
            deleted_char: String::from("-"),
            untracked_char: String::from("*"),
            renamed_char: String::from("^"),
            new_char: String::from("+"),
        };
        BashFormatter { shell: shell }
    }
}

impl OutputFormatter for BashFormatter {
    fn get_output(&self, extractor: Extractor) -> String {
        self.shell.format_output(extractor)
    }
}
