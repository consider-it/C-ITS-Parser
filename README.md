# C-ITS Parser

This library can parse and encode ETSI C-ITS (a.k.a. V2X) messages including GeoNetworking headers and optionally Pcap Radiotap and IEEE 802.11 headers.
It also provides Javascript/ WebAssembly bindings to transcode (decode and re-encode as JER/ XER/ UPER) packets and to encode them from JSON (JER) representation.

This library supports `no_std` environments unless compiling for wasm32.
Just disable the default features to disable the `std` feature flag.

Supported standards:

- GeoNetworking: via [geonetworking](https://crates.io/crates/geonetworking) crate
- CAM:
    * ETSI EN 302 637-2 v1.3.1/ v1.4.1
- CPM:
    * ETSI TR 103 562 v2.1.1 (aka. CPM v1)
    * ETSI TS 103 324 v2.1.1
- DENM:
    * ETSI EN 302 637-3 v1.3.1
    * ETSI TS 103 831 v2.1.1/v2.2.1
- Infrastructure Services (MAPEM, SPATEM, SREM, SSEM, IVIM)
    * ETSI TS 103 301 v1.3.1/ v2.1.1
    * ETSI TS 103 301 v2.2.1 (only IVIM changed)

## Usage

When using in Rust code, get crate via cargo: `cargo add c-its-parser`.

When using in Javascript or NodeJS code, two flavors of NPM packages are available in the GitHub NPM Registry: One to be used in NodeJS environments and one to be used in the browser.
So either use `npm install @consider-it/etsi-web-node` or `npm install @consider-it/etsi-web` depending on the target.

### Rust API

Please refer to [docs.rs](https://docs.rs/c_its_parser) for auto-generated documentation.

### Javascript API

The `decode` function is a catch-all method for ITS messages of undefined type.

```typescript
/**
 * Decodes an ITS message of undefined type.
 * Tries to parse the ITS PDU header to read the message ID that identifies the message type.
 * ### Params
 *  - `message`: binary input containing the ITS message
 *  - `headersPresent`: indicate which headers are present in the binary input. GeoNetworking and transport headers will be decoded and returned, other headers will be skipped.
 *  - `outputEncodingRules`: ASN.1 encoding rules that will be used for re-encoding the message in the `JsonItsMessage`'s `its` field. (UPER output will be rendered as a UTF-8 hex string)
 * Throws string error on decoding errors.
 * @param {Uint8Array} message
 * @param {Headers} headersPresent
 * @param {EncodingRules} outputEncodingRules
 * @returns {JsonItsMessage}
 */
export function decode(
    message: Uint8Array,
    headersPresent: Headers,
    outputEncodingRules: EncodingRules
): JsonItsMessage;
```

For each of the messages (see above), the library exposes a function for encoding. For example, for DENM messages:

```typescript
/**
 * Encodes a DENM message into binary UPER with optional headers
 * The encoder expects either both (GeoNetworking and Transport) headers or none
 * Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
 * Throws string error on encoding error
 * @param {JsonItsMessage} denm
 * @param {number} version
 * @returns {Uint8Array}
 */
export function encodeDenm(denm: JsonItsMessage, version: number): Uint8Array;
```

Data is passed to and from the library in form of the following struct/object.

```typescript
export class JsonItsMessage {
    /**
     * Optional GeoNetworking header, encoded as stringified JSON
     */
    geonetworking?: string;
    /**
     * Optional transport header, encoded as stringified JSON
     */
    transport?: string;
    /**
     * Optional ITS ETSI message, encoded as stringified JSON
     */
    its?: string;
}
```

The following version identifiers are used:

- CAM: `141` (default)
- CPM: `131` (default), `211`
- DENM: `131`, `221` (default)
- MAPEM: `221` (default)
- SPATEM: `221` (default)
- SREM: `221` (default)
- SSEM: `221` (default)
- IVIM: `131`, `211`, `221` (default)


## Testing

For testing of the Rust API the normal cargo testing is used, with:

```shell
cargo test --all-features
```

To test the Javascript API you need to install `wasm-pack` using cargo, with `cargo install wasm-pack`.
Then the test website can be hosted by running one of these commands:

```shell
wasm-pack test --chrome --headless

wasm-pack test --firefox

wasm-pack test --safari
```

To run the test open the local link shown in the console. The results will be shown on the website.

## ASN Update

With updates to the `rasn` library it may be needed to refresh the code generated from the ASN.1 definitions.

First install the [rasn-compiler](https://github.com/librasn/compiler) CLI using cargo: `cargo install rasn-compiler --features cli`.
Then regenerate the code by running `./scripts/recompile-asn1.sh`.

The source files are generated without any formatting, so the script is running `rustfmt` on these file.
Make sure to have the nightly version installed with rust edition 2024.
