#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ENVPATH="$SCRIPT_DIR/.env"

# .envファイルを読み込む
if [ -f $ENVPATH ]; then
    export $(grep -v '^#' $ENVPATH | xargs)
fi

export PATH="$PATH:$COMPETITIVE_PROGRAMING_CONTEST_RUST_PROJECT_BIN_PATH"
