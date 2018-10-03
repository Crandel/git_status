use common::{OutputFormatter, ShellFormatter, Wrapper};
use extractor::Extractor;

pub struct BashFormatter {
    pub shell: ShellFormatter,
}

impl BashFormatter {
    pub fn new() -> BashFormatter {
        let shell = ShellFormatter {
            branch: Wrapper {
                start: String::from("${CYAN}"),
                end: String::from("$NORMAL"),
            },
            ahead: Wrapper {
                start: String::from("${LIGHT_GREEN}{>"),
                end: String::from("}$NORMAL"),
            },
            behind: Wrapper {
                start: String::from("${LIGHT_RED}{<"),
                end: String::from("}$NORMAL"),
            },
            unstaged: Wrapper {
                start: String::from("|${YELLOW}"),
                end: String::from("$NORMAL"),
            },
            staged: Wrapper {
                start: String::from("|${GREEN}"),
                end: String::from("$NORMAL"),
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
