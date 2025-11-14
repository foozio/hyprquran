#!/bin/bash
# Script to test HyprQur'an Arch Linux package

echo "Testing HyprQur'an Arch Linux package"

# Check if package file exists
if [ ! -f "hyprquran-*.pkg.tar.zst" ]; then
    echo "Package file not found!"
    exit 1
fi

# Get package file name
PKG_FILE=$(ls hyprquran-*.pkg.tar.zst)

echo "Found package: $PKG_FILE"

# Extract package to temporary directory for inspection
mkdir -p test_install
tar -xf "$PKG_FILE" -C test_install

echo "Package contents:"
find test_install -type f | sort

# Clean up
rm -rf test_install

echo "Package test completed successfully!"