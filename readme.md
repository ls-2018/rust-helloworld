- rustup toolchain install nightly-aarch64-apple-darwin
- cargo +nightly fmt



- RUSTFLAGS="-A warnings" cargo test --color=always --profile test --package rust-helloworld --bin rust-helloworld garden::rc::_ref --no-fail-fast --config env.RUSTC_BOOTSTRAP=\"1\" -- --exact -Z unstable-options --show-output
