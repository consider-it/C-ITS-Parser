#!/bin/bash
set -euo pipefail

rootdir=$(pwd)

wget_file() {
    local out_filename=$1
    local url=$2
    wget --content-disposition -nv -O $out_filename $url
}

concat_parts() {
    local name=$1
    # concat with additional newline in between each part
    rm -f $name.asn
    for f in $name-part*.asn; do
        (cat "${f}"; echo) >> $name.asn;
    done
    rm $name-part*.asn
}

echo "fetching ASN.1 sources..."
cd $rootdir/autogen/asn.1/
rm -f *.asn

wget_file cdd_1_3_1_1.asn       https://forge.etsi.org/rep/ITS/asn1/cdd_ts102894_2/-/raw/v1.3.1_1/ITS-Container.asn
wget_file cam_1_4_1.asn         https://forge.etsi.org/rep/ITS/asn1/cam_en302637_2/-/raw/v1.4.1/CAM-PDU-Descriptions.asn
wget_file denm_1_3_1.asn        https://forge.etsi.org/rep/ITS/asn1/denm_en302637_3/-/raw/v1.3.1/DENM-PDU-Descriptions.asn
cp -f $rootdir/scripts/asn1-patches/CPM-TR-103562-2_1_1.asn cpm_1.asn

wget_file ivim_2_1_1-part1.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.1.1/IVIM-PDU-Descriptions.asn
wget_file ivim_2_1_1-part2.asn  "https://standards.iso.org/iso/ts/19321/ed-2/en/ISO19321IVIv2.asn"
    cp -f $rootdir/scripts/asn1-patches/ISO14906-EfcDsrcApplicationv6-excerpt.asn ivim_2_1_1-part3.asn
    wget_file ivim_2_1_1-part4.asn https://standards.iso.org/iso/14816/ISO14816%20ASN.1%20repository/ISO14816_AVIAEINumberingAndDataStructures.asn
    wget_file ivim_2_1_1-part5.asn https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.1.1/iso-patched/ISO24534-3_ElectronicRegistrationIdentificationVehicleDataModule-patched.asn
    wget_file ivim_2_1_1-part6.asn https://standards.iso.org/iso/ts/17419/TS%2017419%20ASN.1%20repository/TS17419_2014_CITSapplMgmtIDs.asn
    wget_file ivim_2_1_1-part7.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.1.1/iso-patched/ISO14823-missing.asn


wget_file cdd_2_2_1.asn         https://forge.etsi.org/rep/ITS/asn1/cdd_ts102894_2/-/raw/V2.2.1/ETSI-ITS-CDD.asn
wget_file denm_2_2_1.asn        https://forge.etsi.org/rep/ITS/asn1/denm_ts103831/-/raw/v2.2.1/DENM-PDU-Descriptions.asn

wget_file mapem_2_2_1.asn       https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/MAPEM-PDU-Descriptions.asn
wget_file spatem_2_2_1.asn      https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/SPATEM-PDU-Descriptions.asn
wget_file srem_2_2_1.asn        https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/SREM-PDU-Descriptions.asn
wget_file ssem_2_2_1.asn        https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/SSEM-PDU-Descriptions.asn
wget_file dsrc_2_2_1-part1.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/DSRC.asn
wget_file dsrc_2_2_1-part2.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/DSRC-region.asn
wget_file dsrc_2_2_1-part3.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/DSRC-addgrp-C.asn

wget_file ivim_2_2_1-part1.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.2.1/IVIM-PDU-Descriptions.asn
wget_file ivim_2_2_1-part2.asn  https://standards.iso.org/iso/ts/19321/ed-3/en/ISO19321IVIv3.1.asn
    cp -f $rootdir/scripts/asn1-patches/ISO17573-3-excerpt.asn ivim_2_2_1-part3.asn
    wget_file ivim_2_2_1-part4.asn  https://forge.etsi.org/rep/ITS/asn1/is_ts103301/-/raw/v2.1.1/iso-patched/ISO14823-missing.asn
wget_file ivim_2_2_1-part5.asn  https://standards.iso.org/iso/ts/19321/ed-3/en/ISO19321IVI-IS.asn

wget_file cpm_2_1_1-part1.asn  https://forge.etsi.org/rep/ITS/asn1/cpm_ts103324/-/raw/v2.1.1/asn/CPM-OriginatingStationContainers.asn
wget_file cpm_2_1_1-part2.asn  https://forge.etsi.org/rep/ITS/asn1/cpm_ts103324/-/raw/v2.1.1/asn/CPM-PDU-Descriptions.asn
wget_file cpm_2_1_1-part3.asn  https://forge.etsi.org/rep/ITS/asn1/cpm_ts103324/-/raw/v2.1.1/asn/CPM-PerceivedObjectContainer.asn
wget_file cpm_2_1_1-part4.asn  https://forge.etsi.org/rep/ITS/asn1/cpm_ts103324/-/raw/v2.1.1/asn/CPM-PerceptionRegionContainer.asn
wget_file cpm_2_1_1-part5.asn  https://forge.etsi.org/rep/ITS/asn1/cpm_ts103324/-/raw/v2.1.1/asn/CPM-SensorInformationContainer.asn

echo ""
echo "applying patches..."
cd $rootdir
for f in $rootdir/scripts/asn1-patches/*.patch; do
    echo applying $f...
    git apply $f
done

echo ""
echo "concatenating parts..."
cd $rootdir/autogen/asn.1/
concat_parts ivim_2_1_1
concat_parts dsrc_2_2_1
concat_parts ivim_2_2_1
concat_parts cpm_2_1_1

echo ""
echo "done!"
