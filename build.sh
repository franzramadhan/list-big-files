#!/bin/bash

cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo ""
    echo "Usage:"
    echo "  ./target/release/list_big_files [directory] [min_size_mb]"
    echo ""
    echo "Examples:"
    echo "  ./target/release/list_big_files ."
    echo "  ./target/release/list_big_files /Users/username 50"
    echo ""
    echo "Running: ./target/release/list_big_files . 10"
    ./target/release/list_big_files . 10
fi