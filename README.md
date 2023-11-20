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

If you try to make it into an alias, you should prefer pointing it to `cargo-mommy` directly,
we wouldn't want to break the rustup toolchain picker, now would we?~

If you want to use `cargo-mommy` for not-cargo programs, just set the `CARGO_{ROLE}S_ACTUAL`
environment variable to it, for example

```
$ CARGO_MOMMYS_ACTUAL=date ./target/debug/cargo-mommy
Sun Nov 19 05:33:34 PM CET 2023
what a good girl you are~
```

mommy will also respect `CARGO` to execute the right cargo for you~

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





# Licensing
mommy likes freedom~ ❤️, and is dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

Use either at your choice.
