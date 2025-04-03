#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    sudo apt-get update
    sudo apt-get install -y gcc-multilib
elif [[ "$OSTYPE" == "darwin"* ]]; then
    brew update
    brew install mingw-w64
fi

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
