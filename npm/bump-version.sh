#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "Usage: ./bump-version.sh <version>"
  echo "Example: ./bump-version.sh 0.2.0"
  exit 1
fi

VERSION=$1
echo "Bumping all packages to $VERSION"

# Update main package
npm version "$VERSION" --no-git-tag-version

# Update platform packages
for platform in darwin-arm64 darwin-x64 linux-x64 win32-x64; do
  cd "platforms/$platform"
  npm version "$VERSION" --no-git-tag-version
  cd ../..
done

# Update optionalDependencies versions in main package.json
node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
for (const dep of Object.keys(pkg.optionalDependencies)) {
  pkg.optionalDependencies[dep] = '$VERSION';
}
fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2) + '\n');
"

echo "Done! All packages updated to $VERSION"
