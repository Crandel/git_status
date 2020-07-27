use std::env::args;
use std::process::Command;
use std::str;
use std::string::String;

use git_status::bash::BashFormatter;
use git_status::common::OutputFormatter;
use git_status::extractor::Extractor;
use git_status::zsh::ZshFormatter;

fn main() {
    let shell_vec: Vec<String> = args().collect();
    if shell_vec.len() == 1 {
        panic!("Please tell me a shell name as arg")
    }
    let shell: &str = &shell_vec[1];
    let status_vec = Command::new("git")
        .arg("status")
        .arg("-sb")
        .output()
        .expect("Failed to execute process, git missing");
    /*
    git status -sb return all changes in repository

      ## rrr-43...origin/rrr-43 [ahead 1][behind 3]
      ?? <file 1> - new file
       M <file 2> - modified file
      D  <file 2> - deleted file

    Using this output we could calculate changes and create repository status output for shell.
    */
    let status = str::from_utf8(&status_vec.stdout).unwrap();
    if status.is_empty() {
        return;
    }
    let extractor = Extractor::new(status);
    let bash_formatter = BashFormatter::new();
    let zsh_formatter = ZshFormatter::new();
    let branch_final = match shell {
        "bash" => bash_formatter.get_output(&extractor),
        "zsh" => zsh_formatter.get_output(&extractor),
        _ => String::from(""),
    };
    println!("{}", branch_final);
}
