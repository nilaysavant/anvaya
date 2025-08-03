default: build

# Default library build. PS: This also runs on Github Actions CI.
build:
    cargo build

# Run tests. PS: This also runs on Github Actions CI.
test:
    cargo test --all-features

# Document the repo incl all features, examples. But no external deps. Also open it in web browser.
docs:
    cargo doc --all-features --examples --no-deps --open