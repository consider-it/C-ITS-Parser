# How to Release

The changelog is handled by [git-cliff](https://git-cliff.org) and shall only be auto-generated.

## Required Tools

To update the changelog, install git-cliff using: `cargo install git-cliff`.

To pack the NPM packet, install wasm-back using: `cargo install wasm-pack`.

## Make a Release

- Bump the version in Cargo.toml to reflect the nature of the changes
- Use `./prepare-release.sh` to update the changelog, create a release commit and add a tag
- If everything looks fine, push the changes. The CI will publish new NPM versions

## Manual Publishing

We publish our internal NPM packages to the GitHub package registry.
To do it manually:

```shell
export NODE_AUTH_TOKEN="your_PAT"

# build and publish standard version
rm -r ./pkg/ && wasm-pack build --release --target web
./scripts/publish.sh

# build and publish nodejs version
rm -r ./pkg/ && wasm-pack build --release --target nodejs
./scripts/publish.sh '-node'
```
