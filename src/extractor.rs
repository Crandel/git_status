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

    pub fn get_unstaged(&self, modified: &str, deleted: &str) -> String {
        let mut out = String::new();
        if self.modified_unstaged > 0 {
            out.push_str(&format!("{}{}", modified, self.modified_unstaged))
        }
        if self.deleted_unstaged > 0 {
            out.push_str(&format!("{}{}", deleted, self.deleted_unstaged))
        }
        out
    }

    pub fn get_untracked(&self, untracked: &str) -> String {
        if self.untracked_unstaged > 0 {
            format!("{}{}", untracked, self.untracked_unstaged)
        } else {
            String::new()
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- Extractor::new() branch parsing ---

    #[test]
    fn clean_repo_no_remote() {
        let e = Extractor::new("## main\n");
        assert_eq!(e.branch, "main");
        assert_eq!(e.ahead, "");
        assert_eq!(e.behind, "");
        assert_eq!(e.modified_unstaged, 0);
        assert_eq!(e.deleted_unstaged, 0);
        assert_eq!(e.untracked_unstaged, 0);
        assert_eq!(e.modified_staged, 0);
        assert_eq!(e.deleted_staged, 0);
        assert_eq!(e.renamed_staged, 0);
        assert_eq!(e.new_staged, 0);
    }

    #[test]
    fn tracking_branch_strips_remote() {
        let e = Extractor::new("## main...origin/main\n");
        assert_eq!(e.branch, "main");
    }

    #[test]
    fn ahead_only() {
        let e = Extractor::new("## main...origin/main [ahead 3]\n");
        assert_eq!(e.ahead, "3");
        assert_eq!(e.behind, "");
    }

    #[test]
    fn behind_only() {
        let e = Extractor::new("## main...origin/main [behind 5]\n");
        assert_eq!(e.ahead, "");
        assert_eq!(e.behind, "5");
    }

    #[test]
    fn ahead_and_behind() {
        let e = Extractor::new("## dev...origin/dev [ahead 1][behind 3]\n");
        assert_eq!(e.branch, "dev");
        assert_eq!(e.ahead, "1");
        assert_eq!(e.behind, "3");
    }

    #[test]
    fn branch_no_dots() {
        let e = Extractor::new("## feature-branch\n");
        assert_eq!(e.branch, "feature-branch");
    }

    #[test]
    fn detached_head() {
        let e = Extractor::new("## HEAD (no branch)\n");
        assert_eq!(e.branch, "HEAD (no branch)");
    }

    #[test]
    fn empty_input() {
        let e = Extractor::new("");
        assert_eq!(e.branch, "");
        assert_eq!(e.ahead, "");
        assert_eq!(e.behind, "");
        assert_eq!(e.modified_unstaged, 0);
    }

    #[test]
    fn short_first_line_no_panic() {
        let e = Extractor::new("##\n");
        assert_eq!(e.branch, "");
    }

    // --- Extractor::new() file status parsing ---

    #[test]
    fn modified_unstaged() {
        let e = Extractor::new("## main\n M file.txt\n");
        assert_eq!(e.modified_unstaged, 1);
        assert_eq!(e.modified_staged, 0);
    }

    #[test]
    fn deleted_unstaged() {
        let e = Extractor::new("## main\n D file.txt\n");
        assert_eq!(e.deleted_unstaged, 1);
        assert_eq!(e.deleted_staged, 0);
    }

    #[test]
    fn untracked_file() {
        let e = Extractor::new("## main\n?? file.txt\n");
        assert_eq!(e.untracked_unstaged, 1);
    }

    #[test]
    fn modified_staged() {
        let e = Extractor::new("## main\nM  file.txt\n");
        assert_eq!(e.modified_staged, 1);
        assert_eq!(e.modified_unstaged, 0);
    }

    #[test]
    fn deleted_staged() {
        let e = Extractor::new("## main\nD  file.txt\n");
        assert_eq!(e.deleted_staged, 1);
    }

    #[test]
    fn renamed_staged() {
        let e = Extractor::new("## main\nR  old.txt -> new.txt\n");
        assert_eq!(e.renamed_staged, 1);
    }

    #[test]
    fn new_file_staged() {
        let e = Extractor::new("## main\nA  file.txt\n");
        assert_eq!(e.new_staged, 1);
    }

    #[test]
    fn both_staged_and_unstaged() {
        let e = Extractor::new("## main\nMM file.txt\n");
        assert_eq!(e.modified_staged, 1);
        assert_eq!(e.modified_unstaged, 1);
    }

    #[test]
    fn multiple_files_mixed() {
        let status = "## main...origin/main [ahead 2]\n M f1\n D f2\nM  f3\nA  f4\n?? f5\nR  a -> b\n";
        let e = Extractor::new(status);
        assert_eq!(e.branch, "main");
        assert_eq!(e.ahead, "2");
        assert_eq!(e.modified_unstaged, 1);
        assert_eq!(e.deleted_unstaged, 1);
        assert_eq!(e.untracked_unstaged, 1);
        assert_eq!(e.modified_staged, 1);
        assert_eq!(e.new_staged, 1);
        assert_eq!(e.renamed_staged, 1);
    }

    // --- get_unstaged() ---

    #[test]
    fn get_unstaged_empty() {
        let e = Extractor::new("## main\n");
        assert_eq!(e.get_unstaged("%", "-"), "");
    }

    #[test]
    fn get_unstaged_modified_only() {
        let e = Extractor::new("## main\n M a\n M b\n");
        assert_eq!(e.get_unstaged("%", "-"), "%2");
    }

    #[test]
    fn get_unstaged_deleted_only() {
        let e = Extractor::new("## main\n D a\n");
        assert_eq!(e.get_unstaged("%", "-"), "-1");
    }

    #[test]
    fn get_unstaged_modified_and_deleted() {
        let e = Extractor::new("## main\n M a\n D b\n");
        assert_eq!(e.get_unstaged("%", "-"), "%1-1");
    }

    // --- get_untracked() ---

    #[test]
    fn get_untracked_empty() {
        let e = Extractor::new("## main\n");
        assert_eq!(e.get_untracked("*"), "");
    }

    #[test]
    fn get_untracked_some() {
        let e = Extractor::new("## main\n?? a\n?? b\n?? c\n");
        assert_eq!(e.get_untracked("*"), "*3");
    }

    // --- get_staged() ---

    #[test]
    fn get_staged_empty() {
        let e = Extractor::new("## main\n");
        assert_eq!(e.get_staged("%", "-", "^", "+"), "");
    }

    #[test]
    fn get_staged_all_types() {
        let e = Extractor::new("## main\nM  a\nD  b\nR  c -> d\nA  e\n");
        assert_eq!(e.get_staged("%", "-", "^", "+"), "%1-1^1+1");
    }

    #[test]
    fn get_staged_new_only() {
        let e = Extractor::new("## main\nA  file.txt\n");
        assert_eq!(e.get_staged("%", "-", "^", "+"), "+1");
    }
}
