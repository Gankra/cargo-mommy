# Not Just Cargo

Although Mommy is primarily intended to be invoked as `cargo mommy`, you *can* invoke her binary directly, or setup aliases to it.

If you try to make it into an alias, you should prefer pointing it to cargo-mommy directly, as this tends to play nicer with the rustup toolchain picker~ mommy will also respect CARGO to execute the right cargo for you~


## REALLY Not Just Cargo

Now, this is a Developing Feature that's first shipping in 0.3.1, but...

If you want to use cargo-mommy for not-cargo programs, just set the CARGO_MOMMYS_ACTUAL environment variable to it, for example on linux you can do this:

```
CARGO_MOMMYS_ACTUAL=date cargo-mommy
Sun Nov 19 05:33:34 PM CET 2023
what a good girl you are~ ❤️
```

Enough people have been asking about this that we might end up just making a second dedicated "mommy" binary that supports this usecase more directly. We'll see~
