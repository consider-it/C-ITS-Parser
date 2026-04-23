#!/bin/bash
## Build/ Test/ Clippy all feature permutations
set -eu

mode=${1:-help}
wasm_target="--chrome --headless"
extended=${2:-false}

features_base=(
  geo
  json
  transport
  transport,json
  v2x
  v2x,json
)
features_more=(
  cam
  cpm
  denm
  ivim
  mapem
  spatem
  srem
  ssem
  cpm_1
  denm_1_3_1
  ivim_2_1_1
  cam_1_4_1
  cpm_2_1_1
  denm_2_2_1
  ivim_2_2_1
  mapem_2_2_1
  spatem_2_2_1
  srem_2_2_1
  ssem_2_2_1
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

if [ $extended == "--more" ]; then
  echo "running with extended feature list"
  features=( "${features_base[@]}" "${features_more[@]}" )
else
  echo "running standard feature list"
  features=( "${features_base[@]}" )
fi

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
  echo "Usage: $0 [build | clippy | test | wasm-test | wasm-build | all] [--more]"
esac
