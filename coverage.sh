#!/usr/bin/env bash
set -ex

# Build kcov
KCOV=34
wget https://github.com/SimonKagstrom/kcov/archive/v$KCOV.tar.gz
tar xfz v$KCOV.tar.gz
cd kcov-$KCOV
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=$HOME ..
make install

# Run all tests - and measure coverage
cargo test --no-run --message-format=json |
    jq -r 'select(.profile.test == true) | .filenames[]' |
    xargs -n 1 $HOME/bin/kcov --coveralls-id=$COVERALLS --exclude-pattern=/.cargo,/usr/lib --verify $HOME/coverage
