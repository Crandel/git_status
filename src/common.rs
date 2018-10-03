use extractor::Extractor;
use std::fmt::write;

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
    pub staged: Wrapper,
    pub modified_char: String,
    pub deleted_char: String,
    pub untracked_char: String,
    pub renamed_char: String,
    pub new_char: String,
}

impl ShellFormatter {
    pub fn format_output(&self, extractor: Extractor) -> String {
        let mut branch_final = String::from("");
        let unstaged_counts = extractor.get_unstaged(
            self.modified_char.clone(),
            self.deleted_char.clone(),
            self.untracked_char.clone(),
        );
        let staged_counts = extractor.get_staged(
            self.modified_char.clone(),
            self.deleted_char.clone(),
            self.renamed_char.clone(),
            self.new_char.clone(),
        );

        write(
            &mut branch_final,
            format_args!(
                "{}{}{}",
                self.branch.start.to_owned(),
                extractor.branch.to_owned(),
                self.branch.end.to_owned()
            ),
        ).expect("Error");
        if extractor.ahead.len() > 0 {
            write(
                &mut branch_final,
                format_args!(
                    "{}{}{}",
                    self.ahead.start.to_owned(),
                    extractor.ahead,
                    self.ahead.end.to_owned()
                ),
            ).expect("Error");
        }
        if extractor.behind.len() > 0 {
            write(
                &mut branch_final,
                format_args!(
                    "{}{}{}",
                    self.behind.start.to_owned(),
                    extractor.behind,
                    self.behind.end.to_owned()
                ),
            ).expect("Error");
        }
        if unstaged_counts.len() > 0 {
            write(
                &mut branch_final,
                format_args!(
                    "{}{}{}",
                    self.unstaged.start.to_owned(),
                    unstaged_counts.to_owned(),
                    self.unstaged.end.to_owned()
                ),
            ).expect("Error");
        }
        if staged_counts.len() > 0 {
            write(
                &mut branch_final,
                format_args!(
                    "{}{}{}",
                    self.staged.start.to_owned(),
                    staged_counts.to_owned(),
                    self.staged.end.to_owned()
                ),
            ).expect("Error");
        }
        branch_final
    }
}

pub trait OutputFormatter {
    fn get_output(&self, extractor: Extractor) -> String;
}
