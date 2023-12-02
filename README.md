<div class="oranda-hide">

# `cargo-mommy`

</div>

[![crates.io](https://img.shields.io/crates/v/cargo-mommy.svg)](https://crates.io/crates/cargo-mommy)
[![Rust CI](https://github.com/Gankra/cargo-mommy/workflows/Rust/badge.svg?branch=main)](https://github.com/Gankra/cargo-mommy/actions/workflows/ci.yml)



Mommy's here to support you when running cargo~ ❤️

# Installation

You can `cargo install cargo-mommy`, [see the website for more options](https://faultlore.com/cargo-mommy/)


# Usage

Run whatever cargo command you would normally but add mommy after cargo~

```
cargo mommy check

    Checking bappy-script v0.1.3
error: expected one of `!` or `::`, found `passes`
  --> src\main.rs:20:6
   |
20 | mods passes;
   |      ^^^^^^ expected one of `!` or `::`

error: could not compile `bappy-script` (bin "bappy-script") due to previous error
mommy knows her little girl can do better~ 💞
```

[See the docs for more options](https://faultlore.com/cargo-mommy/book/)


# Configuration

Mommy will read the following environment variables to make her messages better for you~ ❤️

* `CARGO_MOMMYS_LITTLE` - what to call you~ (default: "girl")
* `CARGO_MOMMYS_PRONOUNS` - what pronouns mommy will use for themself~ (default: "her")
* `CARGO_MOMMYS_ROLES` - what role mommy will have~ (default "mommy")
* `CARGO_MOMMYS_EMOTES` - what emotes mommy will have~ (default "❤️/💖/💗/💓/💞")
* `CARGO_MOMMYS_MOODS` - picks the set of possible responses~ (default: "chill", possible values "chill", "ominous")

All of these options can take a `/` separated list. Mommy will randomly select one of them whenever she talks to you~

For example, the phrase "mommy loves her little girl~ 💞" is "CARGO_MOMMYS_ROLE loves CARGO_MOMMYS_PRONOUNS little CARGO_MOMMYS_LITTLE~"

So if you set `CARGO_MOMMYS_ROLES="daddy"`, `CARGO_MOMMYS_PRONOUNS="his/their"`, and `CARGO_MOMMYS_LITTLE="boy/pet/baby"` then you might get any of

* daddy loves their little boy~ ❤️
* daddy loves his little pet~
* daddy loves their little baby~ 💗

And so on~ 💓

<details>

<summary>
OS-specific information:
</summary>

### For macOS, Linux, BSD (or any sufficiently Unix-like operating system)

You're probably using [Zsh](https://zsh.sourceforge.io/) or [Bash](https://www.gnu.org/software/bash/) as your login shell, to check - open up a terminal window (Terminal.app or [iTerm2](https://github.com/gnachman/iTerm2) on macOS; [Console](https://gitlab.gnome.org/GNOME/console), [Terminal](https://gitlab.gnome.org/GNOME/gnome-terminal), [Konsole](https://github.com/KDE/konsole), [Kitty](https://github.com/kovidgoyal/kitty), or [Alacritty](https://github.com/alacritty/alacritty) most likely anywhere else) and run `echo $SHELL`.

You can see all of your current settings by running `export -p | grep CARGO_MOMMY`.

Depending on the one you have, you'll have to modify the contents of your rcfile (with [vim](https://github.com/vim/vim), [nano](https://nano-editor.org/), [helix](https://github.com/helix-editor/helix) etc.), or create one if it doesn't exist.

- Bash:
  Change `~/.bashrc` to include your settings. Set them with `export VAR_NAME=value`, for example:

  ```sh
  $ cat ~/.bashrc

    export CARGO_MOMMYS_LITTLE="boy"
    export CARGO_MOMMYS_ROLES="daddy"
    export CARGO_MOMMYS_PRONOUNS="his"
    export CARGO_MOMMYS_MOODS="yikes"
  ```

- Zsh:
  Change `~/.zshenv`. Set them with `export VAR_NAME=value`, for example:

  ```sh
  $ cat ~/.zshenv

    export CARGO_MOMMYS_LITTLE="girlie"
    export CARGO_MOMMYS_ROLES="momma"
    export CARGO_MOMMYS_PRONOUNS="her/their"
    export CARGO_MOMMYS_MOODS="chill/thirsty"
  ```

To remove any of the settings, simply delete the associated line in the file.

### For our dear Windows friends:

To check which settings you're currently using, open a Command Prompt or Powershell window (a quick way to do this is to press `Windows` + `X` on your keyboard), and type:

- For Command Prompt:

  ```sh
  > set | find "CARGO_MOMMY"
  ```

- For Powershell (now might be a good time to learn the difference between Windows `powershell.exe` and Microsoft `pwsh.exe`):

  ```powershell
  > Get-Item Env:\CARGO_MOMMY*
  ```

It's simultaneously easier and harder to set environment variables on Windows - soo many ways are available, but the easiest are:

- You can use the Control Panel by:
  1. Going to Start, 
  2. Searching "path" and clicking the "Edit the system environment variables" option, 
  3. Then "Environment Variables", 
  4. And finally, "New" (or whichever button is appropriate) under the "User" section (the top one).

- If you happen to have modern Powershell (that's anything with the Microsoft branding/black logo/version 7+), you can run:

  ```powershell
  > [Environment]::SetEnvironmentVariable("VAR_NAME", "value", "User")
  ```

  to add one, and:

  ```powershell
  > [Environment]::SetEnvironmentVariable("VAR_NAME", "", "User")
  ```
  
  to remove it (by setting it to an empty string).

</details>




# Licensing
mommy likes freedom~ ❤️, and is dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

Use either at your choice.
