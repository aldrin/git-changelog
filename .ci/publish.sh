#!/usr/bin/env bash
# Copyright 2017 Aldrin J D'Souza.
# Licensed under the MIT License <https://opensource.org/licenses/MIT>
set -ex

# Login to crates.io
cargo login $CRATES

# Do it.
cargo publish
