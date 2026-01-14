#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod aviaeinumbering_and_data_structures {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=63"))]
    pub struct AlphabetIndicator(pub u8);
    #[doc = " 6 bits, latinAlphabetNo1 recommended "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AviEriDateTime(pub FixedOctetString<10usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS1 {
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "issuerIdentifier")]
        pub issuer_identifier: IssuerIdentifier,
        #[rasn(identifier = "serviceNumber")]
        pub service_number: ServiceNumber,
    }
    impl CS1 {
        pub fn new(
            country_code: CountryCode,
            issuer_identifier: IssuerIdentifier,
            service_number: ServiceNumber,
        ) -> Self {
            Self {
                country_code,
                issuer_identifier,
                service_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS2 {
        #[rasn(identifier = "manufacturerIdentifier")]
        pub manufacturer_identifier: ManufacturerIdentifier,
        #[rasn(identifier = "serviceNumber")]
        pub service_number: ServiceNumber,
    }
    impl CS2 {
        pub fn new(
            manufacturer_identifier: ManufacturerIdentifier,
            service_number: ServiceNumber,
        ) -> Self {
            Self {
                manufacturer_identifier,
                service_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS3 {
        #[rasn(identifier = "startTime")]
        pub start_time: StartTime,
        #[rasn(identifier = "stopTime")]
        pub stop_time: StopTime,
        #[rasn(identifier = "geographLimit")]
        pub geograph_limit: GeoGraphicalLimit,
        #[rasn(identifier = "serviceAppLimit")]
        pub service_app_limit: ServiceApplicationLimit,
    }
    impl CS3 {
        pub fn new(
            start_time: StartTime,
            stop_time: StopTime,
            geograph_limit: GeoGraphicalLimit,
            service_app_limit: ServiceApplicationLimit,
        ) -> Self {
            Self {
                start_time,
                stop_time,
                geograph_limit,
                service_app_limit,
            }
        }
    }
    #[doc = ""]
    #[doc = "CS4::= SEQUENCE {"]
    #[doc = "\tcountryCode \t\tCountryCode,"]
    #[doc = "\talphabetIndicator \tAlphabetIndicator, "]
    #[doc = "\tlicPlateNumber \t\tLicPlateNumber"]
    #[doc = "\t}"]
    #[doc = "\t"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS4 {
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "alphabetIndicator")]
        pub alphabet_indicator: AlphabetIndicator,
        #[rasn(identifier = "licPlateNumber")]
        pub lic_plate_number: LicPlateNumber,
    }
    impl CS4 {
        pub fn new(
            country_code: CountryCode,
            alphabet_indicator: AlphabetIndicator,
            lic_plate_number: LicPlateNumber,
        ) -> Self {
            Self {
                country_code,
                alphabet_indicator,
                lic_plate_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS5 {
        pub vin: VisibleString,
        #[rasn(size("9"))]
        pub fill: BitString,
    }
    impl CS5 {
        pub fn new(vin: VisibleString, fill: BitString) -> Self {
            Self { vin, fill }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CS7(pub FreightContainerData);
    #[doc = " 12 octets"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CS8 {
        #[rasn(size("6"))]
        pub fill: BitString,
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "taxCode")]
        pub tax_code: TaxCode,
    }
    impl CS8 {
        pub fn new(fill: BitString, country_code: CountryCode, tax_code: TaxCode) -> Self {
            Self {
                fill,
                country_code,
                tax_code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CountryCode(pub FixedBitString<10usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct FreightContainerData {
        #[rasn(size("19"), identifier = "ownerCode")]
        pub owner_code: BitString,
        #[rasn(value("0..=1000000"), identifier = "serialNumber")]
        pub serial_number: u32,
        #[rasn(value("0..=10"), identifier = "checkDigit")]
        pub check_digit: u8,
        #[rasn(value("1..=2000"))]
        pub length: u16,
        #[rasn(value("1..=500"))]
        pub height: u16,
        #[rasn(value("200..=300"))]
        pub width: u16,
        #[rasn(value("0..=127"), identifier = "containerTypeCode")]
        pub container_type_code: u8,
        #[rasn(value("19..=500"), identifier = "maximumGrossMass")]
        pub maximum_gross_mass: u16,
        #[rasn(value("0..=99"), identifier = "tareMass")]
        pub tare_mass: u8,
        #[rasn(size("3"))]
        pub fill: BitString,
    }
    impl FreightContainerData {
        pub fn new(
            owner_code: BitString,
            serial_number: u32,
            check_digit: u8,
            length: u16,
            height: u16,
            width: u16,
            container_type_code: u8,
            maximum_gross_mass: u16,
            tare_mass: u8,
            fill: BitString,
        ) -> Self {
            Self {
                owner_code,
                serial_number,
                check_digit,
                length,
                height,
                width,
                container_type_code,
                maximum_gross_mass,
                tare_mass,
                fill,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct GeoGraphicalLimit(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct IssuerIdentifier(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LicPlateNumber(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct ManufacturerIdentifier(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ServiceApplicationLimit(pub FixedBitString<8usize>);
    #[doc = "ServiceNumber::= BIT STRING(SIZE(32))"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ServiceNumber(pub FixedBitString<32usize>);
    #[doc = " YYMMDDhhmm"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct StartTime(pub AviEriDateTime);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct StopTime(pub AviEriDateTime);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TaxCode(pub OctetString);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod citsappl_mgmt_ids {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum Ext1 {
        #[rasn(value("128..=16511"), tag(context, 0))]
        content(u16),
        #[rasn(tag(context, 1))]
        extension(Ext2),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum Ext2 {
        #[rasn(value("16512..=2113663"), tag(context, 0))]
        content(u32),
        #[rasn(tag(context, 1))]
        extension(Ext3),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2113664..=270549119", extensible))]
    pub struct Ext3(pub Integer);
    #[doc = " End of IMPORTS"]
    #[doc = " Types"]
    #[doc = " Variable length data types"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum VarLengthNumber {
        #[rasn(value("0..=127"), tag(context, 0))]
        content(u8),
        #[rasn(tag(context, 1))]
        extension(Ext1),
    }
    #[doc = " four and more octets length"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice)]
    pub enum VarLengthNumber2 {
        #[rasn(value("0..=127"), tag(context, 0))]
        shortNo(u8),
        #[rasn(value("0..=32767"), tag(context, 1))]
        longNo(u16),
    }
    #[doc = " Values"]
    pub const VERSION: u8 = 1;
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod efc_dsrc_application {
    extern crate alloc;
    use super::aviaeinumbering_and_data_structures::{CountryCode, IssuerIdentifier, CS5};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " defined in ISO 14816 "]
    #[doc = " NOTE: The following are the definitions of the action and response"]
    #[doc = " parameters"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AxleWeightLimits {
        #[rasn(identifier = "maxLadenweightOnAxle1")]
        pub max_ladenweight_on_axle1: Int2,
        #[rasn(identifier = "maxLadenweightOnAxle2")]
        pub max_ladenweight_on_axle2: Int2,
        #[rasn(identifier = "maxLadenweightOnAxle3")]
        pub max_ladenweight_on_axle3: Int2,
        #[rasn(identifier = "maxLadenweightOnAxle4")]
        pub max_ladenweight_on_axle4: Int2,
        #[rasn(identifier = "maxLadenweightOnAxle5")]
        pub max_ladenweight_on_axle5: Int2,
    }
    impl AxleWeightLimits {
        pub fn new(
            max_ladenweight_on_axle1: Int2,
            max_ladenweight_on_axle2: Int2,
            max_ladenweight_on_axle3: Int2,
            max_ladenweight_on_axle4: Int2,
            max_ladenweight_on_axle5: Int2,
        ) -> Self {
            Self {
                max_ladenweight_on_axle1,
                max_ladenweight_on_axle2,
                max_ladenweight_on_axle3,
                max_ladenweight_on_axle4,
                max_ladenweight_on_axle5,
            }
        }
    }
    #[doc = " 4 bits, EURO-Classes as defined in EC directive 88/77/EEC, annex 1"]
    #[doc = " and in 91/542/EEC, 96/1/EC, 1999/96/EC, 2001/27/EC, regulation No 595/2009"]
    #[doc = " and for EEV in Section 6.2.1 of Annex I in EC directive 2005/55/EC"]
    #[doc = " EUR-Class VI as defined in Regulation (EC) No 595/2009"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CopValue {
        noEntry = 0,
        co2class1 = 1,
        co2class2 = 2,
        co2class3 = 3,
        co2class4 = 4,
        co2class5 = 5,
        co2class6 = 6,
        co2class7 = 7,
        reservedforUse = 8,
    }
    #[doc = " NOTE: The following are the definitions of EFC attributes"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DateCompact {
        #[rasn(value("1990..=2117"))]
        pub year: u16,
        #[rasn(value("0..=12"))]
        pub month: u8,
        #[rasn(value("0..=31"))]
        pub day: u8,
    }
    impl DateCompact {
        pub fn new(year: u16, month: u8, day: u8) -> Self {
            Self { year, month, day }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DieselEmissionValuesParticulate {
        #[rasn(identifier = "unitType")]
        pub unit_type: UnitType,
        #[rasn(value("0..=32767"))]
        pub value: u16,
    }
    impl DieselEmissionValuesParticulate {
        pub fn new(unit_type: UnitType, value: u16) -> Self {
            Self { unit_type, value }
        }
    }
    #[doc = " The value \"{year 1990, month 0, day 0}\" is a 16-bit all-zero"]
    #[doc = " encoding, and is used to represent \"no date\"."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DieselEmissionValues {
        pub particulate: DieselEmissionValuesParticulate,
        #[rasn(identifier = "absorptionCoeff")]
        pub absorption_coeff: Int2,
    }
    impl DieselEmissionValues {
        pub fn new(particulate: DieselEmissionValuesParticulate, absorption_coeff: Int2) -> Self {
            Self {
                particulate,
                absorption_coeff,
            }
        }
    }
    #[doc = " 4 bits, reserved for carbon dioxide pollution values as defined in"]
    #[doc = " EC directive 2003/127/EC'"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct EngineCharacteristics(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EnvironmentalCharacteristics {
        #[rasn(identifier = "euroValue")]
        pub euro_value: EuroValue,
        #[rasn(identifier = "copValue")]
        pub cop_value: CopValue,
    }
    impl EnvironmentalCharacteristics {
        pub fn new(euro_value: EuroValue, cop_value: CopValue) -> Self {
            Self {
                euro_value,
                cop_value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuroValue {
        noEntry = 0,
        #[rasn(identifier = "euro-1")]
        euro_1 = 1,
        #[rasn(identifier = "euro-2")]
        euro_2 = 2,
        #[rasn(identifier = "euro-3")]
        euro_3 = 3,
        #[rasn(identifier = "euro-4")]
        euro_4 = 4,
        #[rasn(identifier = "euro-5")]
        euro_5 = 5,
        #[rasn(identifier = "euro-6")]
        euro_6 = 6,
        reservedForUse1 = 7,
        reservedForUse2 = 8,
        reservedForUse3 = 9,
        reservedForUse4 = 10,
        reservedForUse5 = 11,
        reservedForUse6 = 12,
        reservedForUse7 = 13,
        reservedForUse8 = 14,
        eev = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ExhaustEmissionValues {
        #[rasn(identifier = "unitType")]
        pub unit_type: UnitType,
        #[rasn(value("0..=32767"), identifier = "emissionCO")]
        pub emission_co: u16,
        #[rasn(identifier = "emissionHC")]
        pub emission_hc: Int2,
        #[rasn(identifier = "emissionNOX")]
        pub emission_nox: Int2,
        #[rasn(identifier = "emissionHCNOX")]
        pub emission_hcnox: Int2,
    }
    impl ExhaustEmissionValues {
        pub fn new(
            unit_type: UnitType,
            emission_co: u16,
            emission_hc: Int2,
            emission_nox: Int2,
            emission_hcnox: Int2,
        ) -> Self {
            Self {
                unit_type,
                emission_co,
                emission_hc,
                emission_nox,
                emission_hcnox,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Int1(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct Int2(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16777215"))]
    pub struct Int3(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct Int4(pub u32);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum LPNAlphabetIndicator {
        latinAlphabetNo1 = 1,
        latinAlphabetNo2 = 2,
        latinAlphabetNo3 = 3,
        latinAlphabetNo4 = 4,
        latinCyrillicAlphabet = 5,
        latinArabicAlphabet = 6,
        latinGreekAlphabet = 7,
        latinHebrewAlphabet = 8,
        latinAlphabetNo5 = 9,
        latinAlphabetNo6 = 10,
        twoOctetBMP = 11,
        fourOctetCanonical = 12,
        reservedForUse1 = 13,
        reservedForUse2 = 14,
        reservedForUse3 = 15,
        reservedForUse4 = 16,
        reservedForUse5 = 17,
        reservedForUse6 = 18,
        reservedForUse7 = 19,
        reservedForUse8 = 20,
        reservedForUse9 = 21,
        reservedForUse10 = 22,
        reservedForUse11 = 23,
        reservedForUse12 = 24,
        reservedForUse13 = 25,
        reservedForUse14 = 26,
        reservedForUse15 = 27,
        reservedForUse16 = 28,
        reservedForUse17 = 29,
        reservedForUse18 = 30,
        reservedForUse19 = 31,
        reservedForUse20 = 32,
        reservedForUse21 = 33,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LPN {
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "alphabetIndicator")]
        pub alphabet_indicator: LPNAlphabetIndicator,
        #[rasn(identifier = "licencePlateNumber")]
        pub licence_plate_number: OctetString,
    }
    impl LPN {
        pub fn new(
            country_code: CountryCode,
            alphabet_indicator: LPNAlphabetIndicator,
            licence_plate_number: OctetString,
        ) -> Self {
            Self {
                country_code,
                alphabet_indicator,
                licence_plate_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PassengerCapacity {
        #[rasn(identifier = "numberOfSeats")]
        pub number_of_seats: Int1,
        #[rasn(identifier = "numberOfStandingPlaces")]
        pub number_of_standing_places: Int1,
    }
    impl PassengerCapacity {
        pub fn new(number_of_seats: Int1, number_of_standing_places: Int1) -> Self {
            Self {
                number_of_seats,
                number_of_standing_places,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PayUnit(pub FixedOctetString<2usize>);
    #[doc = " corresponds to a �3 octets Signed Integer�, associated with the following"]
    #[doc = " examples of line codes:"]
    #[doc = " -8'388'608 : 80 00 00'H"]
    #[doc = " -1 : FF FF FF'H"]
    #[doc = " 0 : 00 00 00'H"]
    #[doc = " 1 : 00 00 01�H"]
    #[doc = " 8'388'607 : 7F FF FF'H"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PaymentMeansUnit(pub PayUnit);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PaymentSecurityData(pub OctetString);
    #[doc = " The unique designation of a Currency as defined in ISO 4217"]
    #[doc = " using the ISO numeric binary coded decimal representation."]
    #[doc = " The code can also express a company specific token or a"]
    #[doc = " \"charging unit code\" as used in the freight.unit in which"]
    #[doc = " the fee is expressed."]
    #[doc = " Value Assignment :"]
    #[doc = " '0xxx'H Currency in main units"]
    #[doc = " '1xxx'H Currency in minor units of 10 :1 ('dime')"]
    #[doc = " '2xxx'H Currency in minor units of 100 :1 ('cents')"]
    #[doc = " '3xxx'H Currency in minor units of 1000 :1"]
    #[doc = " '4xxx'H Currency in 'major' units / 10"]
    #[doc = " (e.g. 10 Belgian Francs)"]
    #[doc = " '5xxx'H Currency in 'major' units / 100"]
    #[doc = " (e.g. 100 Italian Lire)"]
    #[doc = " '6xxx'H Currency in 'major' units / 1000"]
    #[doc = " '7xxx'H Currency in 'major' units / 10000"]
    #[doc = " '8xxx'H Currency in 'major' units / 100000"]
    #[doc = " where xxx is the BCD representation of \"Currency\""]
    #[doc = " as defined in ISO 4217"]
    #[doc = " '9xxx'H Tokens"]
    #[doc = " where xxx is Purse Provider specific coding."]
    #[doc = " 'Axxx'H Charging Unit Codes,"]
    #[doc = " denoting quantification of the service provided"]
    #[doc = " (e.g. man-hours)"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PersonalAccountNumber(pub FixedOctetString<10usize>);
    #[doc = " Personal account number structure � according to ISO/IEC 7812-1"]
    #[doc = " Issuer identifier number (�BIN�)"]
    #[doc = " Major industry identifier (MII, 1 binary coded decimal, BCD)"]
    #[doc = " 0 : reserved for future use by ISO/TC68"]
    #[doc = " 1 : airline sector"]
    #[doc = " 2 : extended airline sector"]
    #[doc = " 3 : travel and tourism sector"]
    #[doc = " 4 : financial banking sector"]
    #[doc = " 5 : financial banking sector"]
    #[doc = " 6 : commerce and banking sector"]
    #[doc = " 7 : petrol industry sector"]
    #[doc = " 8 : telecommunication sector"]
    #[doc = " 9 : reserved for national use"]
    #[doc = " Issuer identifier (5 BCD in the second edition of ISO/IEC 7812-1)"]
    #[doc = " Account number (max 12 BCD)"]
    #[doc = " Control digit (1 BCD)"]
    #[doc = " Padding bits, set to 1'B, in order to accomplish a"]
    #[doc = " total length of 10 octets."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Provider {
        #[rasn(identifier = "countryCode")]
        pub country_code: CountryCode,
        #[rasn(identifier = "providerIdentifier")]
        pub provider_identifier: IssuerIdentifier,
    }
    impl Provider {
        pub fn new(country_code: CountryCode, provider_identifier: IssuerIdentifier) -> Self {
            Self {
                country_code,
                provider_identifier,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum SignedValue {
        #[rasn(value("0..=8388607"))]
        positive(u32),
        #[rasn(value("-8388608..=-1"))]
        negative(i32),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SoundLevel {
        pub soundstationary: Int1,
        pub sounddriveby: Int1,
    }
    impl SoundLevel {
        pub fn new(soundstationary: Int1, sounddriveby: Int1) -> Self {
            Self {
                soundstationary,
                sounddriveby,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum UnitType {
        #[rasn(identifier = "mg-km")]
        mg_km = 0,
        #[rasn(identifier = "mg-kWh")]
        mg_kWh = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleDimensions {
        #[rasn(identifier = "vehicleLengthOverall")]
        pub vehicle_length_overall: Int1,
        #[rasn(identifier = "vehicleHeigthOverall")]
        pub vehicle_heigth_overall: Int1,
        #[rasn(identifier = "vehicleWidthOverall")]
        pub vehicle_width_overall: Int1,
    }
    impl VehicleDimensions {
        pub fn new(
            vehicle_length_overall: Int1,
            vehicle_heigth_overall: Int1,
            vehicle_width_overall: Int1,
        ) -> Self {
            Self {
                vehicle_length_overall,
                vehicle_heigth_overall,
                vehicle_width_overall,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleWeightLimits {
        #[rasn(identifier = "vehicleMaxLadenWeight")]
        pub vehicle_max_laden_weight: Int2,
        #[rasn(identifier = "vehicleTrainMaximumWeight")]
        pub vehicle_train_maximum_weight: Int2,
        #[rasn(identifier = "vehicleWeightUnladen")]
        pub vehicle_weight_unladen: Int2,
    }
    impl VehicleWeightLimits {
        pub fn new(
            vehicle_max_laden_weight: Int2,
            vehicle_train_maximum_weight: Int2,
            vehicle_weight_unladen: Int2,
        ) -> Self {
            Self {
                vehicle_max_laden_weight,
                vehicle_train_maximum_weight,
                vehicle_weight_unladen,
            }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod electronic_registration_identification_vehicle_data_module {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " Electronic Registration Identification (ERI)- Vehicle Data"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum EuVehicleCategoryCode {
        euVehicleCategoryL(EuVehicleCategoryL),
        euVehicleCategoryM(EuVehicleCategoryM),
        euVehicleCategoryN(EuVehicleCategoryN),
        euVehicleCategoryO(EuVehicleCategoryO),
        euVehilcleCategoryT(()),
        euVehilcleCategoryG(()),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryL {
        l1 = 0,
        l2 = 1,
        l3 = 2,
        l4 = 3,
        l5 = 4,
        l6 = 5,
        l7 = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryM {
        m1 = 0,
        m2 = 1,
        m3 = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryN {
        n1 = 0,
        n2 = 1,
        n3 = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryO {
        o1 = 0,
        o2 = 1,
        o3 = 2,
        o4 = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Iso3833VehicleType(pub u8);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod gdd {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " Definition of data elements used in ISO 14823 attributes"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Code-Units", value("0..=15"))]
    pub struct CodeUnits(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DDD-IO")]
    pub struct DDDIO {
        #[rasn(value("0..=7"), identifier = "arrowDirection")]
        pub arrow_direction: u8,
        #[rasn(identifier = "destPlace")]
        pub dest_place: Option<DestinationPlaces>,
        #[rasn(identifier = "destRoad")]
        pub dest_road: Option<DestinationRoads>,
        #[rasn(value("1..=999"), identifier = "roadNumberIdentifier")]
        pub road_number_identifier: Option<u16>,
        #[rasn(value("1..=999"), identifier = "streetName")]
        pub street_name: Option<u16>,
        #[rasn(identifier = "streetNameText")]
        pub street_name_text: Option<Utf8String>,
        #[rasn(identifier = "distanceToDivergingPoint")]
        pub distance_to_diverging_point: Option<DistanceOrDuration>,
        #[rasn(identifier = "distanceToDestinationPlace")]
        pub distance_to_destination_place: Option<DistanceOrDuration>,
    }
    impl DDDIO {
        pub fn new(
            arrow_direction: u8,
            dest_place: Option<DestinationPlaces>,
            dest_road: Option<DestinationRoads>,
            road_number_identifier: Option<u16>,
            street_name: Option<u16>,
            street_name_text: Option<Utf8String>,
            distance_to_diverging_point: Option<DistanceOrDuration>,
            distance_to_destination_place: Option<DistanceOrDuration>,
        ) -> Self {
            Self {
                arrow_direction,
                dest_place,
                dest_road,
                road_number_identifier,
                street_name,
                street_name_text,
                distance_to_diverging_point,
                distance_to_destination_place,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible), identifier = "DDD-IO-LIST")]
    pub struct DDDIOLIST(pub SequenceOf<DDDIO>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct DayOfWeek(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DestinationPlace {
        #[rasn(identifier = "destType")]
        pub dest_type: DestinationType,
        #[rasn(value("0.."), identifier = "destRSCode")]
        pub dest_rscode: Option<GddStructure>,
        #[rasn(identifier = "destBlob")]
        pub dest_blob: Option<OctetString>,
        #[rasn(value("1..=999"), identifier = "placeNameIdentification")]
        pub place_name_identification: Option<u16>,
        #[rasn(identifier = "placeNameText")]
        pub place_name_text: Option<Utf8String>,
    }
    impl DestinationPlace {
        pub fn new(
            dest_type: DestinationType,
            dest_rscode: Option<GddStructure>,
            dest_blob: Option<OctetString>,
            place_name_identification: Option<u16>,
            place_name_text: Option<Utf8String>,
        ) -> Self {
            Self {
                dest_type,
                dest_rscode,
                dest_blob,
                place_name_identification,
                place_name_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct DestinationPlaces(pub SequenceOf<DestinationPlace>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DestinationRoad {
        #[rasn(identifier = "derType")]
        pub der_type: DestinationRoadType,
        #[rasn(value("1..=999"), identifier = "roadNumberIdentifier")]
        pub road_number_identifier: Option<u16>,
        #[rasn(identifier = "roadNumberText")]
        pub road_number_text: Option<Utf8String>,
    }
    impl DestinationRoad {
        pub fn new(
            der_type: DestinationRoadType,
            road_number_identifier: Option<u16>,
            road_number_text: Option<Utf8String>,
        ) -> Self {
            Self {
                der_type,
                road_number_identifier,
                road_number_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct DestinationRoadType(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct DestinationRoads(pub SequenceOf<DestinationRoad>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct DestinationType(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Distance {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        #[rasn(value("2..=8"))]
        pub unit: CodeUnits,
    }
    impl Distance {
        pub fn new(value: u16, unit: CodeUnits) -> Self {
            Self { value, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DistanceOrDuration {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        #[rasn(value("2..=9"))]
        pub unit: CodeUnits,
    }
    impl DistanceOrDuration {
        pub fn new(value: u16, unit: CodeUnits) -> Self {
            Self { value, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum GddAttribute {
        dtm(InternationalSignApplicablePeriod),
        edt(InternationalSignExemptedApplicablePeriod),
        dfl(InternationalSignDirectionalFlowOfLane),
        ved(InternationalSignApplicableVehicleDimensions),
        spe(InternationalSignSpeedLimits),
        roi(InternationalSignRateOfIncline),
        dbv(InternationalSignDistanceBetweenVehicles),
        ddd(InternationalSignDestinationInformation),
        set(InternationalSignSection),
        nol(InternationalSignNumberOfLane),
    }
    #[doc = " Definition of the single ISO 14823 Attributes"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct GddAttributes(pub SequenceOf<GddAttribute>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum GddStructurePictogramCodeServiceCategoryCodeTrafficSignPictogram {
        dangerWarning = 0,
        regulatory = 1,
        informative = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum GddStructurePictogramCodeServiceCategoryCodePublicFacilitiesPictogram {
        publicFacilities = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum GddStructurePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram {
        ambientCondition = 0,
        roadCondition = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum GddStructurePictogramCodeServiceCategoryCode {
        trafficSignPictogram(GddStructurePictogramCodeServiceCategoryCodeTrafficSignPictogram),
        publicFacilitiesPictogram(
            GddStructurePictogramCodeServiceCategoryCodePublicFacilitiesPictogram,
        ),
        ambientOrRoadConditionPictogram(
            GddStructurePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram,
        ),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GddStructurePictogramCodePictogramCategoryCode {
        #[rasn(value("1..=9"))]
        pub nature: u8,
        #[rasn(value("0..=99"), identifier = "serialNumber")]
        pub serial_number: u8,
    }
    impl GddStructurePictogramCodePictogramCategoryCode {
        pub fn new(nature: u8, serial_number: u8) -> Self {
            Self {
                nature,
                serial_number,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GddStructurePictogramCode {
        #[rasn(size("2"), identifier = "countryCode")]
        pub country_code: Option<OctetString>,
        #[rasn(identifier = "serviceCategoryCode")]
        pub service_category_code: GddStructurePictogramCodeServiceCategoryCode,
        #[rasn(identifier = "pictogramCategoryCode")]
        pub pictogram_category_code: GddStructurePictogramCodePictogramCategoryCode,
    }
    impl GddStructurePictogramCode {
        pub fn new(
            country_code: Option<OctetString>,
            service_category_code: GddStructurePictogramCodeServiceCategoryCode,
            pictogram_category_code: GddStructurePictogramCodePictogramCategoryCode,
        ) -> Self {
            Self {
                country_code,
                service_category_code,
                pictogram_category_code,
            }
        }
    }
    #[doc = "Definition of GDD Structure"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct GddStructure {
        #[rasn(identifier = "pictogramCode")]
        pub pictogram_code: GddStructurePictogramCode,
        pub attributes: Option<GddAttributes>,
    }
    impl GddStructure {
        pub fn new(
            pictogram_code: GddStructurePictogramCode,
            attributes: Option<GddAttributes>,
        ) -> Self {
            Self {
                pictogram_code,
                attributes,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct HoursMinutes {
        #[rasn(value("0..=23"))]
        pub hours: u8,
        #[rasn(value("0..=59"))]
        pub mins: u8,
    }
    impl HoursMinutes {
        pub fn new(hours: u8, mins: u8) -> Self {
            Self { hours, mins }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InternationalSignApplicablePeriodYear {
        #[rasn(value("2000..=2127", extensible), identifier = "yearRangeStartYear")]
        pub year_range_start_year: Integer,
        #[rasn(value("2000..=2127", extensible), identifier = "yearRangeEndYear")]
        pub year_range_end_year: Integer,
    }
    impl InternationalSignApplicablePeriodYear {
        pub fn new(year_range_start_year: Integer, year_range_end_year: Integer) -> Self {
            Self {
                year_range_start_year,
                year_range_end_year,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InternationalSignApplicablePeriodMonthDay {
        #[rasn(identifier = "dateRangeStartMonthDay")]
        pub date_range_start_month_day: MonthDay,
        #[rasn(identifier = "dateRangeEndMonthDay")]
        pub date_range_end_month_day: MonthDay,
    }
    impl InternationalSignApplicablePeriodMonthDay {
        pub fn new(
            date_range_start_month_day: MonthDay,
            date_range_end_month_day: MonthDay,
        ) -> Self {
            Self {
                date_range_start_month_day,
                date_range_end_month_day,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct InternationalSignApplicablePeriodHourMinutes {
        #[rasn(identifier = "timeRangeStartTime")]
        pub time_range_start_time: HoursMinutes,
        #[rasn(identifier = "timeRangeEndTime")]
        pub time_range_end_time: HoursMinutes,
    }
    impl InternationalSignApplicablePeriodHourMinutes {
        pub fn new(time_range_start_time: HoursMinutes, time_range_end_time: HoursMinutes) -> Self {
            Self {
                time_range_start_time,
                time_range_end_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "InternationalSign-applicablePeriod")]
    pub struct InternationalSignApplicablePeriod {
        pub year: Option<InternationalSignApplicablePeriodYear>,
        #[rasn(identifier = "month-day")]
        pub month_day: Option<InternationalSignApplicablePeriodMonthDay>,
        #[rasn(identifier = "repeatingPeriodDayTypes")]
        pub repeating_period_day_types: Option<RepeatingPeriodDayTypes>,
        #[rasn(identifier = "hourMinutes")]
        pub hour_minutes: Option<InternationalSignApplicablePeriodHourMinutes>,
        #[rasn(identifier = "dateRangeOfWeek")]
        pub date_range_of_week: Option<DayOfWeek>,
        #[rasn(identifier = "durationHourMinute")]
        pub duration_hour_minute: Option<HoursMinutes>,
    }
    impl InternationalSignApplicablePeriod {
        pub fn new(
            year: Option<InternationalSignApplicablePeriodYear>,
            month_day: Option<InternationalSignApplicablePeriodMonthDay>,
            repeating_period_day_types: Option<RepeatingPeriodDayTypes>,
            hour_minutes: Option<InternationalSignApplicablePeriodHourMinutes>,
            date_range_of_week: Option<DayOfWeek>,
            duration_hour_minute: Option<HoursMinutes>,
        ) -> Self {
            Self {
                year,
                month_day,
                repeating_period_day_types,
                hour_minutes,
                date_range_of_week,
                duration_hour_minute,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "InternationalSign-applicableVehicleDimensions"
    )]
    pub struct InternationalSignApplicableVehicleDimensions {
        #[rasn(identifier = "vehicleHeight")]
        pub vehicle_height: Option<Distance>,
        #[rasn(identifier = "vehicleWidth")]
        pub vehicle_width: Option<Distance>,
        #[rasn(identifier = "vehicleLength")]
        pub vehicle_length: Option<Distance>,
        #[rasn(identifier = "vehicleWeight")]
        pub vehicle_weight: Option<Weight>,
    }
    impl InternationalSignApplicableVehicleDimensions {
        pub fn new(
            vehicle_height: Option<Distance>,
            vehicle_width: Option<Distance>,
            vehicle_length: Option<Distance>,
            vehicle_weight: Option<Weight>,
        ) -> Self {
            Self {
                vehicle_height,
                vehicle_width,
                vehicle_length,
                vehicle_weight,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        automatic_tags,
        identifier = "InternationalSign-destinationInformation"
    )]
    pub struct InternationalSignDestinationInformation {
        #[rasn(value("1..=128"), identifier = "junctionDirection")]
        pub junction_direction: Option<u8>,
        #[rasn(value("1..=128"), identifier = "roundaboutCwDirection")]
        pub roundabout_cw_direction: Option<u8>,
        #[rasn(value("1..=128"), identifier = "roundaboutCcwDirection")]
        pub roundabout_ccw_direction: Option<u8>,
        #[rasn(identifier = "ioList")]
        pub io_list: DDDIOLIST,
    }
    impl InternationalSignDestinationInformation {
        pub fn new(
            junction_direction: Option<u8>,
            roundabout_cw_direction: Option<u8>,
            roundabout_ccw_direction: Option<u8>,
            io_list: DDDIOLIST,
        ) -> Self {
            Self {
                junction_direction,
                roundabout_cw_direction,
                roundabout_ccw_direction,
                io_list,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "InternationalSign-directionalFlowOfLane",
        value("1..=8")
    )]
    pub struct InternationalSignDirectionalFlowOfLane(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "InternationalSign-distanceBetweenVehicles")]
    pub struct InternationalSignDistanceBetweenVehicles(pub Distance);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "InternationalSign-exemptedApplicablePeriod")]
    pub struct InternationalSignExemptedApplicablePeriod(pub InternationalSignApplicablePeriod);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "InternationalSign-numberOfLane",
        value("0..=99")
    )]
    pub struct InternationalSignNumberOfLane(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "InternationalSign-rateOfIncline",
        value("1..=32")
    )]
    pub struct InternationalSignRateOfIncline(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "InternationalSign-section")]
    pub struct InternationalSignSection {
        #[rasn(identifier = "startingPointLength")]
        pub starting_point_length: Option<Distance>,
        #[rasn(identifier = "continuityLength")]
        pub continuity_length: Option<Distance>,
    }
    impl InternationalSignSection {
        pub fn new(
            starting_point_length: Option<Distance>,
            continuity_length: Option<Distance>,
        ) -> Self {
            Self {
                starting_point_length,
                continuity_length,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "InternationalSign-speedLimits")]
    pub struct InternationalSignSpeedLimits {
        #[rasn(value("0..=250"), identifier = "speedLimitMax")]
        pub speed_limit_max: Option<u8>,
        #[rasn(value("0..=250"), identifier = "speedLimitMin")]
        pub speed_limit_min: Option<u8>,
        #[rasn(value("0..=1"))]
        pub unit: CodeUnits,
    }
    impl InternationalSignSpeedLimits {
        pub fn new(
            speed_limit_max: Option<u8>,
            speed_limit_min: Option<u8>,
            unit: CodeUnits,
        ) -> Self {
            Self {
                speed_limit_max,
                speed_limit_min,
                unit,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MonthDay {
        #[rasn(value("1..=12"))]
        pub month: u8,
        #[rasn(value("1..=31"))]
        pub day: u8,
    }
    impl MonthDay {
        pub fn new(month: u8, day: u8) -> Self {
            Self { month, day }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct RepeatingPeriodDayTypes(pub FixedBitString<4usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Weight {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        #[rasn(value("10..=12"))]
        pub unit: CodeUnits,
    }
    impl Weight {
        pub fn new(value: u16, unit: CodeUnits) -> Self {
            Self { value, unit }
        }
    }
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod ivi {
    extern crate alloc;
    use super::super::cdd_1_3_1_1::its_container::{
        ActionID, Altitude, DangerousGoodsBasic, DeltaLatitude, DeltaLongitude,
        DeltaReferencePosition, Heading, HeadingValue, LanePosition, Latitude, Longitude,
        ReferencePosition, RoadType, SpecialTransportType, Speed, SpeedValue, StationType,
        TimestampIts, VehicleRole,
    };
    use super::super::dsrc_2_2_1::etsi_its_dsrc::{
        IntersectionReferenceID, LaneID, RoadSegmentReferenceID,
    };
    use super::citsappl_mgmt_ids::VarLengthNumber;
    use super::efc_dsrc_application::{
        AxleWeightLimits, DieselEmissionValues, EngineCharacteristics,
        EnvironmentalCharacteristics, ExhaustEmissionValues, PassengerCapacity, Provider,
        SoundLevel, VehicleDimensions, VehicleWeightLimits,
    };
    use super::electronic_registration_identification_vehicle_data_module::{
        EuVehicleCategoryCode, Iso3833VehicleType,
    };
    use super::gdd::{
        InternationalSignApplicablePeriod, InternationalSignApplicableVehicleDimensions,
        InternationalSignDestinationInformation, InternationalSignDirectionalFlowOfLane,
        InternationalSignDistanceBetweenVehicles, InternationalSignExemptedApplicablePeriod,
        InternationalSignRateOfIncline, InternationalSignSpeedLimits,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = "  Definition of Data Frames"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AbsolutePosition {
        pub latitude: Latitude,
        pub longitude: Longitude,
    }
    impl AbsolutePosition {
        pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
            Self {
                latitude,
                longitude,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AbsolutePositionWAltitude {
        pub latitude: Latitude,
        pub longitude: Longitude,
        pub altitude: Altitude,
    }
    impl AbsolutePositionWAltitude {
        pub fn new(latitude: Latitude, longitude: Longitude, altitude: Altitude) -> Self {
            Self {
                latitude,
                longitude,
                altitude,
            }
        }
    }
    #[doc = " Definition of data frames which are lists of data frames"]
    #[doc = " note: those definitions are to avoid \"implicit type definitions\" but are bit compatible with V1"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositionsWAltitude(pub SequenceOf<AbsolutePositionWAltitude>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AnyCatalogue {
        pub owner: Provider,
        #[rasn(value("0..=255"))]
        pub version: u8,
        #[rasn(value("0..=65535"), identifier = "pictogramCode")]
        pub pictogram_code: u16,
        #[rasn(value("0..=65535"))]
        pub value: Option<u16>,
        pub unit: Option<RSCUnit>,
        pub attributes: Option<ISO14823Attributes>,
    }
    impl AnyCatalogue {
        pub fn new(
            owner: Provider,
            version: u8,
            pictogram_code: u16,
            value: Option<u16>,
            unit: Option<RSCUnit>,
            attributes: Option<ISO14823Attributes>,
        ) -> Self {
            Self {
                owner,
                version,
                pictogram_code,
                value,
                unit,
                attributes,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct AutomatedVehicleContainer(pub SequenceOf<AvcPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AutomatedVehicleRule {
        pub priority: PriorityLevel,
        #[rasn(identifier = "allowedSaeAutomationLevels")]
        pub allowed_sae_automation_levels: SaeAutomationLevels,
        #[rasn(identifier = "minGapBetweenVehicles")]
        pub min_gap_between_vehicles: Option<GapBetweenVehicles>,
        #[rasn(identifier = "recGapBetweenVehicles")]
        pub rec_gap_between_vehicles: Option<GapBetweenVehicles>,
        #[rasn(identifier = "automatedVehicleMaxSpeedLimit")]
        pub automated_vehicle_max_speed_limit: Option<SpeedValue>,
        #[rasn(identifier = "automatedVehicleMinSpeedLimit")]
        pub automated_vehicle_min_speed_limit: Option<SpeedValue>,
        #[rasn(identifier = "automatedVehicleSpeedRecommendation")]
        pub automated_vehicle_speed_recommendation: Option<SpeedValue>,
        #[rasn(identifier = "roadSignCodes")]
        pub road_sign_codes: Option<RoadSignCodes>,
        #[rasn(identifier = "extraText")]
        pub extra_text: Option<ConstraintTextLines2>,
    }
    impl AutomatedVehicleRule {
        pub fn new(
            priority: PriorityLevel,
            allowed_sae_automation_levels: SaeAutomationLevels,
            min_gap_between_vehicles: Option<GapBetweenVehicles>,
            rec_gap_between_vehicles: Option<GapBetweenVehicles>,
            automated_vehicle_max_speed_limit: Option<SpeedValue>,
            automated_vehicle_min_speed_limit: Option<SpeedValue>,
            automated_vehicle_speed_recommendation: Option<SpeedValue>,
            road_sign_codes: Option<RoadSignCodes>,
            extra_text: Option<ConstraintTextLines2>,
        ) -> Self {
            Self {
                priority,
                allowed_sae_automation_levels,
                min_gap_between_vehicles,
                rec_gap_between_vehicles,
                automated_vehicle_max_speed_limit,
                automated_vehicle_min_speed_limit,
                automated_vehicle_speed_recommendation,
                road_sign_codes,
                extra_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct AutomatedVehicleRules(pub SequenceOf<AutomatedVehicleRule>);
    #[doc = " new container in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AvcPart {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: ZoneIds,
        pub direction: Option<Direction>,
        #[rasn(identifier = "applicableLanes")]
        pub applicable_lanes: Option<LanePositions>,
        #[rasn(identifier = "vehicleCharacteristics")]
        pub vehicle_characteristics: Option<VehicleCharacteristicsList>,
        #[rasn(identifier = "automatedVehicleRules")]
        pub automated_vehicle_rules: Option<AutomatedVehicleRules>,
        #[rasn(identifier = "platooningRules")]
        pub platooning_rules: Option<PlatooningRules>,
    }
    impl AvcPart {
        pub fn new(
            detection_zone_ids: Option<ZoneIds>,
            relevance_zone_ids: ZoneIds,
            direction: Option<Direction>,
            applicable_lanes: Option<LanePositions>,
            vehicle_characteristics: Option<VehicleCharacteristicsList>,
            automated_vehicle_rules: Option<AutomatedVehicleRules>,
            platooning_rules: Option<PlatooningRules>,
        ) -> Self {
            Self {
                detection_zone_ids,
                relevance_zone_ids,
                direction,
                applicable_lanes,
                vehicle_characteristics,
                automated_vehicle_rules,
                platooning_rules,
            }
        }
    }
    #[doc = " Defition of IVI specific data elements "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-20..=21"))]
    pub struct BankingAngle(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct ComparisonOperator(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CompleteVehicleCharacteristics {
        pub tractor: Option<TractorCharacteristics>,
        pub trailer: Option<TrailerCharacteristicsList>,
        pub train: Option<TrainCharacteristics>,
    }
    impl CompleteVehicleCharacteristics {
        pub fn new(
            tractor: Option<TractorCharacteristics>,
            trailer: Option<TrailerCharacteristicsList>,
            train: Option<TrainCharacteristics>,
        ) -> Self {
            Self {
                tractor,
                trailer,
                train,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ComputedSegment {
        #[rasn(identifier = "zoneId")]
        pub zone_id: Zid,
        #[rasn(identifier = "laneNumber")]
        pub lane_number: LanePosition,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: IviLaneWidth,
        #[rasn(value("-32768..=32767"), identifier = "offsetDistance")]
        pub offset_distance: Option<i16>,
        #[rasn(identifier = "offsetPosition")]
        pub offset_position: Option<DeltaReferencePosition>,
    }
    impl ComputedSegment {
        pub fn new(
            zone_id: Zid,
            lane_number: LanePosition,
            lane_width: IviLaneWidth,
            offset_distance: Option<i16>,
            offset_position: Option<DeltaReferencePosition>,
        ) -> Self {
            Self {
                zone_id,
                lane_number,
                lane_width,
                offset_distance,
                offset_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct Condition(pub Integer);
    #[doc = " new DF in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ConnectedDenms(pub SequenceOf<ActionID>);
    #[doc = "size extension in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct ConstraintTextLines1(pub SequenceOf<Text>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct ConstraintTextLines2(pub SequenceOf<Text>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct DefinitionAccuracy(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DeltaPosition {
        #[rasn(identifier = "deltaLatitude")]
        pub delta_latitude: DeltaLatitude,
        #[rasn(identifier = "deltaLongitude")]
        pub delta_longitude: DeltaLongitude,
    }
    impl DeltaPosition {
        pub fn new(delta_latitude: DeltaLatitude, delta_longitude: DeltaLongitude) -> Self {
            Self {
                delta_latitude,
                delta_longitude,
            }
        }
    }
    #[doc = " new DF in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);
    #[doc = "size extension in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaReferencePositions(pub SequenceOf<DeltaReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Depth(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct Direction(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct DriverCharacteristics(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct FrictionCoefficient(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct GapBetweenVehicles(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct GeneralIviContainer(pub SequenceOf<GicPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GeographicLocationContainer {
        #[rasn(identifier = "referencePosition")]
        pub reference_position: ReferencePosition,
        #[rasn(identifier = "referencePositionTime")]
        pub reference_position_time: Option<TimestampIts>,
        #[rasn(identifier = "referencePositionHeading")]
        pub reference_position_heading: Option<Heading>,
        #[rasn(identifier = "referencePositionSpeed")]
        pub reference_position_speed: Option<Speed>,
        pub parts: GlcParts,
    }
    impl GeographicLocationContainer {
        pub fn new(
            reference_position: ReferencePosition,
            reference_position_time: Option<TimestampIts>,
            reference_position_heading: Option<Heading>,
            reference_position_speed: Option<Speed>,
            parts: GlcParts,
        ) -> Self {
            Self {
                reference_position,
                reference_position_time,
                reference_position_heading,
                reference_position_speed,
                parts,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GicPart {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "its-Rrid")]
        pub its_rrid: Option<VarLengthNumber>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: Option<ZoneIds>,
        pub direction: Option<Direction>,
        #[rasn(identifier = "driverAwarenessZoneIds")]
        pub driver_awareness_zone_ids: Option<ZoneIds>,
        #[rasn(value("0..=255"), identifier = "minimumAwarenessTime")]
        pub minimum_awareness_time: Option<u8>,
        #[rasn(identifier = "applicableLanes")]
        pub applicable_lanes: Option<LanePositions>,
        #[rasn(identifier = "iviType")]
        pub ivi_type: IviType,
        #[rasn(identifier = "iviPurpose")]
        pub ivi_purpose: Option<IviPurpose>,
        #[rasn(identifier = "laneStatus")]
        pub lane_status: Option<LaneStatus>,
        #[rasn(identifier = "vehicleCharacteristics")]
        pub vehicle_characteristics: Option<VehicleCharacteristicsList>,
        #[rasn(identifier = "driverCharacteristics")]
        pub driver_characteristics: Option<DriverCharacteristics>,
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Option<Integer>,
        #[rasn(value("1..=64", extensible), identifier = "preStoredlayoutId")]
        pub pre_storedlayout_id: Option<Integer>,
        #[rasn(identifier = "roadSignCodes")]
        pub road_sign_codes: RoadSignCodes,
        #[rasn(identifier = "extraText")]
        pub extra_text: Option<ConstraintTextLines1>,
    }
    impl GicPart {
        pub fn new(
            detection_zone_ids: Option<ZoneIds>,
            its_rrid: Option<VarLengthNumber>,
            relevance_zone_ids: Option<ZoneIds>,
            direction: Option<Direction>,
            driver_awareness_zone_ids: Option<ZoneIds>,
            minimum_awareness_time: Option<u8>,
            applicable_lanes: Option<LanePositions>,
            ivi_type: IviType,
            ivi_purpose: Option<IviPurpose>,
            lane_status: Option<LaneStatus>,
            vehicle_characteristics: Option<VehicleCharacteristicsList>,
            driver_characteristics: Option<DriverCharacteristics>,
            layout_id: Option<Integer>,
            pre_storedlayout_id: Option<Integer>,
            road_sign_codes: RoadSignCodes,
            extra_text: Option<ConstraintTextLines1>,
        ) -> Self {
            Self {
                detection_zone_ids,
                its_rrid,
                relevance_zone_ids,
                direction,
                driver_awareness_zone_ids,
                minimum_awareness_time,
                applicable_lanes,
                ivi_type,
                ivi_purpose,
                lane_status,
                vehicle_characteristics,
                driver_characteristics,
                layout_id,
                pre_storedlayout_id,
                road_sign_codes,
                extra_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GlcPart {
        #[rasn(identifier = "zoneId")]
        pub zone_id: Zid,
        #[rasn(identifier = "laneNumber")]
        pub lane_number: Option<LanePosition>,
        #[rasn(value("0..=255"), identifier = "zoneExtension")]
        pub zone_extension: Option<u8>,
        #[rasn(identifier = "zoneHeading")]
        pub zone_heading: Option<HeadingValue>,
        pub zone: Option<Zone>,
    }
    impl GlcPart {
        pub fn new(
            zone_id: Zid,
            lane_number: Option<LanePosition>,
            zone_extension: Option<u8>,
            zone_heading: Option<HeadingValue>,
            zone: Option<Zone>,
        ) -> Self {
            Self {
                zone_id,
                lane_number,
                zone_extension,
                zone_heading,
                zone,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct GlcParts(pub SequenceOf<GlcPart>);
    #[doc = " new DE in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct GoodsType(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ISO14823Attribute {
        dtm(InternationalSignApplicablePeriod),
        edt(InternationalSignExemptedApplicablePeriod),
        dfl(InternationalSignDirectionalFlowOfLane),
        ved(InternationalSignApplicableVehicleDimensions),
        spe(InternationalSignSpeedLimits),
        roi(InternationalSignRateOfIncline),
        dbv(InternationalSignDistanceBetweenVehicles),
        ddd(InternationalSignDestinationInformation),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ISO14823CodePictogramCodeServiceCategoryCodeTrafficSignPictogram {
        dangerWarning = 0,
        regulatory = 1,
        informative = 2,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ISO14823CodePictogramCodeServiceCategoryCodePublicFacilitiesPictogram {
        publicFacilities = 0,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ISO14823CodePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram {
        ambientCondition = 0,
        roadCondition = 1,
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ISO14823CodePictogramCodeServiceCategoryCode {
        trafficSignPictogram(ISO14823CodePictogramCodeServiceCategoryCodeTrafficSignPictogram),
        publicFacilitiesPictogram(
            ISO14823CodePictogramCodeServiceCategoryCodePublicFacilitiesPictogram,
        ),
        ambientOrRoadConditionPictogram(
            ISO14823CodePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram,
        ),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ISO14823CodePictogramCodePictogramCategoryCode {
        #[rasn(value("1..=9"))]
        pub nature: u8,
        #[rasn(value("0..=99"), identifier = "serialNumber")]
        pub serial_number: u8,
    }
    impl ISO14823CodePictogramCodePictogramCategoryCode {
        pub fn new(nature: u8, serial_number: u8) -> Self {
            Self {
                nature,
                serial_number,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ISO14823CodePictogramCode {
        #[rasn(size("2"), identifier = "countryCode")]
        pub country_code: Option<OctetString>,
        #[rasn(identifier = "serviceCategoryCode")]
        pub service_category_code: ISO14823CodePictogramCodeServiceCategoryCode,
        #[rasn(identifier = "pictogramCategoryCode")]
        pub pictogram_category_code: ISO14823CodePictogramCodePictogramCategoryCode,
    }
    impl ISO14823CodePictogramCode {
        pub fn new(
            country_code: Option<OctetString>,
            service_category_code: ISO14823CodePictogramCodeServiceCategoryCode,
            pictogram_category_code: ISO14823CodePictogramCodePictogramCategoryCode,
        ) -> Self {
            Self {
                country_code,
                service_category_code,
                pictogram_category_code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ISO14823Code {
        #[rasn(identifier = "pictogramCode")]
        pub pictogram_code: ISO14823CodePictogramCode,
        pub attributes: Option<ISO14823Attributes>,
    }
    impl ISO14823Code {
        pub fn new(
            pictogram_code: ISO14823CodePictogramCode,
            attributes: Option<ISO14823Attributes>,
        ) -> Self {
            Self {
                pictogram_code,
                attributes,
            }
        }
    }
    #[doc = "Definition of Containers"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IviContainer {
        glc(GeographicLocationContainer),
        giv(GeneralIviContainer),
        rcc(RoadConfigurationContainer),
        tc(TextContainer),
        lac(LayoutContainer),
        #[rasn(extension_addition)]
        avc(AutomatedVehicleContainer),
        #[rasn(extension_addition)]
        mlc(MapLocationContainer),
        #[rasn(extension_addition)]
        rsc(RoadSurfaceContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct IviContainers(pub SequenceOf<IviContainer>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32767", extensible))]
    pub struct IviIdentificationNumber(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct IviIdentificationNumbers(pub SequenceOf<IviIdentificationNumber>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct IviLaneWidth(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IviManagementContainer {
        #[rasn(identifier = "serviceProviderId")]
        pub service_provider_id: Provider,
        #[rasn(identifier = "iviIdentificationNumber")]
        pub ivi_identification_number: IviIdentificationNumber,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<TimestampIts>,
        #[rasn(identifier = "validFrom")]
        pub valid_from: Option<TimestampIts>,
        #[rasn(identifier = "validTo")]
        pub valid_to: Option<TimestampIts>,
        #[rasn(identifier = "connectedIviStructures")]
        pub connected_ivi_structures: Option<IviIdentificationNumbers>,
        #[rasn(identifier = "iviStatus")]
        pub ivi_status: IviStatus,
        #[rasn(extension_addition, identifier = "connectedDenms")]
        pub connected_denms: Option<ConnectedDenms>,
    }
    impl IviManagementContainer {
        pub fn new(
            service_provider_id: Provider,
            ivi_identification_number: IviIdentificationNumber,
            time_stamp: Option<TimestampIts>,
            valid_from: Option<TimestampIts>,
            valid_to: Option<TimestampIts>,
            connected_ivi_structures: Option<IviIdentificationNumbers>,
            ivi_status: IviStatus,
            connected_denms: Option<ConnectedDenms>,
        ) -> Self {
            Self {
                service_provider_id,
                ivi_identification_number,
                time_stamp,
                valid_from,
                valid_to,
                connected_ivi_structures,
                ivi_status,
                connected_denms,
            }
        }
    }
    #[doc = " only renamed from V1, no change"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct IviPurpose(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviStatus(pub u8);
    #[doc = " Definition of IVI structure"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IviStructure {
        pub mandatory: IviManagementContainer,
        pub optional: Option<IviContainers>,
    }
    impl IviStructure {
        pub fn new(mandatory: IviManagementContainer, optional: Option<IviContainers>) -> Self {
            Self {
                mandatory,
                optional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneCharacteristics {
        #[rasn(identifier = "zoneDefinitionAccuracy")]
        pub zone_definition_accuracy: DefinitionAccuracy,
        #[rasn(identifier = "existinglaneMarkingStatus")]
        pub existinglane_marking_status: LaneMarkingStatus,
        #[rasn(identifier = "newlaneMarkingColour")]
        pub newlane_marking_colour: MarkingColour,
        #[rasn(identifier = "laneDelimitationLeft")]
        pub lane_delimitation_left: LaneDelimitation,
        #[rasn(identifier = "laneDelimitationRight")]
        pub lane_delimitation_right: LaneDelimitation,
        #[rasn(identifier = "mergingWith")]
        pub merging_with: Zid,
    }
    impl LaneCharacteristics {
        pub fn new(
            zone_definition_accuracy: DefinitionAccuracy,
            existinglane_marking_status: LaneMarkingStatus,
            newlane_marking_colour: MarkingColour,
            lane_delimitation_left: LaneDelimitation,
            lane_delimitation_right: LaneDelimitation,
            merging_with: Zid,
        ) -> Self {
            Self {
                zone_definition_accuracy,
                existinglane_marking_status,
                newlane_marking_colour,
                lane_delimitation_left,
                lane_delimitation_right,
                merging_with,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct LaneConfiguration(pub SequenceOf<LaneInformation>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct LaneDelimitation(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct LaneIds(pub SequenceOf<LaneID>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneInformationExtGroupDetectionZoneIds {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "laneCharacteristics")]
        pub lane_characteristics: Option<LaneCharacteristics>,
        #[rasn(identifier = "laneSurfaceStaticCharacteristics")]
        pub lane_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
        #[rasn(identifier = "laneSurfaceDynamicCharacteristics")]
        pub lane_surface_dynamic_characteristics: Option<RoadSurfaceDynamicCharacteristics>,
    }
    impl LaneInformationExtGroupDetectionZoneIds {
        pub fn new(
            detection_zone_ids: Option<ZoneIds>,
            relevance_zone_ids: Option<ZoneIds>,
            lane_characteristics: Option<LaneCharacteristics>,
            lane_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
            lane_surface_dynamic_characteristics: Option<RoadSurfaceDynamicCharacteristics>,
        ) -> Self {
            Self {
                detection_zone_ids,
                relevance_zone_ids,
                lane_characteristics,
                lane_surface_static_characteristics,
                lane_surface_dynamic_characteristics,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LaneInformation {
        #[rasn(identifier = "laneNumber")]
        pub lane_number: LanePosition,
        pub direction: Direction,
        pub validity: Option<InternationalSignApplicablePeriod>,
        #[rasn(identifier = "laneType")]
        pub lane_type: LaneType,
        #[rasn(identifier = "laneTypeQualifier")]
        pub lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
        #[rasn(identifier = "laneStatus")]
        pub lane_status: LaneStatus,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<IviLaneWidth>,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_detection_zone_ids: Option<LaneInformationExtGroupDetectionZoneIds>,
    }
    impl LaneInformation {
        pub fn new(
            lane_number: LanePosition,
            direction: Direction,
            validity: Option<InternationalSignApplicablePeriod>,
            lane_type: LaneType,
            lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
            lane_status: LaneStatus,
            lane_width: Option<IviLaneWidth>,
            ext_group_detection_zone_ids: Option<LaneInformationExtGroupDetectionZoneIds>,
        ) -> Self {
            Self {
                lane_number,
                direction,
                validity,
                lane_type,
                lane_type_qualifier,
                lane_status,
                lane_width,
                ext_group_detection_zone_ids,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct LaneMarkingStatus(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct LanePositions(pub SequenceOf<LanePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct LaneStatus(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct LaneType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LayoutComponent {
        #[rasn(value("1..=8", extensible), identifier = "layoutComponentId")]
        pub layout_component_id: Integer,
        #[rasn(value("10..=73"))]
        pub height: u8,
        #[rasn(value("10..=265"))]
        pub width: u16,
        #[rasn(value("10..=265"))]
        pub x: u16,
        #[rasn(value("10..=73"))]
        pub y: u8,
        #[rasn(value("0..=1"), identifier = "textScripting")]
        pub text_scripting: u8,
    }
    impl LayoutComponent {
        pub fn new(
            layout_component_id: Integer,
            height: u8,
            width: u16,
            x: u16,
            y: u8,
            text_scripting: u8,
        ) -> Self {
            Self {
                layout_component_id,
                height,
                width,
                x,
                y,
                text_scripting,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct LayoutComponents(pub SequenceOf<LayoutComponent>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LayoutContainer {
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Integer,
        #[rasn(value("10..=73"))]
        pub height: Option<u8>,
        #[rasn(value("10..=265"))]
        pub width: Option<u16>,
        #[rasn(identifier = "layoutComponents")]
        pub layout_components: LayoutComponents,
    }
    impl LayoutContainer {
        pub fn new(
            layout_id: Integer,
            height: Option<u8>,
            width: Option<u16>,
            layout_components: LayoutComponents,
        ) -> Self {
            Self {
                layout_id,
                height,
                width,
                layout_components,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LoadType {
        #[rasn(identifier = "goodsType")]
        pub goods_type: GoodsType,
        #[rasn(identifier = "dangerousGoodsType")]
        pub dangerous_goods_type: DangerousGoodsBasic,
        #[rasn(identifier = "specialTransportType")]
        pub special_transport_type: SpecialTransportType,
    }
    impl LoadType {
        pub fn new(
            goods_type: GoodsType,
            dangerous_goods_type: DangerousGoodsBasic,
            special_transport_type: SpecialTransportType,
        ) -> Self {
            Self {
                goods_type,
                dangerous_goods_type,
                special_transport_type,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MapLocationContainer {
        pub reference: MapReference,
        pub parts: MlcParts,
    }
    impl MapLocationContainer {
        pub fn new(reference: MapReference, parts: MlcParts) -> Self {
            Self { reference, parts }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum MapReference {
        roadsegment(RoadSegmentReferenceID),
        intersection(IntersectionReferenceID),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct MarkingColour(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct MaterialType(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=64"))]
    pub struct MaxLenghtOfPlatoon(pub u8);
    #[doc = " new DE in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2..=64"))]
    pub struct MaxNoOfVehicles(pub u8);
    #[doc = " new container part in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MlcPart {
        #[rasn(identifier = "zoneId")]
        pub zone_id: Zid,
        #[rasn(identifier = "laneIds")]
        pub lane_ids: Option<LaneIds>,
    }
    impl MlcPart {
        pub fn new(zone_id: Zid, lane_ids: Option<LaneIds>) -> Self {
            Self { zone_id, lane_ids }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct MlcParts(pub SequenceOf<MlcPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PlatooningRule {
        pub priority: PriorityLevel,
        #[rasn(identifier = "allowedSaeAutomationLevels")]
        pub allowed_sae_automation_levels: SaeAutomationLevels,
        #[rasn(identifier = "maxNoOfVehicles")]
        pub max_no_of_vehicles: Option<MaxNoOfVehicles>,
        #[rasn(identifier = "maxLenghtOfPlatoon")]
        pub max_lenght_of_platoon: Option<MaxLenghtOfPlatoon>,
        #[rasn(identifier = "minGapBetweenVehicles")]
        pub min_gap_between_vehicles: Option<GapBetweenVehicles>,
        #[rasn(identifier = "platoonMaxSpeedLimit")]
        pub platoon_max_speed_limit: Option<SpeedValue>,
        #[rasn(identifier = "platoonMinSpeedLimit")]
        pub platoon_min_speed_limit: Option<SpeedValue>,
        #[rasn(identifier = "platoonSpeedRecommendation")]
        pub platoon_speed_recommendation: Option<SpeedValue>,
        #[rasn(identifier = "roadSignCodes")]
        pub road_sign_codes: Option<RoadSignCodes>,
        #[rasn(identifier = "extraText")]
        pub extra_text: Option<ConstraintTextLines2>,
    }
    impl PlatooningRule {
        pub fn new(
            priority: PriorityLevel,
            allowed_sae_automation_levels: SaeAutomationLevels,
            max_no_of_vehicles: Option<MaxNoOfVehicles>,
            max_lenght_of_platoon: Option<MaxLenghtOfPlatoon>,
            min_gap_between_vehicles: Option<GapBetweenVehicles>,
            platoon_max_speed_limit: Option<SpeedValue>,
            platoon_min_speed_limit: Option<SpeedValue>,
            platoon_speed_recommendation: Option<SpeedValue>,
            road_sign_codes: Option<RoadSignCodes>,
            extra_text: Option<ConstraintTextLines2>,
        ) -> Self {
            Self {
                priority,
                allowed_sae_automation_levels,
                max_no_of_vehicles,
                max_lenght_of_platoon,
                min_gap_between_vehicles,
                platoon_max_speed_limit,
                platoon_min_speed_limit,
                platoon_speed_recommendation,
                road_sign_codes,
                extra_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct PlatooningRules(pub SequenceOf<PlatooningRule>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PolygonalLine {
        deltaPositions(DeltaPositions),
        deltaPositionsWithAltitude(DeltaReferencePositions),
        absolutePositions(AbsolutePositions),
        absolutePositionsWithAltitude(AbsolutePositionsWAltitude),
    }
    #[doc = " new DE in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=2"))]
    pub struct PriorityLevel(pub u8);
    #[doc = " new DE in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct RSCUnit(pub u8);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum RSCodeCode {
        viennaConvention(VcCode),
        iso14823(ISO14823Code),
        #[rasn(value("0..=65535"))]
        itisCodes(u16),
        anyCatalogue(AnyCatalogue),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RSCode {
        #[rasn(value("1..=4", extensible), identifier = "layoutComponentId")]
        pub layout_component_id: Option<Integer>,
        pub code: RSCodeCode,
    }
    impl RSCode {
        pub fn new(layout_component_id: Option<Integer>, code: RSCodeCode) -> Self {
            Self {
                layout_component_id,
                code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RccPart {
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: ZoneIds,
        #[rasn(identifier = "roadType")]
        pub road_type: RoadType,
        #[rasn(identifier = "laneConfiguration")]
        pub lane_configuration: LaneConfiguration,
    }
    impl RccPart {
        pub fn new(
            relevance_zone_ids: ZoneIds,
            road_type: RoadType,
            lane_configuration: LaneConfiguration,
        ) -> Self {
            Self {
                relevance_zone_ids,
                road_type,
                lane_configuration,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct RoadConfigurationContainer(pub SequenceOf<RccPart>);
    #[doc = " new DF in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct RoadSignCodes(pub SequenceOf<RSCode>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct RoadSurfaceContainer(pub SequenceOf<RscPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RoadSurfaceDynamicCharacteristics {
        pub condition: Condition,
        pub temperature: Temperature,
        #[rasn(identifier = "iceOrWaterDepth")]
        pub ice_or_water_depth: Depth,
        pub treatment: TreatmentType,
    }
    impl RoadSurfaceDynamicCharacteristics {
        pub fn new(
            condition: Condition,
            temperature: Temperature,
            ice_or_water_depth: Depth,
            treatment: TreatmentType,
        ) -> Self {
            Self {
                condition,
                temperature,
                ice_or_water_depth,
                treatment,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RoadSurfaceStaticCharacteristics {
        #[rasn(identifier = "frictionCoefficient")]
        pub friction_coefficient: FrictionCoefficient,
        pub material: MaterialType,
        pub wear: WearLevel,
        #[rasn(identifier = "avBankingAngle")]
        pub av_banking_angle: BankingAngle,
    }
    impl RoadSurfaceStaticCharacteristics {
        pub fn new(
            friction_coefficient: FrictionCoefficient,
            material: MaterialType,
            wear: WearLevel,
            av_banking_angle: BankingAngle,
        ) -> Self {
            Self {
                friction_coefficient,
                material,
                wear,
                av_banking_angle,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RscPart {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: ZoneIds,
        pub direction: Option<Direction>,
        #[rasn(identifier = "roadSurfaceStaticCharacteristics")]
        pub road_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
        #[rasn(identifier = "roadSurfaceDynamicCharacteristics")]
        pub road_surface_dynamic_characteristics: Option<RoadSurfaceDynamicCharacteristics>,
    }
    impl RscPart {
        pub fn new(
            detection_zone_ids: Option<ZoneIds>,
            relevance_zone_ids: ZoneIds,
            direction: Option<Direction>,
            road_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
            road_surface_dynamic_characteristics: Option<RoadSurfaceDynamicCharacteristics>,
        ) -> Self {
            Self {
                detection_zone_ids,
                relevance_zone_ids,
                direction,
                road_surface_static_characteristics,
                road_surface_dynamic_characteristics,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=5"))]
    pub struct SaeAutomationLevel(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct SaeAutomationLevels(pub SequenceOf<SaeAutomationLevel>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Segment {
        pub line: PolygonalLine,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<IviLaneWidth>,
    }
    impl Segment {
        pub fn new(line: PolygonalLine, lane_width: Option<IviLaneWidth>) -> Self {
            Self { line, lane_width }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TcPartExtGroupIviType {
        #[rasn(identifier = "iviType")]
        pub ivi_type: IviType,
        #[rasn(identifier = "laneStatus")]
        pub lane_status: Option<LaneStatus>,
        #[rasn(identifier = "vehicleCharacteristics")]
        pub vehicle_characteristics: Option<VehicleCharacteristicsList>,
    }
    impl TcPartExtGroupIviType {
        pub fn new(
            ivi_type: IviType,
            lane_status: Option<LaneStatus>,
            vehicle_characteristics: Option<VehicleCharacteristicsList>,
        ) -> Self {
            Self {
                ivi_type,
                lane_status,
                vehicle_characteristics,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TcPart {
        #[rasn(identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<ZoneIds>,
        #[rasn(identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: ZoneIds,
        pub direction: Option<Direction>,
        #[rasn(identifier = "driverAwarenessZoneIds")]
        pub driver_awareness_zone_ids: Option<ZoneIds>,
        #[rasn(value("0..=255"), identifier = "minimumAwarenessTime")]
        pub minimum_awareness_time: Option<u8>,
        #[rasn(identifier = "applicableLanes")]
        pub applicable_lanes: Option<LanePositions>,
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Option<Integer>,
        #[rasn(value("1..=64", extensible), identifier = "preStoredlayoutId")]
        pub pre_storedlayout_id: Option<Integer>,
        pub text: Option<TextLines>,
        pub data: OctetString,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_ivi_type: Option<TcPartExtGroupIviType>,
    }
    impl TcPart {
        pub fn new(
            detection_zone_ids: Option<ZoneIds>,
            relevance_zone_ids: ZoneIds,
            direction: Option<Direction>,
            driver_awareness_zone_ids: Option<ZoneIds>,
            minimum_awareness_time: Option<u8>,
            applicable_lanes: Option<LanePositions>,
            layout_id: Option<Integer>,
            pre_storedlayout_id: Option<Integer>,
            text: Option<TextLines>,
            data: OctetString,
            ext_group_ivi_type: Option<TcPartExtGroupIviType>,
        ) -> Self {
            Self {
                detection_zone_ids,
                relevance_zone_ids,
                direction,
                driver_awareness_zone_ids,
                minimum_awareness_time,
                applicable_lanes,
                layout_id,
                pre_storedlayout_id,
                text,
                data,
                ext_group_ivi_type,
            }
        }
    }
    #[doc = " new DE in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-100..=151"))]
    pub struct Temperature(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Text {
        #[rasn(value("1..=4", extensible), identifier = "layoutComponentId")]
        pub layout_component_id: Option<Integer>,
        #[rasn(size("10"))]
        pub language: BitString,
        #[rasn(identifier = "textContent")]
        pub text_content: Utf8String,
    }
    impl Text {
        pub fn new(
            layout_component_id: Option<Integer>,
            language: BitString,
            text_content: Utf8String,
        ) -> Self {
            Self {
                layout_component_id,
                language,
                text_content,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct TextContainer(pub SequenceOf<TcPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct TextLines(pub SequenceOf<Text>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TractorCharacteristics {
        #[rasn(identifier = "equalTo")]
        pub equal_to: Option<VehicleCharacteristicsFixValuesList>,
        #[rasn(identifier = "notEqualTo")]
        pub not_equal_to: Option<VehicleCharacteristicsFixValuesList>,
        pub ranges: Option<VehicleCharacteristicsRangesList>,
    }
    impl TractorCharacteristics {
        pub fn new(
            equal_to: Option<VehicleCharacteristicsFixValuesList>,
            not_equal_to: Option<VehicleCharacteristicsFixValuesList>,
            ranges: Option<VehicleCharacteristicsRangesList>,
        ) -> Self {
            Self {
                equal_to,
                not_equal_to,
                ranges,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TrailerCharacteristics {
        #[rasn(identifier = "equalTo")]
        pub equal_to: Option<TrailerCharacteristicsFixValuesList>,
        #[rasn(identifier = "notEqualTo")]
        pub not_equal_to: Option<TrailerCharacteristicsFixValuesList>,
        pub ranges: Option<TrailerCharacteristicsRangesList>,
    }
    impl TrailerCharacteristics {
        pub fn new(
            equal_to: Option<TrailerCharacteristicsFixValuesList>,
            not_equal_to: Option<TrailerCharacteristicsFixValuesList>,
            ranges: Option<TrailerCharacteristicsRangesList>,
        ) -> Self {
            Self {
                equal_to,
                not_equal_to,
                ranges,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct TrailerCharacteristicsFixValuesList(pub SequenceOf<VehicleCharacteristicsFixValues>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3"))]
    pub struct TrailerCharacteristicsList(pub SequenceOf<TrailerCharacteristics>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct TrailerCharacteristicsRangesList(pub SequenceOf<VehicleCharacteristicsRanges>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TrainCharacteristics(pub TractorCharacteristics);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct TreatmentType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ValidityPeriods(pub SequenceOf<InternationalSignApplicablePeriod>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct VcClass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VcCode {
        #[rasn(identifier = "roadSignClass")]
        pub road_sign_class: VcClass,
        #[rasn(value("1..=64"), identifier = "roadSignCode")]
        pub road_sign_code: u8,
        #[rasn(identifier = "vcOption")]
        pub vc_option: VcOption,
        pub validity: Option<ValidityPeriods>,
        #[rasn(value("0..=65535"))]
        pub value: Option<u16>,
        pub unit: Option<RSCUnit>,
    }
    impl VcCode {
        pub fn new(
            road_sign_class: VcClass,
            road_sign_code: u8,
            vc_option: VcOption,
            validity: Option<ValidityPeriods>,
            value: Option<u16>,
            unit: Option<RSCUnit>,
        ) -> Self {
            Self {
                road_sign_class,
                road_sign_code,
                vc_option,
                validity,
                value,
                unit,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct VcOption(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VehicleCharacteristicsFixValues {
        simpleVehicleType(StationType),
        euVehicleCategoryCode(EuVehicleCategoryCode),
        iso3833VehicleType(Iso3833VehicleType),
        euroAndCo2value(EnvironmentalCharacteristics),
        engineCharacteristics(EngineCharacteristics),
        loadType(LoadType),
        usage(VehicleRole),
    }
    #[doc = " new DF in V2"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct VehicleCharacteristicsFixValuesList(pub SequenceOf<VehicleCharacteristicsFixValues>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct VehicleCharacteristicsList(pub SequenceOf<CompleteVehicleCharacteristics>);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VehicleCharacteristicsRangesLimits {
        #[rasn(value("0..=7"))]
        numberOfAxles(u8),
        vehicleDimensions(VehicleDimensions),
        vehicleWeightLimits(VehicleWeightLimits),
        axleWeightLimits(AxleWeightLimits),
        passengerCapacity(PassengerCapacity),
        exhaustEmissionValues(ExhaustEmissionValues),
        dieselEmissionValues(DieselEmissionValues),
        soundLevel(SoundLevel),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleCharacteristicsRanges {
        #[rasn(identifier = "comparisonOperator")]
        pub comparison_operator: ComparisonOperator,
        pub limits: VehicleCharacteristicsRangesLimits,
    }
    impl VehicleCharacteristicsRanges {
        pub fn new(
            comparison_operator: ComparisonOperator,
            limits: VehicleCharacteristicsRangesLimits,
        ) -> Self {
            Self {
                comparison_operator,
                limits,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4", extensible))]
    pub struct VehicleCharacteristicsRangesList(pub SequenceOf<VehicleCharacteristicsRanges>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct WearLevel(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32", extensible))]
    pub struct Zid(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Zone {
        segment(Segment),
        area(PolygonalLine),
        computedSegment(ComputedSegment),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ZoneIds(pub SequenceOf<Zid>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod ivim_pdu_descriptions {
    extern crate alloc;
    use super::super::cdd_1_3_1_1::its_container::ItsPduHeader;
    use super::ivi::IviStructure;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
    #[doc = " In vehicle information Message Message"]
    #[doc = " @brief In vehicle information Message Root"]
    #[doc = " This DF includes DEs for the IVIM protocolVersion, the IVI message type identifier _messageID_,"]
    #[doc = " the station identifier _stationID_ of the originating ITS-S and the IVI data from ISO TS 19321."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IVIM {
        pub header: ItsPduHeader,
        pub ivi: IviStructure,
    }
    impl IVIM {
        pub fn new(header: ItsPduHeader, ivi: IviStructure) -> Self {
            Self { header, ivi }
        }
    }
}
