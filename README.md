# binstall-extra-fixture

A small Rust CLI crate for exercising `cargo-binstall` extra-file
installation from GitHub release archives.

The published release assets include:

- the compiled `binstall-extra-fixture` binary
- a generated man page
- generated shell completions for `bash`, `fish`, and `zsh`

Each release archive is assembled with the default extra-file layout that `cargo-binstall`
recognizes automatically:

```text
binstall-extra-fixture-{target}-v{version}/
  binstall-extra-fixture
  man/
    man1/
      binstall-extra-fixture.1
  completions/
    bash/
      binstall-extra-fixture
    fish/
      binstall-extra-fixture.fish
    zsh/
      _binstall-extra-fixture
```

## Release flow

Tagging `v*` triggers the GitHub Actions workflow, which for each supported target:

1. builds the release binary
2. generates the man page and shell completions
3. assembles the release directory in the expected layout
4. creates a `.tar.gz` archive
5. uploads the archive to the GitHub release

Supported targets:

- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`

## Create a release

```sh
git tag v0.1.0
git push origin v0.1.0
```

## Test with cargo-binstall

```sh
tmpdir=$(mktemp -d)
CARGO_HOME="$tmpdir" cargo-binstall -y --strategies crate-meta-data binstall-extra-fixture
find "$tmpdir"
```

## Local packaging check

On a supported host target you can build and package an archive locally:

```sh
cargo build --release
cargo run --bin package-release -- \
  --target "$(rustc -vV | sed -n 's/^host: //p')" \
  --version 0.1.0 \
  --binary target/release/binstall-extra-fixture
tar -tzf dist/binstall-extra-fixture-$(rustc -vV | sed -n 's/^host: //p')-v0.1.0.tar.gz
```
