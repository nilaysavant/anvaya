default: build

# Default library build.
build:
    cargo build

# Run tests.
test:
    cargo test --all-features

# Document the repo incl examples and open it in web browser.
doc-example:
    cargo doc --examples --open