# Begging 

Daring, are we~? ❤️

Mommy wants to see you beg~ the question is, how much?

## I DONT WANT TO BE HERE

No worries! Mommy disabled begging by default~ 💖 It's not even included in the binary unless you opt into it~

Here are [other ways you can customize](https://faultlore.com/cargo-mommy/book/customize/index.html)

## I want to Beg

You can enable begging by enabling all features

```
cargo install cargo-mommy --all-features
```

Or if you want to manage begging and spiciness independently, you can specify the features directly


```
cargo install cargo-mommy --features thirsty beg
```

## How to Beg

In order to beg for Mommy, invoke `cargo mommy please x` instead of `cargo mommy x`.

Begging makes Mommy very happy~! 💖

Do you prefer a more stubborn Mommy? 😾 You can always beg harder by chaining multiple begs together (`cargo mommy please please please x`), or using all caps (`cargo mommy PLEASE x`), or doing a mixture of both~ 💞


## Configuration


Begging is an independent feature that respects every mood, spicy or not 🥰

Mommy will consider the following environmental variables~

* `CARGO_MOMMYS_BEG_HALF_LIFE` ⏱️
    * Mommy won't require you to beg if your previous successful beg was recent enough. This is the half-life (in seconds) of each successful beg.
    * **Default:** 0 (a value of 0 turns off Mommy's want for begging)
    * **Example:** A value of 300 (5 minutes), will allow the next non-beg a 74% chance of passing at 2 minutes, 50% at 5 minutes, and 25% at 10 minutes.
* `CARGO_MOMMYS_BEG_STUBBORN_CHANCE` 😾
    * If you fail to beg when asked, then this is the percent chance that Mommy will hold a grudge and each subsequent beg might not be good enough. After a beg is finally good enough, the grudge ends.
    * **Default:** 20

## Customize your Begging Trigger

Mommy keeps track of the previous time you begged using a file in `CARGO_HOME`, that being `.cargo/MOMMY-PLEASE.time` ⏱️

You can tell Mommy that you're begging in a round-about way by touching or updating the last modified time of this file. Mommy will update this file herself when you beg to her directly, but you can also drive it externally.

For example, you could set up an AutoHotKey script to a macro button on your keyboard, or write your own code or process to drive the file from other real-world interactions.

Note that a recent beg detected this way only counts as a single lowercase `please`. If Mommy is being especially  stubborn 😾, the best way to break out of her grudge might be a proper and direct `cargo mommy please PLEASE PLEASE`.


## I want to Stop Begging

You can set `CARGO_MOMMYS_BEG_HALF_LIFE` to 0 ⏱️, and Mommy will not care to see you beg.

If you are stuck in a grudge, you can set `CARGO_MOMMYS_BEG_STUBBORN_CHANCE` to a lower value or 0 😾. 

Make sure you relaunch your interface to capture the updated environment.

You can also run ```cargo uninstall cargo-mommy``` to remove your installation with begging enabled, and ```cargo install cargo-mommy``` without all features to return to Mommy without the begging. It's okay honey, begging can be hard~ ❤️

