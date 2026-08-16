#!/usr/bin/env bash
set -e

echo "🔨 Preparing Haptic.app..."
./bundle.sh

DMG_NAME="Haptic.dmg"
STAGING_DIR="dmg_staging"

echo "📦 Creating DMG installer ($DMG_NAME)..."
rm -rf "$STAGING_DIR" "$DMG_NAME"
mkdir -p "$STAGING_DIR"

cp -R "Haptic.app" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

# Create compressed DMG image
hdiutil create -volname "Haptic" \
    -srcfolder "$STAGING_DIR" \
    -ov -format UDZO \
    "$DMG_NAME"

rm -rf "$STAGING_DIR"

echo "🎉 $DMG_NAME created successfully!"
ls -lh "$DMG_NAME"
