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

Mommy will read the following environment variables to make her messages better for you~ ❤️

* `CARGO_MOMMYS_LITTLE` - what to call you~ (default: "girl")
* `CARGO_MOMMYS_PRONOUNS` - what pronouns mommy will use for themself~ (default: "her")
* `CARGO_MOMMYS_ROLES` - what role mommy will have~ (default "mommy")

All of these options can take a `/` separated list. Mommy will randomly select one of them whenever she talks to you~

For example, the phrase "mommy loves her little girl~ ❤️" is "CARGO_MOMMYS_ROLE loves CARGO_MOMMYS_PRONOUNS little CARGO_MOMMYS_LITTLE~ ❤️"

So if you set `CARGO_MOMMYS_ROLES="daddy"`, `CARGO_MOMMYS_PRONOUNS="his/their"`, and `CARGO_MOMMYS_LITTLE="boy/pet/baby"` then you might get any of

* daddy loves their little boy~ ❤️
* daddy loves his little pet~ ❤️
* daddy loves their little baby~ ❤️

And so on~ ❤️

