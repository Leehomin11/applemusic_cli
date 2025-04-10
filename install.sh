set -e

echo "📦 Building release binary..."
cargo build --release

TARGET="$HOME/.cargo/bin/tc"

echo "🧹 Removing old binary (if exists)..."
rm -f "$TARGET"

echo "📁 Copying binary as 'tc' to $TARGET"
cp target/release/applemusic_cli "$TARGET"

chmod +x "$TARGET"

echo ""
echo "✅ Installed successfully!"
echo "👉 You can now run the tool anywhere using: tc"


