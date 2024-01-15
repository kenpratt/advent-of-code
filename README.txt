Cleaning up space:
- The rust target directories take up a lot of space, especially with profiling.
- Periodically run:
find . -type d -name target | xargs rm -rf
find . -type f -name "*.log" | xargs rm

Profiling on OS X:
- https://crates.io/crates/cargo-instruments
- https://github.com/cmyr/cargo-instruments
- cargo install cargo-instruments
- add to Cargo.toml:
[profile.release]
debug = 1
- cargo instruments --release -t time

Profiling on Windows in WSL2:
- https://github.com/wkennedy/wsl2-rust-profiling
- https://ntietz.com/blog/profiling-rust-programs-the-easy-way/
  - using https://github.com/flamegraph-rs/flamegraph
- add to Cargo.toml:
[profile.release]
debug = 1
