#!/usr/bin/env sh

script() {
    echo "#!/usr/bin/env sh" >> "$binary_path"
    echo "" >> "$binary_path"
    echo "path=\$($build_path \$@)" >> "$binary_path"
    echo "if [ \"\$path\" != \"\" ]; then" >> "$binary_path"
    echo "    echo \"moving to \$path\"" >> "$binary_path"
    echo "    cd \"\$path\"" >> "$binary_path"
    echo "    \$SHELL" >> "$binary_path"
    echo "    echo \"exiting from workspace \$path\"" >> "$binary_path"
    echo "fi" >> "$binary_path"
    chmod +x "$binary_path"
}

build_path="$(pwd)/target/release/wk"

if [ ! -f "$build_path" ]; then
    echo "wk not found at $build_path"
    echo "Running cargo build --release"
    if cargo build --release; then
        echo "wk built successfully"
    else
        echo "Failed to build wk"
        exit 1
    fi
fi

binary_path="$HOME/.local/bin/wk"

if [ -f "$binary_path" ]; then
    echo "wk is already installed!"
    echo "Updating wk from $binary_path"
    rm "$binary_path"
    script
    echo "wk updated successfully"
else
    echo "Installing wk to $binary_path"
    script
    echo "wk installed successfully"
fi
