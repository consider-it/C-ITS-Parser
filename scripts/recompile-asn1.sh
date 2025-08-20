#!/bin/bash
for path in src/standards/asn.1/*.asn; do
    filename=${path#*1/}
    filename=${filename%.as*}
    echo $filename
    rasn_compiler_cli -m ${path} -o src/standards/${filename}.rs
done
for file in src/standards/*.rs; do
    rustfmt +nightly ${file}
done
git apply scripts/asn1.patch
for file in src/standards/*.rs; do
    rustfmt +nightly ${file}
done
