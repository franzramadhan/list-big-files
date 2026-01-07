#!/bin/bash

cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo ""
    echo "Usage:"
    echo "  ./target/release/list-big-files [directory] [min_size_mb]"
    echo ""
    echo "Examples:"
    echo "  ./target/release/list-big-files ."
    echo "  ./target/release/list-big-files /Users/username 50"
    echo ""
    echo "Running: ./target/release/list-big-files . 10"
    ./target/release/list-big-files . 10
fi