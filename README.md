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

# Configuration

Mommy will read the following the configuration file and environment variables to make her messages better for you~ ❤️

## Configuration files

### Configuration files folder

| Platform | Path                                     |
| -------- | ---------------------------------------- |
| Linux    | /home/mommy/.config                      |
| macOS    | /Users/Mommy/Library/Application Support |
| Windows  | C:\Users\Mommy\AppData\Roaming           |

### Configuration keys / environment variables

* `responses` - custom positive and negative responses~ (default: [responses.toml](responses.toml))
* `affectionate_terms` / `CARGO_MOMMYS_LITTLE` - what to call you~ (default: "girl")
* `pronouns` / `CARGO_MOMMYS_PRONOUNS` - what pronouns mommy will use for themself~ (default: "her")
* `roles` / `CARGO_MOMMYS_ROLES` - what role mommy will have~ (default "mommy")

All of these options can take a `/` separated list. Mommy will randomly select one of them whenever she talks to you~

For example, the phrase "mommy loves her little girl~ ❤️" is "`roles` loves `pronouns` little `affectionate_terms`~ ❤️"

So if your configs are like the example below:
```toml
# config.toml
responses = { positive = [], negative = [] }

affectionate_terms = "boy/pet/baby"
pronouns = "his/their"
roles = "daddy"
```

you might get any of:

* daddy loves their little boy~ ❤️
* daddy loves his little pet~ ❤️
* daddy loves their little baby~ ❤️

And so on~ ❤️

## Response placeholders

You can create custom responses and add placeholders anywhere you want~

* `AFFECTIONATE_TERM` - will be replaced with `affectionate term`~ (example: boy/pet/baby)
* `MOMMYS_PRONOUN` - will be replaced with `pronouns`~ (example: his/their)
* `MOMMYS_ROLE` - will be replaced with `roles`~ (example: daddy)

So if your positive responses are `MOMMYS_ROLE loved MOMMYS_PRONOUN AFFECTIONATE_TERM's work~` and `MOMMYS_ROLE says MOMMYS_PRONOUN AFFECTIONATE_TERM did a well job~!`, you might get any of:

* daddy loved their pet's work~
* daddy says his baby did a well job~!

And so on~ ❤️
