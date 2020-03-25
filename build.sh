#!/usr/bin/env bash

set -e

TARGET=$1
BASE_DIR=$(pwd)

echo "trying to build for $TARGET"

if [ "$TARGET" = "python" ]; then
    echo "#### setting up environment"
    cd "$BASE_DIR"/python_glue
    python -m virtualenv venv
    source ./venv/bin/activate
    pip install -r ./requirements.txt
    cd "$BASE_DIR"
    echo "#### building python"
    maturin develop
    echo "done"
    exit 0
fi

if [ "$TARGET" = "js" ]; then
    wasm-pack build
    echo "done"
    exit 0
fi

echo "unrecognized target. options are: js, python"
exit 2
