# Locally Building The Website

The [website](https://faultlore.com/cargo-mommy/) (and book it hosts) is automatically built for every PR, and automatically deployed for every push to main. See the [releasing](./releasing.md) page for details.

If you want to run and test it locally, [install oranda](https://opensource.axo.dev/oranda/) and run:

```
oranda dev
```

This will put oranda in a "watch" mode that live-updates the website as you edit files, while serving the result on a localhost url (usually http://127.0.0.1:7979/cargo-mommy).

Oranda internally uses [mdBook](https://rust-lang.github.io/mdBook/) as a library, with several patches to make mdbook have the same theme as the oranda one. If you're familiar with mdbook and just want to stick to that, you *should* be able to just [go to the book/ dir](https://github.com/Gankra/cargo-mommy/blob/main/book/) and run:

```
mdbook serve
```

It just won't have the same theming as the production site.
