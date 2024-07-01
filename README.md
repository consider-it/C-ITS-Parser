# V2X ETSI Web

WebAssembly tools for encoding and decoding ITS messages (GN + Transport + CAM/DENM/IVIM/SSEM/SREM/MAPEM/SPATEM/CPM).

### Features

V2X Etsi Web provides an npm package for de- and encoding the most common ETSI messages. Currently, the following
messages are supported:

-   CAM v1.4.1
-   CPM v1.3.1 ans v2.2.1
-   DENM v1.3.1 and v2.2.1
-   SPATEM v1.3.1
-   MAPEM v1.3.1
-   IVIM v1.3.1 and v2.1.1
-   SREM v1.3.1
-   SSEM v1.3.1
-   CPM v1.3.1 and v2.1.1

### Installation

Install using npm:

```sh
npm install @consider-it/etsi-web
```

or using cargo

```
cargo add --tag v0.5.1 --git ssh://git@github.com/consider-it/V2X-Etsi_web.git
```

### Rust API

The `decode` function is a catch-all method for ITS messages encoded using JER, XER, or UPER. The `headers` argument
specifies which headers are present, RadioTap + 802.11p + LLC + Geonetworking + BTP, only Geonetworking + BTP, or none.
For XER and JER messages, decoding is only supported for messages without headers.
Use the `ItsMessage::encode` method for encoding a value. Here, too, XER and JER encodings do not support header values.

### Javascript API

The `decode` function is a catch-all method for ITS messages of undefined type.

```typescript
/**
 * Decodes an ITS message of undefined type.
 * Tries to parse the ITS PDU header to read the message ID that identifies the message type.
 * ### Params
 *  - `message`: binary input containing the ITS message
 *  - `headersPresent`: indicate which headers are present in the binary input. Geonetworking and transport headers will be decoded and returned, other headers will be skipped.
 *  - `inputEncodingRules`: ASN.1 encoding rules used to encode the ITS message in the input
 *  - `outputEncodingRules`: ASN.1 encoding rules that will be used for re-encoding the message in the `ItsMessage`'s `its` field. (UPER output will be rendered as a UTF-8 hex string)
 * Throws string error on decoding errors.
 * @param {Uint8Array} message
 * @param {Headers} headersPresent
 * @param {EncodingRules} inputEncodingRules
 * @param {EncodingRules} outputEncodingRules
 * @returns {ItsMessage}
 */
export function decode(
    message: Uint8Array,
    headersPresent: Headers,
    inputEncodingRules: EncodingRules,
    outputEncodingRules: EncodingRules
): ItsMessage;
```

For each of the messages (see above), the library exposes a function for encoding. For example, for DENM messages:

```typescript
/**
 * Encodes a DENM message into binary UPER with optional headers
 * The encoder expects either both (GeoNetworking and Transport) headers or none
 * Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
 * Throws string error on encoding error
 * @param {ItsMessage} denm
 * @param {number} version
 * @returns {Uint8Array}
 */
export function encodeDenm(denm: ItsMessage, version: number): Uint8Array;
```

Data is passed to and from the library in form of the following struct/object.

```typescript
export class ItsMessage {
    /**
     * Optional GeoNetworking header, encoded as stringified JSON
     */
    geonetworking?: string;
    /**
     * Optional ITS ETSI message, encoded as stringified JSON
     */
    its?: string;
    /**
     * Optional transport header, encoded as stringified JSON
     */
    transport?: string;
}
```
