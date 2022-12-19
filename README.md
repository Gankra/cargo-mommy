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

Mommy knows her little girl can do better~ ❤️

>_
```

# Configuration

By default mommy will assume you're her little girl, but if you want something else just set `CARGO_MOMMYS_LITTLE` in your environment to your preferred affectionate term~