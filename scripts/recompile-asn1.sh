#!/bin/bash
set -euo pipefail

rootdir=$(pwd)

echo "generating code from ASN.1 sources..."
for path in $rootdir/autogen/asn.1/*.asn; do
    filename=${path#*1/}
    filename=${filename%.as*}
    echo $filename
    rasn_compiler_cli -m ${path} -o src/standards/${filename}.rs
done

echo ""
for file in $rootdir/src/standards/*.rs; do
    rustfmt +nightly ${file}
done

# fix import path for CDD 1.3.1
files=("$rootdir/src/standards/cam_1_4_1.rs" "$rootdir/src/standards/cpm_1.rs" "$rootdir/src/standards/denm_1_3_1.rs" "$rootdir/src/standards/ivim_2_1_1.rs")
for file in ${files[@]}; do
    echo "patching CDD 1.3.1 paths in $file"
    sed -i '' 's/use super::its_container/use super::super::cdd_1_3_1_1::its_container/' $file
done

# fix import path for CDD 2.2.1
for file in $rootdir/src/standards/*_*.rs; do
    echo "patching CDD 2.2.1 paths in $file"
    sed -i '' 's/use super::etsi_its_cdd/use super::super::cdd_2_2_1::etsi_its_cdd/' $file
done

# fix import path for DSRC
for file in $rootdir/src/standards/*_*.rs; do
    echo "patching DSRC 2.2.1 paths in $file"
    sed -i '' 's/use super::etsi_its_dsrc/use super::super::dsrc_2_2_1::etsi_its_dsrc/' $file
done

# apply additional patch files
for file in $rootdir/scripts/asn1_rs-patches/*.patch; do
    filename=$(basename $file)
    name=${filename%.*}
    echo "applying additional patches for $name"
    git apply $file
done

echo ""
echo "formatting code..."
for file in $rootdir/src/standards/*.rs; do
    rustfmt +nightly ${file}
done

echo ""
echo "done!"
