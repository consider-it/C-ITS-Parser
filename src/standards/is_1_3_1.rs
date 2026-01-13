#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod etsi_schema {
    extern crate alloc;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositionsWithAltitude(pub SequenceOf<AbsolutePositionWAltitude>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct AccelerationConfidence(pub u8);
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
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousAdvisorySpeedRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousAdvisorySpeedRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct AdvisorySpeedRegional(pub SequenceOf<AnonymousAdvisorySpeedRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AdvisorySpeed {
        #[rasn(identifier = "type")]
        pub r_type: AdvisorySpeedType,
        pub speed: Option<SpeedAdvice>,
        pub confidence: Option<SAESpeedConfidence>,
        pub distance: Option<ZoneLength>,
        pub class: Option<RestrictionClassID>,
        pub regional: Option<AdvisorySpeedRegional>,
    }
    impl AdvisorySpeed {
        pub fn new(
            r_type: AdvisorySpeedType,
            speed: Option<SpeedAdvice>,
            confidence: Option<SAESpeedConfidence>,
            distance: Option<ZoneLength>,
            class: Option<RestrictionClassID>,
            regional: Option<AdvisorySpeedRegional>,
        ) -> Self {
            Self {
                r_type,
                speed,
                confidence,
                distance,
                class,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct AdvisorySpeedList(pub SequenceOf<AdvisorySpeed>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdvisorySpeedType {
        none = 0,
        greenwave = 1,
        ecoDrive = 2,
        transit = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct AllowedManeuvers(pub FixedBitString<12usize>);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AmbientOrRoadConditionPictogram {
        ambientCondition = 0,
        roadCondition = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct Angle(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct AngleConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AnimalSubclass {
        #[rasn(default = "animal_subclass_r_type_default", identifier = "type")]
        pub r_type: AnimalSubclassType,
        #[rasn(default = "animal_subclass_confidence_default")]
        pub confidence: ClassConfidence,
    }
    impl AnimalSubclass {
        pub fn new(r_type: AnimalSubclassType, confidence: ClassConfidence) -> Self {
            Self { r_type, confidence }
        }
    }
    impl std::default::Default for AnimalSubclass {
        fn default() -> Self {
            Self {
                r_type: animal_subclass_r_type_default(),
                confidence: animal_subclass_confidence_default(),
            }
        }
    }
    fn animal_subclass_r_type_default() -> AnimalSubclassType {
        AnimalSubclassType(0)
    }
    fn animal_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AnimalSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AntennaOffsetSet {
        #[rasn(identifier = "antOffsetX")]
        pub ant_offset_x: OffsetB12,
        #[rasn(identifier = "antOffsetY")]
        pub ant_offset_y: OffsetB09,
        #[rasn(identifier = "antOffsetZ")]
        pub ant_offset_z: OffsetB10,
    }
    impl AntennaOffsetSet {
        pub fn new(
            ant_offset_x: OffsetB12,
            ant_offset_y: OffsetB09,
            ant_offset_z: OffsetB10,
        ) -> Self {
            Self {
                ant_offset_x,
                ant_offset_y,
                ant_offset_z,
            }
        }
    }
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
    #[rasn(delegate, value("0..=15"))]
    pub struct ApproachID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AreaCircular {
        #[rasn(identifier = "nodeCenterPoint")]
        pub node_center_point: Option<OffsetPoint>,
        pub radius: Radius,
    }
    impl AreaCircular {
        pub fn new(node_center_point: Option<OffsetPoint>, radius: Radius) -> Self {
            Self {
                node_center_point,
                radius,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AreaEllipse {
        #[rasn(identifier = "nodeCenterPoint")]
        pub node_center_point: Option<OffsetPoint>,
        #[rasn(identifier = "semiMinorRangeLength")]
        pub semi_minor_range_length: SemiRangeLength,
        #[rasn(identifier = "semiMajorRangeLength")]
        pub semi_major_range_length: SemiRangeLength,
        #[rasn(identifier = "semiMajorRangeOrientation")]
        pub semi_major_range_orientation: WGS84AngleValue,
        #[rasn(identifier = "semiHeight")]
        pub semi_height: Option<SemiRangeLength>,
    }
    impl AreaEllipse {
        pub fn new(
            node_center_point: Option<OffsetPoint>,
            semi_minor_range_length: SemiRangeLength,
            semi_major_range_length: SemiRangeLength,
            semi_major_range_orientation: WGS84AngleValue,
            semi_height: Option<SemiRangeLength>,
        ) -> Self {
            Self {
                node_center_point,
                semi_minor_range_length,
                semi_major_range_length,
                semi_major_range_orientation,
                semi_height,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AreaPolygon {
        #[rasn(identifier = "polyPointList")]
        pub poly_point_list: PolyPointList,
    }
    impl AreaPolygon {
        pub fn new(poly_point_list: PolyPointList) -> Self {
            Self { poly_point_list }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AreaRadial {
        pub range: Range,
        #[rasn(identifier = "stationaryHorizontalOpeningAngleStart")]
        pub stationary_horizontal_opening_angle_start: WGS84AngleValue,
        #[rasn(identifier = "stationaryHorizontalOpeningAngleEnd")]
        pub stationary_horizontal_opening_angle_end: WGS84AngleValue,
        #[rasn(identifier = "verticalOpeningAngleStart")]
        pub vertical_opening_angle_start: Option<CartesianAngleValue>,
        #[rasn(identifier = "verticalOpeningAngleEnd")]
        pub vertical_opening_angle_end: Option<CartesianAngleValue>,
        #[rasn(identifier = "sensorPositionOffset")]
        pub sensor_position_offset: Option<OffsetPoint>,
        #[rasn(identifier = "sensorHeight")]
        pub sensor_height: Option<SensorHeight>,
    }
    impl AreaRadial {
        pub fn new(
            range: Range,
            stationary_horizontal_opening_angle_start: WGS84AngleValue,
            stationary_horizontal_opening_angle_end: WGS84AngleValue,
            vertical_opening_angle_start: Option<CartesianAngleValue>,
            vertical_opening_angle_end: Option<CartesianAngleValue>,
            sensor_position_offset: Option<OffsetPoint>,
            sensor_height: Option<SensorHeight>,
        ) -> Self {
            Self {
                range,
                stationary_horizontal_opening_angle_start,
                stationary_horizontal_opening_angle_end,
                vertical_opening_angle_start,
                vertical_opening_angle_end,
                sensor_position_offset,
                sensor_height,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AreaRectangle {
        #[rasn(identifier = "nodeCenterPoint")]
        pub node_center_point: Option<OffsetPoint>,
        #[rasn(identifier = "semiMajorRangeLength")]
        pub semi_major_range_length: SemiRangeLength,
        #[rasn(identifier = "semiMinorRangeLength")]
        pub semi_minor_range_length: SemiRangeLength,
        #[rasn(identifier = "semiMajorRangeOrientation")]
        pub semi_major_range_orientation: WGS84AngleValue,
        #[rasn(identifier = "semiHeight")]
        pub semi_height: Option<SemiRangeLength>,
    }
    impl AreaRectangle {
        pub fn new(
            node_center_point: Option<OffsetPoint>,
            semi_major_range_length: SemiRangeLength,
            semi_minor_range_length: SemiRangeLength,
            semi_major_range_orientation: WGS84AngleValue,
            semi_height: Option<SemiRangeLength>,
        ) -> Self {
            Self {
                node_center_point,
                semi_major_range_length,
                semi_minor_range_length,
                semi_major_range_orientation,
                semi_height,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "INTEGER", value("0..=127", extensible))]
    pub struct AnonymousAttributeIdList(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=127", extensible))]
    pub struct AttributeIdList(pub SequenceOf<AnonymousAttributeIdList>);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BasicVehicleRole {
        basicVehicle = 0,
        publicTransport = 1,
        specialTransport = 2,
        dangerousGoods = 3,
        roadWork = 4,
        roadRescue = 5,
        emergency = 6,
        safetyCar = 7,
        #[rasn(identifier = "none-unknown")]
        none_unknown = 8,
        truck = 9,
        motorcycle = 10,
        roadSideSource = 11,
        police = 12,
        fire = 13,
        ambulance = 14,
        dot = 15,
        transit = 16,
        slowMoving = 17,
        stopNgo = 18,
        cyclist = 19,
        pedestrian = 20,
        nonMotorized = 21,
        military = 22,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BatteryStatus {
        unknown = 0,
        critical = 1,
        low = 2,
        good = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CPM {
        pub header: ItsPduHeader,
        pub cpm: CollectivePerceptionMessage,
    }
    impl CPM {
        pub fn new(header: ItsPduHeader, cpm: CollectivePerceptionMessage) -> Self {
            Self { header, cpm }
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
    #[rasn(automatic_tags)]
    pub struct CartesianAngle {
        pub value: CartesianAngleValue,
        pub confidence: AngleConfidence,
    }
    impl CartesianAngle {
        pub fn new(value: CartesianAngleValue, confidence: AngleConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct CartesianAngleValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct ClassConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Code {
        viennaConvention(VcCode),
        iso14823(ISO14823Code),
        #[rasn(value("0..=65535"))]
        itisCodes(u16),
        anyCatalogue(AnyCatalogue),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CollectivePerceptionMessage {
        #[rasn(identifier = "generationDeltaTime")]
        pub generation_delta_time: GenerationDeltaTime,
        #[rasn(identifier = "cpmParameters")]
        pub cpm_parameters: CpmParameters,
    }
    impl CollectivePerceptionMessage {
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
            cpm_parameters: CpmParameters,
        ) -> Self {
            Self {
                generation_delta_time,
                cpm_parameters,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct ComparisonOperator(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CompleteVehicleCharacteristics {
        pub tractor: Option<TractorCharacteristics>,
        #[rasn(size("1..=3"))]
        pub trailer: Option<SequenceOf<TrailerCharacteristics>>,
        pub train: Option<TrainCharacteristics>,
    }
    impl CompleteVehicleCharacteristics {
        pub fn new(
            tractor: Option<TractorCharacteristics>,
            trailer: Option<SequenceOf<TrailerCharacteristics>>,
            train: Option<TrainCharacteristics>,
        ) -> Self {
            Self {
                tractor,
                trailer,
                train,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousComputedLaneRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousComputedLaneRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ComputedLaneRegional(pub SequenceOf<AnonymousComputedLaneRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ComputedLane {
        #[rasn(identifier = "referenceLaneId")]
        pub reference_lane_id: LaneID,
        #[rasn(identifier = "offsetXaxis")]
        pub offset_xaxis: OffsetXaxis,
        #[rasn(identifier = "offsetYaxis")]
        pub offset_yaxis: OffsetYaxis,
        #[rasn(identifier = "rotateXY")]
        pub rotate_xy: Option<Angle>,
        #[rasn(identifier = "scaleXaxis")]
        pub scale_xaxis: Option<ScaleB12>,
        #[rasn(identifier = "scaleYaxis")]
        pub scale_yaxis: Option<ScaleB12>,
        pub regional: Option<ComputedLaneRegional>,
    }
    impl ComputedLane {
        pub fn new(
            reference_lane_id: LaneID,
            offset_xaxis: OffsetXaxis,
            offset_yaxis: OffsetYaxis,
            rotate_xy: Option<Angle>,
            scale_xaxis: Option<ScaleB12>,
            scale_yaxis: Option<ScaleB12>,
            regional: Option<ComputedLaneRegional>,
        ) -> Self {
            Self {
                reference_lane_id,
                offset_xaxis,
                offset_yaxis,
                rotate_xy,
                scale_xaxis,
                scale_yaxis,
                regional,
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
        pub lane_width: IVILaneWidth,
        #[rasn(value("-32768..=32767"), identifier = "offsetDistance")]
        pub offset_distance: Option<i16>,
        #[rasn(identifier = "offsetPosition")]
        pub offset_position: Option<DeltaReferencePosition>,
    }
    impl ComputedSegment {
        pub fn new(
            zone_id: Zid,
            lane_number: LanePosition,
            lane_width: IVILaneWidth,
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
    #[rasn(automatic_tags)]
    pub struct ConnectingLane {
        pub lane: LaneID,
        pub maneuver: Option<AllowedManeuvers>,
    }
    impl ConnectingLane {
        pub fn new(lane: LaneID, maneuver: Option<AllowedManeuvers>) -> Self {
            Self { lane, maneuver }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Connection {
        #[rasn(identifier = "connectingLane")]
        pub connecting_lane: ConnectingLane,
        #[rasn(identifier = "remoteIntersection")]
        pub remote_intersection: Option<IntersectionReferenceID>,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: Option<SignalGroupID>,
        #[rasn(identifier = "userClass")]
        pub user_class: Option<RestrictionClassID>,
        #[rasn(identifier = "connectionID")]
        pub connection_id: Option<LaneConnectionID>,
    }
    impl Connection {
        pub fn new(
            connecting_lane: ConnectingLane,
            remote_intersection: Option<IntersectionReferenceID>,
            signal_group: Option<SignalGroupID>,
            user_class: Option<RestrictionClassID>,
            connection_id: Option<LaneConnectionID>,
        ) -> Self {
            Self {
                connecting_lane,
                remote_intersection,
                signal_group,
                user_class,
                connection_id,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousConnectionManeuverAssistRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousConnectionManeuverAssistRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ConnectionManeuverAssistRegional(
        pub SequenceOf<AnonymousConnectionManeuverAssistRegional>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ConnectionManeuverAssist {
        #[rasn(identifier = "connectionID")]
        pub connection_id: LaneConnectionID,
        #[rasn(identifier = "queueLength")]
        pub queue_length: Option<ZoneLength>,
        #[rasn(identifier = "availableStorageLength")]
        pub available_storage_length: Option<ZoneLength>,
        #[rasn(identifier = "waitOnStop")]
        pub wait_on_stop: Option<WaitOnStopline>,
        #[rasn(identifier = "pedBicycleDetect")]
        pub ped_bicycle_detect: Option<PedestrianBicycleDetect>,
        pub regional: Option<ConnectionManeuverAssistRegional>,
    }
    impl ConnectionManeuverAssist {
        pub fn new(
            connection_id: LaneConnectionID,
            queue_length: Option<ZoneLength>,
            available_storage_length: Option<ZoneLength>,
            wait_on_stop: Option<WaitOnStopline>,
            ped_bicycle_detect: Option<PedestrianBicycleDetect>,
            regional: Option<ConnectionManeuverAssistRegional>,
        ) -> Self {
            Self {
                connection_id,
                queue_length,
                available_storage_length,
                wait_on_stop,
                ped_bicycle_detect,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ConnectionManeuverAssist-addGrpC")]
    #[non_exhaustive]
    pub struct ConnectionManeuverAssistAddGrpC {
        #[rasn(identifier = "itsStationPosition")]
        pub its_station_position: Option<ItsStationPositionList>,
    }
    impl ConnectionManeuverAssistAddGrpC {
        pub fn new(its_station_position: Option<ItsStationPositionList>) -> Self {
            Self {
                its_station_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ConnectionTrajectory-addGrpC")]
    #[non_exhaustive]
    pub struct ConnectionTrajectoryAddGrpC {
        pub nodes: NodeSetXY,
        #[rasn(identifier = "connectionID")]
        pub connection_id: LaneConnectionID,
    }
    impl ConnectionTrajectoryAddGrpC {
        pub fn new(nodes: NodeSetXY, connection_id: LaneConnectionID) -> Self {
            Self {
                nodes,
                connection_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ConnectsToList(pub SequenceOf<Connection>);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct CountryCode(pub FixedBitString<10usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CpmManagementContainer {
        #[rasn(identifier = "stationType")]
        pub station_type: StationType,
        #[rasn(identifier = "perceivedObjectContainerSegmentInfo")]
        pub perceived_object_container_segment_info: Option<PerceivedObjectContainerSegmentInfo>,
        #[rasn(identifier = "referencePosition")]
        pub reference_position: ReferencePosition,
    }
    impl CpmManagementContainer {
        pub fn new(
            station_type: StationType,
            perceived_object_container_segment_info: Option<PerceivedObjectContainerSegmentInfo>,
            reference_position: ReferencePosition,
        ) -> Self {
            Self {
                station_type,
                perceived_object_container_segment_info,
                reference_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CpmParameters {
        #[rasn(identifier = "managementContainer")]
        pub management_container: CpmManagementContainer,
        #[rasn(identifier = "stationDataContainer")]
        pub station_data_container: Option<StationDataContainer>,
        #[rasn(identifier = "sensorInformationContainer")]
        pub sensor_information_container: Option<SensorInformationContainer>,
        #[rasn(identifier = "perceivedObjectContainer")]
        pub perceived_object_container: Option<PerceivedObjectContainer>,
        #[rasn(identifier = "freeSpaceAddendumContainer")]
        pub free_space_addendum_container: Option<FreeSpaceAddendumContainer>,
        #[rasn(identifier = "numberOfPerceivedObjects")]
        pub number_of_perceived_objects: NumberOfPerceivedObjects,
    }
    impl CpmParameters {
        pub fn new(
            management_container: CpmManagementContainer,
            station_data_container: Option<StationDataContainer>,
            sensor_information_container: Option<SensorInformationContainer>,
            perceived_object_container: Option<PerceivedObjectContainer>,
            free_space_addendum_container: Option<FreeSpaceAddendumContainer>,
            number_of_perceived_objects: NumberOfPerceivedObjects,
        ) -> Self {
            Self {
                management_container,
                station_data_container,
                sensor_information_container,
                perceived_object_container,
                free_space_addendum_container,
                number_of_perceived_objects,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct DBV(pub Distance);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DDD {
        #[rasn(value("1..=128"))]
        pub dcj: Option<u8>,
        #[rasn(value("1..=128"))]
        pub dcr: Option<u8>,
        #[rasn(value("1..=128"))]
        pub tpl: Option<u8>,
        #[rasn(size("1..=8", extensible), identifier = "ioList")]
        pub io_list: SequenceOf<DDDIO>,
    }
    impl DDD {
        pub fn new(
            dcj: Option<u8>,
            dcr: Option<u8>,
            tpl: Option<u8>,
            io_list: SequenceOf<DDDIO>,
        ) -> Self {
            Self {
                dcj,
                dcr,
                tpl,
                io_list,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "DDD-DEP", value("0..=15", extensible))]
    pub struct DDDDEP(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "DDD-DER", value("0..=15", extensible))]
    pub struct DDDDER(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "DDD-IO")]
    pub struct DDDIO {
        #[rasn(value("0..=7"))]
        pub drn: u8,
        #[rasn(size("1..=4", extensible))]
        pub dp: Option<SequenceOf<DestinationPlace>>,
        #[rasn(size("1..=4", extensible))]
        pub dr: Option<SequenceOf<DestinationRoad>>,
        #[rasn(value("1..=999"))]
        pub rne: Option<u16>,
        #[rasn(value("1..=999"), identifier = "stnId")]
        pub stn_id: Option<u16>,
        #[rasn(identifier = "stnText")]
        pub stn_text: Option<Utf8String>,
        pub dcp: Option<DistanceOrDuration>,
        pub ddp: Option<DistanceOrDuration>,
    }
    impl DDDIO {
        pub fn new(
            drn: u8,
            dp: Option<SequenceOf<DestinationPlace>>,
            dr: Option<SequenceOf<DestinationRoad>>,
            rne: Option<u16>,
            stn_id: Option<u16>,
            stn_text: Option<Utf8String>,
            dcp: Option<DistanceOrDuration>,
            ddp: Option<DistanceOrDuration>,
        ) -> Self {
            Self {
                drn,
                dp,
                dr,
                rne,
                stn_id,
                stn_text,
                dcp,
                ddp,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=8"))]
    pub struct DFL(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct DSecond(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DTM {
        pub year: Option<DtmYear>,
        #[rasn(identifier = "month-day")]
        pub month_day: Option<DtmMonthDay>,
        pub pmd: Option<PMD>,
        #[rasn(identifier = "hourMinutes")]
        pub hour_minutes: Option<DtmHourMinutes>,
        #[rasn(identifier = "dayOfWeek")]
        pub day_of_week: Option<DayOfWeek>,
        pub period: Option<HoursMinutes>,
    }
    impl DTM {
        pub fn new(
            year: Option<DtmYear>,
            month_day: Option<DtmMonthDay>,
            pmd: Option<PMD>,
            hour_minutes: Option<DtmHourMinutes>,
            day_of_week: Option<DayOfWeek>,
            period: Option<HoursMinutes>,
        ) -> Self {
            Self {
                year,
                month_day,
                pmd,
                hour_minutes,
                day_of_week,
                period,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct DYear(pub u16);
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
    pub struct DataParameters {
        #[rasn(size("1..=255"), identifier = "processMethod")]
        pub process_method: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "processAgency")]
        pub process_agency: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "lastCheckedDate")]
        pub last_checked_date: Option<Ia5String>,
        #[rasn(size("1..=255"), identifier = "geoidUsed")]
        pub geoid_used: Option<Ia5String>,
    }
    impl DataParameters {
        pub fn new(
            process_method: Option<Ia5String>,
            process_agency: Option<Ia5String>,
            last_checked_date: Option<Ia5String>,
            geoid_used: Option<Ia5String>,
        ) -> Self {
            Self {
                process_method,
                process_agency,
                last_checked_date,
                geoid_used,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct DayOfWeek(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-12700..=12800"))]
    pub struct DeltaAltitude(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-150..=150"))]
    pub struct DeltaAngle(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLatitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLongitude(pub i32);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositionsWithAltitude(pub SequenceOf<DeltaReferencePosition>);
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
    #[rasn(delegate, value("-122..=121"))]
    pub struct DeltaTime(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=63"))]
    pub struct DescriptiveName(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DestinationPlace {
        #[rasn(identifier = "depType")]
        pub dep_type: DDDDEP,
        #[rasn(identifier = "depRSCode")]
        pub dep_rscode: Option<ISO14823Code>,
        #[rasn(identifier = "depBlob")]
        pub dep_blob: Option<OctetString>,
        #[rasn(value("1..=999"), identifier = "plnId")]
        pub pln_id: Option<u16>,
        #[rasn(identifier = "plnText")]
        pub pln_text: Option<Utf8String>,
    }
    impl DestinationPlace {
        pub fn new(
            dep_type: DDDDEP,
            dep_rscode: Option<ISO14823Code>,
            dep_blob: Option<OctetString>,
            pln_id: Option<u16>,
            pln_text: Option<Utf8String>,
        ) -> Self {
            Self {
                dep_type,
                dep_rscode,
                dep_blob,
                pln_id,
                pln_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DestinationRoad {
        #[rasn(identifier = "derType")]
        pub der_type: DDDDER,
        #[rasn(value("1..=999"), identifier = "ronId")]
        pub ron_id: Option<u16>,
        #[rasn(identifier = "ronText")]
        pub ron_text: Option<Utf8String>,
    }
    impl DestinationRoad {
        pub fn new(der_type: DDDDER, ron_id: Option<u16>, ron_text: Option<Utf8String>) -> Self {
            Self {
                der_type,
                ron_id,
                ron_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum DetectionArea {
        vehicleSensor(VehicleSensor),
        stationarySensorRadial(AreaRadial),
        stationarySensorPolygon(AreaPolygon),
        stationarySensorCircular(AreaCircular),
        stationarySensorEllipse(AreaEllipse),
        stationarySensorRectangle(AreaRectangle),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DieselEmissionValues {
        pub particulate: Particulate,
        #[rasn(identifier = "absorptionCoeff")]
        pub absorption_coeff: Int2,
    }
    impl DieselEmissionValues {
        pub fn new(particulate: Particulate, absorption_coeff: Int2) -> Self {
            Self {
                particulate,
                absorption_coeff,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct Direction(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Distance {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        pub unit: RSCUnit,
    }
    impl Distance {
        pub fn new(value: u16, unit: RSCUnit) -> Self {
            Self { value, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct DistanceConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DistanceOrDuration {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        pub unit: RSCUnit,
    }
    impl DistanceOrDuration {
        pub fn new(value: u16, unit: RSCUnit) -> Self {
            Self { value, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-132768..=132767"))]
    pub struct DistanceValue(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum DriveDirection {
        forward = 0,
        backward = 1,
        unavailable = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-32767..=32767"))]
    pub struct DrivenLineOffsetLg(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-2047..=2047"))]
    pub struct DrivenLineOffsetSm(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct DriverCharacteristics(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DtmHourMinutes {
        pub shm: HoursMinutes,
        pub ehm: HoursMinutes,
    }
    impl DtmHourMinutes {
        pub fn new(shm: HoursMinutes, ehm: HoursMinutes) -> Self {
            Self { shm, ehm }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DtmMonthDay {
        pub smd: MonthDay,
        pub emd: MonthDay,
    }
    impl DtmMonthDay {
        pub fn new(smd: MonthDay, emd: MonthDay) -> Self {
            Self { smd, emd }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct DtmYear {
        #[rasn(value("2000..=2127", extensible))]
        pub syr: Integer,
        #[rasn(value("2000..=2127", extensible))]
        pub eyr: Integer,
    }
    impl DtmYear {
        pub fn new(syr: Integer, eyr: Integer) -> Self {
            Self { syr, eyr }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=2"))]
    pub struct DynamicStatus(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct EDT(pub DTM);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-4096..=61439"))]
    pub struct Elevation(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum ElevationConfidence {
        unavailable = 0,
        #[rasn(identifier = "elev-500-00")]
        elev_500_00 = 1,
        #[rasn(identifier = "elev-200-00")]
        elev_200_00 = 2,
        #[rasn(identifier = "elev-100-00")]
        elev_100_00 = 3,
        #[rasn(identifier = "elev-050-00")]
        elev_050_00 = 4,
        #[rasn(identifier = "elev-020-00")]
        elev_020_00 = 5,
        #[rasn(identifier = "elev-010-00")]
        elev_010_00 = 6,
        #[rasn(identifier = "elev-005-00")]
        elev_005_00 = 7,
        #[rasn(identifier = "elev-002-00")]
        elev_002_00 = 8,
        #[rasn(identifier = "elev-001-00")]
        elev_001_00 = 9,
        #[rasn(identifier = "elev-000-50")]
        elev_000_50 = 10,
        #[rasn(identifier = "elev-000-20")]
        elev_000_20 = 11,
        #[rasn(identifier = "elev-000-10")]
        elev_000_10 = 12,
        #[rasn(identifier = "elev-000-05")]
        elev_000_05 = 13,
        #[rasn(identifier = "elev-000-02")]
        elev_000_02 = 14,
        #[rasn(identifier = "elev-000-01")]
        elev_000_01 = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EmissionType {
        euro1 = 0,
        euro2 = 1,
        euro3 = 2,
        euro4 = 3,
        euro5 = 4,
        euro6 = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct EnabledLaneList(pub SequenceOf<LaneID>);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ExceptionalCondition {
        unknown = 0,
        publicTransportPriority = 1,
        emergencyVehiclePriority = 2,
        trainPriority = 3,
        bridgeOpen = 4,
        vehicleHeight = 5,
        weather = 6,
        trafficJam = 7,
        tunnelClosure = 8,
        meteringActive = 9,
        truckPriority = 10,
        bicyclePlatoonPriority = 11,
        vehiclePlatoonPriority = 12,
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
    #[rasn(choice, automatic_tags)]
    pub enum Ext1 {
        #[rasn(value("128..=16511"))]
        content(u16),
        extension(Ext2),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum Ext2 {
        #[rasn(value("16512..=2113663"))]
        content(u32),
        extension(Ext3),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("2113664..=270549119", extensible))]
    pub struct Ext3(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ExteriorLights(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct FreeSpaceAddendum {
        #[rasn(identifier = "freeSpaceConfidence")]
        pub free_space_confidence: FreeSpaceConfidence,
        #[rasn(identifier = "freeSpaceArea")]
        pub free_space_area: FreeSpaceArea,
        #[rasn(identifier = "sensorIDList")]
        pub sensor_idlist: Option<SensorIdList>,
        #[rasn(
            default = "free_space_addendum_shadowing_applies_default",
            identifier = "shadowingApplies"
        )]
        pub shadowing_applies: ShadowingApplies,
    }
    impl FreeSpaceAddendum {
        pub fn new(
            free_space_confidence: FreeSpaceConfidence,
            free_space_area: FreeSpaceArea,
            sensor_idlist: Option<SensorIdList>,
            shadowing_applies: ShadowingApplies,
        ) -> Self {
            Self {
                free_space_confidence,
                free_space_area,
                sensor_idlist,
                shadowing_applies,
            }
        }
    }
    fn free_space_addendum_shadowing_applies_default() -> ShadowingApplies {
        ShadowingApplies(true)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct FreeSpaceAddendumContainer(pub SequenceOf<FreeSpaceAddendum>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum FreeSpaceArea {
        freeSpacePolygon(AreaPolygon),
        freeSpaceCircular(AreaCircular),
        freeSpaceEllipse(AreaEllipse),
        freeSpaceRectangle(AreaRectangle),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct FreeSpaceConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=50"))]
    pub struct FrontOverhang(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct FuelType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct GNSSstatus(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct GeneralIviContainer(pub SequenceOf<GicPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct GenerationDeltaTime(pub u16);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousGenericLaneRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousGenericLaneRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct GenericLaneRegional(pub SequenceOf<AnonymousGenericLaneRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GenericLane {
        #[rasn(identifier = "laneID")]
        pub lane_id: LaneID,
        pub name: Option<DescriptiveName>,
        #[rasn(identifier = "ingressApproach")]
        pub ingress_approach: Option<ApproachID>,
        #[rasn(identifier = "egressApproach")]
        pub egress_approach: Option<ApproachID>,
        #[rasn(identifier = "laneAttributes")]
        pub lane_attributes: LaneAttributes,
        pub maneuvers: Option<AllowedManeuvers>,
        #[rasn(identifier = "nodeList")]
        pub node_list: NodeListXY,
        #[rasn(identifier = "connectsTo")]
        pub connects_to: Option<ConnectsToList>,
        pub overlays: Option<OverlayLaneList>,
        pub regional: Option<GenericLaneRegional>,
    }
    impl GenericLane {
        pub fn new(
            lane_id: LaneID,
            name: Option<DescriptiveName>,
            ingress_approach: Option<ApproachID>,
            egress_approach: Option<ApproachID>,
            lane_attributes: LaneAttributes,
            maneuvers: Option<AllowedManeuvers>,
            node_list: NodeListXY,
            connects_to: Option<ConnectsToList>,
            overlays: Option<OverlayLaneList>,
            regional: Option<GenericLaneRegional>,
        ) -> Self {
            Self {
                lane_id,
                name,
                ingress_approach,
                egress_approach,
                lane_attributes,
                maneuvers,
                node_list,
                connects_to,
                overlays,
                regional,
            }
        }
    }
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
        #[rasn(size("1..=16", extensible))]
        pub parts: SequenceOf<GlcPart>,
    }
    impl GeographicLocationContainer {
        pub fn new(
            reference_position: ReferencePosition,
            reference_position_time: Option<TimestampIts>,
            reference_position_heading: Option<Heading>,
            reference_position_speed: Option<Speed>,
            parts: SequenceOf<GlcPart>,
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
        #[rasn(size("1..=8", extensible), identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<SequenceOf<Zid>>,
        #[rasn(identifier = "its-Rrid")]
        pub its_rrid: Option<VarLengthNumber>,
        #[rasn(size("1..=8", extensible), identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: Option<SequenceOf<Zid>>,
        pub direction: Option<Direction>,
        #[rasn(size("1..=8", extensible), identifier = "driverAwarenessZoneIds")]
        pub driver_awareness_zone_ids: Option<SequenceOf<Zid>>,
        #[rasn(value("0..=255"), identifier = "minimumAwarenessTime")]
        pub minimum_awareness_time: Option<u8>,
        #[rasn(size("1..=8", extensible), identifier = "applicableLanes")]
        pub applicable_lanes: Option<SequenceOf<LanePosition>>,
        #[rasn(identifier = "iviType")]
        pub ivi_type: IviType,
        #[rasn(identifier = "iviPurpose")]
        pub ivi_purpose: Option<IviPurpose>,
        #[rasn(identifier = "laneStatus")]
        pub lane_status: Option<LaneStatus>,
        #[rasn(size("1..=8", extensible), identifier = "vehicleCharacteristics")]
        pub vehicle_characteristics: Option<SequenceOf<CompleteVehicleCharacteristics>>,
        #[rasn(identifier = "driverCharacteristics")]
        pub driver_characteristics: Option<DriverCharacteristics>,
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Option<Integer>,
        #[rasn(value("1..=64", extensible), identifier = "preStoredlayoutId")]
        pub pre_storedlayout_id: Option<Integer>,
        #[rasn(size("1..=4", extensible), identifier = "roadSignCodes")]
        pub road_sign_codes: SequenceOf<RSCode>,
        #[rasn(size("1..=4", extensible), identifier = "extraText")]
        pub extra_text: Option<SequenceOf<Text>>,
    }
    impl GicPart {
        pub fn new(
            detection_zone_ids: Option<SequenceOf<Zid>>,
            its_rrid: Option<VarLengthNumber>,
            relevance_zone_ids: Option<SequenceOf<Zid>>,
            direction: Option<Direction>,
            driver_awareness_zone_ids: Option<SequenceOf<Zid>>,
            minimum_awareness_time: Option<u8>,
            applicable_lanes: Option<SequenceOf<LanePosition>>,
            ivi_type: IviType,
            ivi_purpose: Option<IviPurpose>,
            lane_status: Option<LaneStatus>,
            vehicle_characteristics: Option<SequenceOf<CompleteVehicleCharacteristics>>,
            driver_characteristics: Option<DriverCharacteristics>,
            layout_id: Option<Integer>,
            pre_storedlayout_id: Option<Integer>,
            road_sign_codes: SequenceOf<RSCode>,
            extra_text: Option<SequenceOf<Text>>,
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
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct GoodsType(pub Integer);
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
    #[rasn(delegate, value("0..=100"))]
    pub struct HitchPointOffset(pub u8);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ISO14823Attribute {
        dtm(DTM),
        edt(EDT),
        dfl(DFL),
        ved(VED),
        spe(SPE),
        roi(ROI),
        dbv(DBV),
        ddd(DDD),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ISO14823Code {
        #[rasn(identifier = "pictogramCode")]
        pub pictogram_code: PictogramCode,
        pub attributes: Option<ISO14823Attributes>,
    }
    impl ISO14823Code {
        pub fn new(pictogram_code: PictogramCode, attributes: Option<ISO14823Attributes>) -> Self {
            Self {
                pictogram_code,
                attributes,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct IVILaneWidth(pub u16);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IVIManagementContainer {
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
        #[rasn(size("1..=8"), identifier = "connectedIviStructures")]
        pub connected_ivi_structures: Option<SequenceOf<IviIdentificationNumber>>,
        #[rasn(identifier = "iviStatus")]
        pub ivi_status: IviStatus,
    }
    impl IVIManagementContainer {
        pub fn new(
            service_provider_id: Provider,
            ivi_identification_number: IviIdentificationNumber,
            time_stamp: Option<TimestampIts>,
            valid_from: Option<TimestampIts>,
            valid_to: Option<TimestampIts>,
            connected_ivi_structures: Option<SequenceOf<IviIdentificationNumber>>,
            ivi_status: IviStatus,
        ) -> Self {
            Self {
                service_provider_id,
                ivi_identification_number,
                time_stamp,
                valid_from,
                valid_to,
                connected_ivi_structures,
                ivi_status,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Identifier(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct InformationQuality(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Int1(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct Int2(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IntersectionAccessPoint {
        lane(LaneID),
        approach(ApproachID),
        connection(LaneConnectionID),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIntersectionGeometryRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousIntersectionGeometryRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionGeometryRegional(pub SequenceOf<AnonymousIntersectionGeometryRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IntersectionGeometry {
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
        pub revision: MsgCount,
        #[rasn(identifier = "refPoint")]
        pub ref_point: Position3D,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<LaneWidth>,
        #[rasn(identifier = "speedLimits")]
        pub speed_limits: Option<SpeedLimitList>,
        #[rasn(identifier = "laneSet")]
        pub lane_set: LaneList,
        #[rasn(identifier = "preemptPriorityData")]
        pub preempt_priority_data: Option<PreemptPriorityList>,
        pub regional: Option<IntersectionGeometryRegional>,
    }
    impl IntersectionGeometry {
        pub fn new(
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
            revision: MsgCount,
            ref_point: Position3D,
            lane_width: Option<LaneWidth>,
            speed_limits: Option<SpeedLimitList>,
            lane_set: LaneList,
            preempt_priority_data: Option<PreemptPriorityList>,
            regional: Option<IntersectionGeometryRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                ref_point,
                lane_width,
                speed_limits,
                lane_set,
                preempt_priority_data,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct IntersectionID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousIntersectionStateRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousIntersectionStateRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionStateRegional(pub SequenceOf<AnonymousIntersectionStateRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct IntersectionState {
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
        pub revision: MsgCount,
        pub status: IntersectionStatusObject,
        pub moy: Option<MinuteOfTheYear>,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<DSecond>,
        #[rasn(identifier = "enabledLanes")]
        pub enabled_lanes: Option<EnabledLaneList>,
        pub states: MovementList,
        #[rasn(identifier = "maneuverAssistList")]
        pub maneuver_assist_list: Option<ManeuverAssistList>,
        pub regional: Option<IntersectionStateRegional>,
    }
    impl IntersectionState {
        pub fn new(
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
            revision: MsgCount,
            status: IntersectionStatusObject,
            moy: Option<MinuteOfTheYear>,
            time_stamp: Option<DSecond>,
            enabled_lanes: Option<EnabledLaneList>,
            states: MovementList,
            maneuver_assist_list: Option<ManeuverAssistList>,
            regional: Option<IntersectionStateRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                status,
                moy,
                time_stamp,
                enabled_lanes,
                states,
                maneuver_assist_list,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "IntersectionState-addGrpC")]
    #[non_exhaustive]
    pub struct IntersectionStateAddGrpC {
        #[rasn(identifier = "activePrioritizations")]
        pub active_prioritizations: Option<PrioritizationResponseList>,
    }
    impl IntersectionStateAddGrpC {
        pub fn new(active_prioritizations: Option<PrioritizationResponseList>) -> Self {
            Self {
                active_prioritizations,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionStateList(pub SequenceOf<IntersectionState>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct IntersectionStatusObject(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Iso3833VehicleType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct IssuerIdentifier(pub u16);
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
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ItsStationPosition {
        #[rasn(identifier = "stationID")]
        pub station_id: StationID,
        #[rasn(identifier = "laneID")]
        pub lane_id: Option<LaneID>,
        #[rasn(identifier = "nodeXY")]
        pub node_xy: Option<NodeOffsetPointXY>,
        #[rasn(identifier = "timeReference")]
        pub time_reference: Option<TimeReference>,
    }
    impl ItsStationPosition {
        pub fn new(
            station_id: StationID,
            lane_id: Option<LaneID>,
            node_xy: Option<NodeOffsetPointXY>,
            time_reference: Option<TimeReference>,
        ) -> Self {
            Self {
                station_id,
                lane_id,
                node_xy,
                time_reference,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct ItsStationPositionList(pub SequenceOf<ItsStationPosition>);
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
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32767", extensible))]
    pub struct IviIdentificationNumber(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct IviPurpose(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviStatus(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct IviStructure {
        pub mandatory: IVIManagementContainer,
        #[rasn(size("1..=8", extensible))]
        pub optional: Option<SequenceOf<IviContainer>>,
    }
    impl IviStructure {
        pub fn new(
            mandatory: IVIManagementContainer,
            optional: Option<SequenceOf<IviContainer>>,
        ) -> Self {
            Self {
                mandatory,
                optional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviType(pub u8);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneAttributesRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl LaneAttributesRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LaneAttributes {
        #[rasn(identifier = "directionalUse")]
        pub directional_use: LaneDirection,
        #[rasn(identifier = "sharedWith")]
        pub shared_with: LaneSharing,
        #[rasn(identifier = "laneType")]
        pub lane_type: LaneTypeAttributes,
        pub regional: Option<LaneAttributesRegional>,
    }
    impl LaneAttributes {
        pub fn new(
            directional_use: LaneDirection,
            shared_with: LaneSharing,
            lane_type: LaneTypeAttributes,
            regional: Option<LaneAttributesRegional>,
        ) -> Self {
            Self {
                directional_use,
                shared_with,
                lane_type,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Barrier")]
    pub struct LaneAttributesBarrier(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Bike")]
    pub struct LaneAttributesBike(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Crosswalk")]
    pub struct LaneAttributesCrosswalk(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Parking")]
    pub struct LaneAttributesParking(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Sidewalk")]
    pub struct LaneAttributesSidewalk(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Striping")]
    pub struct LaneAttributesStriping(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-TrackedVehicle")]
    pub struct LaneAttributesTrackedVehicle(pub FixedBitString<16usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "LaneAttributes-Vehicle", size("8", extensible))]
    pub struct LaneAttributesVehicle(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "LaneAttributes-addGrpC")]
    #[non_exhaustive]
    pub struct LaneAttributesAddGrpC {
        #[rasn(identifier = "maxVehicleHeight")]
        pub max_vehicle_height: Option<VehicleHeight>,
        #[rasn(identifier = "maxVehicleWeight")]
        pub max_vehicle_weight: Option<VehicleMass>,
    }
    impl LaneAttributesAddGrpC {
        pub fn new(
            max_vehicle_height: Option<VehicleHeight>,
            max_vehicle_weight: Option<VehicleMass>,
        ) -> Self {
            Self {
                max_vehicle_height,
                max_vehicle_weight,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneConnectionID(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousLaneDataAttributeRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousLaneDataAttributeRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct LaneDataAttributeRegional(pub SequenceOf<AnonymousLaneDataAttributeRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LaneDataAttribute {
        pathEndPointAngle(DeltaAngle),
        laneCrownPointCenter(RoadwayCrownAngle),
        laneCrownPointLeft(RoadwayCrownAngle),
        laneCrownPointRight(RoadwayCrownAngle),
        laneAngle(MergeDivergeNodeAngle),
        speedLimits(SpeedLimitList),
        regional(LaneDataAttributeRegional),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct LaneDataAttributeList(pub SequenceOf<LaneDataAttribute>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LaneDirection(pub FixedBitString<2usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LaneInformation {
        #[rasn(identifier = "laneNumber")]
        pub lane_number: LanePosition,
        pub direction: Direction,
        pub validity: Option<DTM>,
        #[rasn(identifier = "laneType")]
        pub lane_type: LaneType,
        #[rasn(identifier = "laneTypeQualifier")]
        pub lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
        #[rasn(identifier = "laneStatus")]
        pub lane_status: LaneStatus,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<IVILaneWidth>,
    }
    impl LaneInformation {
        pub fn new(
            lane_number: LanePosition,
            direction: Direction,
            validity: Option<DTM>,
            lane_type: LaneType,
            lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
            lane_status: LaneStatus,
            lane_width: Option<IVILaneWidth>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct LaneList(pub SequenceOf<GenericLane>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1..=14"))]
    pub struct LanePosition(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct LaneSharing(pub FixedBitString<10usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct LaneStatus(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct LaneType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LaneTypeAttributes {
        vehicle(LaneAttributesVehicle),
        crosswalk(LaneAttributesCrosswalk),
        bikeLane(LaneAttributesBike),
        sidewalk(LaneAttributesSidewalk),
        median(LaneAttributesBarrier),
        striping(LaneAttributesStriping),
        trackedVehicle(LaneAttributesTrackedVehicle),
        parking(LaneAttributesParking),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LaneWidth(pub u16);
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
    #[rasn(delegate, value("0..=100"))]
    pub struct LayerID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum LayerType {
        none = 0,
        mixedContent = 1,
        generalMapData = 2,
        intersectionData = 3,
        curveData = 4,
        roadwaySectionData = 5,
        parkingAreaData = 6,
        sharedLaneData = 7,
    }
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
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LayoutContainer {
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Integer,
        #[rasn(value("10..=73"))]
        pub height: Option<u8>,
        #[rasn(value("10..=265"))]
        pub width: Option<u16>,
        #[rasn(size("1..=4", extensible), identifier = "layoutComponents")]
        pub layout_components: SequenceOf<LayoutComponent>,
    }
    impl LayoutContainer {
        pub fn new(
            layout_id: Integer,
            height: Option<u8>,
            width: Option<u16>,
            layout_components: SequenceOf<LayoutComponent>,
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
    #[rasn(automatic_tags)]
    pub struct LongitudinalLanePosition {
        #[rasn(identifier = "longitudinalLanePositionValue")]
        pub longitudinal_lane_position_value: LongitudinalLanePositionValue,
        #[rasn(identifier = "longitudinalLanePositionConfidence")]
        pub longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
    }
    impl LongitudinalLanePosition {
        pub fn new(
            longitudinal_lane_position_value: LongitudinalLanePositionValue,
            longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
        ) -> Self {
            Self {
                longitudinal_lane_position_value,
                longitudinal_lane_position_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct LongitudinalLanePositionConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LongitudinalLanePositionValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MAPEM {
        pub header: ItsPduHeader,
        pub map: MapData,
    }
    impl MAPEM {
        pub fn new(header: ItsPduHeader, map: MapData) -> Self {
            Self { header, map }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ManeuverAssistList(pub SequenceOf<ConnectionManeuverAssist>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMapDataRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMapDataRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MapDataRegional(pub SequenceOf<AnonymousMapDataRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MapData {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        #[rasn(identifier = "msgIssueRevision")]
        pub msg_issue_revision: MsgCount,
        #[rasn(identifier = "layerType")]
        pub layer_type: Option<LayerType>,
        #[rasn(identifier = "layerID")]
        pub layer_id: Option<LayerID>,
        pub intersections: Option<IntersectionGeometryList>,
        #[rasn(identifier = "roadSegments")]
        pub road_segments: Option<RoadSegmentList>,
        #[rasn(identifier = "dataParameters")]
        pub data_parameters: Option<DataParameters>,
        #[rasn(identifier = "restrictionList")]
        pub restriction_list: Option<RestrictionClassList>,
        pub regional: Option<MapDataRegional>,
    }
    impl MapData {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            msg_issue_revision: MsgCount,
            layer_type: Option<LayerType>,
            layer_id: Option<LayerID>,
            intersections: Option<IntersectionGeometryList>,
            road_segments: Option<RoadSegmentList>,
            data_parameters: Option<DataParameters>,
            restriction_list: Option<RestrictionClassList>,
            regional: Option<MapDataRegional>,
        ) -> Self {
            Self {
                time_stamp,
                msg_issue_revision,
                layer_type,
                layer_id,
                intersections,
                road_segments,
                data_parameters,
                restriction_list,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MapData-addGrpC")]
    #[non_exhaustive]
    pub struct MapDataAddGrpC {
        #[rasn(identifier = "signalHeadLocations")]
        pub signal_head_locations: Option<SignalHeadLocationList>,
    }
    impl MapDataAddGrpC {
        pub fn new(signal_head_locations: Option<SignalHeadLocationList>) -> Self {
            Self {
                signal_head_locations,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MatchedPosition {
        #[rasn(identifier = "laneID")]
        pub lane_id: Option<LaneID>,
        #[rasn(identifier = "longitudinalLanePosition")]
        pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
    }
    impl MatchedPosition {
        pub fn new(
            lane_id: Option<LaneID>,
            longitudinal_lane_position: Option<LongitudinalLanePosition>,
        ) -> Self {
            Self {
                lane_id,
                longitudinal_lane_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-180..=180"))]
    pub struct MergeDivergeNodeAngle(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=527040"))]
    pub struct MinuteOfTheYear(pub u32);
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
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMovementEventRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMovementEventRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementEventRegional(pub SequenceOf<AnonymousMovementEventRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MovementEvent {
        #[rasn(identifier = "eventState")]
        pub event_state: MovementPhaseState,
        pub timing: Option<TimeChangeDetails>,
        pub speeds: Option<AdvisorySpeedList>,
        pub regional: Option<MovementEventRegional>,
    }
    impl MovementEvent {
        pub fn new(
            event_state: MovementPhaseState,
            timing: Option<TimeChangeDetails>,
            speeds: Option<AdvisorySpeedList>,
            regional: Option<MovementEventRegional>,
        ) -> Self {
            Self {
                event_state,
                timing,
                speeds,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "MovementEvent-addGrpC")]
    #[non_exhaustive]
    pub struct MovementEventAddGrpC {
        #[rasn(identifier = "stateChangeReason")]
        pub state_change_reason: Option<ExceptionalCondition>,
    }
    impl MovementEventAddGrpC {
        pub fn new(state_change_reason: Option<ExceptionalCondition>) -> Self {
            Self {
                state_change_reason,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct MovementEventList(pub SequenceOf<MovementEvent>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct MovementList(pub SequenceOf<MovementState>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum MovementPhaseState {
        unavailable = 0,
        dark = 1,
        #[rasn(identifier = "stop-Then-Proceed")]
        stop_Then_Proceed = 2,
        #[rasn(identifier = "stop-And-Remain")]
        stop_And_Remain = 3,
        #[rasn(identifier = "pre-Movement")]
        pre_Movement = 4,
        #[rasn(identifier = "permissive-Movement-Allowed")]
        permissive_Movement_Allowed = 5,
        #[rasn(identifier = "protected-Movement-Allowed")]
        protected_Movement_Allowed = 6,
        #[rasn(identifier = "permissive-clearance")]
        permissive_clearance = 7,
        #[rasn(identifier = "protected-clearance")]
        protected_clearance = 8,
        #[rasn(identifier = "caution-Conflicting-Traffic")]
        caution_Conflicting_Traffic = 9,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousMovementStateRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousMovementStateRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementStateRegional(pub SequenceOf<AnonymousMovementStateRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MovementState {
        #[rasn(identifier = "movementName")]
        pub movement_name: Option<DescriptiveName>,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: SignalGroupID,
        #[rasn(identifier = "state-time-speed")]
        pub state_time_speed: MovementEventList,
        #[rasn(identifier = "maneuverAssistList")]
        pub maneuver_assist_list: Option<ManeuverAssistList>,
        pub regional: Option<MovementStateRegional>,
    }
    impl MovementState {
        pub fn new(
            movement_name: Option<DescriptiveName>,
            signal_group: SignalGroupID,
            state_time_speed: MovementEventList,
            maneuver_assist_list: Option<ManeuverAssistList>,
            regional: Option<MovementStateRegional>,
        ) -> Self {
            Self {
                movement_name,
                signal_group,
                state_time_speed,
                maneuver_assist_list,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct MsgCount(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Node {
        pub id: Integer,
        pub lane: Option<LaneID>,
        #[rasn(identifier = "connectionID")]
        pub connection_id: Option<LaneConnectionID>,
        #[rasn(identifier = "intersectionID")]
        pub intersection_id: Option<IntersectionID>,
    }
    impl Node {
        pub fn new(
            id: Integer,
            lane: Option<LaneID>,
            connection_id: Option<LaneConnectionID>,
            intersection_id: Option<IntersectionID>,
        ) -> Self {
            Self {
                id,
                lane,
                connection_id,
                intersection_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-LLmD-64b")]
    pub struct NodeLLmD64b {
        pub lon: Longitude,
        pub lat: Latitude,
    }
    impl NodeLLmD64b {
        pub fn new(lon: Longitude, lat: Latitude) -> Self {
            Self { lon, lat }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-20b")]
    pub struct NodeXY20b {
        pub x: OffsetB10,
        pub y: OffsetB10,
    }
    impl NodeXY20b {
        pub fn new(x: OffsetB10, y: OffsetB10) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-22b")]
    pub struct NodeXY22b {
        pub x: OffsetB11,
        pub y: OffsetB11,
    }
    impl NodeXY22b {
        pub fn new(x: OffsetB11, y: OffsetB11) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-24b")]
    pub struct NodeXY24b {
        pub x: OffsetB12,
        pub y: OffsetB12,
    }
    impl NodeXY24b {
        pub fn new(x: OffsetB12, y: OffsetB12) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-26b")]
    pub struct NodeXY26b {
        pub x: OffsetB13,
        pub y: OffsetB13,
    }
    impl NodeXY26b {
        pub fn new(x: OffsetB13, y: OffsetB13) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-28b")]
    pub struct NodeXY28b {
        pub x: OffsetB14,
        pub y: OffsetB14,
    }
    impl NodeXY28b {
        pub fn new(x: OffsetB14, y: OffsetB14) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Node-XY-32b")]
    pub struct NodeXY32b {
        pub x: OffsetB16,
        pub y: OffsetB16,
    }
    impl NodeXY32b {
        pub fn new(x: OffsetB16, y: OffsetB16) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "NodeAttributeSet-addGrpC")]
    #[non_exhaustive]
    pub struct NodeAttributeSetAddGrpC {
        #[rasn(identifier = "ptvRequest")]
        pub ptv_request: Option<PtvRequestType>,
        #[rasn(identifier = "nodeLink")]
        pub node_link: Option<NodeLink>,
        pub node: Option<Node>,
    }
    impl NodeAttributeSetAddGrpC {
        pub fn new(
            ptv_request: Option<PtvRequestType>,
            node_link: Option<NodeLink>,
            node: Option<Node>,
        ) -> Self {
            Self {
                ptv_request,
                node_link,
                node,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousNodeAttributeSetXYRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousNodeAttributeSetXYRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct NodeAttributeSetXYRegional(pub SequenceOf<AnonymousNodeAttributeSetXYRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NodeAttributeSetXY {
        #[rasn(identifier = "localNode")]
        pub local_node: Option<NodeAttributeXYList>,
        pub disabled: Option<SegmentAttributeXYList>,
        pub enabled: Option<SegmentAttributeXYList>,
        pub data: Option<LaneDataAttributeList>,
        #[rasn(identifier = "dWidth")]
        pub d_width: Option<OffsetB10>,
        #[rasn(identifier = "dElevation")]
        pub d_elevation: Option<OffsetB10>,
        pub regional: Option<NodeAttributeSetXYRegional>,
    }
    impl NodeAttributeSetXY {
        pub fn new(
            local_node: Option<NodeAttributeXYList>,
            disabled: Option<SegmentAttributeXYList>,
            enabled: Option<SegmentAttributeXYList>,
            data: Option<LaneDataAttributeList>,
            d_width: Option<OffsetB10>,
            d_elevation: Option<OffsetB10>,
            regional: Option<NodeAttributeSetXYRegional>,
        ) -> Self {
            Self {
                local_node,
                disabled,
                enabled,
                data,
                d_width,
                d_elevation,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum NodeAttributeXY {
        reserved = 0,
        stopLine = 1,
        roundedCapStyleA = 2,
        roundedCapStyleB = 3,
        mergePoint = 4,
        divergePoint = 5,
        downstreamStopLine = 6,
        downstreamStartNode = 7,
        closedToTraffic = 8,
        safeIsland = 9,
        curbPresentAtStepOff = 10,
        hydrantPresent = 11,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct NodeAttributeXYList(pub SequenceOf<NodeAttributeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct NodeLink(pub SequenceOf<Node>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum NodeListXY {
        nodes(NodeSetXY),
        computed(ComputedLane),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct NodeOffsetPointXYRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl NodeOffsetPointXYRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum NodeOffsetPointXY {
        #[rasn(identifier = "node-XY1")]
        node_XY1(NodeXY20b),
        #[rasn(identifier = "node-XY2")]
        node_XY2(NodeXY22b),
        #[rasn(identifier = "node-XY3")]
        node_XY3(NodeXY24b),
        #[rasn(identifier = "node-XY4")]
        node_XY4(NodeXY26b),
        #[rasn(identifier = "node-XY5")]
        node_XY5(NodeXY28b),
        #[rasn(identifier = "node-XY6")]
        node_XY6(NodeXY32b),
        #[rasn(identifier = "node-LatLon")]
        node_LatLon(NodeLLmD64b),
        regional(NodeOffsetPointXYRegional),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum NodeOffsetPointZ {
        #[rasn(identifier = "node-Z1")]
        node_Z1(OffsetB10),
        #[rasn(identifier = "node-Z2")]
        node_Z2(OffsetB11),
        #[rasn(identifier = "node-Z3")]
        node_Z3(OffsetB12),
        #[rasn(identifier = "node-Z4")]
        node_Z4(OffsetB13),
        #[rasn(identifier = "node-Z5")]
        node_Z5(OffsetB14),
        #[rasn(identifier = "node-Z6")]
        node_Z6(OffsetB16),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("2..=63"))]
    pub struct NodeSetXY(pub SequenceOf<NodeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct NodeXY {
        pub delta: NodeOffsetPointXY,
        pub attributes: Option<NodeAttributeSetXY>,
    }
    impl NodeXY {
        pub fn new(delta: NodeOffsetPointXY, attributes: Option<NodeAttributeSetXY>) -> Self {
            Self { delta, attributes }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct NumberOfPerceivedObjects(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1500"))]
    pub struct ObjectAge(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectClass {
        pub confidence: ClassConfidence,
        pub class: ObjectClassChoice,
    }
    impl ObjectClass {
        pub fn new(confidence: ClassConfidence, class: ObjectClassChoice) -> Self {
            Self { confidence, class }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ObjectClassChoice {
        vehicle(VehicleSubclass),
        person(PersonSubclass),
        animal(AnimalSubclass),
        other(OtherSubclass),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct ObjectClassDescription(pub SequenceOf<ObjectClass>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct ObjectConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectDimension {
        pub value: ObjectDimensionValue,
        pub confidence: ObjectDimensionConfidence,
    }
    impl ObjectDimension {
        pub fn new(value: ObjectDimensionValue, confidence: ObjectDimensionConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct ObjectDimensionConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct ObjectDimensionValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectDistanceWithConfidence {
        pub value: DistanceValue,
        pub confidence: DistanceConfidence,
    }
    impl ObjectDistanceWithConfidence {
        pub fn new(value: DistanceValue, confidence: DistanceConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=8"))]
    pub struct ObjectRefPoint(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B09", value("-256..=255"))]
    pub struct OffsetB09(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B10", value("-512..=511"))]
    pub struct OffsetB10(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B11", value("-1024..=1023"))]
    pub struct OffsetB11(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B12", value("-2048..=2047"))]
    pub struct OffsetB12(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B13", value("-4096..=4095"))]
    pub struct OffsetB13(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B14", value("-8192..=8191"))]
    pub struct OffsetB14(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Offset-B16", value("-32768..=32767"))]
    pub struct OffsetB16(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct OffsetPoint {
        #[rasn(identifier = "nodeOffsetPointxy")]
        pub node_offset_pointxy: NodeOffsetPointXY,
        #[rasn(identifier = "nodeOffsetPointZ")]
        pub node_offset_point_z: Option<NodeOffsetPointZ>,
    }
    impl OffsetPoint {
        pub fn new(
            node_offset_pointxy: NodeOffsetPointXY,
            node_offset_point_z: Option<NodeOffsetPointZ>,
        ) -> Self {
            Self {
                node_offset_pointxy,
                node_offset_point_z,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum OffsetXaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum OffsetYaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum OriginatingRSUContainer {
        intersectionReferenceId(IntersectionReferenceID),
        roadSegmentReferenceId(RoadSegmentReferenceID),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OriginatingVehicleContainer {
        pub heading: Heading,
        pub speed: Speed,
        #[rasn(identifier = "vehicleOrientationAngle")]
        pub vehicle_orientation_angle: Option<WGS84Angle>,
        #[rasn(
            default = "originating_vehicle_container_drive_direction_default",
            identifier = "driveDirection"
        )]
        pub drive_direction: DriveDirection,
        #[rasn(identifier = "longitudinalAcceleration")]
        pub longitudinal_acceleration: Option<LongitudinalAcceleration>,
        #[rasn(identifier = "lateralAcceleration")]
        pub lateral_acceleration: Option<LateralAcceleration>,
        #[rasn(identifier = "verticalAcceleration")]
        pub vertical_acceleration: Option<VerticalAcceleration>,
        #[rasn(identifier = "yawRate")]
        pub yaw_rate: Option<YawRate>,
        #[rasn(identifier = "pitchAngle")]
        pub pitch_angle: Option<CartesianAngle>,
        #[rasn(identifier = "rollAngle")]
        pub roll_angle: Option<CartesianAngle>,
        #[rasn(identifier = "vehicleLength")]
        pub vehicle_length: Option<VehicleLength>,
        #[rasn(identifier = "vehicleWidth")]
        pub vehicle_width: Option<VehicleWidth>,
        #[rasn(identifier = "vehicleHeight")]
        pub vehicle_height: Option<VehicleHeight>,
        #[rasn(identifier = "trailerDataContainer")]
        pub trailer_data_container: Option<TrailerDataContainer>,
    }
    impl OriginatingVehicleContainer {
        pub fn new(
            heading: Heading,
            speed: Speed,
            vehicle_orientation_angle: Option<WGS84Angle>,
            drive_direction: DriveDirection,
            longitudinal_acceleration: Option<LongitudinalAcceleration>,
            lateral_acceleration: Option<LateralAcceleration>,
            vertical_acceleration: Option<VerticalAcceleration>,
            yaw_rate: Option<YawRate>,
            pitch_angle: Option<CartesianAngle>,
            roll_angle: Option<CartesianAngle>,
            vehicle_length: Option<VehicleLength>,
            vehicle_width: Option<VehicleWidth>,
            vehicle_height: Option<VehicleHeight>,
            trailer_data_container: Option<TrailerDataContainer>,
        ) -> Self {
            Self {
                heading,
                speed,
                vehicle_orientation_angle,
                drive_direction,
                longitudinal_acceleration,
                lateral_acceleration,
                vertical_acceleration,
                yaw_rate,
                pitch_angle,
                roll_angle,
                vehicle_length,
                vehicle_width,
                vehicle_height,
                trailer_data_container,
            }
        }
    }
    fn originating_vehicle_container_drive_direction_default() -> DriveDirection {
        DriveDirection::forward
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct OtherSubclass {
        #[rasn(default = "other_subclass_r_type_default", identifier = "type")]
        pub r_type: OtherSublassType,
        #[rasn(default = "other_subclass_confidence_default")]
        pub confidence: ClassConfidence,
    }
    impl OtherSubclass {
        pub fn new(r_type: OtherSublassType, confidence: ClassConfidence) -> Self {
            Self { r_type, confidence }
        }
    }
    impl std::default::Default for OtherSubclass {
        fn default() -> Self {
            Self {
                r_type: other_subclass_r_type_default(),
                confidence: other_subclass_confidence_default(),
            }
        }
    }
    fn other_subclass_r_type_default() -> OtherSublassType {
        OtherSublassType(0)
    }
    fn other_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct OtherSublassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=5"))]
    pub struct OverlayLaneList(pub SequenceOf<LaneID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct PMD(pub FixedBitString<4usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Particulate {
        #[rasn(identifier = "unitType")]
        pub unit_type: UnitType,
        #[rasn(value("0..=32767"))]
        pub value: u16,
    }
    impl Particulate {
        pub fn new(unit_type: UnitType, value: u16) -> Self {
            Self { unit_type, value }
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct PedestrianBicycleDetect(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceivedObject {
        #[rasn(identifier = "objectID")]
        pub object_id: Identifier,
        #[rasn(identifier = "sensorIDList")]
        pub sensor_idlist: Option<SensorIdList>,
        #[rasn(identifier = "timeOfMeasurement")]
        pub time_of_measurement: TimeOfMeasurement,
        #[rasn(identifier = "objectAge")]
        pub object_age: Option<ObjectAge>,
        #[rasn(
            default = "perceived_object_object_confidence_default",
            identifier = "objectConfidence"
        )]
        pub object_confidence: ObjectConfidence,
        #[rasn(identifier = "xDistance")]
        pub x_distance: ObjectDistanceWithConfidence,
        #[rasn(identifier = "yDistance")]
        pub y_distance: ObjectDistanceWithConfidence,
        #[rasn(identifier = "zDistance")]
        pub z_distance: Option<ObjectDistanceWithConfidence>,
        #[rasn(identifier = "xSpeed")]
        pub x_speed: SpeedExtended,
        #[rasn(identifier = "ySpeed")]
        pub y_speed: SpeedExtended,
        #[rasn(identifier = "zSpeed")]
        pub z_speed: Option<SpeedExtended>,
        #[rasn(identifier = "xAcceleration")]
        pub x_acceleration: Option<LongitudinalAcceleration>,
        #[rasn(identifier = "yAcceleration")]
        pub y_acceleration: Option<LateralAcceleration>,
        #[rasn(identifier = "zAcceleration")]
        pub z_acceleration: Option<VerticalAcceleration>,
        #[rasn(identifier = "yawAngle")]
        pub yaw_angle: Option<CartesianAngle>,
        #[rasn(identifier = "planarObjectDimension1")]
        pub planar_object_dimension1: Option<ObjectDimension>,
        #[rasn(identifier = "planarObjectDimension2")]
        pub planar_object_dimension2: Option<ObjectDimension>,
        #[rasn(identifier = "verticalObjectDimension")]
        pub vertical_object_dimension: Option<ObjectDimension>,
        #[rasn(
            default = "perceived_object_object_ref_point_default",
            identifier = "objectRefPoint"
        )]
        pub object_ref_point: ObjectRefPoint,
        #[rasn(identifier = "dynamicStatus")]
        pub dynamic_status: Option<DynamicStatus>,
        pub classification: Option<ObjectClassDescription>,
        #[rasn(identifier = "matchedPosition")]
        pub matched_position: Option<MatchedPosition>,
    }
    impl PerceivedObject {
        pub fn new(
            object_id: Identifier,
            sensor_idlist: Option<SensorIdList>,
            time_of_measurement: TimeOfMeasurement,
            object_age: Option<ObjectAge>,
            object_confidence: ObjectConfidence,
            x_distance: ObjectDistanceWithConfidence,
            y_distance: ObjectDistanceWithConfidence,
            z_distance: Option<ObjectDistanceWithConfidence>,
            x_speed: SpeedExtended,
            y_speed: SpeedExtended,
            z_speed: Option<SpeedExtended>,
            x_acceleration: Option<LongitudinalAcceleration>,
            y_acceleration: Option<LateralAcceleration>,
            z_acceleration: Option<VerticalAcceleration>,
            yaw_angle: Option<CartesianAngle>,
            planar_object_dimension1: Option<ObjectDimension>,
            planar_object_dimension2: Option<ObjectDimension>,
            vertical_object_dimension: Option<ObjectDimension>,
            object_ref_point: ObjectRefPoint,
            dynamic_status: Option<DynamicStatus>,
            classification: Option<ObjectClassDescription>,
            matched_position: Option<MatchedPosition>,
        ) -> Self {
            Self {
                object_id,
                sensor_idlist,
                time_of_measurement,
                object_age,
                object_confidence,
                x_distance,
                y_distance,
                z_distance,
                x_speed,
                y_speed,
                z_speed,
                x_acceleration,
                y_acceleration,
                z_acceleration,
                yaw_angle,
                planar_object_dimension1,
                planar_object_dimension2,
                vertical_object_dimension,
                object_ref_point,
                dynamic_status,
                classification,
                matched_position,
            }
        }
    }
    fn perceived_object_object_confidence_default() -> ObjectConfidence {
        ObjectConfidence(0)
    }
    fn perceived_object_object_ref_point_default() -> ObjectRefPoint {
        ObjectRefPoint(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct PerceivedObjectContainer(pub SequenceOf<PerceivedObject>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PerceivedObjectContainerSegmentInfo {
        #[rasn(identifier = "totalMsgSegments")]
        pub total_msg_segments: SegmentCount,
        #[rasn(identifier = "thisSegmentNum")]
        pub this_segment_num: SegmentCount,
    }
    impl PerceivedObjectContainerSegmentInfo {
        pub fn new(total_msg_segments: SegmentCount, this_segment_num: SegmentCount) -> Self {
            Self {
                total_msg_segments,
                this_segment_num,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PersonSubclass {
        #[rasn(default = "person_subclass_r_type_default", identifier = "type")]
        pub r_type: PersonSubclassType,
        #[rasn(default = "person_subclass_confidence_default")]
        pub confidence: ClassConfidence,
    }
    impl PersonSubclass {
        pub fn new(r_type: PersonSubclassType, confidence: ClassConfidence) -> Self {
            Self { r_type, confidence }
        }
    }
    impl std::default::Default for PersonSubclass {
        fn default() -> Self {
            Self {
                r_type: person_subclass_r_type_default(),
                confidence: person_subclass_confidence_default(),
            }
        }
    }
    fn person_subclass_r_type_default() -> PersonSubclassType {
        PersonSubclassType(0)
    }
    fn person_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PersonSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PictogramCategoryCode {
        #[rasn(value("1..=9"))]
        pub nature: u8,
        #[rasn(value("0..=99"), identifier = "serialNumber")]
        pub serial_number: u8,
    }
    impl PictogramCategoryCode {
        pub fn new(nature: u8, serial_number: u8) -> Self {
            Self {
                nature,
                serial_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PictogramCode {
        #[rasn(size("2"), identifier = "countryCode")]
        pub country_code: Option<OctetString>,
        #[rasn(identifier = "serviceCategoryCode")]
        pub service_category_code: ServiceCategoryCode,
        #[rasn(identifier = "pictogramCategoryCode")]
        pub pictogram_category_code: PictogramCategoryCode,
    }
    impl PictogramCode {
        pub fn new(
            country_code: Option<OctetString>,
            service_category_code: ServiceCategoryCode,
            pictogram_category_code: PictogramCategoryCode,
        ) -> Self {
            Self {
                country_code,
                service_category_code,
                pictogram_category_code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("3..=16", extensible))]
    pub struct PolyPointList(pub SequenceOf<OffsetPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PolygonalLine {
        deltaPositions(DeltaPositions),
        deltaPositionsWithAltitude(DeltaPositionsWithAltitude),
        absolutePositions(AbsolutePositions),
        absolutePositionsWithAltitude(AbsolutePositionsWithAltitude),
    }
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
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousPosition3DRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousPosition3DRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct Position3DRegional(pub SequenceOf<AnonymousPosition3DRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Position3D {
        pub lat: Latitude,
        pub long: Longitude,
        pub elevation: Option<Elevation>,
        pub regional: Option<Position3DRegional>,
    }
    impl Position3D {
        pub fn new(
            lat: Latitude,
            long: Longitude,
            elevation: Option<Elevation>,
            regional: Option<Position3DRegional>,
        ) -> Self {
            Self {
                lat,
                long,
                elevation,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "Position3D-addGrpC")]
    #[non_exhaustive]
    pub struct Position3DAddGrpC {
        pub altitude: Altitude,
    }
    impl Position3DAddGrpC {
        pub fn new(altitude: Altitude) -> Self {
            Self { altitude }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum PositionConfidence {
        unavailable = 0,
        a500m = 1,
        a200m = 2,
        a100m = 3,
        a50m = 4,
        a20m = 5,
        a10m = 6,
        a5m = 7,
        a2m = 8,
        a1m = 9,
        a50cm = 10,
        a20cm = 11,
        a10cm = 12,
        a5cm = 13,
        a2cm = 14,
        a1cm = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PositionConfidenceSet {
        pub pos: PositionConfidence,
        pub elevation: ElevationConfidence,
    }
    impl PositionConfidenceSet {
        pub fn new(pos: PositionConfidence, elevation: ElevationConfidence) -> Self {
            Self { pos, elevation }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct PositionalAccuracy {
        #[rasn(identifier = "semiMajor")]
        pub semi_major: SemiMajorAxisAccuracy,
        #[rasn(identifier = "semiMinor")]
        pub semi_minor: SemiMinorAxisAccuracy,
        pub orientation: SemiMajorAxisOrientation,
    }
    impl PositionalAccuracy {
        pub fn new(
            semi_major: SemiMajorAxisAccuracy,
            semi_minor: SemiMinorAxisAccuracy,
            orientation: SemiMajorAxisOrientation,
        ) -> Self {
            Self {
                semi_major,
                semi_minor,
                orientation,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct PreemptPriorityList(pub SequenceOf<SignalControlZone>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PrioritizationResponse {
        #[rasn(identifier = "stationID")]
        pub station_id: StationID,
        #[rasn(identifier = "priorState")]
        pub prior_state: PrioritizationResponseStatus,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: SignalGroupID,
    }
    impl PrioritizationResponse {
        pub fn new(
            station_id: StationID,
            prior_state: PrioritizationResponseStatus,
            signal_group: SignalGroupID,
        ) -> Self {
            Self {
                station_id,
                prior_state,
                signal_group,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=10"))]
    pub struct PrioritizationResponseList(pub SequenceOf<PrioritizationResponse>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PrioritizationResponseStatus {
        unknown = 0,
        requested = 1,
        processing = 2,
        watchOtherTraffic = 3,
        granted = 4,
        rejected = 5,
        maxPresence = 6,
        reserviceLocked = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PriorityRequestType {
        priorityRequestTypeReserved = 0,
        priorityRequest = 1,
        priorityRequestUpdate = 2,
        priorityCancellation = 3,
    }
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PtvRequestType {
        preRequest = 0,
        mainRequest = 1,
        doorCloseRequest = 2,
        cancelRequest = 3,
        emergencyRequest = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PublicFacilitiesPictogram {
        publicFacilities = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=32"))]
    pub struct ROI(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct RSCUnit(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RSCode {
        #[rasn(value("1..=4", extensible), identifier = "layoutComponentId")]
        pub layout_component_id: Option<Integer>,
        pub code: Code,
    }
    impl RSCode {
        pub fn new(layout_component_id: Option<Integer>, code: Code) -> Self {
            Self {
                layout_component_id,
                code,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Radius(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Range(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RccPart {
        #[rasn(size("1..=8", extensible), identifier = "zoneIds")]
        pub zone_ids: SequenceOf<Zid>,
        #[rasn(identifier = "roadType")]
        pub road_type: RoadType,
        #[rasn(size("1..=16", extensible), identifier = "laneConfiguration")]
        pub lane_configuration: SequenceOf<LaneInformation>,
    }
    impl RccPart {
        pub fn new(
            zone_ids: SequenceOf<Zid>,
            road_type: RoadType,
            lane_configuration: SequenceOf<LaneInformation>,
        ) -> Self {
            Self {
                zone_ids,
                road_type,
                lane_configuration,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=150"))]
    pub struct RearOverhang(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RefPointId(pub u8);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RegionId(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RegulatorySpeedLimit {
        #[rasn(identifier = "type")]
        pub r_type: SpeedLimitType,
        pub speed: Velocity,
    }
    impl RegulatorySpeedLimit {
        pub fn new(r_type: SpeedLimitType, speed: Velocity) -> Self {
            Self { r_type, speed }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RejectedReason {
        unknown = 0,
        exceptionalCondition = 1,
        maxWaitingTimeExceeded = 2,
        ptPriorityDisabled = 3,
        higherPTPriorityGranted = 4,
        vehicleTrackingUnknown = 5,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RequestID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestImportanceLevel {
        requestImportanceLevelUnKnown = 0,
        requestImportanceLevel1 = 1,
        requestImportanceLevel2 = 2,
        requestImportanceLevel3 = 3,
        requestImportanceLevel4 = 4,
        requestImportanceLevel5 = 5,
        requestImportanceLevel6 = 6,
        requestImportanceLevel7 = 7,
        requestImportanceLevel8 = 8,
        requestImportanceLevel9 = 9,
        requestImportanceLevel10 = 10,
        requestImportanceLevel11 = 11,
        requestImportanceLevel12 = 12,
        requestImportanceLevel13 = 13,
        requestImportanceLevel14 = 14,
        requestImportanceReserved = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestResponseIndication {
        request = 0,
        response = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum RequestSubRole {
        requestSubRoleUnKnown = 0,
        requestSubRole1 = 1,
        requestSubRole2 = 2,
        requestSubRole3 = 3,
        requestSubRole4 = 4,
        requestSubRole5 = 5,
        requestSubRole6 = 6,
        requestSubRole7 = 7,
        requestSubRole8 = 8,
        requestSubRole9 = 9,
        requestSubRole10 = 10,
        requestSubRole11 = 11,
        requestSubRole12 = 12,
        requestSubRole13 = 13,
        requestSubRole14 = 14,
        requestSubRoleReserved = 15,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRequesterDescriptionRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRequesterDescriptionRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RequesterDescriptionRegional(pub SequenceOf<AnonymousRequesterDescriptionRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequesterDescription {
        pub id: VehicleID,
        #[rasn(identifier = "type")]
        pub r_type: Option<RequesterType>,
        pub position: Option<RequesterPositionVector>,
        pub name: Option<DescriptiveName>,
        #[rasn(identifier = "routeName")]
        pub route_name: Option<DescriptiveName>,
        #[rasn(identifier = "transitStatus")]
        pub transit_status: Option<TransitVehicleStatus>,
        #[rasn(identifier = "transitOccupancy")]
        pub transit_occupancy: Option<TransitVehicleOccupancy>,
        #[rasn(identifier = "transitSchedule")]
        pub transit_schedule: Option<DeltaTime>,
        pub regional: Option<RequesterDescriptionRegional>,
    }
    impl RequesterDescription {
        pub fn new(
            id: VehicleID,
            r_type: Option<RequesterType>,
            position: Option<RequesterPositionVector>,
            name: Option<DescriptiveName>,
            route_name: Option<DescriptiveName>,
            transit_status: Option<TransitVehicleStatus>,
            transit_occupancy: Option<TransitVehicleOccupancy>,
            transit_schedule: Option<DeltaTime>,
            regional: Option<RequesterDescriptionRegional>,
        ) -> Self {
            Self {
                id,
                r_type,
                position,
                name,
                route_name,
                transit_status,
                transit_occupancy,
                transit_schedule,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "RequesterDescription-addGrpC")]
    #[non_exhaustive]
    pub struct RequesterDescriptionAddGrpC {
        pub fuel: Option<FuelType>,
        #[rasn(identifier = "batteryStatus")]
        pub battery_status: Option<BatteryStatus>,
    }
    impl RequesterDescriptionAddGrpC {
        pub fn new(fuel: Option<FuelType>, battery_status: Option<BatteryStatus>) -> Self {
            Self {
                fuel,
                battery_status,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequesterPositionVector {
        pub position: Position3D,
        pub heading: Option<Angle>,
        pub speed: Option<TransmissionAndSpeed>,
    }
    impl RequesterPositionVector {
        pub fn new(
            position: Position3D,
            heading: Option<Angle>,
            speed: Option<TransmissionAndSpeed>,
        ) -> Self {
            Self {
                position,
                heading,
                speed,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RequesterTypeRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl RequesterTypeRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RequesterType {
        pub role: BasicVehicleRole,
        pub subrole: Option<RequestSubRole>,
        pub request: Option<RequestImportanceLevel>,
        pub iso3883: Option<Iso3833VehicleType>,
        #[rasn(identifier = "hpmsType")]
        pub hpms_type: Option<VehicleType>,
        pub regional: Option<RequesterTypeRegional>,
    }
    impl RequesterType {
        pub fn new(
            role: BasicVehicleRole,
            subrole: Option<RequestSubRole>,
            request: Option<RequestImportanceLevel>,
            iso3883: Option<Iso3833VehicleType>,
            hpms_type: Option<VehicleType>,
            regional: Option<RequesterTypeRegional>,
        ) -> Self {
            Self {
                role,
                subrole,
                request,
                iso3883,
                hpms_type,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum RestrictionAppliesTo {
        none = 0,
        equippedTransit = 1,
        equippedTaxis = 2,
        equippedOther = 3,
        emissionCompliant = 4,
        equippedBicycle = 5,
        weightCompliant = 6,
        heightCompliant = 7,
        pedestrians = 8,
        slowMovingPersons = 9,
        wheelchairUsers = 10,
        visualDisabilities = 11,
        audioDisabilities = 12,
        otherUnknownDisabilities = 13,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct RestrictionClassAssignment {
        pub id: RestrictionClassID,
        pub users: RestrictionUserTypeList,
    }
    impl RestrictionClassAssignment {
        pub fn new(id: RestrictionClassID, users: RestrictionUserTypeList) -> Self {
            Self { id, users }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RestrictionClassID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=254"))]
    pub struct RestrictionClassList(pub SequenceOf<RestrictionClassAssignment>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRestrictionUserTypeRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRestrictionUserTypeRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RestrictionUserTypeRegional(pub SequenceOf<AnonymousRestrictionUserTypeRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum RestrictionUserType {
        basicType(RestrictionAppliesTo),
        regional(RestrictionUserTypeRegional),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "RestrictionUserType-addGrpC")]
    #[non_exhaustive]
    pub struct RestrictionUserTypeAddGrpC {
        pub emission: Option<EmissionType>,
        pub fuel: Option<FuelType>,
    }
    impl RestrictionUserTypeAddGrpC {
        pub fn new(emission: Option<EmissionType>, fuel: Option<FuelType>) -> Self {
            Self { emission, fuel }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16"))]
    pub struct RestrictionUserTypeList(pub SequenceOf<RestrictionUserType>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct RoadConfigurationContainer(pub SequenceOf<RccPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=255"))]
    pub struct RoadLaneSetList(pub SequenceOf<GenericLane>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadRegulatorID(pub u16);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRoadSegmentRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRoadSegmentRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RoadSegmentRegional(pub SequenceOf<AnonymousRoadSegmentRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RoadSegment {
        pub name: Option<DescriptiveName>,
        pub id: RoadSegmentReferenceID,
        pub revision: MsgCount,
        #[rasn(identifier = "refPoint")]
        pub ref_point: Position3D,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<LaneWidth>,
        #[rasn(identifier = "speedLimits")]
        pub speed_limits: Option<SpeedLimitList>,
        #[rasn(identifier = "roadLaneSet")]
        pub road_lane_set: RoadLaneSetList,
        pub regional: Option<RoadSegmentRegional>,
    }
    impl RoadSegment {
        pub fn new(
            name: Option<DescriptiveName>,
            id: RoadSegmentReferenceID,
            revision: MsgCount,
            ref_point: Position3D,
            lane_width: Option<LaneWidth>,
            speed_limits: Option<SpeedLimitList>,
            road_lane_set: RoadLaneSetList,
            regional: Option<RoadSegmentRegional>,
        ) -> Self {
            Self {
                name,
                id,
                revision,
                ref_point,
                lane_width,
                speed_limits,
                road_lane_set,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadSegmentID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct RoadSegmentList(pub SequenceOf<RoadSegment>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[rasn(delegate, value("-128..=127"))]
    pub struct RoadwayCrownAngle(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct SAEHeading(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum SAEHeadingConfidence {
        unavailable = 0,
        prec10deg = 1,
        prec05deg = 2,
        prec01deg = 3,
        #[rasn(identifier = "prec0-1deg")]
        prec0_1deg = 4,
        #[rasn(identifier = "prec0-05deg")]
        prec0_05deg = 5,
        #[rasn(identifier = "prec0-01deg")]
        prec0_01deg = 6,
        #[rasn(identifier = "prec0-0125deg")]
        prec0_0125deg = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum SAESpeedConfidence {
        unavailable = 0,
        prec100ms = 1,
        prec10ms = 2,
        prec5ms = 3,
        prec1ms = 4,
        #[rasn(identifier = "prec0-1ms")]
        prec0_1ms = 5,
        #[rasn(identifier = "prec0-05ms")]
        prec0_05ms = 6,
        #[rasn(identifier = "prec0-01ms")]
        prec0_01ms = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum SAEThrottleConfidence {
        unavailable = 0,
        prec10percent = 1,
        prec1percent = 2,
        #[rasn(identifier = "prec0-5percent")]
        prec0_5percent = 3,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSPATRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSPATRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SPATRegional(pub SequenceOf<AnonymousSPATRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SPAT {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub name: Option<DescriptiveName>,
        pub intersections: IntersectionStateList,
        pub regional: Option<SPATRegional>,
    }
    impl SPAT {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            name: Option<DescriptiveName>,
            intersections: IntersectionStateList,
            regional: Option<SPATRegional>,
        ) -> Self {
            Self {
                time_stamp,
                name,
                intersections,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SPATEM {
        pub header: ItsPduHeader,
        pub spat: SPAT,
    }
    impl SPATEM {
        pub fn new(header: ItsPduHeader, spat: SPAT) -> Self {
            Self { header, spat }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SPE {
        #[rasn(value("0..=250"))]
        pub spm: Option<u8>,
        #[rasn(value("0..=250"))]
        pub mns: Option<u8>,
        pub unit: RSCUnit,
    }
    impl SPE {
        pub fn new(spm: Option<u8>, mns: Option<u8>, unit: RSCUnit) -> Self {
            Self { spm, mns, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SREM {
        pub header: ItsPduHeader,
        pub srm: SignalRequestMessage,
    }
    impl SREM {
        pub fn new(header: ItsPduHeader, srm: SignalRequestMessage) -> Self {
            Self { header, srm }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SSEM {
        pub header: ItsPduHeader,
        pub ssm: SignalStatusMessage,
    }
    impl SSEM {
        pub fn new(header: ItsPduHeader, ssm: SignalStatusMessage) -> Self {
            Self { header, ssm }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, identifier = "Scale-B12", value("-2048..=2047"))]
    pub struct ScaleB12(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Segment {
        pub line: PolygonalLine,
        #[rasn(identifier = "laneWidth")]
        pub lane_width: Option<IVILaneWidth>,
    }
    impl Segment {
        pub fn new(line: PolygonalLine, lane_width: Option<IVILaneWidth>) -> Self {
            Self { line, lane_width }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SegmentAttributeXY {
        reserved = 0,
        doNotBlock = 1,
        whiteLine = 2,
        mergingLaneLeft = 3,
        mergingLaneRight = 4,
        curbOnLeft = 5,
        curbOnRight = 6,
        loadingzoneOnLeft = 7,
        loadingzoneOnRight = 8,
        turnOutPointOnLeft = 9,
        turnOutPointOnRight = 10,
        adjacentParkingOnLeft = 11,
        adjacentParkingOnRight = 12,
        adjacentBikeLaneOnLeft = 13,
        adjacentBikeLaneOnRight = 14,
        sharedBikeLane = 15,
        bikeBoxInFront = 16,
        transitStopOnLeft = 17,
        transitStopOnRight = 18,
        transitStopInLane = 19,
        sharedWithTrackedVehicle = 20,
        safeIsland = 21,
        lowCurbsPresent = 22,
        rumbleStripPresent = 23,
        audibleSignalingPresent = 24,
        adaptiveTimingPresent = 25,
        rfSignalRequestPresent = 26,
        partialCurbIntrusion = 27,
        taperToLeft = 28,
        taperToRight = 29,
        taperToCenterLine = 30,
        parallelParking = 31,
        headInParking = 32,
        freeParking = 33,
        timeRestrictionsOnParking = 34,
        costToPark = 35,
        midBlockCurbPresent = 36,
        unEvenPavementPresent = 37,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8"))]
    pub struct SegmentAttributeXYList(pub SequenceOf<SegmentAttributeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SegmentCount(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct SemiAxisLength(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMajorAxisAccuracy(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SemiMajorAxisOrientation(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMinorAxisAccuracy(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct SemiRangeLength(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-5000..=5000"))]
    pub struct SensorHeight(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorIdList(pub SequenceOf<Identifier>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorInformation {
        #[rasn(identifier = "sensorID")]
        pub sensor_id: Identifier,
        #[rasn(identifier = "type")]
        pub r_type: SensorType,
        #[rasn(identifier = "detectionArea")]
        pub detection_area: DetectionArea,
        #[rasn(identifier = "freeSpaceConfidence")]
        pub free_space_confidence: Option<FreeSpaceConfidence>,
    }
    impl SensorInformation {
        pub fn new(
            sensor_id: Identifier,
            r_type: SensorType,
            detection_area: DetectionArea,
            free_space_confidence: Option<FreeSpaceConfidence>,
        ) -> Self {
            Self {
                sensor_id,
                r_type,
                detection_area,
                free_space_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct SensorType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SequenceNumber(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ServiceCategoryCode {
        trafficSignPictogram(TrafficSignPictogram),
        publicFacilitiesPictogram(PublicFacilitiesPictogram),
        ambientOrRoadConditionPictogram(AmbientOrRoadConditionPictogram),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct ShadowingApplies(pub bool);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SignalControlZoneZone {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl SignalControlZoneZone {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalControlZone {
        pub zone: SignalControlZoneZone,
    }
    impl SignalControlZone {
        pub fn new(zone: SignalControlZoneZone) -> Self {
            Self { zone }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalGroupID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalHeadLocation {
        #[rasn(identifier = "nodeXY")]
        pub node_xy: NodeOffsetPointXY,
        #[rasn(identifier = "nodeZ")]
        pub node_z: DeltaAltitude,
        #[rasn(identifier = "signalGroupID")]
        pub signal_group_id: SignalGroupID,
    }
    impl SignalHeadLocation {
        pub fn new(
            node_xy: NodeOffsetPointXY,
            node_z: DeltaAltitude,
            signal_group_id: SignalGroupID,
        ) -> Self {
            Self {
                node_xy,
                node_z,
                signal_group_id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=64"))]
    pub struct SignalHeadLocationList(pub SequenceOf<SignalHeadLocation>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestRegional(pub SequenceOf<AnonymousSignalRequestRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequest {
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "requestID")]
        pub request_id: RequestID,
        #[rasn(identifier = "requestType")]
        pub request_type: PriorityRequestType,
        #[rasn(identifier = "inBoundLane")]
        pub in_bound_lane: IntersectionAccessPoint,
        #[rasn(identifier = "outBoundLane")]
        pub out_bound_lane: Option<IntersectionAccessPoint>,
        pub regional: Option<SignalRequestRegional>,
    }
    impl SignalRequest {
        pub fn new(
            id: IntersectionReferenceID,
            request_id: RequestID,
            request_type: PriorityRequestType,
            in_bound_lane: IntersectionAccessPoint,
            out_bound_lane: Option<IntersectionAccessPoint>,
            regional: Option<SignalRequestRegional>,
        ) -> Self {
            Self {
                id,
                request_id,
                request_type,
                in_bound_lane,
                out_bound_lane,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalRequestList(pub SequenceOf<SignalRequestPackage>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestMessageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestMessageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestMessageRegional(pub SequenceOf<AnonymousSignalRequestMessageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequestMessage {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub second: DSecond,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: Option<MsgCount>,
        pub requests: Option<SignalRequestList>,
        pub requester: RequesterDescription,
        pub regional: Option<SignalRequestMessageRegional>,
    }
    impl SignalRequestMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            requests: Option<SignalRequestList>,
            requester: RequesterDescription,
            regional: Option<SignalRequestMessageRegional>,
        ) -> Self {
            Self {
                time_stamp,
                second,
                sequence_number,
                requests,
                requester,
                regional,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalRequestPackageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalRequestPackageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestPackageRegional(pub SequenceOf<AnonymousSignalRequestPackageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequestPackage {
        pub request: SignalRequest,
        pub minute: Option<MinuteOfTheYear>,
        pub second: Option<DSecond>,
        pub duration: Option<DSecond>,
        pub regional: Option<SignalRequestPackageRegional>,
    }
    impl SignalRequestPackage {
        pub fn new(
            request: SignalRequest,
            minute: Option<MinuteOfTheYear>,
            second: Option<DSecond>,
            duration: Option<DSecond>,
            regional: Option<SignalRequestPackageRegional>,
        ) -> Self {
            Self {
                request,
                minute,
                second,
                duration,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequesterInfo {
        pub id: VehicleID,
        pub request: RequestID,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: MsgCount,
        pub role: Option<BasicVehicleRole>,
        #[rasn(identifier = "typeData")]
        pub type_data: Option<RequesterType>,
    }
    impl SignalRequesterInfo {
        pub fn new(
            id: VehicleID,
            request: RequestID,
            sequence_number: MsgCount,
            role: Option<BasicVehicleRole>,
            type_data: Option<RequesterType>,
        ) -> Self {
            Self {
                id,
                request,
                sequence_number,
                role,
                type_data,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusRegional(pub SequenceOf<AnonymousSignalStatusRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatus {
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: MsgCount,
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "sigStatus")]
        pub sig_status: SignalStatusPackageList,
        pub regional: Option<SignalStatusRegional>,
    }
    impl SignalStatus {
        pub fn new(
            sequence_number: MsgCount,
            id: IntersectionReferenceID,
            sig_status: SignalStatusPackageList,
            regional: Option<SignalStatusRegional>,
        ) -> Self {
            Self {
                sequence_number,
                id,
                sig_status,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusList(pub SequenceOf<SignalStatus>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusMessageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusMessageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusMessageRegional(pub SequenceOf<AnonymousSignalStatusMessageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatusMessage {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub second: DSecond,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: Option<MsgCount>,
        pub status: SignalStatusList,
        pub regional: Option<SignalStatusMessageRegional>,
    }
    impl SignalStatusMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            status: SignalStatusList,
            regional: Option<SignalStatusMessageRegional>,
        ) -> Self {
            Self {
                time_stamp,
                second,
                sequence_number,
                status,
                regional,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousSignalStatusPackageRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousSignalStatusPackageRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusPackageRegional(pub SequenceOf<AnonymousSignalStatusPackageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatusPackage {
        pub requester: Option<SignalRequesterInfo>,
        #[rasn(identifier = "inboundOn")]
        pub inbound_on: IntersectionAccessPoint,
        #[rasn(identifier = "outboundOn")]
        pub outbound_on: Option<IntersectionAccessPoint>,
        pub minute: Option<MinuteOfTheYear>,
        pub second: Option<DSecond>,
        pub duration: Option<DSecond>,
        pub status: PrioritizationResponseStatus,
        pub regional: Option<SignalStatusPackageRegional>,
    }
    impl SignalStatusPackage {
        pub fn new(
            requester: Option<SignalRequesterInfo>,
            inbound_on: IntersectionAccessPoint,
            outbound_on: Option<IntersectionAccessPoint>,
            minute: Option<MinuteOfTheYear>,
            second: Option<DSecond>,
            duration: Option<DSecond>,
            status: PrioritizationResponseStatus,
            regional: Option<SignalStatusPackageRegional>,
        ) -> Self {
            Self {
                requester,
                inbound_on,
                outbound_on,
                minute,
                second,
                duration,
                status,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "SignalStatusPackage-addGrpC")]
    #[non_exhaustive]
    pub struct SignalStatusPackageAddGrpC {
        #[rasn(identifier = "synchToSchedule")]
        pub synch_to_schedule: Option<DeltaTime>,
        #[rasn(identifier = "rejectedReason")]
        pub rejected_reason: Option<RejectedReason>,
    }
    impl SignalStatusPackageAddGrpC {
        pub fn new(
            synch_to_schedule: Option<DeltaTime>,
            rejected_reason: Option<RejectedReason>,
        ) -> Self {
            Self {
                synch_to_schedule,
                rejected_reason,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusPackageList(pub SequenceOf<SignalStatusPackage>);
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
    #[rasn(delegate, value("0..=500"))]
    pub struct SpeedAdvice(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SpeedConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SpeedExtended {
        pub value: SpeedValueExtended,
        pub confidence: SpeedConfidence,
    }
    impl SpeedExtended {
        pub fn new(value: SpeedValueExtended, confidence: SpeedConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=9"))]
    pub struct SpeedLimitList(pub SequenceOf<RegulatorySpeedLimit>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SpeedLimitType {
        unknown = 0,
        maxSpeedInSchoolZone = 1,
        maxSpeedInSchoolZoneWhenChildrenArePresent = 2,
        maxSpeedInConstructionZone = 3,
        vehicleMinSpeed = 4,
        vehicleMaxSpeed = 5,
        vehicleNightMaxSpeed = 6,
        truckMinSpeed = 7,
        truckMaxSpeed = 8,
        truckNightMaxSpeed = 9,
        vehiclesWithTrailersMinSpeed = 10,
        vehiclesWithTrailersMaxSpeed = 11,
        vehiclesWithTrailersNightMaxSpeed = 12,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct SpeedValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-16383..=16383"))]
    pub struct SpeedValueExtended(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SpeedandHeadingandThrottleConfidence {
        pub heading: SAEHeadingConfidence,
        pub speed: SAESpeedConfidence,
        pub throttle: SAEThrottleConfidence,
    }
    impl SpeedandHeadingandThrottleConfidence {
        pub fn new(
            heading: SAEHeadingConfidence,
            speed: SAESpeedConfidence,
            throttle: SAEThrottleConfidence,
        ) -> Self {
            Self {
                heading,
                speed,
                throttle,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum StationDataContainer {
        originatingVehicleContainer(OriginatingVehicleContainer),
        originatingRSUContainer(OriginatingRSUContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct StationID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationType(pub u8);
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
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TcPart {
        #[rasn(size("1..=8", extensible), identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<SequenceOf<Zid>>,
        #[rasn(size("1..=8", extensible), identifier = "relevanceZoneIds")]
        pub relevance_zone_ids: SequenceOf<Zid>,
        pub direction: Option<Direction>,
        #[rasn(size("1..=8", extensible), identifier = "driverAwarenessZoneIds")]
        pub driver_awareness_zone_ids: Option<SequenceOf<Zid>>,
        #[rasn(value("0..=255"), identifier = "minimumAwarenessTime")]
        pub minimum_awareness_time: Option<u8>,
        #[rasn(size("1..=8", extensible), identifier = "applicableLanes")]
        pub applicable_lanes: Option<SequenceOf<LanePosition>>,
        #[rasn(value("1..=4", extensible), identifier = "layoutId")]
        pub layout_id: Option<Integer>,
        #[rasn(value("1..=64", extensible), identifier = "preStoredlayoutId")]
        pub pre_storedlayout_id: Option<Integer>,
        #[rasn(size("1..=4", extensible))]
        pub text: Option<SequenceOf<Text>>,
        pub data: OctetString,
    }
    impl TcPart {
        pub fn new(
            detection_zone_ids: Option<SequenceOf<Zid>>,
            relevance_zone_ids: SequenceOf<Zid>,
            direction: Option<Direction>,
            driver_awareness_zone_ids: Option<SequenceOf<Zid>>,
            minimum_awareness_time: Option<u8>,
            applicable_lanes: Option<SequenceOf<LanePosition>>,
            layout_id: Option<Integer>,
            pre_storedlayout_id: Option<Integer>,
            text: Option<SequenceOf<Text>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TemporaryID(pub FixedOctetString<4usize>);
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
    #[rasn(automatic_tags)]
    pub struct TimeChangeDetails {
        #[rasn(identifier = "startTime")]
        pub start_time: Option<TimeMark>,
        #[rasn(identifier = "minEndTime")]
        pub min_end_time: TimeMark,
        #[rasn(identifier = "maxEndTime")]
        pub max_end_time: Option<TimeMark>,
        #[rasn(identifier = "likelyTime")]
        pub likely_time: Option<TimeMark>,
        pub confidence: Option<TimeIntervalConfidence>,
        #[rasn(identifier = "nextTime")]
        pub next_time: Option<TimeMark>,
    }
    impl TimeChangeDetails {
        pub fn new(
            start_time: Option<TimeMark>,
            min_end_time: TimeMark,
            max_end_time: Option<TimeMark>,
            likely_time: Option<TimeMark>,
            confidence: Option<TimeIntervalConfidence>,
            next_time: Option<TimeMark>,
        ) -> Self {
            Self {
                start_time,
                min_end_time,
                max_end_time,
                likely_time,
                confidence,
                next_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TimeConfidence {
        unavailable = 0,
        #[rasn(identifier = "time-100-000")]
        time_100_000 = 1,
        #[rasn(identifier = "time-050-000")]
        time_050_000 = 2,
        #[rasn(identifier = "time-020-000")]
        time_020_000 = 3,
        #[rasn(identifier = "time-010-000")]
        time_010_000 = 4,
        #[rasn(identifier = "time-002-000")]
        time_002_000 = 5,
        #[rasn(identifier = "time-001-000")]
        time_001_000 = 6,
        #[rasn(identifier = "time-000-500")]
        time_000_500 = 7,
        #[rasn(identifier = "time-000-200")]
        time_000_200 = 8,
        #[rasn(identifier = "time-000-100")]
        time_000_100 = 9,
        #[rasn(identifier = "time-000-050")]
        time_000_050 = 10,
        #[rasn(identifier = "time-000-020")]
        time_000_020 = 11,
        #[rasn(identifier = "time-000-010")]
        time_000_010 = 12,
        #[rasn(identifier = "time-000-005")]
        time_000_005 = 13,
        #[rasn(identifier = "time-000-002")]
        time_000_002 = 14,
        #[rasn(identifier = "time-000-001")]
        time_000_001 = 15,
        #[rasn(identifier = "time-000-000-5")]
        time_000_000_5 = 16,
        #[rasn(identifier = "time-000-000-2")]
        time_000_000_2 = 17,
        #[rasn(identifier = "time-000-000-1")]
        time_000_000_1 = 18,
        #[rasn(identifier = "time-000-000-05")]
        time_000_000_05 = 19,
        #[rasn(identifier = "time-000-000-02")]
        time_000_000_02 = 20,
        #[rasn(identifier = "time-000-000-01")]
        time_000_000_01 = 21,
        #[rasn(identifier = "time-000-000-005")]
        time_000_000_005 = 22,
        #[rasn(identifier = "time-000-000-002")]
        time_000_000_002 = 23,
        #[rasn(identifier = "time-000-000-001")]
        time_000_000_001 = 24,
        #[rasn(identifier = "time-000-000-000-5")]
        time_000_000_000_5 = 25,
        #[rasn(identifier = "time-000-000-000-2")]
        time_000_000_000_2 = 26,
        #[rasn(identifier = "time-000-000-000-1")]
        time_000_000_000_1 = 27,
        #[rasn(identifier = "time-000-000-000-05")]
        time_000_000_000_05 = 28,
        #[rasn(identifier = "time-000-000-000-02")]
        time_000_000_000_02 = 29,
        #[rasn(identifier = "time-000-000-000-01")]
        time_000_000_000_01 = 30,
        #[rasn(identifier = "time-000-000-000-005")]
        time_000_000_000_005 = 31,
        #[rasn(identifier = "time-000-000-000-002")]
        time_000_000_000_002 = 32,
        #[rasn(identifier = "time-000-000-000-001")]
        time_000_000_000_001 = 33,
        #[rasn(identifier = "time-000-000-000-000-5")]
        time_000_000_000_000_5 = 34,
        #[rasn(identifier = "time-000-000-000-000-2")]
        time_000_000_000_000_2 = 35,
        #[rasn(identifier = "time-000-000-000-000-1")]
        time_000_000_000_000_1 = 36,
        #[rasn(identifier = "time-000-000-000-000-05")]
        time_000_000_000_000_05 = 37,
        #[rasn(identifier = "time-000-000-000-000-02")]
        time_000_000_000_000_02 = 38,
        #[rasn(identifier = "time-000-000-000-000-01")]
        time_000_000_000_000_01 = 39,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct TimeIntervalConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=36001"))]
    pub struct TimeMark(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1500..=1500"))]
    pub struct TimeOfMeasurement(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=60000"))]
    pub struct TimeReference(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=4398046511103"))]
    pub struct TimestampIts(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TractorCharacteristics {
        #[rasn(size("1..=4", extensible), identifier = "equalTo")]
        pub equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
        #[rasn(size("1..=4", extensible), identifier = "notEqualTo")]
        pub not_equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
        #[rasn(size("1..=4", extensible))]
        pub ranges: Option<SequenceOf<VehicleCharacteristicsRanges>>,
    }
    impl TractorCharacteristics {
        pub fn new(
            equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
            not_equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
            ranges: Option<SequenceOf<VehicleCharacteristicsRanges>>,
        ) -> Self {
            Self {
                equal_to,
                not_equal_to,
                ranges,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TrafficSignPictogram {
        dangerWarning = 0,
        regulatory = 1,
        informative = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TrailerCharacteristics {
        #[rasn(size("1..=4", extensible), identifier = "equalTo")]
        pub equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
        #[rasn(size("1..=4", extensible), identifier = "notEqualTo")]
        pub not_equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
        #[rasn(size("1..=4", extensible))]
        pub ranges: Option<SequenceOf<VehicleCharacteristicsRanges>>,
    }
    impl TrailerCharacteristics {
        pub fn new(
            equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
            not_equal_to: Option<SequenceOf<VehicleCharacteristicsFixValues>>,
            ranges: Option<SequenceOf<VehicleCharacteristicsRanges>>,
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
    #[non_exhaustive]
    pub struct TrailerData {
        #[rasn(identifier = "refPointId")]
        pub ref_point_id: RefPointId,
        #[rasn(identifier = "hitchPointOffset")]
        pub hitch_point_offset: HitchPointOffset,
        #[rasn(identifier = "frontOverhang")]
        pub front_overhang: FrontOverhang,
        #[rasn(identifier = "rearOverhang")]
        pub rear_overhang: RearOverhang,
        #[rasn(identifier = "trailerWidth")]
        pub trailer_width: Option<VehicleWidth>,
        #[rasn(identifier = "hitchAngle")]
        pub hitch_angle: Option<CartesianAngle>,
    }
    impl TrailerData {
        pub fn new(
            ref_point_id: RefPointId,
            hitch_point_offset: HitchPointOffset,
            front_overhang: FrontOverhang,
            rear_overhang: RearOverhang,
            trailer_width: Option<VehicleWidth>,
            hitch_angle: Option<CartesianAngle>,
        ) -> Self {
            Self {
                ref_point_id,
                hitch_point_offset,
                front_overhang,
                rear_overhang,
                trailer_width,
                hitch_angle,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=2"))]
    pub struct TrailerDataContainer(pub SequenceOf<TrailerData>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TrainCharacteristics(pub TractorCharacteristics);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TransitVehicleOccupancy {
        occupancyUnknown = 0,
        occupancyEmpty = 1,
        occupancyVeryLow = 2,
        occupancyLow = 3,
        occupancyMed = 4,
        occupancyHigh = 5,
        occupancyNearlyFull = 6,
        occupancyFull = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct TransitVehicleStatus(pub FixedBitString<8usize>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct TransmissionAndSpeed {
        pub transmisson: TransmissionState,
        pub speed: Velocity,
    }
    impl TransmissionAndSpeed {
        pub fn new(transmisson: TransmissionState, speed: Velocity) -> Self {
            Self { transmisson, speed }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum TransmissionState {
        neutral = 0,
        park = 1,
        forwardGears = 2,
        reverseGears = 3,
        reserved1 = 4,
        reserved2 = 5,
        reserved3 = 6,
        unavailable = 7,
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
    pub struct VED {
        pub hei: Option<Distance>,
        pub wid: Option<Distance>,
        pub vln: Option<Distance>,
        pub wei: Option<Weight>,
    }
    impl VED {
        pub fn new(
            hei: Option<Distance>,
            wid: Option<Distance>,
            vln: Option<Distance>,
            wei: Option<Weight>,
        ) -> Self {
            Self { hei, wid, vln, wei }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=86400"))]
    pub struct ValidityDuration(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum VarLengthNumber {
        #[rasn(value("0..=127"))]
        content(u8),
        extension(Ext1),
    }
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
        #[rasn(size("1..=8", extensible))]
        pub validity: Option<SequenceOf<DTM>>,
        #[rasn(value("0..=65535"))]
        pub value: Option<u16>,
        pub unit: Option<RSCUnit>,
    }
    impl VcCode {
        pub fn new(
            road_sign_class: VcClass,
            road_sign_code: u8,
            vc_option: VcOption,
            validity: Option<SequenceOf<DTM>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VehicleCharacteristicsRangeLimits {
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
        pub limits: VehicleCharacteristicsRangeLimits,
    }
    impl VehicleCharacteristicsRanges {
        pub fn new(
            comparison_operator: ComparisonOperator,
            limits: VehicleCharacteristicsRangeLimits,
        ) -> Self {
            Self {
                comparison_operator,
                limits,
            }
        }
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
    #[rasn(delegate, value("0..=127"))]
    pub struct VehicleHeight(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum VehicleID {
        entityID(TemporaryID),
        stationID(StationID),
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
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleSensor {
        #[rasn(
            default = "vehicle_sensor_ref_point_id_default",
            identifier = "refPointId"
        )]
        pub ref_point_id: RefPointId,
        #[rasn(identifier = "xSensorOffset")]
        pub x_sensor_offset: XSensorOffset,
        #[rasn(identifier = "ySensorOffset")]
        pub y_sensor_offset: YSensorOffset,
        #[rasn(identifier = "zSensorOffset")]
        pub z_sensor_offset: Option<ZSensorOffset>,
        #[rasn(identifier = "vehicleSensorPropertyList")]
        pub vehicle_sensor_property_list: VehicleSensorPropertyList,
    }
    impl VehicleSensor {
        pub fn new(
            ref_point_id: RefPointId,
            x_sensor_offset: XSensorOffset,
            y_sensor_offset: YSensorOffset,
            z_sensor_offset: Option<ZSensorOffset>,
            vehicle_sensor_property_list: VehicleSensorPropertyList,
        ) -> Self {
            Self {
                ref_point_id,
                x_sensor_offset,
                y_sensor_offset,
                z_sensor_offset,
                vehicle_sensor_property_list,
            }
        }
    }
    fn vehicle_sensor_ref_point_id_default() -> RefPointId {
        RefPointId(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleSensorProperties {
        pub range: Range,
        #[rasn(identifier = "horizontalOpeningAngleStart")]
        pub horizontal_opening_angle_start: CartesianAngleValue,
        #[rasn(identifier = "horizontalOpeningAngleEnd")]
        pub horizontal_opening_angle_end: CartesianAngleValue,
        #[rasn(identifier = "verticalOpeningAngleStart")]
        pub vertical_opening_angle_start: Option<CartesianAngleValue>,
        #[rasn(identifier = "verticalOpeningAngleEnd")]
        pub vertical_opening_angle_end: Option<CartesianAngleValue>,
    }
    impl VehicleSensorProperties {
        pub fn new(
            range: Range,
            horizontal_opening_angle_start: CartesianAngleValue,
            horizontal_opening_angle_end: CartesianAngleValue,
            vertical_opening_angle_start: Option<CartesianAngleValue>,
            vertical_opening_angle_end: Option<CartesianAngleValue>,
        ) -> Self {
            Self {
                range,
                horizontal_opening_angle_start,
                horizontal_opening_angle_end,
                vertical_opening_angle_start,
                vertical_opening_angle_end,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=10"))]
    pub struct VehicleSensorPropertyList(pub SequenceOf<VehicleSensorProperties>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct VehicleSubclass {
        #[rasn(default = "vehicle_subclass_r_type_default", identifier = "type")]
        pub r_type: VehicleSubclassType,
        #[rasn(default = "vehicle_subclass_confidence_default")]
        pub confidence: ClassConfidence,
    }
    impl VehicleSubclass {
        pub fn new(r_type: VehicleSubclassType, confidence: ClassConfidence) -> Self {
            Self { r_type, confidence }
        }
    }
    impl std::default::Default for VehicleSubclass {
        fn default() -> Self {
            Self {
                r_type: vehicle_subclass_r_type_default(),
                confidence: vehicle_subclass_confidence_default(),
            }
        }
    }
    fn vehicle_subclass_r_type_default() -> VehicleSubclassType {
        VehicleSubclassType(0)
    }
    fn vehicle_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct VehicleSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum VehicleType {
        none = 0,
        unknown = 1,
        special = 2,
        moto = 3,
        car = 4,
        carOther = 5,
        bus = 6,
        axleCnt2 = 7,
        axleCnt3 = 8,
        axleCnt4 = 9,
        axleCnt4Trailer = 10,
        axleCnt5Trailer = 11,
        axleCnt6Trailer = 12,
        axleCnt5MultiTrailer = 13,
        axleCnt6MultiTrailer = 14,
        axleCnt7MultiTrailer = 15,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=62"))]
    pub struct VehicleWidth(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=8191"))]
    pub struct Velocity(pub u16);
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
    #[rasn(automatic_tags)]
    pub struct WGS84Angle {
        pub value: WGS84AngleValue,
        pub confidence: AngleConfidence,
    }
    impl WGS84Angle {
        pub fn new(value: WGS84AngleValue, confidence: AngleConfidence) -> Self {
            Self { value, confidence }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct WGS84AngleValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct WaitOnStopline(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct Weight {
        #[rasn(value("1..=16384"))]
        pub value: u16,
        pub unit: RSCUnit,
    }
    impl Weight {
        pub fn new(value: u16, unit: RSCUnit) -> Self {
            Self { value, unit }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-5000..=0"))]
    pub struct XSensorOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1000..=1000"))]
    pub struct YSensorOffset(pub i16);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1000"))]
    pub struct ZSensorOffset(pub u16);
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
    #[rasn(delegate, value("0..=10000"))]
    pub struct ZoneLength(pub u16);
    pub const ADD_GRP_A: RegionId = RegionId(1);
    pub const ADD_GRP_B: RegionId = RegionId(2);
    pub const ADD_GRP_C: RegionId = RegionId(3);
    pub const DIESEL: FuelType = FuelType(3);
    pub const ELECTRIC: FuelType = FuelType(4);
    pub const ETHANOL: FuelType = FuelType(2);
    pub const GASOLINE: FuelType = FuelType(1);
    pub const HYBRID: FuelType = FuelType(5);
    pub const HYDROGEN: FuelType = FuelType(6);
    pub const NAT_GAS_COMP: FuelType = FuelType(8);
    pub const NAT_GAS_LIQUID: FuelType = FuelType(7);
    pub const NO_REGION: RegionId = RegionId(0);
    pub const PROPANE: FuelType = FuelType(9);
    pub const UNKNOWN_FUEL: FuelType = FuelType(0);
}
