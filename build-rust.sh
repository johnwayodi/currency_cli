#!/bin/bash

# Build the Rust project
cargo build --release

# Copy the executable to the root of the project
cp target/release/currency_cli .
