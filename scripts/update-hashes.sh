#!/usr/bin/env bash
set -euo pipefail

VERSION=$(grep 'version =' flake.nix | head -1 | cut -d '"' -f2)
echo "Current version: $VERSION"

ARCHITECTURES=("darwin-x86_64" "darwin-aarch64" "linux-x86_64" "linux-aarch64")

for arch in "${ARCHITECTURES[@]}"; do
  echo "Fetching hash for $arch..."
  BINARY_NAME="fuckmit-$arch-$VERSION.tar.gz"
  URL="https://github.com/mingeme/fuckmit/releases/download/v$VERSION/$BINARY_NAME"
  
  HASH=$(nix-prefetch-url "$URL" 2>/dev/null || echo "HASH_NOT_FOUND")
  
  if [ "$HASH" != "HASH_NOT_FOUND" ]; then
    echo "Hash for $arch: $HASH"
    
    sed -i '' "s|\"$arch\" = \"[0-9a-z]*\"|\"$arch\" = \"$HASH\"|g" flake.nix
  else
    echo "Warning: Could not fetch hash for $arch"
  fi
done

echo "Hash values updated in flake.nix"
