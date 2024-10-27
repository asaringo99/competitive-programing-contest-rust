#!/bin/bash

# .envファイルを読み込む
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

export PATH="$PATH:$COMPETITIVE_PROGRAMING_CONTEST_RUST_PROJECT_BIN_PATH"
