#! /bin/bash

# Util for fetching challenge data.
# Should be run inside of crypto/data directory.
# TODO: have this script run before building, as unit tests will fail without
# https://doc.rust-lang.org/cargo/reference/build-scripts.html
for NUM in 4 6 7; do
curl "https://cryptopals.com/static/challenge-data/${NUM}.txt" > ${NUM}.txt
done
