#!/usr/bin/env bash
# Copyright 2017-2018 by Aldrin J D'Souza.
# Licensed under the MIT License <https://opensource.org/licenses/MIT>

set -ex

# The coverage data directory
declare -r coverage="$TRAVIS_BUILD_DIR/target/coverage"

# If we don't have kcov already
if [[ ! -f $HOME/bin/kcov ]]; then
    # Download and build it
    KCOV=34
    wget https://github.com/SimonKagstrom/kcov/archive/v$KCOV.tar.gz
    tar xfz v$KCOV.tar.gz
    cd kcov-$KCOV
    mkdir build
    cd build
    cmake -DCMAKE_INSTALL_PREFIX=$HOME ..
    make install
    cd $TRAVIS_BUILD_DIR
fi

# Run all tests - and measure coverage
cargo test --no-run --message-format=json |
    jq -r 'select(.profile.test == true) | .filenames[]' |
    xargs -n 1 $HOME/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify $coverage

# Publish reports to code-code
bash <(curl -s https://codecov.io/bash)
