#!/bin/bash

set -euxo pipefail
cd "$(git rev-parse --show-toplevel)"

VERSION=$(
    cargo metadata --offline --no-deps --format-version 1 \
    | jq -r '.packages[] | select(.name == "bstringify") | .version'
)
mv target/doc{,2}
mkdir target/doc
mv target/doc2 target/doc/$VERSION
{
    echo '<!DOCTYPE html><html><head><title>bstringify</title><meta http-equiv = "refresh" content = "0; url = '
    echo $VERSION/
    echo 'bstringify/" /></head><body></body></html>'
} > target/doc/index.html
