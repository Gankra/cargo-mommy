# cargo-mommy

Mommy's here to support you when running cargo~ ❤️

# Installation

Install cargo-mommy like you would any other cargo extension~

```text
> cargo install cargo-mommy
```

# Usage

Run whatever cargo command you would normally but add mommy after cargo~

```text
> cargo mommy test

    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src\main.rs (target\debug\deps\cargo_mommy-3804b5c850d46137.exe)

running 1 test
test test ... FAILED

failures:

---- test stdout ----
thread 'test' panicked at 'oops!!', src\main.rs:26:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin cargo-mommy`

mommy knows her little girl can do better~ ❤️

>_
```

If you try to make it into an alias, you should prefer pointing it to `cargo-mommy` directly,
we wouldn't want to break the rustup toolchain picker, now would we?~

# Configuration

Mommy will read the following environment variables to make her messages better for you~ ❤️

* `CARGO_MOMMYS_LITTLE` - what to call you~ (default: "girl")
* `CARGO_MOMMYS_PRONOUNS` - what pronouns mommy will use for themself~ (default: "her")
* `CARGO_MOMMYS_ROLES` - what role mommy will have~ (default "mommy")
* `CARGO_MOMMYS_EMOTES` - what emotes mommy will have~ (default "❤️/💖/💗/💓/💞")

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

# Configuration (kink)

<details>

<summary>
<b>THIS IS NSFW, STOP READING IF YOU WANT MOMMY TO REMAIN INNOCENT!</b>
</summary>

...

...

Good pet~ ❤️

All of mommy's NSFW content is hidden behind CARGO_MOMMYS_MOODS, where "thirsty" is heavy teasing/flirting and "yikes" is full harsh dommy mommy kink~

You can enable "true mommy chaos mode" by setting `CARGO_MOMMYS_MOODS="chill/thirsty/yikes"`, making mommy oscillate wildly between light positive affirmation and trying to break you in half~

* `CARGO_MOMMYS_MOODS` - how kinky mommy will be~ (default: "chill", possible values "chill", "thirsty", "yikes")
* `CARGO_MOMMYS_PARTS` - what part of mommy you should crave~ (default: "milk")
* `CARGO_MOMMYS_FUCKING` - what to call mommy's pet~ (default: "slut/toy/pet/pervert/whore")

-----

**Here's some examples of mommy being thirsty~ ❤️**

*tugs your leash*
that's a VERY good girl~ 💞

*smooches your forehead*
good job~

are you just keysmashing now~?
cute~ 💖

if you don't learn how to code better, mommy is going to put you in time-out~ 💓

-----

**And here's some examples of mommy being yikes~ 💞**

good slut~
you've earned five minutes with the buzzy wand~ 💗

*slides her finger in your mouth*
that's a good little toy~ ❤️

get on your knees and beg mommy for forgiveness you pervert~

mommy is starting to wonder if you should just give up and become her breeding stock~ 💗

</details>

# Licensing
mommy likes freedom~ ❤️, and is dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

Use either at your choice.