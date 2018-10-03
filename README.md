# Git status utility
## Parse git repo status for bash/zsh prompt

There are three main characteristics (`#` - number of commits or files):

* Number of commits:
    * **ahead** remote repo (`<#` in *green* color)
    * **behind** remote repo (`>#` in *red* color)

* Unstaged files (*yellow* color):
    * modified (`%#`)
    * deleted (`-#`)
    * renamed (`^#`)
    * new (`+#`)

* Staged files (*green* color):
    * modified (`%#`)
    * deleted (`-#`)
    * untracked (`*#`)

## Installation

```
cargo build --release

target/release/<binary file> bash|zsh
```
