## [unreleased]

### 🐛 Bug Fixes

- *(de)* Use correct version IDs for DENM and IVIM

### 📚 Documentation

- *(standard)* Fix doxygen style doc attributes
- *(transport)* Remove invalid rust-doc syntax

### ⚙️ Miscellaneous Tasks

- *(ci)* Remove irrelevant step
- Generate first changelog
## [1.1.0] - 2026-04-09

### 🚀 Features

- *(conversions)* Add HeadingValue to degrees
- *(conversions)* Add "try" methods to values with unavailable value
- *(conversions)* Add YawRateValue conversion
- *(conversions)* Add vehicle size conversions
- *(conversions)* Add acceleration conversions
- *(conversions)* Add CurvatureValue conversion
- *(conversions)* Add SteeringWheelAngleValue conversion
- *(conversions)* Add km/h getters to speeds
- *(conversions)* Add "unavailable" value support to length values
- *(conversions)* Consider "unavailable" value when creating ETSI value
- *(conversions)* Add unit tests

### 🐛 Bug Fixes

- *(ci)* Disable headless chrome wasm tests for now
- *(conversions)* Fix CDD 2.2.1 YawRateValue "unavailable" value

### 🚜 Refactor

- *(conversions)* Add "must_use" attribute everywhere

### 📚 Documentation

- *(conversions)* Fix doc comments

### ⚙️ Miscellaneous Tasks

- *(ci)* Update dependencies
- Bump version number
## [1.0.0] - 2026-03-04

### 🚀 Features

- *(asn1)* Restructure ETSI sources and update IS 103 301
- Add feature flags for individual messages
- *(de)* Support old CDD XER for MAPEM, SPATEM, SREM, SSEM

### 🐛 Bug Fixes

- *(asn1)* Workaround parsing issues of old SREM with new definition

### ⚙️ Miscellaneous Tasks

- Upgrade xmltree
## [0.8.1] - 2026-01-19

### 🚀 Features

- *(ci)* Add wasm build to PR pipeline

### 🐛 Bug Fixes

- *(wasm)* Rename wasm "decode" to "decode_to"

### ⚙️ Miscellaneous Tasks

- Bump version number
## [0.8.0] - 2026-01-14

### 🚀 Features

- *(extensions)* Add ItsMessageId enum with conversions
- *(extensions)* Add StationType enum
- *(extensions)* Implement Default for bitstring types
- *(extensions)* Impl TryFrom<String> for DescriptiveName
- *(conversions)* Add SREM data types
- *(geo_utils)* Add from Point to Position3D
- *(time_utils)* Add MOY and DSecond conversions
- *(time_utils)* Add TimestampIts conversions
- *(extensions)* Add enums for DENM sub cause codes
- *(asn1)* Update for rasn 0.15.0 and later
- *(asn1)* Cleanup IS 1.3.1 asn.1 source
- *(asn1)* Remove unsupported messages from IS 1.3.1

### 🚜 Refactor

- *(asn1)* Simplify `DEFAULT_VALIDITY` value

### ⚙️ Miscellaneous Tasks

- *(asn1)* Cleanup patch file
- *(asn1)* Fix formatting of ASN.1 source
- Bump version number
## [0.7.7] - 2026-01-12

### 🐛 Bug Fixes

- *(conversion)* Actually apply custom conversion factor for Velocity

### ⚙️ Miscellaneous Tasks

- Bump version number
## [0.7.6] - 2026-01-06

### 🚀 Features

- *(ci)* Build/ test individual features as well
- *(conversions)* Add velocity conversion (used in MAPEM)
- *(conversions)* Add convenience getter for SpeedLimitList
- *(ci)* Only run on PRs
- *(ci)* Publish NPM package(s) on tags

### 🐛 Bug Fixes

- *(ci)* Run wasm tests in headless chrome
## [0.7.5] - 2025-11-18

### 🚀 Features

- *(feature-flags)* Make geo_utils usable without etsi parser
- *(feature-flags)* Even more feature flags
- *(cargo)* Tie wasm dependencies to features
## [0.7.4] - 2025-11-14

### 🚀 Features

- *(geo_utils)* Make delta XY to geo public
- *(conversions)* Add CPM data types
## [0.7.3] - 2025-11-13

### 🐛 Bug Fixes

- *(geo_utils)* NodeOffsetPointXY::node_LatLon is an absolute position
## [0.7.2] - 2025-10-23

### 🚀 Features

- Add conversions to geo-types
- *(extensions)* Add Display for LaneAttributes
- *(conversions)* Add `from_meters` to OffsetB* types

