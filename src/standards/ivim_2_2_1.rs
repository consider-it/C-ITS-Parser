extern crate alloc;

use rasn::prelude::*;
// =====================================================
// ElectronicRegistrationIdentificationVehicleDataModule
// { iso(1) standard(0) iso24534(24534) vehicleData(1) version1(1) }
// =====================================================

/// Electronic Registration Identification (ERI)- Vehicle Data

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EuVehicleCategoryCode {
    EuVehicleCategoryL(EuVehicleCategoryL),
    EuVehicleCategoryM(EuVehicleCategoryM),
    EuVehicleCategoryN(EuVehicleCategoryN),
    EuVehicleCategoryO(EuVehicleCategoryO),
    EuVehilcleCategoryT(()),
    EuVehilcleCategoryG(()),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryL {
    L1 = 0,
    L2 = 1,
    L3 = 2,
    L4 = 3,
    L5 = 4,
    L6 = 5,
    L7 = 6,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryM {
    M1 = 0,
    M2 = 1,
    M3 = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryN {
    N1 = 0,
    N2 = 1,
    N3 = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryO {
    O1 = 0,
    O2 = 1,
    O3 = 2,
    O4 = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext1 {
    #[rasn(value("128..=16511"))]
    Content(u16),
    Extension(Ext2),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext2 {
    #[rasn(value("16512..=2113663"))]
    Content(u32),
    Extension(Ext3),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2113664..=270549119", extensible))]
pub struct Ext3(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct IntersectionID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IntersectionReferenceID {
    pub region: Option<RoadRegulatorID>,
    pub id: IntersectionID,
}

impl IntersectionReferenceID {
    pub fn new(region: Option<RoadRegulatorID>, id: IntersectionID) -> Self {
        Self { region, id }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Iso3833VehicleType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct LaneID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct RoadRegulatorID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct RoadSegmentID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadSegmentReferenceID {
    pub region: Option<RoadRegulatorID>,
    pub id: RoadSegmentID,
}

impl RoadSegmentReferenceID {
    pub fn new(region: Option<RoadRegulatorID>, id: RoadSegmentID) -> Self {
        Self { region, id }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum VarLengthNumber {
    #[rasn(value("0..=127"))]
    Content(u8),
    Extension(Ext1),
}

// =====================================================
// GDD
// { iso(1) standard(0) gdd(14823) version1(0) }
// =====================================================

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ActionID {
    pub originating_station_id: StationID,
    pub sequence_number: SequenceNumber,
}

impl ActionID {
    pub fn new(originating_station_id: StationID, sequence_number: SequenceNumber) -> Self {
        Self {
            originating_station_id,
            sequence_number,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Altitude {
    pub altitude_value: AltitudeValue,
    pub altitude_confidence: AltitudeConfidence,
}

impl Altitude {
    pub fn new(altitude_value: AltitudeValue, altitude_confidence: AltitudeConfidence) -> Self {
        Self {
            altitude_value,
            altitude_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AltitudeConfidence {
    Alt00001 = 0,
    Alt00002 = 1,
    Alt00005 = 2,
    Alt00010 = 3,
    Alt00020 = 4,
    Alt00050 = 5,
    Alt00100 = 6,
    Alt00200 = 7,
    Alt00500 = 8,
    Alt01000 = 9,
    Alt02000 = 10,
    Alt05000 = 11,
    Alt10000 = 12,
    Alt20000 = 13,
    OutOfRange = 14,
    Unavailable = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-100000..=800001"))]
pub struct AltitudeValue(pub i32);

/// Definition of data elements used in ISO 14823 attributes

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct CodeUnits(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DDDIO {
    #[rasn(value("0..=7"))]
    pub arrow_direction: u8,
    pub dest_place: Option<DestinationPlaces>,
    pub dest_road: Option<DestinationRoads>,
    #[rasn(value("1..=999"))]
    pub road_number_identifier: Option<u16>,
    #[rasn(value("1..=999"))]
    pub street_name: Option<u16>,
    pub street_name_text: Option<Utf8String>,
    pub distance_to_diverging_point: Option<DistanceOrDuration>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct DDDIOLIST(pub SequenceOf<DDDIO>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum DangerousGoodsBasic {
    Explosives1 = 0,
    Explosives2 = 1,
    Explosives3 = 2,
    Explosives4 = 3,
    Explosives5 = 4,
    Explosives6 = 5,
    FlammableGases = 6,
    NonFlammableGases = 7,
    ToxicGases = 8,
    FlammableLiquids = 9,
    FlammableSolids = 10,
    SubstancesLiableToSpontaneousCombustion = 11,
    SubstancesEmittingFlammableGasesUponContactWithWater = 12,
    OxidizingSubstances = 13,
    OrganicPeroxides = 14,
    ToxicSubstances = 15,
    InfectiousSubstances = 16,
    RadioactiveMaterial = 17,
    CorrosiveSubstances = 18,
    MiscellaneousDangerousSubstances = 19,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct DayOfWeek(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-12700..=12800"))]
pub struct DeltaAltitude(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLatitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLongitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DeltaReferencePosition {
    pub delta_latitude: DeltaLatitude,
    pub delta_longitude: DeltaLongitude,
    pub delta_altitude: DeltaAltitude,
}

impl DeltaReferencePosition {
    pub fn new(
        delta_latitude: DeltaLatitude,
        delta_longitude: DeltaLongitude,
        delta_altitude: DeltaAltitude,
    ) -> Self {
        Self {
            delta_latitude,
            delta_longitude,
            delta_altitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DestinationPlace {
    pub dest_type: DestinationType,
    #[rasn(value("0.."))]
    pub dest_r_s_code: Option<GddStructure>,
    pub dest_blob: Option<OctetString>,
    #[rasn(value("1..=999"))]
    pub place_name_identification: Option<u16>,
    pub place_name_text: Option<Utf8String>,
}

impl DestinationPlace {
    pub fn new(
        dest_type: DestinationType,
        dest_r_s_code: Option<GddStructure>,
        dest_blob: Option<OctetString>,
        place_name_identification: Option<u16>,
        place_name_text: Option<Utf8String>,
    ) -> Self {
        Self {
            dest_type,
            dest_r_s_code,
            dest_blob,
            place_name_identification,
            place_name_text,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct DestinationPlaces(pub SequenceOf<DestinationPlace>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DestinationRoad {
    pub der_type: DestinationRoadType,
    #[rasn(value("1..=999"))]
    pub road_number_identifier: Option<u16>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct DestinationRoadType(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct DestinationRoads(pub SequenceOf<DestinationRoad>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct DestinationType(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum GddAttribute {
    Dtm(InternationalSignApplicablePeriod),
    Edt(InternationalSignExemptedApplicablePeriod),
    Dfl(InternationalSignDirectionalFlowOfLane),
    Ved(InternationalSignApplicableVehicleDimensions),
    Spe(InternationalSignSpeedLimits),
    Roi(InternationalSignRateOfIncline),
    Dbv(InternationalSignDistanceBetweenVehicles),
    Ddd(InternationalSignDestinationInformation),
    Set(InternationalSignSection),
    Nol(InternationalSignNumberOfLane),
}

/// Definition of the single ISO 14823 Attributes

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct GddAttributes(pub SequenceOf<GddAttribute>);

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum GddStructurePictogramCodeServiceCategoryCodeTrafficSignPictogram {
    DangerWarning = 0,
    Regulatory = 1,
    Informative = 2,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum GddStructurePictogramCodeServiceCategoryCodePublicFacilitiesPictogram {
    PublicFacilities = 0,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum GddStructurePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram {
    AmbientCondition = 0,
    RoadCondition = 1,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum GddStructurePictogramCodeServiceCategoryCode {
    TrafficSignPictogram(GddStructurePictogramCodeServiceCategoryCodeTrafficSignPictogram),
    PublicFacilitiesPictogram(
        GddStructurePictogramCodeServiceCategoryCodePublicFacilitiesPictogram,
    ),
    AmbientOrRoadConditionPictogram(
        GddStructurePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram,
    ),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GddStructurePictogramCodePictogramCategoryCode {
    #[rasn(value("1..=9"))]
    pub nature: u8,
    #[rasn(value("0..=99"))]
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

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GddStructurePictogramCode {
    #[rasn(size("2"))]
    pub country_code: Option<OctetString>,
    pub service_category_code: GddStructurePictogramCodeServiceCategoryCode,
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

///Definition of GDD Structure

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GddStructure {
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Heading {
    pub heading_value: HeadingValue,
    pub heading_confidence: HeadingConfidence,
}

impl Heading {
    pub fn new(heading_value: HeadingValue, heading_confidence: HeadingConfidence) -> Self {
        Self {
            heading_value,
            heading_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct HeadingConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct HeadingValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignApplicablePeriodYear {
    #[rasn(value("2000..=2127", extensible))]
    pub year_range_start_year: u16,
    #[rasn(value("2000..=2127", extensible))]
    pub year_range_end_year: u16,
}

impl InternationalSignApplicablePeriodYear {
    pub fn new(year_range_start_year: u16, year_range_end_year: u16) -> Self {
        Self {
            year_range_start_year,
            year_range_end_year,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignApplicablePeriodMonthDay {
    pub date_range_start_month_day: MonthDay,
    pub date_range_end_month_day: MonthDay,
}

impl InternationalSignApplicablePeriodMonthDay {
    pub fn new(date_range_start_month_day: MonthDay, date_range_end_month_day: MonthDay) -> Self {
        Self {
            date_range_start_month_day,
            date_range_end_month_day,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignApplicablePeriodHourMinutes {
    pub time_range_start_time: HoursMinutes,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignApplicablePeriod {
    pub year: Option<InternationalSignApplicablePeriodYear>,
    pub month_day: Option<InternationalSignApplicablePeriodMonthDay>,
    pub repeating_period_day_types: Option<RepeatingPeriodDayTypes>,
    pub hour_minutes: Option<InternationalSignApplicablePeriodHourMinutes>,
    pub date_range_of_week: Option<DayOfWeek>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignApplicableVehicleDimensions {
    pub vehicle_height: Option<Distance>,
    pub vehicle_width: Option<Distance>,
    pub vehicle_length: Option<Distance>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignDestinationInformation {
    #[rasn(value("1..=128"))]
    pub junction_direction: Option<u8>,
    #[rasn(value("1..=128"))]
    pub roundabout_cw_direction: Option<u8>,
    #[rasn(value("1..=128"))]
    pub roundabout_ccw_direction: Option<u8>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=8"))]
pub struct InternationalSignDirectionalFlowOfLane(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct InternationalSignDistanceBetweenVehicles(pub Distance);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct InternationalSignExemptedApplicablePeriod(pub InternationalSignApplicablePeriod);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=99"))]
pub struct InternationalSignNumberOfLane(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32"))]
pub struct InternationalSignRateOfIncline(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignSection {
    pub starting_point_length: Option<Distance>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InternationalSignSpeedLimits {
    #[rasn(value("0..=250"))]
    pub speed_limit_max: Option<u8>,
    #[rasn(value("0..=250"))]
    pub speed_limit_min: Option<u8>,
    #[rasn(value("0..=1"))]
    pub unit: CodeUnits,
}

impl InternationalSignSpeedLimits {
    pub fn new(speed_limit_max: Option<u8>, speed_limit_min: Option<u8>, unit: CodeUnits) -> Self {
        Self {
            speed_limit_max,
            speed_limit_min,
            unit,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsPduHeader {
    #[rasn(value("0..=255"))]
    pub protocol_version: u8,
    #[rasn(value("0..=255"))]
    pub message_id: u8,
    pub station_id: StationID,
}

impl ItsPduHeader {
    pub fn new(protocol_version: u8, message_id: u8, station_id: StationID) -> Self {
        Self {
            protocol_version,
            message_id,
            station_id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1..=14"))]
pub struct LanePosition(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct Latitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1800000000..=1800000001"))]
pub struct Longitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PosConfidenceEllipse {
    pub semi_major_confidence: SemiAxisLength,
    pub semi_minor_confidence: SemiAxisLength,
    pub semi_major_orientation: HeadingValue,
}

impl PosConfidenceEllipse {
    pub fn new(
        semi_major_confidence: SemiAxisLength,
        semi_minor_confidence: SemiAxisLength,
        semi_major_orientation: HeadingValue,
    ) -> Self {
        Self {
            semi_major_confidence,
            semi_minor_confidence,
            semi_major_orientation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ReferencePosition {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub position_confidence_ellipse: PosConfidenceEllipse,
    pub altitude: Altitude,
}

impl ReferencePosition {
    pub fn new(
        latitude: Latitude,
        longitude: Longitude,
        position_confidence_ellipse: PosConfidenceEllipse,
        altitude: Altitude,
    ) -> Self {
        Self {
            latitude,
            longitude,
            position_confidence_ellipse,
            altitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct RepeatingPeriodDayTypes(pub BitString);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RoadType {
    UrbanNoStructuralSeparationToOppositeLanes = 0,
    UrbanWithStructuralSeparationToOppositeLanes = 1,
    NonUrbanNoStructuralSeparationToOppositeLanes = 2,
    NonUrbanWithStructuralSeparationToOppositeLanes = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4095"))]
pub struct SemiAxisLength(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct SequenceNumber(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct SpecialTransportType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Speed {
    pub speed_value: SpeedValue,
    pub speed_confidence: SpeedConfidence,
}

impl Speed {
    pub fn new(speed_value: SpeedValue, speed_confidence: SpeedConfidence) -> Self {
        Self {
            speed_value,
            speed_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SpeedConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct SpeedValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct StationID(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StationType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4398046511103"))]
pub struct TimestampIts(pub u64);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VehicleRole {
    Default = 0,
    PublicTransport = 1,
    SpecialTransport = 2,
    DangerousGoods = 3,
    RoadWork = 4,
    Rescue = 5,
    Emergency = 6,
    SafetyCar = 7,
    Agriculture = 8,
    Commercial = 9,
    Military = 10,
    RoadOperator = 11,
    Taxi = 12,
    Reserved1 = 13,
    Reserved2 = 14,
    Reserved3 = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

// =====================================================
// IVI
// { iso(1) standard(0) ivi(19321) version2(2) }
// =====================================================

///  Definition of Data Frames

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

/// Definition of data frames which are lists of data frames
/// note: those definitions are to avoid "implicit type definitions" but are bit compatible with V1

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct AbsolutePositionsWAltitude(pub SequenceOf<AbsolutePositionWAltitude>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AnyCatalogue {
    pub owner: Provider,
    #[rasn(value("0..=255"))]
    pub version: u8,
    #[rasn(value("0..=65535"))]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct AutomatedVehicleContainer(pub SequenceOf<AvcPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AutomatedVehicleRule {
    pub priority: PriorityLevel,
    pub allowed_sae_automation_levels: SaeAutomationLevels,
    pub min_gap_between_vehicles: Option<GapBetweenVehicles>,
    pub rec_gap_between_vehicles: Option<GapBetweenVehicles>,
    pub automated_vehicle_max_speed_limit: Option<SpeedValue>,
    pub automated_vehicle_min_speed_limit: Option<SpeedValue>,
    pub automated_vehicle_speed_recommendation: Option<SpeedValue>,
    pub road_sign_codes: Option<RoadSignCodes>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct AutomatedVehicleRules(pub SequenceOf<AutomatedVehicleRule>);

/// new container in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AvcPart {
    pub detection_zone_ids: Option<ZoneIds>,
    pub relevance_zone_ids: ZoneIds,
    pub direction: Option<Direction>,
    pub applicable_lanes: Option<LanePositions>,
    pub vehicle_characteristics: Option<VehicleCharacteristicsList>,
    pub automated_vehicle_rules: Option<AutomatedVehicleRules>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AxleWeightLimits {
    pub max_ladenweight_on_axle1: Int2,
    pub max_ladenweight_on_axle2: Int2,
    pub max_ladenweight_on_axle3: Int2,
    pub max_ladenweight_on_axle4: Int2,
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

/// Defition of IVI specific data elements

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-20..=21"))]
pub struct BankingAngle(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct ComparisonOperator(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ComputedSegment {
    pub zone_id: Zid,
    pub lane_number: LanePosition,
    pub lane_width: IviLaneWidth,
    #[rasn(value("-32768..=32767"))]
    pub offset_distance: Option<i16>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct Condition(pub Integer);

/// new DF in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct ConnectedDenms(pub SequenceOf<ActionID>);

///size extension in V2

#[derive(AsnType, Debug, Clone, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct ConstraintTextLines1(pub SequenceOf<Text>);

impl rasn::Decode for ConstraintTextLines1 {
    fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
        decoder: &mut D,
        tag: rasn::Tag,
        constraints: rasn::types::Constraints<'constraints>,
    ) -> core::result::Result<Self, D::Error> {
        match tag {
            rasn::Tag::EOC => Ok(Self(<SequenceOf<Text>>::decode(decoder)?)),
            _ => <SequenceOf<Text> as rasn::Decode>::decode_with_tag_and_constraints(
                decoder,
                tag,
                <SequenceOf<Text> as rasn::AsnType>::CONSTRAINTS.override_constraints(constraints),
            )
            .map(Self),
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct ConstraintTextLines2(pub SequenceOf<Text>);

/// 4 bits, EURO-Classes as defined in EC directive 88/77/EEC, annex 1
/// and in 91/542/EEC, 96/1/EC, 1999/96/EC, 2001/27/EC, regulation No 595/2009
/// and for EEV in Section 6.2.1 of Annex I in EC directive 2005/55/EC
/// EUR-Class VI as defined in Regulation (EC) No 595/2009

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum CopValue {
    NoEntry = 0,
    Co2class1 = 1,
    Co2class2 = 2,
    Co2class3 = 3,
    Co2class4 = 4,
    Co2class5 = 5,
    Co2class6 = 6,
    Co2class7 = 7,
    ReservedforUse = 8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct CountryCode(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct DefinitionAccuracy(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DeltaPosition {
    pub delta_latitude: DeltaLatitude,
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

/// new DF in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32", extensible))]
pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);

///size extension in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32", extensible))]
pub struct DeltaReferencePositions(pub SequenceOf<DeltaReferencePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Depth(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DieselEmissionValuesParticulate {
    pub unit_type: UnitType,
    #[rasn(value("0..=32767"))]
    pub value: u16,
}

impl DieselEmissionValuesParticulate {
    pub fn new(unit_type: UnitType, value: u16) -> Self {
        Self { unit_type, value }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DieselEmissionValues {
    pub particulate: DieselEmissionValuesParticulate,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct Direction(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct DriverCharacteristics(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct EngineCharacteristics(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EnvironmentalCharacteristics {
    pub euro_value: EuroValue,
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

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuroValue {
    NoEntry = 0,
    Euro1 = 1,
    Euro2 = 2,
    Euro3 = 3,
    Euro4 = 4,
    Euro5 = 5,
    Euro6 = 6,
    ReservedForUse1 = 7,
    ReservedForUse2 = 8,
    ReservedForUse3 = 9,
    ReservedForUse4 = 10,
    ReservedForUse5 = 11,
    ReservedForUse6 = 12,
    ReservedForUse7 = 13,
    ReservedForUse8 = 14,
    Eev = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ExhaustEmissionValues {
    pub unit_type: UnitType,
    #[rasn(value("0..=32767"))]
    pub emission_c_o: u16,
    pub emission_h_c: Int2,
    pub emission_n_o_x: Int2,
    pub emission_h_c_n_o_x: Int2,
}

impl ExhaustEmissionValues {
    pub fn new(
        unit_type: UnitType,
        emission_c_o: u16,
        emission_h_c: Int2,
        emission_n_o_x: Int2,
        emission_h_c_n_o_x: Int2,
    ) -> Self {
        Self {
            unit_type,
            emission_c_o,
            emission_h_c,
            emission_n_o_x,
            emission_h_c_n_o_x,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=101"))]
pub struct FrictionCoefficient(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct GapBetweenVehicles(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct GeneralIviContainer(pub SequenceOf<GicPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GeographicLocationContainer {
    pub reference_position: ReferencePosition,
    pub reference_position_time: Option<TimestampIts>,
    pub reference_position_heading: Option<Heading>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GicPart {
    pub detection_zone_ids: Option<ZoneIds>,
    pub its__rrid: Option<VarLengthNumber>,
    pub relevance_zone_ids: Option<ZoneIds>,
    pub direction: Option<Direction>,
    pub driver_awareness_zone_ids: Option<ZoneIds>,
    #[rasn(value("0..=255"))]
    pub minimum_awareness_time: Option<u8>,
    pub applicable_lanes: Option<LanePositions>,
    pub ivi_type: IviType,
    pub ivi_purpose: Option<IviPurpose>,
    pub lane_status: Option<LaneStatus>,
    pub vehicle_characteristics: Option<VehicleCharacteristicsList>,
    pub driver_characteristics: Option<DriverCharacteristics>,
    #[rasn(value("1..=4", extensible))]
    pub layout_id: Option<u8>,
    #[rasn(value("1..=64", extensible))]
    pub pre_storedlayout_id: Option<u8>,
    pub road_sign_codes: RoadSignCodes,
    pub extra_text: Option<ConstraintTextLines1>,
}

impl GicPart {
    pub fn new(
        detection_zone_ids: Option<ZoneIds>,
        its__rrid: Option<VarLengthNumber>,
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
        layout_id: Option<u8>,
        pre_storedlayout_id: Option<u8>,
        road_sign_codes: RoadSignCodes,
        extra_text: Option<ConstraintTextLines1>,
    ) -> Self {
        Self {
            detection_zone_ids,
            its__rrid,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GlcPart {
    pub zone_id: Zid,
    pub lane_number: Option<LanePosition>,
    #[rasn(value("0..=255"))]
    pub zone_extension: Option<u8>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct GlcParts(pub SequenceOf<GlcPart>);

/// new DE in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct GoodsType(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum ISO14823Attribute {
    Dtm(InternationalSignApplicablePeriod),
    Edt(InternationalSignExemptedApplicablePeriod),
    Dfl(InternationalSignDirectionalFlowOfLane),
    Ved(InternationalSignApplicableVehicleDimensions),
    Spe(InternationalSignSpeedLimits),
    Roi(InternationalSignRateOfIncline),
    Dbv(InternationalSignDistanceBetweenVehicles),
    Ddd(InternationalSignDestinationInformation),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ISO14823CodePictogramCodeServiceCategoryCodeTrafficSignPictogram {
    DangerWarning = 0,
    Regulatory = 1,
    Informative = 2,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ISO14823CodePictogramCodeServiceCategoryCodePublicFacilitiesPictogram {
    PublicFacilities = 0,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ISO14823CodePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram {
    AmbientCondition = 0,
    RoadCondition = 1,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ISO14823CodePictogramCodeServiceCategoryCode {
    TrafficSignPictogram(ISO14823CodePictogramCodeServiceCategoryCodeTrafficSignPictogram),
    PublicFacilitiesPictogram(
        ISO14823CodePictogramCodeServiceCategoryCodePublicFacilitiesPictogram,
    ),
    AmbientOrRoadConditionPictogram(
        ISO14823CodePictogramCodeServiceCategoryCodeAmbientOrRoadConditionPictogram,
    ),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ISO14823CodePictogramCodePictogramCategoryCode {
    #[rasn(value("1..=9"))]
    pub nature: u8,
    #[rasn(value("0..=99"))]
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

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ISO14823CodePictogramCode {
    #[rasn(size("2"))]
    pub country_code: Option<OctetString>,
    pub service_category_code: ISO14823CodePictogramCodeServiceCategoryCode,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ISO14823Code {
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Int1(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct Int2(pub u16);

/// Value assignment is done in accordance with ISO 3166-1 and by
/// using the ITA.2 alphabet.

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct IssuerIdentifier(pub u16);

///Definition of Containers

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IviContainer {
    Glc(GeographicLocationContainer),
    Giv(GeneralIviContainer),
    Rcc(RoadConfigurationContainer),
    Tc(TextContainer),
    Lac(LayoutContainer),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct IviContainers(pub SequenceOf<IviContainer>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32767", extensible))]
pub struct IviIdentificationNumber(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct IviIdentificationNumbers(pub SequenceOf<IviIdentificationNumber>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct IviLaneWidth(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IviManagementContainer {
    pub service_provider_id: Provider,
    pub ivi_identification_number: IviIdentificationNumber,
    pub time_stamp: Option<TimestampIts>,
    pub valid_from: Option<TimestampIts>,
    pub valid_to: Option<TimestampIts>,
    pub connected_ivi_structures: Option<IviIdentificationNumbers>,
    pub ivi_status: IviStatus,
    #[rasn(extension_addition)]
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

/// only renamed from V1, no change

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct IviPurpose(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct IviStatus(pub u8);

/// Definition of IVI structure

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct IviType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LaneCharacteristics {
    pub zone_definition_accuracy: DefinitionAccuracy,
    pub existinglane_marking_status: LaneMarkingStatus,
    pub newlane_marking_colour: MarkingColour,
    pub lane_delimitation_left: LaneDelimitation,
    pub lane_delimitation_right: LaneDelimitation,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct LaneConfiguration(pub SequenceOf<LaneInformation>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct LaneDelimitation(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct LaneIds(pub SequenceOf<LaneID>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LaneInformationExtGroupDetectionZoneIds {
    pub detection_zone_ids: Option<ZoneIds>,
    pub relevance_zone_ids: Option<ZoneIds>,
    pub lane_characteristics: Option<LaneCharacteristics>,
    pub lane_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LaneInformation {
    pub lane_number: LanePosition,
    pub direction: Direction,
    pub validity: Option<InternationalSignApplicablePeriod>,
    pub lane_type: LaneType,
    pub lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
    pub lane_status: LaneStatus,
    pub lane_width: Option<IviLaneWidth>,
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
    ) -> Self {
        Self {
            lane_number,
            direction,
            validity,
            lane_type,
            lane_type_qualifier,
            lane_status,
            lane_width,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct LaneMarkingStatus(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct LanePositions(pub SequenceOf<LanePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct LaneStatus(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct LaneType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LayoutComponent {
    #[rasn(value("1..=8", extensible))]
    pub layout_component_id: u8,
    #[rasn(value("10..=73"))]
    pub height: u8,
    #[rasn(value("10..=265"))]
    pub width: u16,
    #[rasn(value("10..=265"))]
    pub x: u16,
    #[rasn(value("10..=73"))]
    pub y: u8,
    #[rasn(value("0..=1"))]
    pub text_scripting: u8,
}

impl LayoutComponent {
    pub fn new(
        layout_component_id: u8,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct LayoutComponents(pub SequenceOf<LayoutComponent>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LayoutContainer {
    #[rasn(value("1..=4", extensible))]
    pub layout_id: u8,
    #[rasn(value("10..=73"))]
    pub height: Option<u8>,
    #[rasn(value("10..=265"))]
    pub width: Option<u16>,
    pub layout_components: LayoutComponents,
}

impl LayoutContainer {
    pub fn new(
        layout_id: u8,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LoadType {
    pub goods_type: GoodsType,
    pub dangerous_goods_type: DangerousGoodsBasic,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum MapReference {
    Roadsegment(RoadSegmentReferenceID),
    Intersection(IntersectionReferenceID),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct MarkingColour(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct MaterialType(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=64"))]
pub struct MaxLenghtOfPlatoon(pub u8);

/// new DE in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2..=64"))]
pub struct MaxNoOfVehicles(pub u8);

/// new container part in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MlcPart {
    pub zone_id: Zid,
    pub lane_ids: Option<LaneIds>,
}

impl MlcPart {
    pub fn new(zone_id: Zid, lane_ids: Option<LaneIds>) -> Self {
        Self { zone_id, lane_ids }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct MlcParts(pub SequenceOf<MlcPart>);

/// 4 bits, reserved for carbon dioxide pollution values as defined in

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PassengerCapacity {
    pub number_of_seats: Int1,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PlatooningRule {
    pub priority: PriorityLevel,
    pub allowed_sae_automation_levels: SaeAutomationLevels,
    pub max_no_of_vehicles: Option<MaxNoOfVehicles>,
    pub max_lenght_of_platoon: Option<MaxLenghtOfPlatoon>,
    pub min_gap_between_vehicles: Option<GapBetweenVehicles>,
    pub platoon_max_speed_limit: Option<SpeedValue>,
    pub platoon_min_speed_limit: Option<SpeedValue>,
    pub platoon_speed_recommendation: Option<SpeedValue>,
    pub road_sign_codes: Option<RoadSignCodes>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct PlatooningRules(pub SequenceOf<PlatooningRule>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PolygonalLine {
    DeltaPositions(DeltaPositions),
    DeltaPositionsWithAltitude(DeltaReferencePositions),
    AbsolutePositions(AbsolutePositions),
    AbsolutePositionsWithAltitude(AbsolutePositionsWAltitude),
}

/// new DE in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=2"))]
pub struct PriorityLevel(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Provider {
    pub country_code: CountryCode,
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

/// new DE in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct RSCUnit(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum RSCodeCode {
    ViennaConvention(VcCode),
    Iso14823(ISO14823Code),
    #[rasn(value("0..=65535"))]
    ItisCodes(u16),
    AnyCatalogue(AnyCatalogue),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RSCode {
    #[rasn(value("1..=4", extensible))]
    pub layout_component_id: Option<u8>,
    pub code: RSCodeCode,
}

impl RSCode {
    pub fn new(layout_component_id: Option<u8>, code: RSCodeCode) -> Self {
        Self {
            layout_component_id,
            code,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RccPart {
    pub relevance_zone_ids: ZoneIds,
    pub road_type: RoadType,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct RoadConfigurationContainer(pub SequenceOf<RccPart>);

/// new DF in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct RoadSignCodes(pub SequenceOf<RSCode>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct RoadSurfaceContainer(pub SequenceOf<RscPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadSurfaceDynamicCharacteristics {
    pub condition: Condition,
    pub temperature: Temperature,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadSurfaceStaticCharacteristics {
    pub friction_coefficient: FrictionCoefficient,
    pub material: MaterialType,
    pub wear: WearLevel,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RscPart {
    pub detection_zone_ids: Option<ZoneIds>,
    pub relevance_zone_ids: ZoneIds,
    pub direction: Option<Direction>,
    pub road_surface_static_characteristics: Option<RoadSurfaceStaticCharacteristics>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=5"))]
pub struct SaeAutomationLevel(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct SaeAutomationLevels(pub SequenceOf<SaeAutomationLevel>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Segment {
    pub line: PolygonalLine,
    pub lane_width: Option<IviLaneWidth>,
}

impl Segment {
    pub fn new(line: PolygonalLine, lane_width: Option<IviLaneWidth>) -> Self {
        Self { line, lane_width }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TcPartExtGroupIviType {
    pub ivi_type: IviType,
    pub lane_status: Option<LaneStatus>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TcPart {
    pub detection_zone_ids: Option<ZoneIds>,
    pub relevance_zone_ids: ZoneIds,
    pub direction: Option<Direction>,
    pub driver_awareness_zone_ids: Option<ZoneIds>,
    #[rasn(value("0..=255"))]
    pub minimum_awareness_time: Option<u8>,
    pub applicable_lanes: Option<LanePositions>,
    #[rasn(value("1..=4", extensible))]
    pub layout_id: Option<u8>,
    #[rasn(value("1..=64", extensible))]
    pub pre_storedlayout_id: Option<u8>,
    pub text: Option<TextLines>,
    pub data: OctetString,
}

impl TcPart {
    pub fn new(
        detection_zone_ids: Option<ZoneIds>,
        relevance_zone_ids: ZoneIds,
        direction: Option<Direction>,
        driver_awareness_zone_ids: Option<ZoneIds>,
        minimum_awareness_time: Option<u8>,
        applicable_lanes: Option<LanePositions>,
        layout_id: Option<u8>,
        pre_storedlayout_id: Option<u8>,
        text: Option<TextLines>,
        data: OctetString,
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
        }
    }
}

/// new DE in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-100..=151"))]
pub struct Temperature(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Text {
    #[rasn(value("1..=4", extensible))]
    pub layout_component_id: Option<u8>,
    #[rasn(size("10"))]
    pub language: BitString,
    pub text_content: Utf8String,
}

impl Text {
    pub fn new(
        layout_component_id: Option<u8>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct TextContainer(pub SequenceOf<TcPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TextLines(pub SequenceOf<Text>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TractorCharacteristics {
    pub equal_to: Option<VehicleCharacteristicsFixValuesList>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TrailerCharacteristics {
    pub equal_to: Option<TrailerCharacteristicsFixValuesList>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TrailerCharacteristicsFixValuesList(pub SequenceOf<VehicleCharacteristicsFixValues>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3"))]
pub struct TrailerCharacteristicsList(pub SequenceOf<TrailerCharacteristics>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TrailerCharacteristicsRangesList(pub SequenceOf<VehicleCharacteristicsRanges>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct TrainCharacteristics(pub TractorCharacteristics);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct TreatmentType(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum UnitType {
    MgKm = 0,
    MgKWh = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct ValidityPeriods(pub SequenceOf<InternationalSignApplicablePeriod>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct VcClass(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VcCode {
    pub road_sign_class: VcClass,
    #[rasn(value("1..=64"))]
    pub road_sign_code: u8,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct VcOption(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VehicleCharacteristicsFixValues {
    SimpleVehicleType(StationType),
    EuVehicleCategoryCode(EuVehicleCategoryCode),
    Iso3833VehicleType(Iso3833VehicleType),
    EuroAndCo2value(EnvironmentalCharacteristics),
    EngineCharacteristics(EngineCharacteristics),
    LoadType(LoadType),
    Usage(VehicleRole),
}

/// new DF in V2

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct VehicleCharacteristicsFixValuesList(pub SequenceOf<VehicleCharacteristicsFixValues>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct VehicleCharacteristicsList(pub SequenceOf<CompleteVehicleCharacteristics>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VehicleCharacteristicsRangesLimits {
    #[rasn(value("0..=7"))]
    NumberOfAxles(u8),
    VehicleDimensions(VehicleDimensions),
    VehicleWeightLimits(VehicleWeightLimits),
    AxleWeightLimits(AxleWeightLimits),
    PassengerCapacity(PassengerCapacity),
    ExhaustEmissionValues(ExhaustEmissionValues),
    DieselEmissionValues(DieselEmissionValues),
    SoundLevel(SoundLevel),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleCharacteristicsRanges {
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct VehicleCharacteristicsRangesList(pub SequenceOf<VehicleCharacteristicsRanges>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleDimensions {
    pub vehicle_length_overall: Int1,
    pub vehicle_heigth_overall: Int1,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleWeightLimits {
    pub vehicle_max_laden_weight: Int2,
    pub vehicle_train_maximum_weight: Int2,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct WearLevel(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32", extensible))]
pub struct Zid(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Zone {
    Segment(Segment),
    Area(PolygonalLine),
    ComputedSegment(ComputedSegment),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct ZoneIds(pub SequenceOf<Zid>);

// =====================================================
// IVIM-PDU-Descriptions
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) ts103301(103301) ivim(2) major-version-2(2) minor-version-1(1) }
// =====================================================

///*
///* In vehicle information Message Message
///* This DF includes DEs for the IVIM protocolVersion, the IVI message type identifier `messageID`,
///* the station identifier `stationID` of the originating ITS-S and the IVI data from ISO TS 19321.
///*
///* @field header: The DE `protocolVersion` is used to select the appropriate protocol decoder at the receiving ITS-S.
///*                It shall be set to 2.
///*                The DE `messageID` shall be ivim(6).
///* @field ivi:    contains the IVI data as defined in ISO TS 19321.
///*
///* @category: Basic Information
///* @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
