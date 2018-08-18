extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env::args;
use std::fmt::write;
use std::process::Command;
use std::str;
use std::string::String;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Repo {
    Branch,
    Ahead,
    Behind,
    Unstaged,
    Staged,
}

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
        /*
        git status -sb return all changes in repository

          ## rrr-43...origin/rrr-43 [ahead 1][behind 3]
          ?? <file 1> - new file
           M <file 2> - modified file
          D  <file 2> - deleted file

        Using this output we could calculate changes and create repository status output for shell.
         */

        let mut modified_unstaged = 0;
        let mut deleted_unstaged = 0;
        let mut untracked_unstaged = 0;
        let mut modified_staged = 0;
        let mut deleted_staged = 0;
        let mut renamed_staged = 0;
        let mut new_staged = 0;

        // First we split lines
        let vec_strings = status.split("\n").collect::<Vec<&str>>();
        // First line give us a repo name and relation to remote server
        let repository_diff = match vec_strings.get(0) {
            Some(valid_str) => valid_str,
            _ => "",
        };
        // Common regex for extracting numbers
        let number = Regex::new("[0-9+]").unwrap();

        // get ahead count
        let ahead_reg = Regex::new("ahead [0-9]+*").unwrap();
        let ahead_count = match ahead_reg.find(repository_diff) {
            Some(ahead_str) => {
                let ahead_str = &repository_diff[ahead_str.start()..ahead_str.end()];
                match number.find(ahead_str) {
                    Some(res) => &ahead_str[res.start()..res.end()],
                    _ => "",
                }
            }
            _ => "",
        };

        // get behind count
        let behind = Regex::new("behind [0-9]+*").unwrap();
        let behind_count = match behind.find(repository_diff) {
            Some(behind_str) => {
                let behind_str = &repository_diff[behind_str.start()..behind_str.end()];
                match number.find(behind_str) {
                    Some(res) => &behind_str[res.start()..res.end()],
                    _ => "",
                }
            }
            _ => "",
        };

        // get branch
        let branch_vec = &repository_diff[3..repository_diff.len()]
            .split('.')
            .collect::<Vec<&str>>();
        let branch = match branch_vec.get(0) {
            Some(local_name) => local_name,
            _ => "",
        };

        for i in 1..vec_strings.len() {
            let current_str = vec_strings[i].chars().collect::<Vec<char>>();
            if current_str.len() > 2 {
                let staged_ch = current_str[0];
                let unstaged_ch = current_str[1];
                match unstaged_ch {
                    'M' => modified_unstaged += 1,
                    'm' => modified_unstaged += 1,
                    'D' => deleted_unstaged += 1,
                    'd' => deleted_unstaged += 1,
                    '?' => untracked_unstaged += 1,
                    _ => (),
                };
                match staged_ch {
                    'M' => modified_staged += 1,
                    'm' => modified_staged += 1,
                    'D' => deleted_staged += 1,
                    'd' => deleted_staged += 1,
                    'R' => renamed_staged += 1,
                    'r' => renamed_staged += 1,
                    'A' => new_staged += 1,
                    'a' => new_staged += 1,
                    _ => (),
                };
            }
        }

        let mut branch_final = String::from("");
        let mut staged_counts = String::from("");
        let mut unstaged_counts = String::from("");
        if modified_unstaged > 0 {
            unstaged_counts.push_str(&format!("%%{}", modified_unstaged))
        }
        if deleted_unstaged > 0 {
            unstaged_counts.push_str(&format!("-{}", deleted_unstaged))
        }
        if untracked_unstaged > 0 {
            unstaged_counts.push_str(&format!("*{}", untracked_unstaged))
        }

        if modified_staged > 0 {
            staged_counts.push_str(&format!("%%{}", modified_staged))
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

        let mut bash_parser = HashMap::new();
        bash_parser.insert(Repo::Branch, ("${CYAN}", "$NORMAL"));
        bash_parser.insert(Repo::Ahead, ("${LIGHT_GREEN}{>", "}$NORMAL"));
        bash_parser.insert(Repo::Behind, ("${LIGHT_RED}{<", "}$NORMAL"));
        bash_parser.insert(Repo::Unstaged, ("|${YELLOW}", "$NORMAL"));
        bash_parser.insert(Repo::Staged, ("|${GREEN}", "$NORMAL"));

        let mut zsh_parser = HashMap::new();
        zsh_parser.insert(Repo::Branch, ("%F{cyan}", "%f"));
        zsh_parser.insert(Repo::Ahead, ("%F{green}{>", "}%f"));
        zsh_parser.insert(Repo::Behind, ("%F{red}{<", "}%f"));
        zsh_parser.insert(Repo::Unstaged, ("|%F{yellow}", "%f"));
        zsh_parser.insert(Repo::Staged, ("|%F{green}", "%f"));

        let mut repo_parser = HashMap::new();
        repo_parser.insert(String::from("bash"), bash_parser);
        repo_parser.insert(String::from("zsh"), zsh_parser);

        match repo_parser.get(shell) {
            Some(parser) => {
                let (branch_start, branch_end) = parser.get(&Repo::Branch).unwrap();
                write(
                    &mut branch_final,
                    format_args!(
                        "{}",
                        branch_start.to_owned().to_owned() + branch + branch_end.to_owned()
                    ),
                ).expect("Error");
                if ahead_count.len() > 0 {
                    let (ahead_start, ahead_end) = parser.get(&Repo::Ahead).unwrap();

                    write(
                        &mut branch_final,
                        format_args!(
                            "{}",
                            ahead_start.to_owned().to_owned() + ahead_count + ahead_end.to_owned()
                        ),
                    ).expect("Error");
                }
                if behind_count.len() > 0 {
                    let (behind_start, behind_end) = parser.get(&Repo::Behind).unwrap();

                    write(
                        &mut branch_final,
                        format_args!(
                            "{}",
                            behind_start.to_owned().to_owned()
                                + behind_count
                                + behind_end.to_owned()
                        ),
                    ).expect("Error");
                }
                if unstaged_counts.len() > 0 {
                    let (unstaged_start, unstaged_end) = parser.get(&Repo::Unstaged).unwrap();

                    write(
                        &mut branch_final,
                        format_args!(
                            "{}",
                            unstaged_start.to_owned().to_owned()
                                + &unstaged_counts
                                + unstaged_end.to_owned()
                        ),
                    ).expect("Error");
                }
                if staged_counts.len() > 0 {
                    let (staged_start, staged_end) = parser.get(&Repo::Staged).unwrap();

                    write(
                        &mut branch_final,
                        format_args!(
                            "{}",
                            staged_start.to_owned().to_owned()
                                + &staged_counts
                                + staged_end.to_owned()
                        ),
                    ).expect("Error");
                }
            }
            _ => {}
        }
        println!("{}", branch_final);
    }
}
