# wmde-greeter
libcosmic greeter for greetd, which can be run inside wmde-comp

## Development

This project uses [just](https://github.com/casey/just) as a command runner.

### Available Commands

#### Building
- `just build-debug` - Compile with debug profile
- `just build-release` - Compile with release profile (default)
- `just build-vendored` - Compile release profile with vendored dependencies
    - Requires vendoring first, which can be done with `just vendor`

#### Testing & Development
- `just mock` - Run greeter in a windowed compositor for quick testing (builds and runs the mock server example)
- `just run` - Run with debug logs (`RUST_LOG=debug` and `RUST_BACKTRACE=full`)

#### Code Quality
- `just check` - Run clippy linter with pedantic warnings
- `just check-json` - Run clippy with JSON output format

#### Installation
The two roles of the binary install separately, matching the two packages the PKGBUILD builds.

- `just install-locker` - the screen locker: the binary, its PAM stack and the background
  picture. This is all a WMDE session needs.
- `just install-dm` - the display manager on top: the daemon, the start script, the D-Bus
  policy, sysusers/tmpfiles, both systemd units and the greetd config.
- `just install` - both of the above
- `just uninstall` - Remove all installed files

#### Cleanup
- `just clean` - Run `cargo clean`
- `just clean-dist` - Run `cargo clean` and remove vendored dependencies

#### Vendoring
- `just vendor` - Vendor dependencies locally and create vendor.tar
- `just vendor-extract` - Extract vendored dependencies from vendor.tar
