# Example plugin for Mesh LLM

A minimal external [Mesh](https://github.com/Mesh-LLM/mesh-llm) plugin: a
Rust binary that connects to the Mesh host over the plugin control protocol,
binds a loopback HTTP health server, and answers host health checks. Use it as
the starting point for your own plugin.

## Layout

- `plugin.toml` — plugin manifest: name, version, description and the command
  the host launches.
- `src/lib.rs` — plugin entrypoint: argument parsing, the loopback HTTP
  server, the `plugin!` manifest declaration (capabilities, inference
  provider, health hook) and the host runtime loop.
- `src/server.rs` — the plugin's HTTP surface with its unit tests.
- `.github/workflows/ci.yml` — formatting, Clippy, tests and build on Linux,
  macOS and Windows.
- `.github/workflows/release.yml` — tagged release builds publishing
  `tar.gz` archives for Linux/macOS and a `zip` archive for Windows.

## Build and test

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Install into Mesh

Build or download the binary, then reference it from your Mesh
configuration:

```toml
[[plugin]]
name = "example"
command = "/absolute/path/to/mesh-example"
```

The plugin's only configuration in this template is the greeting the health
endpoint returns, set with `--greeting` or `EXAMPLE_GREETING`.

## Release

Push a `v*` tag (or dispatch the Release workflow against an existing tag).
The workflow builds and tests on Linux, macOS and Windows, packages
`plugin.toml` and this README next to the binary, and publishes both a
stable-named archive (`mesh-example-<target>.tar.gz` or `.zip`) and a
versioned one (`mesh-example-<version>-<target>.…`) to the GitHub release.
