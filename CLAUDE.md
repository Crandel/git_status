# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (requires shell argument)
./target/release/git_status bash
./target/release/git_status zsh

# Check and lint
cargo check
cargo clippy

# Test
cargo test
```

## Architecture

This is a CLI tool that runs `git status -sb`, parses its output, and emits a shell-prompt-ready colored string for bash or zsh.

**Data flow:** `main.rs` → runs `git status -sb` → `Extractor::new(status)` parses the raw output → `BashFormatter` or `ZshFormatter` calls `ShellFormatter::format_output()` → prints the result.

**Key design:**
- `extractor.rs` — parses `git status -sb` output into structured counts (ahead/behind commits, staged/unstaged file changes by type). The first line gives branch + remote tracking; subsequent lines give per-file status using two-character codes (column 0 = staged, column 1 = unstaged).
- `common.rs` — `ShellFormatter` holds shell-escape wrappers (start/end strings for color codes) and character symbols per change type. `OutputFormatter` trait is the interface both formatters implement.
- `bash.rs` / `zsh.rs` — each constructs a `ShellFormatter` with shell-specific ANSI/prompt escape sequences and delegates to `format_output()`.

**Adding a new shell:** implement `OutputFormatter` and construct a `ShellFormatter` with the appropriate escape sequences for that shell.

**Output format:** `<branch>[ahead/behind][|unstaged][|staged]` — sections are omitted when empty.
