#!/bin/bash
set -e

# Usage: ./copy-binary.sh <platform> <binary-path>
# Example: ./copy-binary.sh darwin-arm64 ../rust/target/release/hash-omikuji

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: ./copy-binary.sh <platform> <binary-path>"
  echo "Platforms: darwin-arm64, darwin-x64, linux-x64, win32-x64"
  exit 1
fi

PLATFORM=$1
BINARY=$2

if [ ! -f "$BINARY" ]; then
  echo "Binary not found: $BINARY"
  exit 1
fi

cp "$BINARY" "platforms/$PLATFORM/"
echo "Copied $BINARY to platforms/$PLATFORM/"
