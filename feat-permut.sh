#!/bin/bash
## Build/ Test/ Clippy all feature permutations
set -eu

mode=${1:-help}
wasm_target="--chrome --headless"

features=(
  geo
  json
  transport
  transport,json
  etsi
  etsi,json
)


runBuild () {
  for feat in "${features[@]}"; do
    echo "cargo build for $feat ..."
    cargo build --all-targets --no-default-features -F $feat
  done
}

runTest () {
  for feat in "${features[@]}"; do
    echo "cargo test for $feat ..."
    cargo test --no-default-features -F $feat
  done
}

runWasmBuild () {
  for feat in "${features[@]}"; do
    echo "wasm-pack build for $feat ..."
    wasm-pack build --target nodejs --no-default-features -F $feat
  done
}

runWasmTest () {
  for feat in "${features[@]}"; do
    echo "wasm-pack test for $feat ..."
    wasm-pack test $wasm_target --no-default-features -F $feat
  done
}

runClippy () {
  for feat in "${features[@]}"; do
    echo "clippy for $feat ..."
    cargo clippy --all-targets --no-default-features -F $feat -- -Wclippy::pedantic
  done
}


case $mode in
build)
  runBuild
  ;;
clippy)
  runClippy
  ;;
test)
  runTest
  ;;
wasm-test)
  runWasmTest
  ;;
wasm-build)
  runWasmBuild
  ;;
all)
  runBuild
  runClippy
  runTest
  runWasmBuild
  runWasmTest
  ;;
*)
  echo "Usage: $0 [build | clippy | test | wasm-test | wasm-build | all]"
esac
