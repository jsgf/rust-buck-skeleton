#!/bin/bash

# Set default to nightly to enable save-analysis
rustup override set nightly

# Set up third-party dependencies from crates.io
cd third-party/tp
RUSTFLAGS=-Zsave-analysis cargo build --release
