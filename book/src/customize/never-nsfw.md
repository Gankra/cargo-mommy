# Removing NSFW Features

By default mommy operates in a totally safe-for-work mode, where she just provides positive encouragement, but she has support for more intense experiences, and if that concerns you, you can statically compile them out with feature flags~

To get a totally safe binary, all you need to do is disable mommy's default features~

```
cargo install cargo-mommy --no-default-features
```

All nsfw strings, moods, and variables will be eliminated from the binary, leaving only encouragement~ ❤️

...and the "ominous" mood too~ 🎭