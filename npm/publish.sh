#!/bin/bash
set -e

VERSION=$(node -p "require('./package.json').version")
echo "Publishing version $VERSION"

# Publish platform packages first
for platform in darwin-arm64 darwin-x64 linux-x64 win32-x64; do
  echo ""
  echo "Publishing @hash-omikuji/$platform..."
  cd "platforms/$platform"
  npm publish --access public
  cd ../..
done

# Publish main package
echo ""
echo "Publishing hash-omikuji..."
npm publish --access public

echo ""
echo "Done! Published hash-omikuji@$VERSION"
