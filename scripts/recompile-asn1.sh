#!/bin/bash
echo "generating code from ASN.1 sources..."
for path in src/standards/asn.1/*.asn; do
    filename=${path#*1/}
    filename=${filename%.as*}
    echo $filename
    rasn_compiler_cli -m ${path} -o src/standards/${filename}.rs
done

echo ""
echo "applying patches..."
for file in src/standards/*.rs; do
    rustfmt +nightly ${file}
done
git apply scripts/asn1.patch

echo ""
echo "formatting code..."
for file in src/standards/*.rs; do
    rustfmt +nightly ${file}
done

echo ""
echo "done!"
