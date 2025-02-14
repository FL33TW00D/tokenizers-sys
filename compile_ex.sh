#!/bin/bash
cargo build --release
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
clang -arch arm64 \
    examples/c/tokenize.c \
    -Wall \
    -Wextra \
    -L target/release \
    -ltokenizers_sys \
    -lc++ \
    -framework Security \
    -framework Foundation \
    -rpath @executable_path/../target/release \
    -o examples/tokenize 
echo "Build completed with status: $?"
echo "Run ./examples/tokenize"
