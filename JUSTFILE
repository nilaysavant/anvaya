default: build

# Default library build.
build:
    cargo build

# Run tests.
test:
    cargo test --all-features

# Document the repo incl all features, examples. But no external deps. Also open it in web browser.
docs:
    cargo doc --all-features --examples --no-deps --open