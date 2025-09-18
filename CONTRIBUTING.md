# Contribution guidelines

First off, thank you for considering contributing to the package `measures-rs`, including the crate `measures`.

If your contribution is not straightforward, please first discuss the change you wish to make, by creating a new issue before creating the pull request.

## Reporting issues

Before reporting an issue on the [issue tracker](https://github.com/carlomilanesi/measures-rs/issues), please check that it has not already been reported, by searching for some related keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the document [CHANGELOG](https://github.com/carlomilanesi/measures-rs/blob/master/CHANGELOG.md) with the changes you have made under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections, depending on the types of changes defined by [Keep a changelog](https://keepachangelog.com/en/1.0.0/):
- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it.

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/carlomilanesi/measures-rs
cd measures-rs
cargo build
```

### Useful Commands

Because most of the code is in macros, the commands `cargo fmt` and `cargo check` and `cargo build` are almost useless to format or check the code.

Instead, use the following commands:

```sh
# To compile all the library code and run all the tests:
cargo t

# To format all the code:
./format_measures.sh

# To perform a quick performance check
cargo r --release --example bench

# To exercise all the code, showing use cases.
cargo r --example full
```

Before committing any change, except for those in documentation only, this script must be run successfully from the root folder of the repository:
```sh
./build.sh
```
It will:
* clear all the targets,
* format all the code,
* run Clippy with fatal warnings for all the targets,
* and run all the tests.

If successful, the last printed line is "`OK`".
