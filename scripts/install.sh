#!/bin/bash

if ! command -v cargo &> /dev/null
then
    echo "Rust and Cargo must be installed first."
    exit 1
fi

cargo install --path .
echo "OxideBuild installed successfully!"
