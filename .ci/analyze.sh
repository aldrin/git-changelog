#!/usr/bin/env bash
# Copyright 2017 Aldrin J D'Souza.
# Licensed under the MIT License <https://opensource.org/licenses/MIT>

set -ex

# Install rustfmt and clippy
which rustfmt || cargo install rustfmt-nightly
which cargo-clippy || cargo install clippy

# Check code style
cargo fmt -- --write-mode=diff
cargo clippy
