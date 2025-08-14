#!/bin/sh
DIR_PATH="$(dirname "$(realpath "$0")")"
BIN_NAME=$(basename "$DIR_PATH")
cd $DIR_PATH
cargo build --release
mv ./target/release/$BIN_NAME ~/.local/bin/$BIN_NAME
