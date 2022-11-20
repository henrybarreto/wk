#!/usr/bin/env sh

binary_path="$HOME/.local/bin/wk"

if [ -f "$binary_path" ]; then
    echo "Uninstalling wk from $binary_path"
    rm "$binary_path"
    echo "Uninstalled!"
else
  echo "Wk is not installed!"
fi