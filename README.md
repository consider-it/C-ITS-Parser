# V2X ETSI Web
WebAssembly tools for encoding and decoding ETSI messages (GN + Transport + CAM/DENM/IVIM/SSEM/SREM/MAPEM/SPATEM).

### Features
V2X Etsi Web provides an npm package for de- and encoding the most common ETSI messages. Currently, the following messages are supported:
* CAM v1.4.1
* CPM v1.3.1
* DENM v1.3.1 and v2.2.1
* SPATEM v1.3.1
* MAPEM v1.3.1
* IVIM v1.3.1 and v2.1.1
* SREM v1.3.1
* SSEM v1.3.1
* CPM v1.3.1 and v2.1.1

### Installation
Install using npm:
```sh
npm install @consider-it/etsi-web
 ```

### API
The `decode` function is a catch-all method for decoding ITS messages of undefined type.
```typescript
* Decodes an ITS message of undefined type.
* Tries to parse the ITS PDU header to read the message ID that identifies the message type.
* Set `includesHeaders` to `false` if the given binary message does not contain GeoNetworking or Transport headers.
* Throws string error on decoding errors.
* @param {Uint8Array} message
* @param {boolean} includesHeaders
* @returns {EtsiJson}
*/
export function decode(message: Uint8Array, includesHeaders: boolean): EtsiJson;
```
For each of the messages (see above), the library exposes two functions for decoding and one for encoding.
For example, for DENM messages:
```typescript
/**
* Decodes a DENM message with the default decoding options.
* The default options expect a message with headers and version 2.2.1
* Throws string error on decoding errors.
* @param {Uint8Array} denm
* @returns {EtsiJson}
*/
export function decodeDenm(denm: Uint8Array): EtsiJson;
/**
* Decodes a DENM message with custom decoding options.
* Currently, the library supports DENM versions v2.1.1 (211) and v1.3.1 (131)
* Set `includesHeaders` to `false` if the given binary denm does not contain GeoNetworking or Transport headers.
* Throws string error on decoding errors.
* @param {Uint8Array} denm
* @param {number | undefined} version
* @param {boolean} includesHeaders
* @returns {EtsiJson}
*/
export function decodeDenmVersion(denm: Uint8Array, version: number | undefined, includesHeaders: boolean): EtsiJson;
/**
* Encodes a DENM message into binary UPER with optional headers 
* The encoder expects either both (GeoNetworking and Transport) headers or none
* Currently, denms of the following versions are supported: v2.1.1 (211) and v1.3.1 (131)
* Throws string error on encoding error
* @param {EtsiJson} denm
* @param {number} version
* @returns {Uint8Array}
*/
export function encodeDenm(denm: EtsiJson, version: number): Uint8Array;
```
Data is passed to and from the library in form of the following struct/object.
```typescript
export class EtsiJson {
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
