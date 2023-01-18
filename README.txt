Cleaning up space:
- The rust target directories take up a lot of space, especially with profiling.
- Periodically run:
find . -type d -name target | xargs rm -rf
find . -type f -name "*.log"|xargs rm

Profiling on OS X:
- https://crates.io/crates/cargo-instruments
- https://github.com/cmyr/cargo-instruments
- cargo install cargo-instruments
- add to Cargo.toml:
[profile.release]
debug = 1
- cargo instruments --release -t time
