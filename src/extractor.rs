use regex::Regex;
use std::string::String;

#[derive(Debug)]
pub struct Extractor {
    pub branch: String,
    pub ahead: String,
    pub behind: String,
    pub modified_unstaged: u16,
    pub deleted_unstaged: u16,
    pub untracked_unstaged: u16,
    pub modified_staged: u16,
    pub deleted_staged: u16,
    pub renamed_staged: u16,
    pub new_staged: u16,
}

impl Extractor {
    // Account constructor
    pub fn new(status: &str) -> Extractor {
        let mut extractor = Extractor {
            branch: String::from(""),
            ahead: String::from(""),
            behind: String::from(""),
            modified_unstaged: 0,
            deleted_unstaged: 0,
            untracked_unstaged: 0,
            modified_staged: 0,
            deleted_staged: 0,
            renamed_staged: 0,
            new_staged: 0,
        };

        let number = Regex::new("[0-9]+").unwrap();
        let ahead_reg = Regex::new("ahead [0-9]+*").unwrap();
        let behind = Regex::new("behind [0-9]+*").unwrap();

        let vec_strings = status.split('\n').collect::<Vec<&str>>();
        // First line give us a repo name and relation to remote server
        let input = match vec_strings.get(0) {
            Some(valid_str) => valid_str,
            _ => "",
        };

        // get ahead count
        let ahead = match ahead_reg.find(input) {
            Some(ahead_str) => {
                let ahead_str = &input[ahead_str.start()..ahead_str.end()];
                match number.find(ahead_str) {
                    Some(res) => &ahead_str[res.start()..res.end()],
                    _ => "",
                }
            }
            _ => "",
        };
        extractor.ahead = String::from(ahead);

        // get behind count
        let behind = match behind.find(input) {
            Some(behind_str) => {
                let behind_str = &input[behind_str.start()..behind_str.end()];
                match number.find(behind_str) {
                    Some(res) => &behind_str[res.start()..res.end()],
                    _ => "",
                }
            }
            _ => "",
        };
        extractor.behind = String::from(behind);

        // ## rrr-43...origin/rrr-43 [ahead 1][behind 3]
        // we should ignore first 3 symbols and split by "..." to get the branch
        let branch_vec = &input[3..input.len()].split("...").collect::<Vec<&str>>();
        let branch = match branch_vec.get(0) {
            Some(local_name) => local_name,
            _ => "",
        };
        extractor.branch = String::from(branch);

        for item in vec_strings.iter().skip(1) {
            let current_str = item.chars().collect::<Vec<char>>();
            if current_str.len() > 2 {
                let staged_ch = current_str[0];
                let unstaged_ch = current_str[1];
                match unstaged_ch {
                    'M' => extractor.modified_unstaged += 1,
                    'm' => extractor.modified_unstaged += 1,
                    'D' => extractor.deleted_unstaged += 1,
                    'd' => extractor.deleted_unstaged += 1,
                    '?' => extractor.untracked_unstaged += 1,
                    _ => (),
                };
                match staged_ch {
                    'M' => extractor.modified_staged += 1,
                    'm' => extractor.modified_staged += 1,
                    'D' => extractor.deleted_staged += 1,
                    'd' => extractor.deleted_staged += 1,
                    'R' => extractor.renamed_staged += 1,
                    'r' => extractor.renamed_staged += 1,
                    'A' => extractor.new_staged += 1,
                    'a' => extractor.new_staged += 1,
                    _ => (),
                };
            }
        }
        extractor
    }

    pub fn get_unstaged(&self, modified: &str, deleted: &str, untracked: &str) -> String {
        let mut unstaged_counts = String::from("");

        if self.modified_unstaged > 0 {
            unstaged_counts.push_str(&format!("{}{}", modified, self.modified_unstaged))
        }
        if self.deleted_unstaged > 0 {
            unstaged_counts.push_str(&format!("{}{}", deleted, self.deleted_unstaged))
        }
        if self.untracked_unstaged > 0 {
            unstaged_counts.push_str(&format!("{}{}", untracked, self.untracked_unstaged))
        }
        unstaged_counts
    }

    pub fn get_staged(&self, modified: &str, deleted: &str, renamed: &str, new: &str) -> String {
        let mut staged_counts = String::from("");

        if self.modified_staged > 0 {
            staged_counts.push_str(&format!("{}{}", modified, self.modified_staged))
        }
        if self.deleted_staged > 0 {
            staged_counts.push_str(&format!("{}{}", deleted, self.deleted_staged))
        }
        if self.renamed_staged > 0 {
            staged_counts.push_str(&format!("{}{}", renamed, self.renamed_staged))
        }
        if self.new_staged > 0 {
            staged_counts.push_str(&format!("{}{}", new, self.new_staged))
        }
        staged_counts
    }
}
