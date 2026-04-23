#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod its_container {
    extern crate alloc;
    use core::borrow::Borrow;

    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct AccelerationConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AccelerationControl(pub FixedBitString<7usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AccidentSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ActionID {
        #[rasn(identifier = "originatingStationID")]
        pub originating_station_id: StationID,
        #[rasn(identifier = "sequenceNumber")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-AdhesionSubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-ExtremeWeatherConditionSubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-PrecipitationSubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "AdverseWeatherCondition-VisibilitySubCauseCode",
        value("0..=255")
    )]
    pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Altitude {
        #[rasn(identifier = "altitudeValue")]
        pub altitude_value: AltitudeValue,
        #[rasn(identifier = "altitudeConfidence")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum AltitudeConfidence {
        #[rasn(identifier = "alt-000-01")]
        alt_000_01 = 0,
        #[rasn(identifier = "alt-000-02")]
        alt_000_02 = 1,
        #[rasn(identifier = "alt-000-05")]
        alt_000_05 = 2,
        #[rasn(identifier = "alt-000-10")]
        alt_000_10 = 3,
        #[rasn(identifier = "alt-000-20")]
        alt_000_20 = 4,
        #[rasn(identifier = "alt-000-50")]
        alt_000_50 = 5,
        #[rasn(identifier = "alt-001-00")]
        alt_001_00 = 6,
        #[rasn(identifier = "alt-002-00")]
        alt_002_00 = 7,
        #[rasn(identifier = "alt-005-00")]
        alt_005_00 = 8,
        #[rasn(identifier = "alt-010-00")]
        alt_010_00 = 9,
        #[rasn(identifier = "alt-020-00")]
        alt_020_00 = 10,
        #[rasn(identifier = "alt-050-00")]
        alt_050_00 = 11,
        #[rasn(identifier = "alt-100-00")]
        alt_100_00 = 12,
        #[rasn(identifier = "alt-200-00")]
        alt_200_00 = 13,
        outOfRange = 14,
        unavailable = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-100000..=800001"))]
    pub struct AltitudeValue(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CauseCode {
        #[rasn(identifier = "causeCode")]
        pub cause_code: CauseCodeType,
        #[rasn(identifier = "subCauseCode")]
        pub sub_cause_code: SubCauseCodeType,
    }
    impl CauseCode {
        pub fn new(cause_code: CauseCodeType, sub_cause_code: SubCauseCodeType) -> Self {
            Self {
                cause_code,
                sub_cause_code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CauseCodeType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CenDsrcTollingZone {
        #[rasn(identifier = "protectedZoneLatitude")]
        pub protected_zone_latitude: Latitude,
        #[rasn(identifier = "protectedZoneLongitude")]
        pub protected_zone_longitude: Longitude,
        #[rasn(identifier = "cenDsrcTollingZoneID")]
        pub cen_dsrc_tolling_zone_id: Option<CenDsrcTollingZoneID>,
    }
    impl CenDsrcTollingZone {
        pub fn new(
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            cen_dsrc_tolling_zone_id: Option<CenDsrcTollingZoneID>,
        ) -> Self {
            Self {
                protected_zone_latitude,
                protected_zone_longitude,
                cen_dsrc_tolling_zone_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CenDsrcTollingZoneID(pub ProtectedZoneID);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClosedLanes {
        #[rasn(identifier = "innerhardShoulderStatus")]
        pub innerhard_shoulder_status: Option<HardShoulderStatus>,
        #[rasn(identifier = "outerhardShoulderStatus")]
        pub outerhard_shoulder_status: Option<HardShoulderStatus>,
        #[rasn(identifier = "drivingLaneStatus")]
        pub driving_lane_status: Option<DrivingLaneStatus>,
    }
    impl ClosedLanes {
        pub fn new(
            innerhard_shoulder_status: Option<HardShoulderStatus>,
            outerhard_shoulder_status: Option<HardShoulderStatus>,
            driving_lane_status: Option<DrivingLaneStatus>,
        ) -> Self {
            Self {
                innerhard_shoulder_status,
                outerhard_shoulder_status,
                driving_lane_status,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CollisionRiskSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Curvature {
        #[rasn(identifier = "curvatureValue")]
        pub curvature_value: CurvatureValue,
        #[rasn(identifier = "curvatureConfidence")]
        pub curvature_confidence: CurvatureConfidence,
    }
    impl Curvature {
        pub fn new(
            curvature_value: CurvatureValue,
            curvature_confidence: CurvatureConfidence,
        ) -> Self {
            Self {
                curvature_value,
                curvature_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CurvatureCalculationMode {
        yawRateUsed = 0,
        yawRateNotUsed = 1,
        unavailable = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum CurvatureConfidence {
        #[rasn(identifier = "onePerMeter-0-00002")]
        onePerMeter_0_00002 = 0,
        #[rasn(identifier = "onePerMeter-0-0001")]
        onePerMeter_0_0001 = 1,
        #[rasn(identifier = "onePerMeter-0-0005")]
        onePerMeter_0_0005 = 2,
        #[rasn(identifier = "onePerMeter-0-002")]
        onePerMeter_0_002 = 3,
        #[rasn(identifier = "onePerMeter-0-01")]
        onePerMeter_0_01 = 4,
        #[rasn(identifier = "onePerMeter-0-1")]
        onePerMeter_0_1 = 5,
        outOfRange = 6,
        unavailable = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1023..=1023"))]
    pub struct CurvatureValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousEndOfQueueSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DangerousGoodsBasic {
        explosives1 = 0,
        explosives2 = 1,
        explosives3 = 2,
        explosives4 = 3,
        explosives5 = 4,
        explosives6 = 5,
        flammableGases = 6,
        nonFlammableGases = 7,
        toxicGases = 8,
        flammableLiquids = 9,
        flammableSolids = 10,
        substancesLiableToSpontaneousCombustion = 11,
        substancesEmittingFlammableGasesUponContactWithWater = 12,
        oxidizingSubstances = 13,
        organicPeroxides = 14,
        toxicSubstances = 15,
        infectiousSubstances = 16,
        radioactiveMaterial = 17,
        corrosiveSubstances = 18,
        miscellaneousDangerousSubstances = 19,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct DangerousGoodsExtended {
        #[rasn(identifier = "dangerousGoodsType")]
        pub dangerous_goods_type: DangerousGoodsBasic,
        #[rasn(value("0..=9999"), identifier = "unNumber")]
        pub un_number: u16,
        #[rasn(identifier = "elevatedTemperature")]
        pub elevated_temperature: bool,
        #[rasn(identifier = "tunnelsRestricted")]
        pub tunnels_restricted: bool,
        #[rasn(identifier = "limitedQuantity")]
        pub limited_quantity: bool,
        #[rasn(size("1..=24"), identifier = "emergencyActionCode")]
        pub emergency_action_code: Option<Ia5String>,
        #[rasn(identifier = "phoneNumber")]
        pub phone_number: Option<PhoneNumber>,
        #[rasn(identifier = "companyName")]
        pub company_name: Option<Utf8String>,
    }
    impl DangerousGoodsExtended {
        pub fn new(
            dangerous_goods_type: DangerousGoodsBasic,
            un_number: u16,
            elevated_temperature: bool,
            tunnels_restricted: bool,
            limited_quantity: bool,
            emergency_action_code: Option<Ia5String>,
            phone_number: Option<PhoneNumber>,
            company_name: Option<Utf8String>,
        ) -> Self {
            Self {
                dangerous_goods_type,
                un_number,
                elevated_temperature,
                tunnels_restricted,
                limited_quantity,
                emergency_action_code,
                phone_number,
                company_name,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousSituationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-12700..=12800"))]
    pub struct DeltaAltitude(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLatitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLongitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DeltaReferencePosition {
        #[rasn(identifier = "deltaLatitude")]
        pub delta_latitude: DeltaLatitude,
        #[rasn(identifier = "deltaLongitude")]
        pub delta_longitude: DeltaLongitude,
        #[rasn(identifier = "deltaAltitude")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256"))]
    pub struct DigitalMap(pub SequenceOf<ReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DriveDirection {
        forward = 0,
        backward = 1,
        unavailable = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=13"))]
    pub struct DrivingLaneStatus(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct EmbarkationStatus(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EmergencyPriority(pub FixedBitString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EnergyStorageType(pub FixedBitString<7usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=23"))]
    pub struct EventHistory(pub SequenceOf<EventPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct EventPoint {
        #[rasn(identifier = "eventPosition")]
        pub event_position: DeltaReferencePosition,
        #[rasn(identifier = "eventDeltaTime")]
        pub event_delta_time: Option<PathDeltaTime>,
        #[rasn(identifier = "informationQuality")]
        pub information_quality: InformationQuality,
    }
    impl EventPoint {
        pub fn new(
            event_position: DeltaReferencePosition,
            event_delta_time: Option<PathDeltaTime>,
            information_quality: InformationQuality,
        ) -> Self {
            Self {
                event_position,
                event_delta_time,
                information_quality,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ExteriorLights(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum HardShoulderStatus {
        availableForStopping = 0,
        closed = 1,
        availableForDriving = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-AnimalOnTheRoadSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-DangerousCurveSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-ObstacleOnTheRoadSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(
        delegate,
        identifier = "HazardousLocation-SurfaceConditionSubCauseCode",
        value("0..=255")
    )]
    pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Heading {
        #[rasn(identifier = "headingValue")]
        pub heading_value: HeadingValue,
        #[rasn(identifier = "headingConfidence")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct HeadingConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct HeadingValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=100"))]
    pub struct HeightLonCarr(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanProblemSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct InformationQuality(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=40"))]
    pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ItsPduHeader {
        #[rasn(value("0..=255"), identifier = "protocolVersion")]
        pub protocol_version: u8,
        #[rasn(value("0..=255"), identifier = "messageID")]
        pub message_id: u8,
        #[rasn(identifier = "stationID")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1..=14"))]
    pub struct LanePosition(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LateralAcceleration {
        #[rasn(identifier = "lateralAccelerationValue")]
        pub lateral_acceleration_value: LateralAccelerationValue,
        #[rasn(identifier = "lateralAccelerationConfidence")]
        pub lateral_acceleration_confidence: AccelerationConfidence,
    }
    impl LateralAcceleration {
        pub fn new(
            lateral_acceleration_value: LateralAccelerationValue,
            lateral_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                lateral_acceleration_value,
                lateral_acceleration_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LateralAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-900000000..=900000001"))]
    pub struct Latitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LightBarSirenInUse(pub FixedBitString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1800000000..=1800000001"))]
    pub struct Longitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LongitudinalAcceleration {
        #[rasn(identifier = "longitudinalAccelerationValue")]
        pub longitudinal_acceleration_value: LongitudinalAccelerationValue,
        #[rasn(identifier = "longitudinalAccelerationConfidence")]
        pub longitudinal_acceleration_confidence: AccelerationConfidence,
    }
    impl LongitudinalAcceleration {
        pub fn new(
            longitudinal_acceleration_value: LongitudinalAccelerationValue,
            longitudinal_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                longitudinal_acceleration_value,
                longitudinal_acceleration_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LongitudinalAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct NumberOfOccupants(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct OpeningDaysHours(pub Utf8String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=65535", extensible))]
    pub struct PathDeltaTime(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=40"))]
    pub struct PathHistory(pub SequenceOf<PathPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PathPoint {
        #[rasn(identifier = "pathPosition")]
        pub path_position: DeltaReferencePosition,
        #[rasn(identifier = "pathDeltaTime")]
        pub path_delta_time: Option<PathDeltaTime>,
    }
    impl PathPoint {
        pub fn new(
            path_position: DeltaReferencePosition,
            path_delta_time: Option<PathDeltaTime>,
        ) -> Self {
            Self {
                path_position,
                path_delta_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct PerformanceClass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct PhoneNumber(pub NumericString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=63"))]
    pub struct PosCentMass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PosConfidenceEllipse {
        #[rasn(identifier = "semiMajorConfidence")]
        pub semi_major_confidence: SemiAxisLength,
        #[rasn(identifier = "semiMinorConfidence")]
        pub semi_minor_confidence: SemiAxisLength,
        #[rasn(identifier = "semiMajorOrientation")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=20"))]
    pub struct PosFrontAx(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct PosLonCarr(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=30"))]
    pub struct PosPillar(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PositionOfOccupants(pub FixedBitString<20usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct PositionOfPillars(pub SequenceOf<PosPillar>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PositioningSolutionType {
        noPositioningSolution = 0,
        sGNSS = 1,
        dGNSS = 2,
        sGNSSplusDR = 3,
        dGNSSplusDR = 4,
        dR = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PostCrashSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ProtectedCommunicationZone {
        #[rasn(identifier = "protectedZoneType")]
        pub protected_zone_type: ProtectedZoneType,
        #[rasn(identifier = "expiryTime")]
        pub expiry_time: Option<TimestampIts>,
        #[rasn(identifier = "protectedZoneLatitude")]
        pub protected_zone_latitude: Latitude,
        #[rasn(identifier = "protectedZoneLongitude")]
        pub protected_zone_longitude: Longitude,
        #[rasn(identifier = "protectedZoneRadius")]
        pub protected_zone_radius: Option<ProtectedZoneRadius>,
        #[rasn(identifier = "protectedZoneID")]
        pub protected_zone_id: Option<ProtectedZoneID>,
    }
    impl ProtectedCommunicationZone {
        pub fn new(
            protected_zone_type: ProtectedZoneType,
            expiry_time: Option<TimestampIts>,
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            protected_zone_radius: Option<ProtectedZoneRadius>,
            protected_zone_id: Option<ProtectedZoneID>,
        ) -> Self {
            Self {
                protected_zone_type,
                expiry_time,
                protected_zone_latitude,
                protected_zone_longitude,
                protected_zone_radius,
                protected_zone_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=134217727"))]
    pub struct ProtectedZoneID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255", extensible))]
    pub struct ProtectedZoneRadius(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProtectedZoneType {
        permanentCenDsrcTolling = 0,
        #[rasn(extension_addition)]
        temporaryCenDsrcTolling = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PtActivation {
        #[rasn(identifier = "ptActivationType")]
        pub pt_activation_type: PtActivationType,
        #[rasn(identifier = "ptActivationData")]
        pub pt_activation_data: PtActivationData,
    }
    impl PtActivation {
        pub fn new(
            pt_activation_type: PtActivationType,
            pt_activation_data: PtActivationData,
        ) -> Self {
            Self {
                pt_activation_type,
                pt_activation_data,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=20"))]
    pub struct PtActivationData(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PtActivationType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ReferencePosition {
        pub latitude: Latitude,
        pub longitude: Longitude,
        #[rasn(identifier = "positionConfidenceEllipse")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RelevanceDistance {
        lessThan50m = 0,
        lessThan100m = 1,
        lessThan200m = 2,
        lessThan500m = 3,
        lessThan1000m = 4,
        lessThan5km = 5,
        lessThan10km = 6,
        over10km = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RelevanceTrafficDirection {
        allTrafficDirections = 0,
        upstreamTraffic = 1,
        downstreamTraffic = 2,
        oppositeTraffic = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestResponseIndication {
        request = 0,
        response = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct RestrictedTypes(pub SequenceOf<StationType>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RoadType {
        #[rasn(identifier = "urban-NoStructuralSeparationToOppositeLanes")]
        urban_NoStructuralSeparationToOppositeLanes = 0,
        #[rasn(identifier = "urban-WithStructuralSeparationToOppositeLanes")]
        urban_WithStructuralSeparationToOppositeLanes = 1,
        #[rasn(identifier = "nonUrban-NoStructuralSeparationToOppositeLanes")]
        nonUrban_NoStructuralSeparationToOppositeLanes = 2,
        #[rasn(identifier = "nonUrban-WithStructuralSeparationToOppositeLanes")]
        nonUrban_WithStructuralSeparationToOppositeLanes = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RoadworksSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct SemiAxisLength(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SequenceNumber(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalViolationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SlowVehicleSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct SpecialTransportType(pub FixedBitString<4usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Speed {
        #[rasn(identifier = "speedValue")]
        pub speed_value: SpeedValue,
        #[rasn(identifier = "speedConfidence")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SpeedConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct SpeedLimit(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct SpeedValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct StationID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum StationarySince {
        lessThan1Minute = 0,
        lessThan2Minutes = 1,
        lessThan15Minutes = 2,
        equalOrGreater15Minutes = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationaryVehicleSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SteeringWheelAngle {
        #[rasn(identifier = "steeringWheelAngleValue")]
        pub steering_wheel_angle_value: SteeringWheelAngleValue,
        #[rasn(identifier = "steeringWheelAngleConfidence")]
        pub steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
    }
    impl SteeringWheelAngle {
        pub fn new(
            steering_wheel_angle_value: SteeringWheelAngleValue,
            steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
        ) -> Self {
            Self {
                steering_wheel_angle_value,
                steering_wheel_angle_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SteeringWheelAngleConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-511..=512"))]
    pub struct SteeringWheelAngleValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SubCauseCodeType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-60..=67"))]
    pub struct Temperature(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4398046511103"))]
    pub struct TimestampIts(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=7"))]
    pub struct Traces(pub SequenceOf<PathHistory>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct TrafficConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TrafficRule {
        noPassing = 0,
        noPassingForTrucks = 1,
        passToRight = 2,
        passToLeft = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=10000"))]
    pub struct TransmissionInterval(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct TurningRadius(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("6"))]
    pub struct VDS(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=86400"))]
    pub struct ValidityDuration(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct VehicleBreakdownSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleIdentification {
        #[rasn(identifier = "wMInumber")]
        pub w_minumber: Option<WMInumber>,
        #[rasn(identifier = "vDS")]
        pub v_ds: Option<VDS>,
    }
    impl VehicleIdentification {
        pub fn new(w_minumber: Option<WMInumber>, v_ds: Option<VDS>) -> Self {
            Self { w_minumber, v_ds }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleLength {
        #[rasn(identifier = "vehicleLengthValue")]
        pub vehicle_length_value: VehicleLengthValue,
        #[rasn(identifier = "vehicleLengthConfidenceIndication")]
        pub vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
    }
    impl VehicleLength {
        pub fn new(
            vehicle_length_value: VehicleLengthValue,
            vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
        ) -> Self {
            Self {
                vehicle_length_value,
                vehicle_length_confidence_indication,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum VehicleLengthConfidenceIndication {
        noTrailerPresent = 0,
        trailerPresentWithKnownLength = 1,
        trailerPresentWithUnknownLength = 2,
        trailerPresenceIsUnknown = 3,
        unavailable = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1023"))]
    pub struct VehicleLengthValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=1024"))]
    pub struct VehicleMass(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum VehicleRole {
        default = 0,
        publicTransport = 1,
        specialTransport = 2,
        dangerousGoods = 3,
        roadWork = 4,
        rescue = 5,
        emergency = 6,
        safetyCar = 7,
        agriculture = 8,
        commercial = 9,
        military = 10,
        roadOperator = 11,
        taxi = 12,
        reserved1 = 13,
        reserved2 = 14,
        reserved3 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=62"))]
    pub struct VehicleWidth(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VerticalAcceleration {
        #[rasn(identifier = "verticalAccelerationValue")]
        pub vertical_acceleration_value: VerticalAccelerationValue,
        #[rasn(identifier = "verticalAccelerationConfidence")]
        pub vertical_acceleration_confidence: AccelerationConfidence,
    }
    impl VerticalAcceleration {
        pub fn new(
            vertical_acceleration_value: VerticalAccelerationValue,
            vertical_acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                vertical_acceleration_value,
                vertical_acceleration_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct VerticalAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=3"))]
    pub struct WMInumber(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct WheelBaseVehicle(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct WrongWayDrivingSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct YawRate {
        #[rasn(identifier = "yawRateValue")]
        pub yaw_rate_value: YawRateValue,
        #[rasn(identifier = "yawRateConfidence")]
        pub yaw_rate_confidence: YawRateConfidence,
    }
    impl YawRate {
        pub fn new(yaw_rate_value: YawRateValue, yaw_rate_confidence: YawRateConfidence) -> Self {
            Self {
                yaw_rate_value,
                yaw_rate_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum YawRateConfidence {
        #[rasn(identifier = "degSec-000-01")]
        degSec_000_01 = 0,
        #[rasn(identifier = "degSec-000-05")]
        degSec_000_05 = 1,
        #[rasn(identifier = "degSec-000-10")]
        degSec_000_10 = 2,
        #[rasn(identifier = "degSec-001-00")]
        degSec_001_00 = 3,
        #[rasn(identifier = "degSec-005-00")]
        degSec_005_00 = 4,
        #[rasn(identifier = "degSec-010-00")]
        degSec_010_00 = 5,
        #[rasn(identifier = "degSec-100-00")]
        degSec_100_00 = 6,
        outOfRange = 7,
        unavailable = 8,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-32766..=32767"))]
    pub struct YawRateValue(pub i16);
}