### 🐛 Bug Fixes

- *(geo_utils)* NodeSetXY shall not contain reference position

### ⚙️ Miscellaneous Tasks

- Fix clippy warning
- Bump version number
## [0.7.1] - 2025-10-02

### 🚀 Features

- Add missing access to named BIT STRING bits
## [0.7.0] - 2025-09-23

### 🚀 Features

- Remove strict dependency on geonetworking's json feature

### ⚙️ Miscellaneous Tasks

- Bump version number
## [0.6.1] - 2025-09-17

### ⚙️ Miscellaneous Tasks

- Upgrade to etherparse 0.14
- Upgrade to etherparse 0.15
- Upgrade to etherparse 0.19
- Upgrade nom to 8.0
- Update dependencies, bump to v0.6.1
## [0.6.0] - 2025-09-16

### 🚀 Features

- Parse DENM Rel1 XER/JER properly
- *(ci)* Add CI with basic checks

### 🐛 Bug Fixes

- *(test)* Fix failing tests

### 🚜 Refactor

- Use rasn v0.27.2 and recompile sources

### ⚙️ Miscellaneous Tasks

- Run cargo fmt
- Fix compiler warnings
- Pack large enum variants into Box
- Follow clippy recommendations
- Run cargo-fmt
- Bump to v0.6.0
## [0.5.4] - 2025-05-15

### 🚀 Features

- *(transport)* Add convenience fn for decoding tp header

### ⚙️ Miscellaneous Tasks

- Re-export geonetworking decode
## [0.5.3] - 2025-05-15

### 💼 Other

- Add feature to exclude etsi standards

### ⚙️ Miscellaneous Tasks

- Bump to v0.5.3
## [0.5.2] - 2024-08-29

### 🚀 Features

- Expose GN/BTP header decoding function
## [0.5.1] - 2024-07-01

### ⚙️ Miscellaneous Tasks

- Update rasn dependency
- Bump to v0.5.1
## [0.5.0] - 2024-06-19

### 🚜 Refactor

- Create rust api
## [0.4.4] - 2024-06-18

### 🚀 Features

- Update deps and bump
## [0.4.3] - 2024-06-13

### 🚀 Features

- *(xer)* Handle namespaces while recognizing message id

### ⚙️ Miscellaneous Tasks

- Bump to v0.4.3
## [0.4.2] - 2024-05-27

### 🚀 Features

- Make decode headers fn public
- Export remove_pcap_headers
## [0.4.1] - 2024-05-08

### 🚀 Features

- Add xer codec

### 🐛 Bug Fixes

- *(xer)* Version detection & xer decoding

### 📚 Documentation

- Update API docs

### ⚙️ Miscellaneous Tasks

- Bump to 0.4.1
## [0.3.2] - 2024-04-09

### 🚀 Features

- Detect protocol version and return message type

### ⚙️ Miscellaneous Tasks

- Bump to v0.3.2
## [0.3.1] - 2024-04-09

### 🚜 Refactor

- Update asn1 bindings

### ⚙️ Miscellaneous Tasks

- Add asn1 compile script
- Bump to 0.3.1
## [0.3.0] - 2024-04-05

### 🚀 Features

- Add standards
- Decode headers
- Provide decoding API
- Encode
- Add catch-all `decode`
- *(standards)* Add IVIM 2.2.1
- Support CPM v2.1.1
- Do best-efford version parsing
- Update standards
- *(de)* Decode standard pcap headers

### 🐛 Bug Fixes

- Payload retrieval on secured packets
- Replace all gen'd 'i_d's with 'id'

### 💼 Other

- Add wasm pack script
- Make wasm cargo feature
- Remove all wasm-only from default features
- Bump etsi-geonetworking to 0.1.3
- Tag geonetworking version and suppress doctests
- Separate package for node.js

### 🚜 Refactor

- Pass JSON as Strings
- Use IVIM 2.2.1 by default
- Change decoding method names
- React to upstream API changes
- Move transports library into project

### 📚 Documentation

- Document API
- Add install note
- Update supported versions

### 🎨 Styling

- Cargo fmt

### 🧪 Testing

- Add unit tests
- Pcap header decoding

### ⚙️ Miscellaneous Tasks

- Initial commit
- Bump version to v0.1.1
- Ignore `.DS_Store`
- Bump to v0.1.2
- Bump to v0.1.3
- Bump to v0.1.4
- Bump to v0.1.5
- Update geonetworking
- Bump to v0.1.6
- Bump to v0.1.7
- Add repo and bump to v0.1.8
- Add asn1 source files
- Update rasn
- Bump to v0.3.0
