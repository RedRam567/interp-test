#!/bin/sh
set -x
NAME=interp_test
# ASSETS="assets"
package_dir="$CARGO_TARGET_DIR/package"
# package_dir="package"

zip_file="$package_dir/$NAME.zip"
exec="$CARGO_TARGET_DIR/release/$NAME"

echo "Building for Linux"
cargo build --release
mkdir "$package_dir"
rm "$zip_file"
zip -j "$zip_file" "$exec" 1>&2
# zip -r "$zip_file" "$ASSETS" 1>&2
echo "$zip_file"


zip_file="$package_dir/$NAME.exe.zip"
exec="$CARGO_TARGET_DIR/x86_64-pc-windows-gnu/release/$NAME.exe"
echo "Building for Windows"
cargo build --release --target x86_64-pc-windows-gnu
mkdir "$package_dir"
rm "$zip_file"
zip -j "$zip_file" "$exec" 1>&2
# zip -r "$zip_file" "$ASSETS" 1>&2
echo "$zip_file"