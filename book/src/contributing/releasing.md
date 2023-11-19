# Releasing

Mommy's release process is simple but does a lot~


## Cutting a Release

1. Checkout main
2. pull
3. Update the "Unreleased" section of [CHANGELOG.md](https://github.com/Gankra/cargo-mommy/blob/main/CHANGELOG.md) (DO NOT CHANGE THE HEADING!)
4. Commit
5. Run `cargo release {version}` (e.g. `cargo release 1.0.0`)

This will trigger this cascade of events:

1. [cargo-release](https://github.com/crate-ci/cargo-release) will get the ball rolling:
    a. [the changelog heading will be updated to "Version {version} ({date})"](https://github.com/Gankra/cargo-mommy/blob/0d96506db241003166e32deb22ad0ab0fc52c16c/Cargo.toml#L50-L53)
    b. the version in the Cargo.toml will be update to "{version}"
    b. [a commit will be made with "release: {version}"](https://github.com/Gankra/cargo-mommy/blob/0d96506db241003166e32deb22ad0ab0fc52c16c/Cargo.toml#L30)
    c. [a "v{version}" git tag will be created](https://github.com/Gankra/cargo-mommy/blob/0d96506db241003166e32deb22ad0ab0fc52c16c/Cargo.toml#L29)
    d. `cargo publish` will be run
    e. A more robust equivalent of `git push && git push --tags` will run
2. [cargo-dist](https://opensource.axo.dev/cargo-dist/) will see the tagged commit and build binaries/archives/installers/announcements:
    a. [release.yml](https://github.com/Gankra/cargo-mommy/blob/main/.github/workflows/release.yml) will spawn
    b. binaries and archives (zips/tarballs) will get built for [all supported targets](https://github.com/Gankra/cargo-mommy/blob/0d96506db241003166e32deb22ad0ab0fc52c16c/Cargo.toml#L41)
    c. [installers will be built](https://github.com/Gankra/cargo-mommy/blob/0d96506db241003166e32deb22ad0ab0fc52c16c/Cargo.toml#L39) that can fetch the archives
    d. A [Github Release](https://github.com/Gankra/cargo-mommy/releases) will be made, with the release notes and installers/archives
3. [oranda](https://opensource.axo.dev/oranda/) will see a workflow called "Release" completed, and build and deploy docs
    a. [web.yml](https://github.com/Gankra/cargo-mommy/blob/main/.github/workflows/web.yml) will spawn
    b. [oranda will autodetect, retheme, and build](https://opensource.axo.dev/oranda/book/configuration/mdbook.html) the [cargo-mommy mdbook](https://github.com/Gankra/cargo-mommy/tree/main/src)
    c. oranda will generate [a website with releases, the book, and a platform-autodetecting install widget for the latest release](https://faultlore.com/cargo-mommy/)
    d. oranda will push the website to [the gh-pages branch](https://github.com/Gankra/cargo-mommy/tree/gh-pages)
4. Github Pages will trigger
    a. [pages-build-deployment](https://github.com/Gankra/cargo-mommy/actions/workflows/pages/pages-build-deployment) will spawn
    b. [the live website will update](https://faultlore.com/cargo-mommy/)

Note that steps 3 and 4 trigger on all pushes to main, so updating docs doesn't require a new release~




## Updating The Release Process

The cargo-release step was created by hand-copying some config values that Gankra likes to use in her projects. There shouldn't be any need to mess with it.

The cargo-dist step was created by locally running `cargo dist init`, selecting some targets/installers in the interactive prompts, and committing the results. It can be updated by running `cargo dist init` again.

The oranda step was created by running `oranda generate ci`, selecting some of the options, and committing the results. A [custom oranda.json](https://github.com/Gankra/cargo-mommy/blob/main/oranda.json) was then added to fix path-prefix with github-pages, and to pick a non-default theme. The CI is setup to always use the "latest" oranda, so it's self-updating and may never need to be touched. Any changes that are desired will probably be changes to oranda.json.

The Github Pages step was doing in the Github settings UI for cargo-mommy. The website it deploys to is Gankra's.



## Ensuring The Release Process Works

All pull requests run the `cargo dist plan` [part of the releases ci](https://github.com/Gankra/cargo-mommy/blob/main/.github/workflows/release.yml) to check that the release process still works.

All pull requests run the `oranda build` [part of the website ci](https://github.com/Gankra/cargo-mommy/blob/main/.github/workflows/web.yml) to check that the website and mdbook still works.

(This is just the default behaviour of the CI scripts that cargo-dist and oranda generate!)



## Historical Releases

This process was setup for 0.3.0, everything before that was Gankra being intentionally sloppy because all of the above is her actual job and sometimes you just don't want to think about work and just want to mash `cargo publish` and nothing more on your shitpost that everyone took way too seriously.
