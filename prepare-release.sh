#!/bin/sh
set -eu

# read version from Cargo.toml
version=$(sed -rn 's/^version = "(.*)"/\1/p' Cargo.toml)

prev_git_tag=$(git describe --tags --abbrev=0)
if [ "$prev_git_tag" = "v${version}" ]; then
  echo "ERROR: bump version in Cargo.toml first"
  exit 1
fi

echo "updating changelog for upcoming $version..."
git-cliff -o CHANGELOG.md --tag $version
git add CHANGELOG.md

echo "adding commit and tag..."
git commit -m "chore(release): Bump version number, update changelog"
git tag v${version}
