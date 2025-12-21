#!/bin/bash
# Script to verify HyprQur'an Arch Linux package contents

echo "Verifying HyprQur'an Arch Linux package"

# Wait for package to be built
while [ ! -f "hyprquran-*.pkg.tar.zst" ]; do
    echo "Waiting for package to be built..."
    sleep 10
done

# Get package file name
PKG_FILE=$(ls hyprquran-*.pkg.tar.zst)

echo "Found package: $PKG_FILE"

# Create temporary directory for extraction
TMP_DIR=$(mktemp -d)

# Extract package
echo "Extracting package..."
tar -xf "$PKG_FILE" -C "$TMP_DIR"

echo "Package contents:"
find "$TMP_DIR" -type f | sort

# Verify key files exist
echo ""
echo "Verifying key files:"
if [ -f "$TMP_DIR/usr/bin/hyprquran" ]; then
    echo "✓ Main binary found"
else
    echo "✗ Main binary missing"
fi

if [ -f "$TMP_DIR/usr/bin/hyprquran-import" ]; then
    echo "✓ Import binary found"
else
    echo "✗ Import binary missing"
fi

if [ -f "$TMP_DIR/usr/bin/hyprquran-tanzil-import" ]; then
    echo "✓ Tanzil import binary found"
else
    echo "✗ Tanzil import binary missing"
fi

if [ -f "$TMP_DIR/usr/share/applications/hyprquran.desktop" ]; then
    echo "✓ Desktop file found"
else
    echo "✗ Desktop file missing"
fi

if [ -f "$TMP_DIR/usr/share/icons/hicolor/scalable/apps/hyprquran.svg" ]; then
    echo "✓ Icon file found"
else
    echo "✗ Icon file missing"
fi

# Clean up
rm -rf "$TMP_DIR"

echo ""
echo "Package verification completed!"