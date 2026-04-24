use std::env::args;
use std::process::Command;
use std::str;

use git_status::ansi::AnsiFormatter;
use git_status::bash::BashFormatter;
use git_status::common::{Chars, OutputFormatter};
use git_status::extractor::Extractor;
use git_status::zsh::ZshFormatter;

fn main() {
    let shell_vec: Vec<String> = args().collect();
    let shell: &str = if shell_vec.len() == 2 {
        &shell_vec[1]
    } else {
        "ansi"
    };
    let status_vec = Command::new("git")
        .arg("status")
        .arg("-b")
        .arg("--porcelain")
        .output()
        .expect("Failed to execute process, git missing");
    /*
    git status -sb return all changes in repository

      ## rrr-43...origin/rrr-43 [ahead 1][behind 3]
      A  <file> - new staged file
      M <file> - modified file
      D <file> - deleted file
      R <file> - renamed file
      T <file> - file type changed
      U <file> - updated but unmerged
      ?? <file> - new file

    Using this output we could calculate changes and create repository status output for shell.
    */
    let status = str::from_utf8(&status_vec.stdout).unwrap();
    if status.is_empty() {
        return;
    }
    let extractor = Extractor::new(status);
    let chars = Chars::new();
    let branch_final = match shell {
        "bash" => BashFormatter::new(chars).get_output(&extractor),
        "zsh" => ZshFormatter::new(chars).get_output(&extractor),
        "ansi" => AnsiFormatter::new(chars).get_output(&extractor),
        _ => String::from(""),
    };
    println!("{}", branch_final);
}
