# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

Prefer `task` (go-task) over raw `cargo` commands:

```bash
task           # build release (default)
task build     # cargo build --release
task install   # build and install to ~/.local/bin
task test      # cargo test
task run-bash  # build + run for bash prompt
task run-zsh   # build + run for zsh prompt
```

Raw cargo commands when needed:

```bash
cargo check
cargo clippy
cargo build --release
./target/release/git_status ansi   # ansi is the default when no arg given
```

## Architecture

This is a CLI tool that runs `git status -sb`, parses its output, and emits a shell-prompt-ready colored string for bash or zsh.

**Data flow:** `main.rs` → runs `git status -sb` → `Extractor::new(status)` parses the raw output → `BashFormatter` or `ZshFormatter` calls `ShellFormatter::format_output()` → prints the result.

**Key design:**
- `extractor.rs` — parses `git status -sb` output into structured counts (ahead/behind commits, staged/unstaged file changes by type). The first line gives branch + remote tracking; subsequent lines give per-file status using two-character codes (column 0 = staged, column 1 = unstaged).
- `common.rs` — `ShellFormatter` holds shell-escape wrappers (start/end strings for color codes) and character symbols per change type. `OutputFormatter` trait is the interface both formatters implement.
- `bash.rs` / `zsh.rs` / `ansi.rs` — each constructs a `ShellFormatter` with shell-specific escape sequences and delegates to `format_output()`. `ansi` uses raw `\x1b[…m` bytes (for status bars / non-PS1 contexts); `bash` wraps codes in `\[…\]`; `zsh` uses `%F{…}%f`.

**Adding a new shell:** implement `OutputFormatter` and construct a `ShellFormatter` with the appropriate escape sequences for that shell.

**Output format:** `<branch>[ahead/behind][|unstaged][|untracked][|staged]` — sections are omitted when empty. Default shell argument is `ansi`.
