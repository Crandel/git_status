extern crate regex;

use regex::Regex;
use std::env::args;
use std::process::Command;
use std::str;
use std::string::String;

fn main() {
    let status_vec = Command::new("git")
        .arg("status")
        .arg("-sb")
        .output()
        .expect("failed to execute process");
    let status = str::from_utf8(&status_vec.stdout).unwrap();
    let shell_vec: Vec<String> = args().collect();
    if shell_vec.len() == 1 {
        panic!("Please tell me a shell as arg")
    }
    let shell: &str = &shell_vec[1];
    if status.len() > 0 {
        let mut modified_unstaged = 0;
        let mut deleted_unstaged = 0;
        let mut untracked_unstaged = 0;
        let mut modified_staged = 0;
        let mut deleted_staged = 0;
        let mut renamed_staged = 0;
        let mut new_staged = 0;

        let vec_strings = status.split("\n").collect::<Vec<&str>>();
        let repository_diff = match vec_strings.get(0) {
            Some(valid_str) => valid_str,
            _ => "",
        };
        let number = Regex::new("[0-9+]").unwrap();
        let ahead = Regex::new("ahead [0-9]+*").unwrap();
        let branch_full_rex = Regex::new(r"## \w+").unwrap();
        let branch_rex = Regex::new(r"\w+").unwrap();
        // get ahead count
        let ahead_str = match ahead.find(repository_diff) {
            Some(ahead_str) => &repository_diff[ahead_str.start()..ahead_str.end()],
            None => "",
        };
        let ahead_count = match number.find(ahead_str) {
            Some(res) => &ahead_str[res.start()..res.end()],
            None => "",
        };

        // get behind count
        let behind = Regex::new("behind [0-9]+*").unwrap();
        let behind_str = match behind.find(repository_diff) {
            Some(behind_str) => &repository_diff[behind_str.start()..behind_str.end()],
            None => "",
        };
        let behind_count = match number.find(behind_str) {
            Some(res) => &behind_str[res.start()..res.end()],
            None => "",
        };
        // get branch
        let branch = match branch_full_rex.find(repository_diff) {
            Some(br) => {
                let br_str = &repository_diff[br.start()..br.end()];
                match branch_rex.find(br_str) {
                    Some(branch_local) => &br_str[branch_local.start()..branch_local.end()],
                    _ => "",
                }
            }
            _ => "",
        };

        for i in 1..vec_strings.len() {
            let current_str = vec_strings[i].chars().collect::<Vec<char>>();
            if current_str.len() > 2 {
                let staged_ch = current_str[0];
                let unstaged_ch = current_str[1];
                match unstaged_ch {
                    'M' => modified_unstaged += 1,
                    'D' => deleted_unstaged += 1,
                    '?' => untracked_unstaged += 1,
                    _ => (),
                };
                match staged_ch {
                    'M' => modified_staged += 1,
                    'D' => deleted_staged += 1,
                    'R' => renamed_staged += 1,
                    'A' => new_staged += 1,
                    _ => (),
                };
            }
        }

        let mut branch_final = String::from(branch);
        let mut staged_counts = String::from("");
        let mut unstaged_counts = String::from("");
        if modified_unstaged > 0 {
            unstaged_counts.push_str(&format!("%{}", modified_unstaged))
        }
        if deleted_unstaged > 0 {
            unstaged_counts.push_str(&format!("-{}", deleted_unstaged))
        }
        if untracked_unstaged > 0 {
            unstaged_counts.push_str(&format!("*{}", untracked_unstaged))
        }

        if modified_staged > 0 {
            staged_counts.push_str(&format!("%{}", modified_staged))
        }
        if deleted_staged > 0 {
            staged_counts.push_str(&format!("-{}", deleted_staged))
        }
        if renamed_staged > 0 {
            staged_counts.push_str(&format!("^{}", renamed_staged))
        }
        if new_staged > 0 {
            staged_counts.push_str(&format!("+{}", new_staged))
        }

        match shell {
            "zsh" => {
                branch_final = format!("%F{{cyan}}{}%f", branch_final);
                if ahead_count.len() > 0 {
                    branch_final = format!("{}%F{{green}}{{>{}}}%f", branch_final, ahead_count);
                }
                if behind_count.len() > 0 {
                    branch_final = format!("{}%F{{red}}{{<{}}}%f", branch_final, behind_count);
                }
                if unstaged_counts.len() > 0 {
                    branch_final = format!("{}|%F{{yellow}}{}%f", branch_final, unstaged_counts)
                }
                if staged_counts.len() > 0 {
                    branch_final = format!("{}|%F{{green}}{}%f", branch_final, staged_counts)
                }
            }
            "bash" => {
                branch_final = format!("${{CYAN}}{}", branch_final);
                if ahead_count.len() > 0 {
                    branch_final = format!("{}${{LIGHT_GREEN}}{{>{}}}", branch_final, ahead_count);
                }
                if behind_count.len() > 0 {
                    branch_final = format!("{}${{LIGHT_RED}}{{<{}}}", branch_final, behind_count);
                }
                if unstaged_counts.len() > 0 {
                    branch_final = format!("{}|${{YELLOW}}{}", branch_final, unstaged_counts)
                }
                if staged_counts.len() > 0 {
                    branch_final = format!("{}|${{GREEN}}{}", branch_final, staged_counts)
                }
            }
            _ => {}
        }
        println!("{}", branch_final);
    }
}
