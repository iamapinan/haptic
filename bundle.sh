#!/usr/bin/env bash
set -e

echo "🔨 Building release binary..."
cargo build --release

# Ensure AppIcon.icns exists
if [ ! -f "AppIcon.icns" ]; then
    echo "🎨 Generating AppIcon.icns..."
    cargo run --bin gen_icon
fi

APP_NAME="Haptic.app"
CONTENTS_DIR="$APP_NAME/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "📦 Creating $APP_NAME bundle..."
rm -rf "$APP_NAME"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

cp target/release/haptic-mac "$MACOS_DIR/haptic-mac"
chmod +x "$MACOS_DIR/haptic-mac"
cp Info.plist "$CONTENTS_DIR/Info.plist"
cp AppIcon.icns "$RESOURCES_DIR/AppIcon.icns"

echo "🔏 Code signing $APP_NAME..."
codesign --force --deep --sign - "$APP_NAME"

echo "✅ Created $APP_NAME successfully!"
