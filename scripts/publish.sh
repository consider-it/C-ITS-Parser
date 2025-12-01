#!/bin/sh
set -eu

org="@consider-it/"
postfix=${1-}
registry="https://npm.pkg.github.com"
pkgdir="./pkg"

echo "packaging for org '$org' with suffix '$postfix'..."

## add npmrc
cp -f scripts/npmrc-ci $pkgdir/.npmrc

# patch package name and add registry
cd $pkgdir
if [ ! -f package.json.orig ]; then
  mv package.json package.json.orig
fi
jq ".name = (\"${org}\" + .name + \"${postfix}\") | .publishConfig.registry = \"${registry}\"" package.json.orig > package.json

# publish with npm
npm publish
