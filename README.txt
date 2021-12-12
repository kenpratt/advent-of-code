Profiling on OS X:
- https://crates.io/crates/cargo-instruments
- https://github.com/cmyr/cargo-instruments
- cargo install cargo-instruments
- add to Cargo.toml:
[profile.release]
debug = 1
- cargo instruments --release -t time