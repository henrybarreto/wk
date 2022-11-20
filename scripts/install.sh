#!/usr/bin/env sh

build_path="$(pwd)/target/release/wk"
binary_path="$HOME/.local/bin/wk"

if [ -f "$binary_path" ]; then
    echo "Wk is already installed!"
    echo "Uninstalling wk from $binary_path"
    rm "$binary_path"
    echo "Uninstalled!"
fi

echo "Installing wk to $binary_path"
echo "#!/usr/bin/env sh" >> "$binary_path"
echo "" >> "$binary_path"
echo "path=\$($build_path \$@)" >> "$binary_path"
echo "if [ \"\$path\" != \"\" ]; then" >> "$binary_path"
echo "    cd \"\$path\"" >> "$binary_path"
echo "    \$SHELL" >> "$binary_path"
echo "fi" >> "$binary_path"
chmod +x "$binary_path"
echo "Installed!"
