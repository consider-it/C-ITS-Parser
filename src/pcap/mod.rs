pub(crate) fn remove_pcap_headers(data: &[u8]) -> Result<&[u8], String> {
    remove_radiotap_hdr(data).and_then(remove_80211_hdr).and_then(remove_llc_hdr)
}

fn remove_radiotap_hdr(data: &[u8]) -> Result<&[u8], String> {
    /*
     * Radiotap Header has the following format
     * - u_int8_t   header version (currently always zero)
     * - u_int8_t   (padding for byte alignment);
     * - u_int16_t  entire header length;
     * - u_int32_t  bitmask which subsequent data is present
     */

    let radiotap_version: u8 = data[0];
    if radiotap_version != 0 {
        return Err(format!("Unknown header version {:#x}", radiotap_version));
    }

    let hdr_len: usize = data[2].into();
    let (_, remaining) = data.split_at(hdr_len);

    Ok(remaining)
}

fn remove_80211_hdr(data: &[u8]) -> Result<&[u8], String> {
    /*
     * IEEE 802.11 Header has the following format (26-32 bytes)
     * - 2 bytes frame control
     *      byte 0: .... ..00: Version 0
     *      byte 0: .... 10..: Type data frame
     *      byte 0: 1000 ....: QoS Data
     * - 2 bytes duration
     * - 6 bytes addr 1 (receiver)
     * - 6 bytes addr 2 (transmitter)
     * - 6 bytes addr 3 (BSSID)
     * - 2 bytes sequence control
     * - 0 or 6 bytes addr 4
     * - 0 or 2 bytes QOS control
     * - 0 or 4 bytes HT control
     */

    let ieee80211_framecontrol: u8 = data[0];

    let ieee80211_fc_version: u8 = ieee80211_framecontrol & 0x03; // 0000.00xx
    if ieee80211_fc_version != 0 {
        return Err(format!("Unknown 802.11 header version {}", ieee80211_fc_version).to_string());
    }

    let ieee80211_fc_type: u8 = (ieee80211_framecontrol & 0x0c) >> 2; // 0000.xx00
    if ieee80211_fc_type != 0b10 {
        // only select data frames
        return Err(format!("Unsupported 802.11 frame type {}", ieee80211_fc_type).to_string());
    }

    let ieee80211_fc_subtype: u8 = (ieee80211_framecontrol & 0xf0) >> 4; // xxxx.0000
    if ieee80211_fc_subtype != 0b1000 {
        // only select QoS frames
        return Err(format!(
            "Unsupported 802.11 frame subtype {:#04x}",
            ieee80211_fc_type
        )
        .to_string());
    }

    let hdr_len: usize = 26; // QoS data frame is usually 26 bytes (sequence control, no addr 4, QoS, no HT)
    let (_, remaining) = data.split_at(hdr_len);

    Ok(remaining)
}

fn remove_llc_hdr(data: &[u8]) -> Result<&[u8], String> {
    /*
     * LLC Header has the following format (8 bytes)
     * - 1 bytes DSAP
     * - 1 bytes SSAP
     * - 1 bytes control field
     * - 3 bytes organization code
     * - 2 bytes Type (0x8947 BE is GeoNetworking)
     */

    let llc_type: u16 = ((data[6] as u16) << 8) | (data[7] as u16);

    if llc_type != 0x8947 {
        return Err(format!("Unknown LLC payload type {:#x}", llc_type).to_string());
    }

    let hdr_len: usize = 8; // TODO: Is this the right size?
    let (_, remaining) = data.split_at(hdr_len);

    Ok(remaining)
}