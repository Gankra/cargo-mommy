# True Roles

While [Variables like Roles And Pronouns](./roles-and-pronouns.md) change *output*, the True Role exists to change *input*~

> TL;DR
>
> ```
> cargo mommy i mean daddy
>    mommy is now daddy~
> cargo daddy check
>    daddy loves you~
> ```

This feature is useful for:

* folks who *really* don't want to say/see "mommy" ever again
* folks who want to maintain several independent sets of configuration (perhaps for plural reasons)

Specifically if you change the True Role from "mommy" to e.g. "daddy", the following changes will occur:

* You will be able to invoke it as `cargo daddy`
* Instead of reading env vars like `CARGO_MOMMYS_MOODS` for config, it will read `CARGO_DADDYS_MOODS` (note the extra "S"!)
* If `CARGO_{TRUE_ROLE}S_ROLE` isn't set, it will default to the True Role

The value "daddy" is arbitrary here, you can pick any value. Make yourself a `cargo burger-chimes` if you want!

All you have to do to change the True Role is to rename the cargo-mommy(.exe) binary to cargo-daddy(.exe).


## I Mean...

As a convenience, `cargo mommy i mean daddy` finds the current binary and makes a *copy* (not a symlink or move) with the new name for you. Execution is halted immediately after seeing such an incantation, so all other input is ignored.

`cargo mommy i mean mommy`, or any other "idempotent i mean" is treated as sugar for "cargo mommy mommy" (which has always worked and produces 2 messages). e.g.

```
cargo mommy i mean mommy i mean mommy i mean mommy check
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
that's mommy's clever little girl~ 💓
*wraps you in a big hug* 💗
you did it~! ❤️
good girl~
mommy's so proud of you~
```