#!/usr/bin/env bash

APP_NAME="vmail-cli"
OUT_PATH="shell"
SHELLS=("zsh:_${APP_NAME}" "bash:${APP_NAME}.bash" "fish:${APP_NAME}.fish")

for shell in ${SHELLS[@]}; do
  file="${shell#*:}"
  shell="${shell%:*}"
  path="$OUT_PATH/$file"
  echo "Produce $shell completion in $path"
  cargo run -- completions $shell > $path
done
