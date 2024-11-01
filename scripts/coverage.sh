
#!/bin/bash
# Set environment variables for code coverage
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export RUSTDOCFLAGS="-Cpanic=abort"
export LLVM_PROFILE_FILE="target/debug/%p-%m.profraw"

# Clean previous data and build
cargo clean

# Run tests
cargo test

# Generate coverage report
grcov . --binary-path ./target/debug/ -s . -t lcov --ignore-not-existing --ignore "/*" -o lcov.info

# Check if the upload flag is provided for Codecov
if [[ "$1" == "--upload" ]]; then
    # Upload the report to Codecov
    if [[ -z "$CODECOV_TOKEN" ]]; then
        echo "CODECOV_TOKEN is not set. Please upload the token to GitHub Secrets."
        exit 1
    fi

    echo "Uploading report to Codecov..."
    bash <(curl -s https://codecov.io/bash) -t "$CODECOV_TOKEN" -f lcov.info
fi
