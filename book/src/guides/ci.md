# Using cargo-mommy in CI

While you can of course `cargo install cargo-mommy` in CI, this can significantly slow down your CI runs~

To help with this, every cargo-mommy release since 0.3.0 comes with prebuilt binaries and shell/powershell installers, allowing mommy to be setup in CI quickly and easily~ 💕

See [the install page](https://faultlore.com/cargo-mommy/artifacts/) for `curl | sh` and `irm | iex` expressions~

Note that if you use a Github CI Matrix and it covers Windows and Not Windows, `run` expressions will implicitly be a shell/powershell polyglot. If so, we recommend putting the "install cargo-mommy" expression as an argument to the Matrix (powershell on windows, shell everywhere else)~
