use regex::Regex;

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
    pub fn new(status: &str) -> Extractor {
        let mut extractor = Extractor {
            branch: String::new(),
            ahead: String::new(),
            behind: String::new(),
            modified_unstaged: 0,
            deleted_unstaged: 0,
            untracked_unstaged: 0,
            modified_staged: 0,
            deleted_staged: 0,
            renamed_staged: 0,
            new_staged: 0,
        };

        let number = Regex::new(r"\d+").unwrap();
        let ahead_reg = Regex::new(r"ahead \d+").unwrap();
        let behind_reg = Regex::new(r"behind \d+").unwrap();

        let vec_strings = status.split('\n').collect::<Vec<&str>>();
        // First line gives branch name and relation to remote: ## branch...origin/branch [ahead N][behind N]
        let input = vec_strings.first().copied().unwrap_or("");

        let ahead = match ahead_reg.find(input) {
            Some(m) => {
                let s = &input[m.start()..m.end()];
                match number.find(s) {
                    Some(r) => &s[r.start()..r.end()],
                    _ => "",
                }
            }
            _ => "",
        };
        extractor.ahead = String::from(ahead);

        let behind = match behind_reg.find(input) {
            Some(m) => {
                let s = &input[m.start()..m.end()];
                match number.find(s) {
                    Some(r) => &s[r.start()..r.end()],
                    _ => "",
                }
            }
            _ => "",
        };
        extractor.behind = String::from(behind);

        // ## rrr-43...origin/rrr-43 [ahead 1][behind 3]
        // Skip "## " prefix, then split on "..." to isolate the local branch name
        let branch_str = input.get(3..).unwrap_or("");
        extractor.branch = branch_str
            .split("...")
            .next()
            .unwrap_or("")
            .to_string();

        for item in vec_strings.iter().skip(1) {
            let mut chars = item.chars();
            let staged_ch = chars.next();
            let unstaged_ch = chars.next();
            if item.len() > 2 {
                match unstaged_ch {
                    Some('M' | 'm') => extractor.modified_unstaged += 1,
                    Some('D' | 'd') => extractor.deleted_unstaged += 1,
                    Some('?') => extractor.untracked_unstaged += 1,
                    _ => (),
                };
                match staged_ch {
                    Some('M' | 'm') => extractor.modified_staged += 1,
                    Some('D' | 'd') => extractor.deleted_staged += 1,
                    Some('R' | 'r') => extractor.renamed_staged += 1,
                    Some('A' | 'a') => extractor.new_staged += 1,
                    _ => (),
                };
            }
        }
        extractor
    }

    pub fn get_unstaged(&self, modified: &str, deleted: &str, untracked: &str) -> String {
        let mut out = String::new();
        if self.modified_unstaged > 0 {
            out.push_str(&format!("{}{}", modified, self.modified_unstaged))
        }
        if self.deleted_unstaged > 0 {
            out.push_str(&format!("{}{}", deleted, self.deleted_unstaged))
        }
        if self.untracked_unstaged > 0 {
            out.push_str(&format!("{}{}", untracked, self.untracked_unstaged))
        }
        out
    }

    pub fn get_staged(&self, modified: &str, deleted: &str, renamed: &str, new: &str) -> String {
        let mut out = String::new();
        if self.modified_staged > 0 {
            out.push_str(&format!("{}{}", modified, self.modified_staged))
        }
        if self.deleted_staged > 0 {
            out.push_str(&format!("{}{}", deleted, self.deleted_staged))
        }
        if self.renamed_staged > 0 {
            out.push_str(&format!("{}{}", renamed, self.renamed_staged))
        }
        if self.new_staged > 0 {
            out.push_str(&format!("{}{}", new, self.new_staged))
        }
        out
    }
}
