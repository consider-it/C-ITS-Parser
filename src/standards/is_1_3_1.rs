#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]
    extern crate alloc;
    use core::borrow::Borrow;
    use lazy_static::lazy_static;
    use rasn::prelude::*;
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ACKResponseService {
        #[rasn(value("-32768..=32767"), identifier = "ackRespDelayAdjust")]
        pub ack_resp_delay_adjust: i16,
        #[rasn(value("0..=65535"), identifier = "ackRespDelayStdDev")]
        pub ack_resp_delay_std_dev: u16,
    }
    impl ACKResponseService {
        pub fn new(ack_resp_delay_adjust: i16, ack_resp_delay_std_dev: u16) -> Self {
            Self {
                ack_resp_delay_adjust,
                ack_resp_delay_std_dev,
            }
        }
    }
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct AbsolutePositionsWithAltitude(pub SequenceOf<AbsolutePositionWAltitude>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum AccelOrDecel {
        accelerate = 0,
        decelerate = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct AccelerationChangeIndication {
        #[rasn(identifier = "accelOrDecel")]
        pub accel_or_decel: AccelOrDecel,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: ActionDeltaTime,
    }
    impl AccelerationChangeIndication {
        pub fn new(accel_or_decel: AccelOrDecel, action_delta_time: ActionDeltaTime) -> Self {
            Self {
                accel_or_decel,
                action_delta_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct AccelerationConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("7"))]
    pub struct AccelerationControl(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AccidentSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum AckNackIndication {
        aCK = 0,
        nACK = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct ActionDeltaTime(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ActionID {
        #[rasn(identifier = "originatingStationID")]
        pub originating_station_i_d: StationID,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: SequenceNumber,
    }
    impl ActionID {
        pub fn new(originating_station_i_d: StationID, sequence_number: SequenceNumber) -> Self {
            Self {
                originating_station_i_d,
                sequence_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "AdverseWeatherCondition-AdhesionSubCauseCode"
    )]
    pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "AdverseWeatherCondition-ExtremeWeatherConditionSubCauseCode"
    )]
    pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "AdverseWeatherCondition-PrecipitationSubCauseCode"
    )]
    pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "AdverseWeatherCondition-VisibilitySubCauseCode"
    )]
    pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct AdvertiserIdentifier(pub Utf8String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct AdvertiserPermissions(pub SequenceOf<ChannelIdentifier>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct AdvisorySpeedRegional(pub SequenceOf<AnonymousAdvisorySpeedRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AdvisorySpeed {
        #[rasn(identifier = "type")]
        pub r_type: AdvisorySpeedType,
        pub speed: Option<SpeedAdvice>,
        pub confidence: Option<SAESpeedConfidence>,
        pub distance: Option<ZoneLength>,
        pub class: Option<RestrictionClassID>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<AdvisorySpeedRegional>>,
    }
    impl AdvisorySpeed {
        pub fn new(
            r_type: AdvisorySpeedType,
            speed: Option<SpeedAdvice>,
            confidence: Option<SAESpeedConfidence>,
            distance: Option<ZoneLength>,
            class: Option<RestrictionClassID>,
            regional: Option<SequenceOf<AdvisorySpeedRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct AdvisorySpeedList(pub SequenceOf<AdvisorySpeed>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AdvisorySpeedType {
        none = 0,
        greenwave = 1,
        ecoDrive = 2,
        transit = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct AlacarteContainer {
        #[rasn(identifier = "lanePosition")]
        pub lane_position: Option<LanePosition>,
        #[rasn(identifier = "impactReduction")]
        pub impact_reduction: Option<ImpactReductionContainer>,
        #[rasn(identifier = "externalTemperature")]
        pub external_temperature: Option<Temperature>,
        #[rasn(identifier = "roadWorks")]
        pub road_works: Option<RoadWorksContainerExtended>,
        #[rasn(identifier = "positioningSolution")]
        pub positioning_solution: Option<PositioningSolutionType>,
        #[rasn(identifier = "stationaryVehicle")]
        pub stationary_vehicle: Option<StationaryVehicleContainer>,
        #[rasn(extension_addition, identifier = "preCrashContainer")]
        pub pre_crash_container: Option<PreCrashContainer>,
    }
    impl AlacarteContainer {
        pub fn new(
            lane_position: Option<LanePosition>,
            impact_reduction: Option<ImpactReductionContainer>,
            external_temperature: Option<Temperature>,
            road_works: Option<RoadWorksContainerExtended>,
            positioning_solution: Option<PositioningSolutionType>,
            stationary_vehicle: Option<StationaryVehicleContainer>,
            pre_crash_container: Option<PreCrashContainer>,
        ) -> Self {
            Self {
                lane_position,
                impact_reduction,
                external_temperature,
                road_works,
                positioning_solution,
                stationary_vehicle,
                pre_crash_container,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("12"))]
    pub struct AllowedManeuvers(pub BitString);
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllsamContexts_ContextInfo {
        CCtxTypeSystemNull(NullCtx),
        CCtxTypeSystemMandApp(MandAppCtx),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum AltitudeConfidence {
        alt_000_01 = 0,
        alt_000_02 = 1,
        alt_000_05 = 2,
        alt_000_10 = 3,
        alt_000_20 = 4,
        alt_000_50 = 5,
        alt_001_00 = 6,
        alt_002_00 = 7,
        alt_005_00 = 8,
        alt_010_00 = 9,
        alt_020_00 = 10,
        alt_050_00 = 11,
        alt_100_00 = 12,
        alt_200_00 = 13,
        outOfRange = 14,
        unavailable = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-100000..=800001"))]
    pub struct AltitudeValue(pub i32);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum AmbientOrRoadConditionPictogram {
        ambientCondition = 0,
        roadCondition = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct Angle(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct AngleConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    fn animal_subclass_r_type_default() -> AnimalSubclassType {
        AnimalSubclassType(0)
    }
    fn animal_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct AnimalSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct ApproachID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=127", extensible), identifier = "INTEGER")]
    pub struct AnonymousAttributeIdList(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=127", extensible))]
    pub struct AttributeIdList(pub SequenceOf<AnonymousAttributeIdList>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum BasePublicEncryptionKey {
        eciesNistP256(EccP256CurvePoint),
        eciesBrainpoolP256r1(EccP256CurvePoint),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct BasicContainer {
        #[rasn(identifier = "stationType")]
        pub station_type: StationType,
        #[rasn(identifier = "referencePosition")]
        pub reference_position: ReferencePosition,
    }
    impl BasicContainer {
        pub fn new(station_type: StationType, reference_position: ReferencePosition) -> Self {
            Self {
                station_type,
                reference_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct BasicVehicleContainerHighFrequency {
        pub heading: Heading,
        pub speed: Speed,
        #[rasn(identifier = "driveDirection")]
        pub drive_direction: DriveDirection,
        #[rasn(identifier = "vehicleLength")]
        pub vehicle_length: VehicleLength,
        #[rasn(identifier = "vehicleWidth")]
        pub vehicle_width: VehicleWidth,
        #[rasn(identifier = "longitudinalAcceleration")]
        pub longitudinal_acceleration: LongitudinalAcceleration,
        pub curvature: Curvature,
        #[rasn(identifier = "curvatureCalculationMode")]
        pub curvature_calculation_mode: CurvatureCalculationMode,
        #[rasn(identifier = "yawRate")]
        pub yaw_rate: YawRate,
        #[rasn(identifier = "accelerationControl")]
        pub acceleration_control: Option<AccelerationControl>,
        #[rasn(identifier = "lanePosition")]
        pub lane_position: Option<LanePosition>,
        #[rasn(identifier = "steeringWheelAngle")]
        pub steering_wheel_angle: Option<SteeringWheelAngle>,
        #[rasn(identifier = "lateralAcceleration")]
        pub lateral_acceleration: Option<LateralAcceleration>,
        #[rasn(identifier = "verticalAcceleration")]
        pub vertical_acceleration: Option<VerticalAcceleration>,
        #[rasn(identifier = "performanceClass")]
        pub performance_class: Option<PerformanceClass>,
        #[rasn(identifier = "cenDsrcTollingZone")]
        pub cen_dsrc_tolling_zone: Option<CenDsrcTollingZone>,
    }
    impl BasicVehicleContainerHighFrequency {
        pub fn new(
            heading: Heading,
            speed: Speed,
            drive_direction: DriveDirection,
            vehicle_length: VehicleLength,
            vehicle_width: VehicleWidth,
            longitudinal_acceleration: LongitudinalAcceleration,
            curvature: Curvature,
            curvature_calculation_mode: CurvatureCalculationMode,
            yaw_rate: YawRate,
            acceleration_control: Option<AccelerationControl>,
            lane_position: Option<LanePosition>,
            steering_wheel_angle: Option<SteeringWheelAngle>,
            lateral_acceleration: Option<LateralAcceleration>,
            vertical_acceleration: Option<VerticalAcceleration>,
            performance_class: Option<PerformanceClass>,
            cen_dsrc_tolling_zone: Option<CenDsrcTollingZone>,
        ) -> Self {
            Self {
                heading,
                speed,
                drive_direction,
                vehicle_length,
                vehicle_width,
                longitudinal_acceleration,
                curvature,
                curvature_calculation_mode,
                yaw_rate,
                acceleration_control,
                lane_position,
                steering_wheel_angle,
                lateral_acceleration,
                vertical_acceleration,
                performance_class,
                cen_dsrc_tolling_zone,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct BasicVehicleContainerLowFrequency {
        #[rasn(identifier = "vehicleRole")]
        pub vehicle_role: VehicleRole,
        #[rasn(identifier = "exteriorLights")]
        pub exterior_lights: ExteriorLights,
        #[rasn(identifier = "pathHistory")]
        pub path_history: PathHistory,
    }
    impl BasicVehicleContainerLowFrequency {
        pub fn new(
            vehicle_role: VehicleRole,
            exterior_lights: ExteriorLights,
            path_history: PathHistory,
        ) -> Self {
            Self {
                vehicle_role,
                exterior_lights,
                path_history,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum BatteryStatus {
        unknown = 0,
        critical = 1,
        low = 2,
        good = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=31"))]
    pub struct BitmapSsp(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct BitmapSspRange {
        #[rasn(size("1..=32"), identifier = "sspValue")]
        pub ssp_value: OctetString,
        #[rasn(size("1..=32"), identifier = "sspBitmask")]
        pub ssp_bitmask: OctetString,
    }
    impl BitmapSspRange {
        pub fn new(ssp_value: OctetString, ssp_bitmask: OctetString) -> Self {
            Self {
                ssp_value,
                ssp_bitmask,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct CAM {
        pub header: ItsPduHeader,
        pub cam: CoopAwareness,
    }
    impl CAM {
        pub fn new(header: ItsPduHeader, cam: CoopAwareness) -> Self {
            Self { header, cam }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CamParameters {
        #[rasn(identifier = "basicContainer")]
        pub basic_container: BasicContainer,
        #[rasn(identifier = "highFrequencyContainer")]
        pub high_frequency_container: HighFrequencyContainer,
        #[rasn(identifier = "lowFrequencyContainer")]
        pub low_frequency_container: Option<LowFrequencyContainer>,
        #[rasn(identifier = "specialVehicleContainer")]
        pub special_vehicle_container: Option<SpecialVehicleContainer>,
    }
    impl CamParameters {
        pub fn new(
            basic_container: BasicContainer,
            high_frequency_container: HighFrequencyContainer,
            low_frequency_container: Option<LowFrequencyContainer>,
            special_vehicle_container: Option<SpecialVehicleContainer>,
        ) -> Self {
            Self {
                basic_container,
                high_frequency_container,
                low_frequency_container,
                special_vehicle_container,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct CartesianAngleValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CauseCodeType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CenDsrcTollingZone {
        #[rasn(identifier = "protectedZoneLatitude")]
        pub protected_zone_latitude: Latitude,
        #[rasn(identifier = "protectedZoneLongitude")]
        pub protected_zone_longitude: Longitude,
        #[rasn(identifier = "cenDsrcTollingZoneID")]
        pub cen_dsrc_tolling_zone_i_d: Option<CenDsrcTollingZoneID>,
    }
    impl CenDsrcTollingZone {
        pub fn new(
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            cen_dsrc_tolling_zone_i_d: Option<CenDsrcTollingZoneID>,
        ) -> Self {
            Self {
                protected_zone_latitude,
                protected_zone_longitude,
                cen_dsrc_tolling_zone_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct CenDsrcTollingZoneID(pub ProtectedZoneID);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ChInfoOptions {
        pub option1: Option<()>,
        pub option2: Option<()>,
        pub option3: Option<()>,
        pub option4: Option<()>,
        pub option5: Option<()>,
        pub option6: Option<()>,
        pub option7: Option<()>,
        pub extensions: Option<ChannelInfoExts>,
    }
    impl ChInfoOptions {
        pub fn new(
            option1: Option<()>,
            option2: Option<()>,
            option3: Option<()>,
            option4: Option<()>,
            option5: Option<()>,
            option6: Option<()>,
            option7: Option<()>,
            extensions: Option<ChannelInfoExts>,
        ) -> Self {
            Self {
                option1,
                option2,
                option3,
                option4,
                option5,
                option6,
                option7,
                extensions,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ChInfoTypes_Type {
        ChInfoTypeUnknown(()),
        ChInfoTypeAny(()),
        ChInfoTypeM5(ChannelInfo),
    }
    impl ChInfoTypes_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &MedType,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &CH_INFO_TYPE_UNKNOWN => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::ChInfoTypeUnknown)?),
                i if i == &CH_INFO_TYPE_ANY => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::ChInfoTypeAny)?),
                i if i == &CH_INFO_TYPE__M5 => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::ChInfoTypeM5)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &MedType,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::ChInfoTypeUnknown(inner), i) if i == &CH_INFO_TYPE_UNKNOWN => {
                    inner.encode(encoder)
                }
                (Self::ChInfoTypeAny(inner), i) if i == &CH_INFO_TYPE_ANY => inner.encode(encoder),
                (Self::ChInfoTypeM5(inner), i) if i == &CH_INFO_TYPE__M5 => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ChannelAccess80211(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ChannelIdentifier {
        #[rasn(size("3"), identifier = "countryString")]
        pub country_string: OctetString,
        #[rasn(identifier = "operatingClass")]
        pub operating_class: Uint8,
        #[rasn(identifier = "channelNumber")]
        pub channel_number: Uint8,
    }
    impl ChannelIdentifier {
        pub fn new(
            country_string: OctetString,
            operating_class: Uint8,
            channel_number: Uint8,
        ) -> Self {
            Self {
                country_string,
                operating_class,
                channel_number,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct ChannelIndex(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ChannelInfo {
        #[rasn(identifier = "operatingClass")]
        pub operating_class: OperatingClass80211,
        #[rasn(identifier = "channelNumber")]
        pub channel_number: ChannelNumber80211,
        #[rasn(identifier = "powerLevel")]
        pub power_level: TXpower80211,
        #[rasn(identifier = "dataRate")]
        pub data_rate: WsaChInfoDataRate,
        pub extensions: ChInfoOptions,
    }
    impl ChannelInfo {
        pub fn new(
            operating_class: OperatingClass80211,
            channel_number: ChannelNumber80211,
            power_level: TXpower80211,
            data_rate: WsaChInfoDataRate,
            extensions: ChInfoOptions,
        ) -> Self {
            Self {
                operating_class,
                channel_number,
                power_level,
                data_rate,
                extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ChannelInfoExt {
        #[rasn(value("0..=255"), identifier = "extensionId")]
        pub extension_id: u8,
        pub value: Any,
    }
    impl ChannelInfoExt {
        pub fn new(extension_id: u8, value: Any) -> Self {
            Self {
                extension_id,
                value,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ChannelInfoExtTypes_ExtValue {
        CEDCAparameterSet(EdcaParameterSet),
        CChannelAccess(ChannelAccess80211),
    }
    impl ChannelInfoExtTypes_ExtValue {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &C__E_D_C_APARAMETER_SET => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CEDCAparameterSet)?),
                i if i == &C__CHANNEL_ACCESS => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CChannelAccess)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::CEDCAparameterSet(inner), i) if i == &C__E_D_C_APARAMETER_SET => {
                    inner.encode(encoder)
                }
                (Self::CChannelAccess(inner), i) if i == &C__CHANNEL_ACCESS => {
                    inner.encode(encoder)
                }
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ChannelInfoExts(pub SequenceOf<ChannelInfoExt>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ChannelInfos(pub SequenceOf<ChannelInfo>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ChannelNumber80211(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ChannelOptions {
        #[rasn(identifier = "systemService")]
        pub system_service: Option<SystemService>,
        #[rasn(identifier = "serviceProviderPort")]
        pub service_provider_port: Option<ReplyAddress>,
        pub extensions: Option<ServiceInfoExts>,
    }
    impl ChannelOptions {
        pub fn new(
            system_service: Option<SystemService>,
            service_provider_port: Option<ReplyAddress>,
            extensions: Option<ServiceInfoExts>,
        ) -> Self {
            Self {
                system_service,
                service_provider_port,
                extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ChannelSpecificProviderPermission {
        #[rasn(identifier = "channelId")]
        pub channel_id: ChannelIdentifier,
        #[rasn(identifier = "permittedPsids")]
        pub permitted_psids: Option<SequenceOfPsid>,
        #[rasn(identifier = "permittedEdcaParameters")]
        pub permitted_edca_parameters: Option<SequenceOfEdcaIdentifier>,
        #[rasn(identifier = "maximumTransmitPower")]
        pub maximum_transmit_power: Option<Uint8>,
    }
    impl ChannelSpecificProviderPermission {
        pub fn new(
            channel_id: ChannelIdentifier,
            permitted_psids: Option<SequenceOfPsid>,
            permitted_edca_parameters: Option<SequenceOfEdcaIdentifier>,
            maximum_transmit_power: Option<Uint8>,
        ) -> Self {
            Self {
                channel_id,
                permitted_psids,
                permitted_edca_parameters,
                maximum_transmit_power,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ChargingSpotType(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct CircularRegion {
        pub center: Dot2TwoDLocation,
        pub radius: Uint16,
    }
    impl CircularRegion {
        pub fn new(center: Dot2TwoDLocation, radius: Uint16) -> Self {
            Self { center, radius }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct ClassConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ClusterBoundingBoxShape {
        clusterRectangle(AreaRectangle),
        clusterCircle(AreaCircular),
        clusterPolygon(AreaPolygon),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterBreakupInfo {
        #[rasn(identifier = "clusterBreakupReason")]
        pub cluster_breakup_reason: ClusterBreakupReason,
        #[rasn(identifier = "breakupTime")]
        pub breakup_time: VruClusterOpTimestamp,
    }
    impl ClusterBreakupInfo {
        pub fn new(
            cluster_breakup_reason: ClusterBreakupReason,
            breakup_time: VruClusterOpTimestamp,
        ) -> Self {
            Self {
                cluster_breakup_reason,
                breakup_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum ClusterBreakupReason {
        notProvided = 0,
        clusteringPurposeCompleted = 1,
        leaderMovedOutOfClusterBoundingBox = 2,
        joiningAnotherCluster = 3,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct ClusterId(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterJoinInfo {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: ClusterId,
        #[rasn(identifier = "joinTime")]
        pub join_time: VruClusterOpTimestamp,
    }
    impl ClusterJoinInfo {
        pub fn new(cluster_id: ClusterId, join_time: VruClusterOpTimestamp) -> Self {
            Self {
                cluster_id,
                join_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ClusterLeaveInfo {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: ClusterId,
        #[rasn(identifier = "clusterLeaveReason")]
        pub cluster_leave_reason: ClusterLeaveReason,
    }
    impl ClusterLeaveInfo {
        pub fn new(cluster_id: ClusterId, cluster_leave_reason: ClusterLeaveReason) -> Self {
            Self {
                cluster_id,
                cluster_leave_reason,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum ClusterLeaveReason {
        notProvided = 0,
        clusterLeaderLost = 1,
        clusterDisbandedByLeader = 2,
        outOfClusterBoundingBox = 3,
        outOfClusterSpeedRange = 4,
        joiningAnotherCluster = 5,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("4"))]
    pub struct ClusterProfiles(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Code {
        viennaConvention(VcCode),
        iso14823(ISO14823Code),
        #[rasn(value("0..=65535"))]
        itisCodes(u16),
        anyCatalogue(AnyCatalogue),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CollisionRiskSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct ComparisonOperator(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ComputedLaneRegional(pub SequenceOf<AnonymousComputedLaneRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        pub rotate_x_y: Option<Angle>,
        #[rasn(identifier = "scaleXaxis")]
        pub scale_xaxis: Option<ScaleB12>,
        #[rasn(identifier = "scaleYaxis")]
        pub scale_yaxis: Option<ScaleB12>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<ComputedLaneRegional>>,
    }
    impl ComputedLane {
        pub fn new(
            reference_lane_id: LaneID,
            offset_xaxis: OffsetXaxis,
            offset_yaxis: OffsetYaxis,
            rotate_x_y: Option<Angle>,
            scale_xaxis: Option<ScaleB12>,
            scale_yaxis: Option<ScaleB12>,
            regional: Option<SequenceOf<ComputedLaneRegional>>,
        ) -> Self {
            Self {
                reference_lane_id,
                offset_xaxis,
                offset_yaxis,
                rotate_x_y,
                scale_xaxis,
                scale_yaxis,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        pub connection_i_d: Option<LaneConnectionID>,
    }
    impl Connection {
        pub fn new(
            connecting_lane: ConnectingLane,
            remote_intersection: Option<IntersectionReferenceID>,
            signal_group: Option<SignalGroupID>,
            user_class: Option<RestrictionClassID>,
            connection_i_d: Option<LaneConnectionID>,
        ) -> Self {
            Self {
                connecting_lane,
                remote_intersection,
                signal_group,
                user_class,
                connection_i_d,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ConnectionManeuverAssistRegional(
        pub SequenceOf<AnonymousConnectionManeuverAssistRegional>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ConnectionManeuverAssist {
        #[rasn(identifier = "connectionID")]
        pub connection_i_d: LaneConnectionID,
        #[rasn(identifier = "queueLength")]
        pub queue_length: Option<ZoneLength>,
        #[rasn(identifier = "availableStorageLength")]
        pub available_storage_length: Option<ZoneLength>,
        #[rasn(identifier = "waitOnStop")]
        pub wait_on_stop: Option<WaitOnStopline>,
        #[rasn(identifier = "pedBicycleDetect")]
        pub ped_bicycle_detect: Option<PedestrianBicycleDetect>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<ConnectionManeuverAssistRegional>>,
    }
    impl ConnectionManeuverAssist {
        pub fn new(
            connection_i_d: LaneConnectionID,
            queue_length: Option<ZoneLength>,
            available_storage_length: Option<ZoneLength>,
            wait_on_stop: Option<WaitOnStopline>,
            ped_bicycle_detect: Option<PedestrianBicycleDetect>,
            regional: Option<SequenceOf<ConnectionManeuverAssistRegional>>,
        ) -> Self {
            Self {
                connection_i_d,
                queue_length,
                available_storage_length,
                wait_on_stop,
                ped_bicycle_detect,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "ConnectionTrajectory-addGrpC")]
    #[non_exhaustive]
    pub struct ConnectionTrajectoryAddGrpC {
        pub nodes: NodeSetXY,
        #[rasn(identifier = "connectionID")]
        pub connection_i_d: LaneConnectionID,
    }
    impl ConnectionTrajectoryAddGrpC {
        pub fn new(nodes: NodeSetXY, connection_i_d: LaneConnectionID) -> Self {
            Self {
                nodes,
                connection_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ConnectsToList(pub SequenceOf<Connection>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct CoopAwareness {
        #[rasn(identifier = "generationDeltaTime")]
        pub generation_delta_time: GenerationDeltaTime,
        #[rasn(identifier = "camParameters")]
        pub cam_parameters: CamParameters,
    }
    impl CoopAwareness {
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
            cam_parameters: CamParameters,
        ) -> Self {
            Self {
                generation_delta_time,
                cam_parameters,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct CountryAndRegions {
        #[rasn(identifier = "countryOnly")]
        pub country_only: CountryOnly,
        pub regions: SequenceOfUint8,
    }
    impl CountryAndRegions {
        pub fn new(country_only: CountryOnly, regions: SequenceOfUint8) -> Self {
            Self {
                country_only,
                regions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct CountryAndSubregions {
        pub country: CountryOnly,
        #[rasn(identifier = "regionAndSubregions")]
        pub region_and_subregions: SequenceOfRegionAndSubregions,
    }
    impl CountryAndSubregions {
        pub fn new(
            country: CountryOnly,
            region_and_subregions: SequenceOfRegionAndSubregions,
        ) -> Self {
            Self {
                country,
                region_and_subregions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("10"))]
    pub struct CountryCode(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct CountryOnly(pub Uint16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct CrlSeries(pub Uint16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct CtxRef(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum CurvatureCalculationMode {
        yawRateUsed = 0,
        yawRateNotUsed = 1,
        unavailable = 2,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum CurvatureConfidence {
        onePerMeter_0_00002 = 0,
        onePerMeter_0_0001 = 1,
        onePerMeter_0_0005 = 2,
        onePerMeter_0_002 = 3,
        onePerMeter_0_01 = 4,
        onePerMeter_0_1 = 5,
        outOfRange = 6,
        unavailable = 7,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1023..=1023"))]
    pub struct CurvatureValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct DBV(pub Distance);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15", extensible), identifier = "DDD-DEP")]
    pub struct DDDDEP(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15", extensible), identifier = "DDD-DER")]
    pub struct DDDDER(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct DDateTime {
        pub year: Option<DYear>,
        pub month: Option<DMonth>,
        pub day: Option<DDay>,
        pub hour: Option<DHour>,
        pub minute: Option<DMinute>,
        pub second: Option<DSecond>,
        pub offset: Option<DOffset>,
    }
    impl DDateTime {
        pub fn new(
            year: Option<DYear>,
            month: Option<DMonth>,
            day: Option<DDay>,
            hour: Option<DHour>,
            minute: Option<DMinute>,
            second: Option<DSecond>,
            offset: Option<DOffset>,
        ) -> Self {
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                offset,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct DDay(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct DENM {
        pub header: ItsPduHeader,
        pub denm: DecentralizedEnvironmentalNotificationMessage,
    }
    impl DENM {
        pub fn new(
            header: ItsPduHeader,
            denm: DecentralizedEnvironmentalNotificationMessage,
        ) -> Self {
            Self { header, denm }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=8"))]
    pub struct DFL(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct DHour(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=60"))]
    pub struct DMinute(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=12"))]
    pub struct DMonth(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-840..=840"))]
    pub struct DOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct DSRCmsgID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct DSecond(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct DYear(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousEndOfQueueSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct DangerousGoodsContainer {
        #[rasn(identifier = "dangerousGoodsBasic")]
        pub dangerous_goods_basic: DangerousGoodsBasic,
    }
    impl DangerousGoodsContainer {
        pub fn new(dangerous_goods_basic: DangerousGoodsBasic) -> Self {
            Self {
                dangerous_goods_basic,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=24"), identifier = "companyName")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DangerousSituationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct DataRate80211(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct DayOfWeek(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct DecentralizedEnvironmentalNotificationMessage {
        pub management: ManagementContainer,
        pub situation: Option<SituationContainer>,
        pub location: Option<LocationContainer>,
        pub alacarte: Option<AlacarteContainer>,
    }
    impl DecentralizedEnvironmentalNotificationMessage {
        pub fn new(
            management: ManagementContainer,
            situation: Option<SituationContainer>,
            location: Option<LocationContainer>,
            alacarte: Option<AlacarteContainer>,
        ) -> Self {
            Self {
                management,
                situation,
                location,
                alacarte,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-12700..=12800"))]
    pub struct DeltaAltitude(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-150..=150"))]
    pub struct DeltaAngle(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLatitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-131071..=131072"))]
    pub struct DeltaLongitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32", extensible))]
    pub struct DeltaPositionsWithAltitude(pub SequenceOf<DeltaReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-122..=121"))]
    pub struct DeltaTime(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=63"))]
    pub struct DescriptiveName(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct DestinationPlace {
        #[rasn(identifier = "depType")]
        pub dep_type: DDDDEP,
        #[rasn(identifier = "depRSCode")]
        pub dep_r_s_code: Option<ISO14823Code>,
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
            dep_r_s_code: Option<ISO14823Code>,
            dep_blob: Option<OctetString>,
            pln_id: Option<u16>,
            pln_text: Option<Utf8String>,
        ) -> Self {
            Self {
                dep_type,
                dep_r_s_code,
                dep_blob,
                pln_id,
                pln_text,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=256"))]
    pub struct DigitalMap(pub SequenceOf<ReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct Direction(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct DistanceConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-132768..=132767"))]
    pub struct DistanceValue(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Dot2Elevation(pub ElevInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Dot2Latitude(pub NinetyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Dot2Longitude(pub OneEightyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Dot2ThreeDLocation {
        pub latitude: Dot2Latitude,
        pub longitude: Dot2Longitude,
        pub elevation: Dot2Elevation,
    }
    impl Dot2ThreeDLocation {
        pub fn new(
            latitude: Dot2Latitude,
            longitude: Dot2Longitude,
            elevation: Dot2Elevation,
        ) -> Self {
            Self {
                latitude,
                longitude,
                elevation,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Dot2TwoDLocation {
        pub latitude: Dot2Latitude,
        pub longitude: Dot2Longitude,
    }
    impl Dot2TwoDLocation {
        pub fn new(latitude: Dot2Latitude, longitude: Dot2Longitude) -> Self {
            Self {
                latitude,
                longitude,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum DriveDirection {
        forward = 0,
        backward = 1,
        unavailable = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-32767..=32767"))]
    pub struct DrivenLineOffsetLg(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-2047..=2047"))]
    pub struct DrivenLineOffsetSm(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct DriverCharacteristics(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=13"))]
    pub struct DrivingLaneStatus(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum Duration {
        microseconds(Uint16),
        milliseconds(Uint16),
        seconds(Uint16),
        minutes(Uint16),
        hours(Uint16),
        sixtyHours(Uint16),
        years(Uint16),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=2"))]
    pub struct DynamicStatus(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct EDT(pub DTM);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EVChargingSpotNotificationPOIMessage {
        #[rasn(identifier = "poiHeader")]
        pub poi_header: ItsPOIHeader,
        #[rasn(identifier = "evcsnData")]
        pub evcsn_data: ItsEVCSNData,
    }
    impl EVChargingSpotNotificationPOIMessage {
        pub fn new(poi_header: ItsPOIHeader, evcsn_data: ItsEVCSNData) -> Self {
            Self {
                poi_header,
                evcsn_data,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EccP256CurvePointUncompressedP256 {
        #[rasn(size("32"))]
        pub x: OctetString,
        #[rasn(size("32"))]
        pub y: OctetString,
    }
    impl EccP256CurvePointUncompressedP256 {
        pub fn new(x: OctetString, y: OctetString) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum EccP256CurvePoint {
        #[rasn(size("32"), identifier = "x-only")]
        x_only(OctetString),
        fill(()),
        #[rasn(size("32"), identifier = "compressed-y-0")]
        compressed_y_0(OctetString),
        #[rasn(size("32"), identifier = "compressed-y-1")]
        compressed_y_1(OctetString),
        uncompressedP256(EccP256CurvePointUncompressedP256),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EccP384CurvePointUncompressedP384 {
        #[rasn(size("48"))]
        pub x: OctetString,
        #[rasn(size("48"))]
        pub y: OctetString,
    }
    impl EccP384CurvePointUncompressedP384 {
        pub fn new(x: OctetString, y: OctetString) -> Self {
            Self { x, y }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum EccP384CurvePoint {
        #[rasn(size("48"), identifier = "x-only")]
        x_only(OctetString),
        fill(()),
        #[rasn(size("48"), identifier = "compressed-y-0")]
        compressed_y_0(OctetString),
        #[rasn(size("48"), identifier = "compressed-y-1")]
        compressed_y_1(OctetString),
        uncompressedP384(EccP384CurvePointUncompressedP384),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EcdsaP256Signature {
        #[rasn(identifier = "rSig")]
        pub r_sig: EccP256CurvePoint,
        #[rasn(size("32"), identifier = "sSig")]
        pub s_sig: OctetString,
    }
    impl EcdsaP256Signature {
        pub fn new(r_sig: EccP256CurvePoint, s_sig: OctetString) -> Self {
            Self { r_sig, s_sig }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EcdsaP384Signature {
        #[rasn(identifier = "rSig")]
        pub r_sig: EccP384CurvePoint,
        #[rasn(size("48"), identifier = "sSig")]
        pub s_sig: OctetString,
    }
    impl EcdsaP384Signature {
        pub fn new(r_sig: EccP384CurvePoint, s_sig: OctetString) -> Self {
            Self { r_sig, s_sig }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EciesP256EncryptedKey {
        pub v: EccP256CurvePoint,
        #[rasn(size("16"))]
        pub c: OctetString,
        #[rasn(size("16"))]
        pub t: OctetString,
    }
    impl EciesP256EncryptedKey {
        pub fn new(v: EccP256CurvePoint, c: OctetString, t: OctetString) -> Self {
            Self { v, c, t }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum EdcaIdentifier {
        #[rasn(identifier = "enum")]
        R_enum(EnumeratedEdcaIdentifier),
        explicit(ExplicitEdcaIdentifier),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EdcaParameterRecord {
        #[rasn(value("0..=1"))]
        pub res: u8,
        #[rasn(value("0..=3"))]
        pub aci: u8,
        #[rasn(value("0..=1"))]
        pub acm: u8,
        #[rasn(value("0..=15"))]
        pub aifsn: u8,
        #[rasn(value("0..=15"), identifier = "ecwMax")]
        pub ecw_max: u8,
        #[rasn(value("0..=15"), identifier = "ecwMin")]
        pub ecw_min: u8,
        #[rasn(value("0..=65535"), identifier = "txopLimit")]
        pub txop_limit: u16,
    }
    impl EdcaParameterRecord {
        pub fn new(
            res: u8,
            aci: u8,
            acm: u8,
            aifsn: u8,
            ecw_max: u8,
            ecw_min: u8,
            txop_limit: u16,
        ) -> Self {
            Self {
                res,
                aci,
                acm,
                aifsn,
                ecw_max,
                ecw_min,
                txop_limit,
            }
        }
    }
    #[doc = "ChannelInfo extension elements"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EdcaParameterSet {
        #[rasn(identifier = "acbeRecord")]
        pub acbe_record: EdcaParameterRecord,
        #[rasn(identifier = "acbkRecord")]
        pub acbk_record: EdcaParameterRecord,
        #[rasn(identifier = "acviRecord")]
        pub acvi_record: EdcaParameterRecord,
        #[rasn(identifier = "acvoRecord")]
        pub acvo_record: EdcaParameterRecord,
    }
    impl EdcaParameterSet {
        pub fn new(
            acbe_record: EdcaParameterRecord,
            acbk_record: EdcaParameterRecord,
            acvi_record: EdcaParameterRecord,
            acvo_record: EdcaParameterRecord,
        ) -> Self {
            Self {
                acbe_record,
                acbk_record,
                acvi_record,
                acvo_record,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ElevInt(pub Uint16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-4096..=61439"))]
    pub struct Elevation(pub i32);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum ElevationConfidence {
        unavailable = 0,
        elev_500_00 = 1,
        elev_200_00 = 2,
        elev_100_00 = 3,
        elev_050_00 = 4,
        elev_020_00 = 5,
        elev_010_00 = 6,
        elev_005_00 = 7,
        elev_002_00 = 8,
        elev_001_00 = 9,
        elev_000_50 = 10,
        elev_000_20 = 11,
        elev_000_10 = 12,
        elev_000_05 = 13,
        elev_000_02 = 14,
        elev_000_01 = 15,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct EmbarkationStatus(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EmergencyContainer {
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: LightBarSirenInUse,
        #[rasn(identifier = "incidentIndication")]
        pub incident_indication: Option<CauseCode>,
        #[rasn(identifier = "emergencyPriority")]
        pub emergency_priority: Option<EmergencyPriority>,
    }
    impl EmergencyContainer {
        pub fn new(
            light_bar_siren_in_use: LightBarSirenInUse,
            incident_indication: Option<CauseCode>,
            emergency_priority: Option<EmergencyPriority>,
        ) -> Self {
            Self {
                light_bar_siren_in_use,
                incident_indication,
                emergency_priority,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("2"))]
    pub struct EmergencyPriority(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct EnabledLaneList(pub SequenceOf<LaneID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum EncryptionKey {
        public(PublicEncryptionKey),
        symmetric(SymmetricEncryptionKey),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("7"))]
    pub struct EnergyStorageType(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct EngineCharacteristics(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum EnumeratedEdcaIdentifier {
        us_j2945_bsm = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum EuVehicleCategoryCode {
        euVehicleCategoryL(EuVehicleCategoryL),
        euVehicleCategoryM(EuVehicleCategoryM),
        euVehicleCategoryN(EuVehicleCategoryN),
        euVehicleCategoryO(EuVehicleCategoryO),
        euVehilcleCategoryT(()),
        euVehilcleCategoryG(()),
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryM {
        m1 = 0,
        m2 = 1,
        m3 = 2,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryN {
        n1 = 0,
        n2 = 1,
        n3 = 2,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum EuVehicleCategoryO {
        o1 = 0,
        o2 = 1,
        o3 = 2,
        o4 = 3,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum EuroValue {
        noEntry = 0,
        euro_1 = 1,
        euro_2 = 2,
        euro_3 = 3,
        euro_4 = 4,
        euro_5 = 5,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct EvcsnPdu {
        pub header: ItsPduHeader,
        pub evcsn: EVChargingSpotNotificationPOIMessage,
    }
    impl EvcsnPdu {
        pub fn new(header: ItsPduHeader, evcsn: EVChargingSpotNotificationPOIMessage) -> Self {
            Self { header, evcsn }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=23"))]
    pub struct EventHistory(pub SequenceOf<EventPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ExhaustEmissionValues {
        #[rasn(identifier = "unitType")]
        pub unit_type: UnitType,
        #[rasn(value("0..=32767"), identifier = "emissionCO")]
        pub emission_c_o: u16,
        #[rasn(identifier = "emissionHC")]
        pub emission_h_c: Int2,
        #[rasn(identifier = "emissionNOX")]
        pub emission_n_o_x: Int2,
        #[rasn(identifier = "emissionHCNOX")]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ExplicitEdcaIdentifier {
        #[rasn(identifier = "qosInfo")]
        pub qos_info: Uint8,
        pub reserved: Uint8,
        #[rasn(size("4"))]
        pub set1: OctetString,
        #[rasn(size("4"))]
        pub set2: OctetString,
        #[rasn(size("4"))]
        pub set3: OctetString,
        #[rasn(size("4"))]
        pub set4: OctetString,
    }
    impl ExplicitEdcaIdentifier {
        pub fn new(
            qos_info: Uint8,
            reserved: Uint8,
            set1: OctetString,
            set2: OctetString,
            set3: OctetString,
            set4: OctetString,
        ) -> Self {
            Self {
                qos_info,
                reserved,
                set1,
                set2,
                set3,
                set4,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum Ext1 {
        #[rasn(value("128..=16511"))]
        content(u16),
        extension(Ext2),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum Ext2 {
        #[rasn(value("16512..=2113663"))]
        content(u32),
        extension(Ext3),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("2113664..=270549119", extensible))]
    pub struct Ext3(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ExtendedChannelInfo {
        #[rasn(identifier = "medId")]
        pub med_id: MedType,
        pub value: Any,
    }
    impl ExtendedChannelInfo {
        pub fn new(med_id: MedType, value: Any) -> Self {
            Self { med_id, value }
        }
    }
    impl ExtendedChannelInfo {
        pub fn decode_value<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<ChInfoTypes_Type, D::Error> {
            ChInfoTypes_Type::decode(decoder, Some(&self.value), &self.med_id)
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ExtendedChannelInfos(pub SequenceOf<ExtendedChannelInfo>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct ExteriorLights(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct FirstIntersection {
        #[rasn(value("1..=32"), identifier = "numberOfIntersections")]
        pub number_of_intersections: u8,
        #[rasn(identifier = "partialIntersection")]
        pub partial_intersection: PartialIntersection,
    }
    impl FirstIntersection {
        pub fn new(number_of_intersections: u8, partial_intersection: PartialIntersection) -> Self {
            Self {
                number_of_intersections,
                partial_intersection,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct FirstSpat {
        #[rasn(value("1..=32"), identifier = "numberOfIntersections")]
        pub number_of_intersections: u8,
        #[rasn(identifier = "partialSpatIntersection")]
        pub partial_spat_intersection: PartialSpatIntersection,
    }
    impl FirstSpat {
        pub fn new(
            number_of_intersections: u8,
            partial_spat_intersection: PartialSpatIntersection,
        ) -> Self {
            Self {
                number_of_intersections,
                partial_spat_intersection,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct FreeSpaceAddendum {
        #[rasn(identifier = "freeSpaceConfidence")]
        pub free_space_confidence: FreeSpaceConfidence,
        #[rasn(identifier = "freeSpaceArea")]
        pub free_space_area: FreeSpaceArea,
        #[rasn(identifier = "sensorIDList")]
        pub sensor_i_d_list: Option<SensorIdList>,
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
            sensor_i_d_list: Option<SensorIdList>,
            shadowing_applies: ShadowingApplies,
        ) -> Self {
            Self {
                free_space_confidence,
                free_space_area,
                sensor_i_d_list,
                shadowing_applies,
            }
        }
    }
    fn free_space_addendum_shadowing_applies_default() -> ShadowingApplies {
        ShadowingApplies(true)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct FreeSpaceAddendumContainer(pub SequenceOf<FreeSpaceAddendum>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum FreeSpaceArea {
        freeSpacePolygon(AreaPolygon),
        freeSpaceCircular(AreaCircular),
        freeSpaceEllipse(AreaEllipse),
        freeSpaceRectangle(AreaRectangle),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct FreeSpaceConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=50"))]
    pub struct FrontOverhang(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct FuelType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct FullPositionVector {
        #[rasn(identifier = "utcTime")]
        pub utc_time: Option<DDateTime>,
        pub long: Longitude,
        pub lat: Latitude,
        pub elevation: Option<Elevation>,
        pub heading: Option<SAEHeading>,
        pub speed: Option<TransmissionAndSpeed>,
        #[rasn(identifier = "posAccuracy")]
        pub pos_accuracy: Option<PositionalAccuracy>,
        #[rasn(identifier = "timeConfidence")]
        pub time_confidence: Option<TimeConfidence>,
        #[rasn(identifier = "posConfidence")]
        pub pos_confidence: Option<PositionConfidenceSet>,
        #[rasn(identifier = "speedConfidence")]
        pub speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
    }
    impl FullPositionVector {
        pub fn new(
            utc_time: Option<DDateTime>,
            long: Longitude,
            lat: Latitude,
            elevation: Option<Elevation>,
            heading: Option<SAEHeading>,
            speed: Option<TransmissionAndSpeed>,
            pos_accuracy: Option<PositionalAccuracy>,
            time_confidence: Option<TimeConfidence>,
            pos_confidence: Option<PositionConfidenceSet>,
            speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
        ) -> Self {
            Self {
                utc_time,
                long,
                lat,
                elevation,
                heading,
                speed,
                pos_accuracy,
                time_confidence,
                pos_confidence,
                speed_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct GNSSstatus(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct GatewayMacAddress(pub MACaddress);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct GeneralIviContainer(pub SequenceOf<GicPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct GenerationDeltaTime(pub u16);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct GenericLaneRegional(pub SequenceOf<AnonymousGenericLaneRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GenericLane {
        #[rasn(identifier = "laneID")]
        pub lane_i_d: LaneID,
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<GenericLaneRegional>>,
    }
    impl GenericLane {
        pub fn new(
            lane_i_d: LaneID,
            name: Option<DescriptiveName>,
            ingress_approach: Option<ApproachID>,
            egress_approach: Option<ApproachID>,
            lane_attributes: LaneAttributes,
            maneuvers: Option<AllowedManeuvers>,
            node_list: NodeListXY,
            connects_to: Option<ConnectsToList>,
            overlays: Option<OverlayLaneList>,
            regional: Option<SequenceOf<GenericLaneRegional>>,
        ) -> Self {
            Self {
                lane_i_d,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum GeographicRegion {
        circularRegion(CircularRegion),
        rectangularRegion(SequenceOfRectangularRegion),
        polygonalRegion(PolygonalRegion),
        identifiedRegion(SequenceOfIdentifiedRegion),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct GicPart {
        #[rasn(size("1..=8", extensible), identifier = "detectionZoneIds")]
        pub detection_zone_ids: Option<SequenceOf<Zid>>,
        #[rasn(identifier = "its-Rrid")]
        pub its__rrid: Option<VarLengthNumber>,
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
            its__rrid: Option<VarLengthNumber>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15", extensible))]
    pub struct GoodsType(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct GroupLinkageValue {
        #[rasn(size("4"), identifier = "jValue")]
        pub j_value: OctetString,
        #[rasn(size("9"))]
        pub value: OctetString,
    }
    impl GroupLinkageValue {
        pub fn new(j_value: OctetString, value: OctetString) -> Self {
            Self { j_value, value }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum HardShoulderStatus {
        availableForStopping = 0,
        closed = 1,
        availableForDriving = 2,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum HashAlgorithm {
        sha256 = 0,
        #[rasn(extension_addition)]
        sha384 = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("10"))]
    pub struct HashedId10(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("3"))]
    pub struct HashedId3(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct HashedId8(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "HazardousLocation-AnimalOnTheRoadSubCauseCode"
    )]
    pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "HazardousLocation-DangerousCurveSubCauseCode"
    )]
    pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "HazardousLocation-ObstacleOnTheRoadSubCauseCode"
    )]
    pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(
        delegate,
        value("0..=255"),
        identifier = "HazardousLocation-SurfaceConditionSubCauseCode"
    )]
    pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct HeadingChangeIndication {
        pub direction: LeftOrRight,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: ActionDeltaTime,
    }
    impl HeadingChangeIndication {
        pub fn new(direction: LeftOrRight, action_delta_time: ActionDeltaTime) -> Self {
            Self {
                direction,
                action_delta_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct HeadingConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct HeadingValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=100"))]
    pub struct HeightLonCarr(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum HighFrequencyContainer {
        basicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
        rsuContainerHighFrequency(RSUContainerHighFrequency),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=100"))]
    pub struct HitchPointOffset(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=255"))]
    pub struct Hostname(pub Utf8String);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct HumanProblemSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"))]
    pub struct IPv6Address(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ITSRangingSAMAppData {
        #[rasn(value("0..=255"), identifier = "protocolVersion")]
        pub protocol_version: u8,
        #[rasn(identifier = "ackResponseService")]
        pub ack_response_service: ACKResponseService,
        #[rasn(identifier = "groundAltitude")]
        pub ground_altitude: Option<Altitude>,
        #[rasn(identifier = "roadAngles")]
        pub road_angles: Option<RoadAngles>,
    }
    impl ITSRangingSAMAppData {
        pub fn new(
            protocol_version: u8,
            ack_response_service: ACKResponseService,
            ground_altitude: Option<Altitude>,
            road_angles: Option<RoadAngles>,
        ) -> Self {
            Self {
                protocol_version,
                ack_response_service,
                ground_altitude,
                road_angles,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ITSaid(pub VarLengthNumber);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct IVILaneWidth(pub u16);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct IValue(pub Uint16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IdentifiedRegion {
        countryOnly(CountryOnly),
        countryAndRegions(CountryAndRegions),
        countryAndSubregions(CountryAndSubregions),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Identifier(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ImpactReductionContainer {
        #[rasn(identifier = "heightLonCarrLeft")]
        pub height_lon_carr_left: HeightLonCarr,
        #[rasn(identifier = "heightLonCarrRight")]
        pub height_lon_carr_right: HeightLonCarr,
        #[rasn(identifier = "posLonCarrLeft")]
        pub pos_lon_carr_left: PosLonCarr,
        #[rasn(identifier = "posLonCarrRight")]
        pub pos_lon_carr_right: PosLonCarr,
        #[rasn(identifier = "positionOfPillars")]
        pub position_of_pillars: PositionOfPillars,
        #[rasn(identifier = "posCentMass")]
        pub pos_cent_mass: PosCentMass,
        #[rasn(identifier = "wheelBaseVehicle")]
        pub wheel_base_vehicle: WheelBaseVehicle,
        #[rasn(identifier = "turningRadius")]
        pub turning_radius: TurningRadius,
        #[rasn(identifier = "posFrontAx")]
        pub pos_front_ax: PosFrontAx,
        #[rasn(identifier = "positionOfOccupants")]
        pub position_of_occupants: PositionOfOccupants,
        #[rasn(identifier = "vehicleMass")]
        pub vehicle_mass: VehicleMass,
        #[rasn(identifier = "requestResponseIndication")]
        pub request_response_indication: RequestResponseIndication,
    }
    impl ImpactReductionContainer {
        pub fn new(
            height_lon_carr_left: HeightLonCarr,
            height_lon_carr_right: HeightLonCarr,
            pos_lon_carr_left: PosLonCarr,
            pos_lon_carr_right: PosLonCarr,
            position_of_pillars: PositionOfPillars,
            pos_cent_mass: PosCentMass,
            wheel_base_vehicle: WheelBaseVehicle,
            turning_radius: TurningRadius,
            pos_front_ax: PosFrontAx,
            position_of_occupants: PositionOfOccupants,
            vehicle_mass: VehicleMass,
            request_response_indication: RequestResponseIndication,
        ) -> Self {
            Self {
                height_lon_carr_left,
                height_lon_carr_right,
                pos_lon_carr_left,
                pos_lon_carr_right,
                position_of_pillars,
                pos_cent_mass,
                wheel_base_vehicle,
                turning_radius,
                pos_front_ax,
                position_of_occupants,
                vehicle_mass,
                request_response_indication,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum ImpactSection {
        unavailable = 0,
        rear = 1,
        front = 2,
        sideLeftFront = 3,
        sideLeftBack = 4,
        sideRightFront = 5,
        sideRightBack = 6,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct InformationQuality(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Int1(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct Int2(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IntersectionAccessPoint {
        lane(LaneID),
        approach(ApproachID),
        connection(LaneConnectionID),
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionGeometryRegional(pub SequenceOf<AnonymousIntersectionGeometryRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<IntersectionGeometryRegional>>,
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
            regional: Option<SequenceOf<IntersectionGeometryRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);
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
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct IntersectionStateRegional(pub SequenceOf<AnonymousIntersectionStateRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<IntersectionStateRegional>>,
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
            regional: Option<SequenceOf<IntersectionStateRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct IntersectionStateList(pub SequenceOf<IntersectionState>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"))]
    pub struct IntersectionStatusObject(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"))]
    pub struct IpV6Prefix(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct IpV6PrefixLength(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Iso3833VehicleType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct IssuerIdentifier(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=40"))]
    pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ItsAidCtxRef {
        pub itsaid: ITSaid,
        pub ctx: CtxRef,
    }
    impl ItsAidCtxRef {
        pub fn new(itsaid: ITSaid, ctx: CtxRef) -> Self {
            Self { itsaid, ctx }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ItsChargingSpotDataElements {
        #[rasn(identifier = "type")]
        pub r_type: ChargingSpotType,
        #[rasn(identifier = "evEquipmentID")]
        pub ev_equipment_i_d: Option<Utf8String>,
        #[rasn(identifier = "typeOfReceptacle")]
        pub type_of_receptacle: TypeOfReceptacle,
        #[rasn(identifier = "energyAvailability")]
        pub energy_availability: Utf8String,
        #[rasn(identifier = "parkingPlacesData")]
        pub parking_places_data: Option<ParkingPlacesData>,
    }
    impl ItsChargingSpotDataElements {
        pub fn new(
            r_type: ChargingSpotType,
            ev_equipment_i_d: Option<Utf8String>,
            type_of_receptacle: TypeOfReceptacle,
            energy_availability: Utf8String,
            parking_places_data: Option<ParkingPlacesData>,
        ) -> Self {
            Self {
                r_type,
                ev_equipment_i_d,
                type_of_receptacle,
                energy_availability,
                parking_places_data,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ItsChargingSpots(pub SequenceOf<ItsChargingSpotDataElements>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ItsChargingStationData {
        #[rasn(identifier = "chargingStationID")]
        pub charging_station_i_d: StationID,
        #[rasn(size("1..=32"), identifier = "utilityDistributorId")]
        pub utility_distributor_id: Option<Utf8String>,
        #[rasn(size("1..=32"), identifier = "providerID")]
        pub provider_i_d: Option<Utf8String>,
        #[rasn(identifier = "chargingStationLocation")]
        pub charging_station_location: ReferencePosition,
        pub address: Option<Utf8String>,
        #[rasn(size("1..=16"), identifier = "phoneNumber")]
        pub phone_number: Option<NumericString>,
        #[rasn(size("1..=32"))]
        pub accessibility: Utf8String,
        #[rasn(identifier = "digitalMap")]
        pub digital_map: Option<DigitalMap>,
        #[rasn(identifier = "openingDaysHours")]
        pub opening_days_hours: Utf8String,
        pub pricing: Utf8String,
        #[rasn(identifier = "bookingContactInfo")]
        pub booking_contact_info: Option<Utf8String>,
        pub payment: Option<Utf8String>,
        #[rasn(identifier = "chargingSpotsAvailable")]
        pub charging_spots_available: ItsChargingSpots,
    }
    impl ItsChargingStationData {
        pub fn new(
            charging_station_i_d: StationID,
            utility_distributor_id: Option<Utf8String>,
            provider_i_d: Option<Utf8String>,
            charging_station_location: ReferencePosition,
            address: Option<Utf8String>,
            phone_number: Option<NumericString>,
            accessibility: Utf8String,
            digital_map: Option<DigitalMap>,
            opening_days_hours: Utf8String,
            pricing: Utf8String,
            booking_contact_info: Option<Utf8String>,
            payment: Option<Utf8String>,
            charging_spots_available: ItsChargingSpots,
        ) -> Self {
            Self {
                charging_station_i_d,
                utility_distributor_id,
                provider_i_d,
                charging_station_location,
                address,
                phone_number,
                accessibility,
                digital_map,
                opening_days_hours,
                pricing,
                booking_contact_info,
                payment,
                charging_spots_available,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ItsEVCSNData {
        #[rasn(identifier = "totalNumberOfStations")]
        pub total_number_of_stations: NumberStations,
        #[rasn(size("1..=256"), identifier = "chargingStationsData")]
        pub charging_stations_data: SequenceOf<ItsChargingStationData>,
    }
    impl ItsEVCSNData {
        pub fn new(
            total_number_of_stations: NumberStations,
            charging_stations_data: SequenceOf<ItsChargingStationData>,
        ) -> Self {
            Self {
                total_number_of_stations,
                charging_stations_data,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ItsPOIHeader {
        #[rasn(identifier = "poiType")]
        pub poi_type: POIType,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: TimestampIts,
        #[rasn(identifier = "relayCapable")]
        pub relay_capable: bool,
    }
    impl ItsPOIHeader {
        pub fn new(poi_type: POIType, time_stamp: TimestampIts, relay_capable: bool) -> Self {
            Self {
                poi_type,
                time_stamp,
                relay_capable,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ItsPduHeader {
        #[rasn(value("0..=255"), identifier = "protocolVersion")]
        pub protocol_version: u8,
        #[rasn(value("0..=255"), identifier = "messageID")]
        pub message_i_d: u8,
        #[rasn(identifier = "stationID")]
        pub station_i_d: StationID,
    }
    impl ItsPduHeader {
        pub fn new(protocol_version: u8, message_i_d: u8, station_i_d: StationID) -> Self {
            Self {
                protocol_version,
                message_i_d,
                station_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ItsStationPosition {
        #[rasn(identifier = "stationID")]
        pub station_i_d: StationID,
        #[rasn(identifier = "laneID")]
        pub lane_i_d: Option<LaneID>,
        #[rasn(identifier = "nodeXY")]
        pub node_x_y: Option<NodeOffsetPointXY>,
        #[rasn(identifier = "timeReference")]
        pub time_reference: Option<TimeReference>,
    }
    impl ItsStationPosition {
        pub fn new(
            station_i_d: StationID,
            lane_i_d: Option<LaneID>,
            node_x_y: Option<NodeOffsetPointXY>,
            time_reference: Option<TimeReference>,
        ) -> Self {
            Self {
                station_i_d,
                lane_i_d,
                node_x_y,
                time_reference,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=5"))]
    pub struct ItsStationPositionList(pub SequenceOf<ItsStationPosition>);
    #[doc = "Definition of Containers"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum IviContainer {
        glc(GeographicLocationContainer),
        giv(GeneralIviContainer),
        rcc(RoadConfigurationContainer),
        tc(TextContainer),
        lac(LayoutContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=32767", extensible))]
    pub struct IviIdentificationNumber(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3"))]
    pub struct IviPurpose(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviStatus(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct IviType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("-900000000..=900000000"))]
    pub struct KnownLatitude(pub NinetyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("-1799999999..=1800000000"))]
    pub struct KnownLongitude(pub OneEightyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LMchannelBusyRatio(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("2"))]
    pub struct LaId(pub OctetString);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Barrier")]
    pub struct LaneAttributesBarrier(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Bike")]
    pub struct LaneAttributesBike(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Crosswalk")]
    pub struct LaneAttributesCrosswalk(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Parking")]
    pub struct LaneAttributesParking(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Sidewalk")]
    pub struct LaneAttributesSidewalk(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-Striping")]
    pub struct LaneAttributesStriping(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"), identifier = "LaneAttributes-TrackedVehicle")]
    pub struct LaneAttributesTrackedVehicle(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8", extensible), identifier = "LaneAttributes-Vehicle")]
    pub struct LaneAttributesVehicle(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneConnectionID(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct LaneDataAttributeRegional(pub SequenceOf<AnonymousLaneDataAttributeRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LaneDataAttribute {
        pathEndPointAngle(DeltaAngle),
        laneCrownPointCenter(RoadwayCrownAngle),
        laneCrownPointLeft(RoadwayCrownAngle),
        laneCrownPointRight(RoadwayCrownAngle),
        laneAngle(MergeDivergeNodeAngle),
        speedLimits(SpeedLimitList),
        #[rasn(size("1..=4"))]
        regional(SequenceOf<LaneDataAttributeRegional>),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8"))]
    pub struct LaneDataAttributeList(pub SequenceOf<LaneDataAttribute>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("2"))]
    pub struct LaneDirection(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct LaneID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=255"))]
    pub struct LaneList(pub SequenceOf<GenericLane>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1..=14"))]
    pub struct LanePosition(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("10"))]
    pub struct LaneSharing(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7", extensible))]
    pub struct LaneStatus(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=31"))]
    pub struct LaneType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LaneWidth(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LateralAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-900000000..=900000001"))]
    pub struct Latitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=100"))]
    pub struct LayerID(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum LeftOrRight {
        left = 0,
        right = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("2"))]
    pub struct LightBarSirenInUse(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16"))]
    pub struct LinkageSeed(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("9"))]
    pub struct LinkageValue(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LocationContainer {
        #[rasn(identifier = "eventSpeed")]
        pub event_speed: Option<Speed>,
        #[rasn(identifier = "eventPositionHeading")]
        pub event_position_heading: Option<Heading>,
        pub traces: Traces,
        #[rasn(identifier = "roadType")]
        pub road_type: Option<RoadType>,
    }
    impl LocationContainer {
        pub fn new(
            event_speed: Option<Speed>,
            event_position_heading: Option<Heading>,
            traces: Traces,
            road_type: Option<RoadType>,
        ) -> Self {
            Self {
                event_speed,
                event_position_heading,
                traces,
                road_type,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1800000000..=1800000001"))]
    pub struct Longitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct LongitudinalAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct LongitudinalLanePositionConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=32767"))]
    pub struct LongitudinalLanePositionValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LowFrequencyContainer {
        basicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("6"))]
    pub struct MACaddress(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "MCDM-ApplicationContainer")]
    #[non_exhaustive]
    pub struct MCDMApplicationContainer {}
    impl MCDMApplicationContainer {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "MCDM-LocationContainer")]
    #[non_exhaustive]
    pub struct MCDMLocationContainer {
        #[rasn(identifier = "eventPosition")]
        pub event_position: ReferencePosition,
    }
    impl MCDMLocationContainer {
        pub fn new(event_position: ReferencePosition) -> Self {
            Self { event_position }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "MCDM-ManagementContainer")]
    #[non_exhaustive]
    pub struct MCDMManagementContainer {
        #[rasn(identifier = "actionID")]
        pub action_i_d: ActionID,
        pub request: Option<RequestResponseIndication>,
        pub ack: Option<AckNackIndication>,
        #[rasn(identifier = "detectionTime")]
        pub detection_time: Option<TimestampIts>,
        #[rasn(identifier = "referenceTime")]
        pub reference_time: TimestampIts,
        #[rasn(identifier = "linkedDenm")]
        pub linked_denm: Option<ActionID>,
        #[rasn(identifier = "validityDuration")]
        pub validity_duration: Option<ValidityDuration>,
        #[rasn(identifier = "stationType")]
        pub station_type: Option<StationType>,
        #[rasn(
            value("0..=4294967295"),
            default = "m_c_d_m_management_container_number_of_m_d_us_default",
            identifier = "numberOfMDUs"
        )]
        pub number_of_m_d_us: u32,
        #[rasn(
            value("1..=4294967295"),
            default = "m_c_d_m_management_container_number_of_p_d_us_default",
            identifier = "numberOfPDUs"
        )]
        pub number_of_p_d_us: u32,
        #[rasn(
            value("1..=4294967295"),
            default = "m_c_d_m_management_container_pdu_sequence_number_default",
            identifier = "pduSequenceNumber"
        )]
        pub pdu_sequence_number: u32,
        #[rasn(identifier = "mediaTypes")]
        pub media_types: Option<SequenceOf<MediaTypeOfMDUs>>,
        pub urls: Option<SequenceOf<URLOfMDUs>>,
        #[rasn(
            default = "m_c_d_m_management_container_real_time_default",
            identifier = "realTime"
        )]
        pub real_time: bool,
        #[rasn(value("0..=4294967295"))]
        pub size: Option<u32>,
    }
    impl MCDMManagementContainer {
        pub fn new(
            action_i_d: ActionID,
            request: Option<RequestResponseIndication>,
            ack: Option<AckNackIndication>,
            detection_time: Option<TimestampIts>,
            reference_time: TimestampIts,
            linked_denm: Option<ActionID>,
            validity_duration: Option<ValidityDuration>,
            station_type: Option<StationType>,
            number_of_m_d_us: u32,
            number_of_p_d_us: u32,
            pdu_sequence_number: u32,
            media_types: Option<SequenceOf<MediaTypeOfMDUs>>,
            urls: Option<SequenceOf<URLOfMDUs>>,
            real_time: bool,
            size: Option<u32>,
        ) -> Self {
            Self {
                action_i_d,
                request,
                ack,
                detection_time,
                reference_time,
                linked_denm,
                validity_duration,
                station_type,
                number_of_m_d_us,
                number_of_p_d_us,
                pdu_sequence_number,
                media_types,
                urls,
                real_time,
                size,
            }
        }
    }
    fn m_c_d_m_management_container_number_of_m_d_us_default() -> u32 {
        1
    }
    fn m_c_d_m_management_container_number_of_p_d_us_default() -> u32 {
        1
    }
    fn m_c_d_m_management_container_pdu_sequence_number_default() -> u32 {
        1
    }
    fn m_c_d_m_management_container_real_time_default() -> bool {
        false
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=7"), identifier = "MCDM-MultimediaContainer")]
    pub struct MCDMMultimediaContainer(pub SequenceOf<MultimediaDataUnit>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "MCDM-SituationContainer")]
    #[non_exhaustive]
    pub struct MCDMSituationContainer {
        #[rasn(identifier = "eventType")]
        pub event_type: CauseCode,
        #[rasn(identifier = "linkedCause")]
        pub linked_cause: Option<CauseCode>,
        #[rasn(value("0..=100"), identifier = "authorizedPercentageLoss")]
        pub authorized_percentage_loss: Option<u8>,
        #[rasn(identifier = "informationQuality")]
        pub information_quality: InformationQuality,
    }
    impl MCDMSituationContainer {
        pub fn new(
            event_type: CauseCode,
            linked_cause: Option<CauseCode>,
            authorized_percentage_loss: Option<u8>,
            information_quality: InformationQuality,
        ) -> Self {
            Self {
                event_type,
                linked_cause,
                authorized_percentage_loss,
                information_quality,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ManagementContainer {
        #[rasn(identifier = "actionID")]
        pub action_i_d: ActionID,
        #[rasn(identifier = "detectionTime")]
        pub detection_time: TimestampIts,
        #[rasn(identifier = "referenceTime")]
        pub reference_time: TimestampIts,
        pub termination: Option<Termination>,
        #[rasn(identifier = "eventPosition")]
        pub event_position: ReferencePosition,
        #[rasn(identifier = "relevanceDistance")]
        pub relevance_distance: Option<RelevanceDistance>,
        #[rasn(identifier = "relevanceTrafficDirection")]
        pub relevance_traffic_direction: Option<RelevanceTrafficDirection>,
        #[rasn(
            default = "management_container_validity_duration_default",
            identifier = "validityDuration"
        )]
        pub validity_duration: ValidityDuration,
        #[rasn(identifier = "transmissionInterval")]
        pub transmission_interval: Option<TransmissionInterval>,
        #[rasn(identifier = "stationType")]
        pub station_type: StationType,
    }
    impl ManagementContainer {
        pub fn new(
            action_i_d: ActionID,
            detection_time: TimestampIts,
            reference_time: TimestampIts,
            termination: Option<Termination>,
            event_position: ReferencePosition,
            relevance_distance: Option<RelevanceDistance>,
            relevance_traffic_direction: Option<RelevanceTrafficDirection>,
            validity_duration: ValidityDuration,
            transmission_interval: Option<TransmissionInterval>,
            station_type: StationType,
        ) -> Self {
            Self {
                action_i_d,
                detection_time,
                reference_time,
                termination,
                event_position,
                relevance_distance,
                relevance_traffic_direction,
                validity_duration,
                transmission_interval,
                station_type,
            }
        }
    }
    fn management_container_validity_duration_default() -> ValidityDuration {
        (*DEFAULT_VALIDITY).clone()
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct MandAppCtx(pub SequenceOf<ItsAidCtxRef>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ManeuverAssistList(pub SequenceOf<ConnectionManeuverAssist>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MapDataRegional(pub SequenceOf<AnonymousMapDataRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        pub layer_i_d: Option<LayerID>,
        pub intersections: Option<IntersectionGeometryList>,
        #[rasn(identifier = "roadSegments")]
        pub road_segments: Option<RoadSegmentList>,
        #[rasn(identifier = "dataParameters")]
        pub data_parameters: Option<DataParameters>,
        #[rasn(identifier = "restrictionList")]
        pub restriction_list: Option<RestrictionClassList>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<MapDataRegional>>,
    }
    impl MapData {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            msg_issue_revision: MsgCount,
            layer_type: Option<LayerType>,
            layer_i_d: Option<LayerID>,
            intersections: Option<IntersectionGeometryList>,
            road_segments: Option<RoadSegmentList>,
            data_parameters: Option<DataParameters>,
            restriction_list: Option<RestrictionClassList>,
            regional: Option<SequenceOf<MapDataRegional>>,
        ) -> Self {
            Self {
                time_stamp,
                msg_issue_revision,
                layer_type,
                layer_i_d,
                intersections,
                road_segments,
                data_parameters,
                restriction_list,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct MapPosition {
        #[rasn(identifier = "intersectionId")]
        pub intersection_id: IntersectionReferenceID,
        pub lane: LaneID,
    }
    impl MapPosition {
        pub fn new(intersection_id: IntersectionReferenceID, lane: LaneID) -> Self {
            Self {
                intersection_id,
                lane,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MatchedPosition {
        #[rasn(identifier = "laneID")]
        pub lane_i_d: Option<LaneID>,
        #[rasn(identifier = "longitudinalLanePosition")]
        pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
    }
    impl MatchedPosition {
        pub fn new(
            lane_i_d: Option<LaneID>,
            longitudinal_lane_position: Option<LongitudinalLanePosition>,
        ) -> Self {
            Self {
                lane_i_d,
                longitudinal_lane_position,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct McdmInfo {
        pub management: MCDMManagementContainer,
        pub situation: Option<MCDMSituationContainer>,
        pub location: Option<MCDMLocationContainer>,
        pub application: Option<MCDMApplicationContainer>,
        pub multimedia: Option<MCDMMultimediaContainer>,
    }
    impl McdmInfo {
        pub fn new(
            management: MCDMManagementContainer,
            situation: Option<MCDMSituationContainer>,
            location: Option<MCDMLocationContainer>,
            application: Option<MCDMApplicationContainer>,
            multimedia: Option<MCDMMultimediaContainer>,
        ) -> Self {
            Self {
                management,
                situation,
                location,
                application,
                multimedia,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct McdmPdu {
        pub header: ItsPduHeader,
        #[rasn(identifier = "mcdmInfo")]
        pub mcdm_info: McdmInfo,
    }
    impl McdmPdu {
        pub fn new(header: ItsPduHeader, mcdm_info: McdmInfo) -> Self {
            Self { header, mcdm_info }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct MedType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct MediaTypeOfMDUs {
        #[rasn(identifier = "startingMDU")]
        pub starting_m_d_u: Option<SequenceNumber>,
        #[rasn(identifier = "endingMDU")]
        pub ending_m_d_u: Option<SequenceNumber>,
        #[rasn(identifier = "mediaType")]
        pub media_type: Ia5String,
    }
    impl MediaTypeOfMDUs {
        pub fn new(
            starting_m_d_u: Option<SequenceNumber>,
            ending_m_d_u: Option<SequenceNumber>,
            media_type: Ia5String,
        ) -> Self {
            Self {
                starting_m_d_u,
                ending_m_d_u,
                media_type,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-180..=180"))]
    pub struct MergeDivergeNodeAngle(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=527040"))]
    pub struct MinuteOfTheYear(pub u32);
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
    #[non_exhaustive]
    pub struct MotorcylistSpecialContainer {
        #[rasn(identifier = "vruSubProfileMotorcyclist")]
        pub vru_sub_profile_motorcyclist: VruSubProfileMotorcyclist,
        #[rasn(identifier = "vruSizeClass")]
        pub vru_size_class: VruSizeClass,
        #[rasn(identifier = "rollAngle")]
        pub roll_angle: Option<VruRollAngle>,
        #[rasn(identifier = "vruSafeDistance")]
        pub vru_safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
        #[rasn(identifier = "pathHistory")]
        pub path_history: Option<PathHistory>,
        #[rasn(identifier = "pathPrediction")]
        pub path_prediction: Option<PathPrediction>,
        #[rasn(identifier = "stabilityChangeIndication")]
        pub stability_change_indication: Option<StabilityChangeIndication>,
    }
    impl MotorcylistSpecialContainer {
        pub fn new(
            vru_sub_profile_motorcyclist: VruSubProfileMotorcyclist,
            vru_size_class: VruSizeClass,
            roll_angle: Option<VruRollAngle>,
            vru_safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
            path_history: Option<PathHistory>,
            path_prediction: Option<PathPrediction>,
            stability_change_indication: Option<StabilityChangeIndication>,
        ) -> Self {
            Self {
                vru_sub_profile_motorcyclist,
                vru_size_class,
                roll_angle,
                vru_safe_distance,
                path_history,
                path_prediction,
                stability_change_indication,
            }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementEventRegional(pub SequenceOf<AnonymousMovementEventRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct MovementEvent {
        #[rasn(identifier = "eventState")]
        pub event_state: MovementPhaseState,
        pub timing: Option<TimeChangeDetails>,
        pub speeds: Option<AdvisorySpeedList>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<MovementEventRegional>>,
    }
    impl MovementEvent {
        pub fn new(
            event_state: MovementPhaseState,
            timing: Option<TimeChangeDetails>,
            speeds: Option<AdvisorySpeedList>,
            regional: Option<SequenceOf<MovementEventRegional>>,
        ) -> Self {
            Self {
                event_state,
                timing,
                speeds,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct MovementEventList(pub SequenceOf<MovementEvent>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=255"))]
    pub struct MovementList(pub SequenceOf<MovementState>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum MovementPhaseState {
        unavailable = 0,
        dark = 1,
        stop_Then_Proceed = 2,
        stop_And_Remain = 3,
        pre_Movement = 4,
        permissive_Movement_Allowed = 5,
        protected_Movement_Allowed = 6,
        permissive_clearance = 7,
        protected_clearance = 8,
        caution_Conflicting_Traffic = 9,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct MovementStateRegional(pub SequenceOf<AnonymousMovementStateRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<MovementStateRegional>>,
    }
    impl MovementState {
        pub fn new(
            movement_name: Option<DescriptiveName>,
            signal_group: SignalGroupID,
            state_time_speed: MovementEventList,
            maneuver_assist_list: Option<ManeuverAssistList>,
            regional: Option<SequenceOf<MovementStateRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct MsgCount(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum MultimediaDataUnit {
        mediaContentUTF8(Utf8String),
        mediaContentOctet(OctetString),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-900000000..=900000001"))]
    pub struct NinetyDegreeInt(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Node {
        pub id: Integer,
        pub lane: Option<LaneID>,
        #[rasn(identifier = "connectionID")]
        pub connection_i_d: Option<LaneConnectionID>,
        #[rasn(identifier = "intersectionID")]
        pub intersection_i_d: Option<IntersectionID>,
    }
    impl Node {
        pub fn new(
            id: Integer,
            lane: Option<LaneID>,
            connection_i_d: Option<LaneConnectionID>,
            intersection_i_d: Option<IntersectionID>,
        ) -> Self {
            Self {
                id,
                lane,
                connection_i_d,
                intersection_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct NodeAttributeSetXYRegional(pub SequenceOf<AnonymousNodeAttributeSetXYRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<NodeAttributeSetXYRegional>>,
    }
    impl NodeAttributeSetXY {
        pub fn new(
            local_node: Option<NodeAttributeXYList>,
            disabled: Option<SegmentAttributeXYList>,
            enabled: Option<SegmentAttributeXYList>,
            data: Option<LaneDataAttributeList>,
            d_width: Option<OffsetB10>,
            d_elevation: Option<OffsetB10>,
            regional: Option<SequenceOf<NodeAttributeSetXYRegional>>,
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8"))]
    pub struct NodeAttributeXYList(pub SequenceOf<NodeAttributeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=5"))]
    pub struct NodeLink(pub SequenceOf<Node>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum NodeListXY {
        nodes(NodeSetXY),
        computed(ComputedLane),
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("2..=63"))]
    pub struct NodeSetXY(pub SequenceOf<NodeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum NonIslandLanePosition {
        offRoadLanePosition(OffRoadLanePosition),
        vehicularLanePosition(LanePosition),
        mapPosition(MapPosition),
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct NullCtx(());
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct NumberOfOccupants(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct NumberOfPerceivedObjects(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=256"))]
    pub struct NumberStations(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=1500"))]
    pub struct ObjectAge(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum ObjectClassChoice {
        vehicle(VehicleSubclass),
        person(PersonSubclass),
        animal(AnimalSubclass),
        other(OtherSubclass),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8"))]
    pub struct ObjectClassDescription(pub SequenceOf<ObjectClass>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=101"))]
    pub struct ObjectConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=102"))]
    pub struct ObjectDimensionConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=1023"))]
    pub struct ObjectDimensionValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=8"))]
    pub struct ObjectRefPoint(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum OffRoadLanePosition {
        unavailable = 0,
        sidewalk = 1,
        parkingLane = 2,
        bikeLane = 3,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-256..=255"), identifier = "Offset-B09")]
    pub struct OffsetB09(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-512..=511"), identifier = "Offset-B10")]
    pub struct OffsetB10(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1024..=1023"), identifier = "Offset-B11")]
    pub struct OffsetB11(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-2048..=2047"), identifier = "Offset-B12")]
    pub struct OffsetB12(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-4096..=4095"), identifier = "Offset-B13")]
    pub struct OffsetB13(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-8192..=8191"), identifier = "Offset-B14")]
    pub struct OffsetB14(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-32768..=32767"), identifier = "Offset-B16")]
    pub struct OffsetB16(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum OffsetXaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum OffsetYaxis {
        small(DrivenLineOffsetSm),
        large(DrivenLineOffsetLg),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1799999999..=1800000001"))]
    pub struct OneEightyDegreeInt(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Opaque(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct OpeningDaysHours(pub Utf8String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct OperatingClass80211(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum OriginatingRSUContainer {
        intersectionReferenceId(IntersectionReferenceID),
        roadSegmentReferenceId(RoadSegmentReferenceID),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    fn other_subclass_r_type_default() -> OtherSublassType {
        OtherSublassType(0)
    }
    fn other_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct OtherSublassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=5"))]
    pub struct OverlayLaneList(pub SequenceOf<LaneID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("4"))]
    pub struct PMD(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct POIType(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct ParkingPlacesData(pub SequenceOf<SpotAvailability>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PartialIntersection {
        #[rasn(size("4"), identifier = "dummyBitmap")]
        pub dummy_bitmap: BitString,
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
    }
    impl PartialIntersection {
        pub fn new(
            dummy_bitmap: BitString,
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
        ) -> Self {
            Self {
                dummy_bitmap,
                name,
                id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PartialMapData {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        #[rasn(identifier = "msgIssueRevision")]
        pub msg_issue_revision: MsgCount,
        #[rasn(identifier = "layerType")]
        pub layer_type: Option<LayerType>,
        #[rasn(identifier = "layerID")]
        pub layer_i_d: Option<LayerID>,
        #[rasn(identifier = "firstIntersection")]
        pub first_intersection: Option<FirstIntersection>,
        pub dummy1: Option<()>,
        pub dummy2: Option<()>,
        pub dummy3: Option<()>,
        pub dummy4: Option<()>,
    }
    impl PartialMapData {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            msg_issue_revision: MsgCount,
            layer_type: Option<LayerType>,
            layer_i_d: Option<LayerID>,
            first_intersection: Option<FirstIntersection>,
            dummy1: Option<()>,
            dummy2: Option<()>,
            dummy3: Option<()>,
            dummy4: Option<()>,
        ) -> Self {
            Self {
                time_stamp,
                msg_issue_revision,
                layer_type,
                layer_i_d,
                first_intersection,
                dummy1,
                dummy2,
                dummy3,
                dummy4,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PartialMapem {
        pub header: ItsPduHeader,
        pub map: PartialMapData,
    }
    impl PartialMapem {
        pub fn new(header: ItsPduHeader, map: PartialMapData) -> Self {
            Self { header, map }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PartialSpat {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub name: Option<DescriptiveName>,
        pub intersections: FirstSpat,
        pub dummy: Option<()>,
    }
    impl PartialSpat {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            name: Option<DescriptiveName>,
            intersections: FirstSpat,
            dummy: Option<()>,
        ) -> Self {
            Self {
                time_stamp,
                name,
                intersections,
                dummy,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PartialSpatIntersection {
        #[rasn(size("5"), identifier = "dummyBitmap")]
        pub dummy_bitmap: BitString,
        pub name: Option<DescriptiveName>,
        pub id: IntersectionReferenceID,
    }
    impl PartialSpatIntersection {
        pub fn new(
            dummy_bitmap: BitString,
            name: Option<DescriptiveName>,
            id: IntersectionReferenceID,
        ) -> Self {
            Self {
                dummy_bitmap,
                name,
                id,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PartialSpatem {
        pub header: ItsPduHeader,
        pub spat: PartialSpat,
    }
    impl PartialSpatem {
        pub fn new(header: ItsPduHeader, spat: PartialSpat) -> Self {
            Self { header, spat }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=65535", extensible))]
    pub struct PathDeltaTime(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=40"))]
    pub struct PathHistory(pub SequenceOf<PathPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct PathPrediction(pub PathHistory);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct PedestrianBicycleDetect(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceivedObject {
        #[rasn(identifier = "objectID")]
        pub object_i_d: Identifier,
        #[rasn(identifier = "sensorIDList")]
        pub sensor_i_d_list: Option<SensorIdList>,
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
            object_i_d: Identifier,
            sensor_i_d_list: Option<SensorIdList>,
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
                object_i_d,
                sensor_i_d_list,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct PerceivedObjectContainer(pub SequenceOf<PerceivedObject>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct PerformanceClass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    fn person_subclass_r_type_default() -> PersonSubclassType {
        PersonSubclassType(0)
    }
    fn person_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PersonSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct PhoneNumber(pub NumericString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("3..=16", extensible))]
    pub struct PolyPointList(pub SequenceOf<OffsetPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PolygonalLine {
        deltaPositions(DeltaPositions),
        deltaPositionsWithAltitude(DeltaPositionsWithAltitude),
        absolutePositions(AbsolutePositions),
        absolutePositionsWithAltitude(AbsolutePositionsWithAltitude),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("3.."))]
    pub struct PolygonalRegion(pub SequenceOf<Dot2TwoDLocation>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct PortNumber(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=63"))]
    pub struct PosCentMass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=20"))]
    pub struct PosFrontAx(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct PosLonCarr(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=30"))]
    pub struct PosPillar(pub u8);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct Position3DRegional(pub SequenceOf<AnonymousPosition3DRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct Position3D {
        pub lat: Latitude,
        pub long: Longitude,
        pub elevation: Option<Elevation>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<Position3DRegional>>,
    }
    impl Position3D {
        pub fn new(
            lat: Latitude,
            long: Longitude,
            elevation: Option<Elevation>,
            regional: Option<SequenceOf<Position3DRegional>>,
        ) -> Self {
            Self {
                lat,
                long,
                elevation,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("20"))]
    pub struct PositionOfOccupants(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct PositionOfPillars(pub SequenceOf<PosPillar>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PostCrashSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PreCrashContainer {
        #[rasn(identifier = "perceivedObject")]
        pub perceived_object: PerceivedObject,
        #[rasn(identifier = "objectStationId")]
        pub object_station_id: Option<StationID>,
        #[rasn(identifier = "timeToCollision")]
        pub time_to_collision: Option<TransmissionInterval>,
        #[rasn(identifier = "impactSection")]
        pub impact_section: Option<ImpactSection>,
        #[rasn(identifier = "hostVehicleOrientation")]
        pub host_vehicle_orientation: WGS84Angle,
    }
    impl PreCrashContainer {
        pub fn new(
            perceived_object: PerceivedObject,
            object_station_id: Option<StationID>,
            time_to_collision: Option<TransmissionInterval>,
            impact_section: Option<ImpactSection>,
            host_vehicle_orientation: WGS84Angle,
        ) -> Self {
            Self {
                perceived_object,
                object_station_id,
                time_to_collision,
                impact_section,
                host_vehicle_orientation,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct PreemptPriorityList(pub SequenceOf<SignalControlZone>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PrioritizationResponse {
        #[rasn(identifier = "stationID")]
        pub station_i_d: StationID,
        #[rasn(identifier = "priorState")]
        pub prior_state: PrioritizationResponseStatus,
        #[rasn(identifier = "signalGroup")]
        pub signal_group: SignalGroupID,
    }
    impl PrioritizationResponse {
        pub fn new(
            station_i_d: StationID,
            prior_state: PrioritizationResponseStatus,
            signal_group: SignalGroupID,
        ) -> Self {
            Self {
                station_i_d,
                prior_state,
                signal_group,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=10"))]
    pub struct PrioritizationResponseList(pub SequenceOf<PrioritizationResponse>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PriorityRequestType {
        priorityRequestTypeReserved = 0,
        priorityRequest = 1,
        priorityRequestUpdate = 2,
        priorityCancellation = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        pub protected_zone_i_d: Option<ProtectedZoneID>,
    }
    impl ProtectedCommunicationZone {
        pub fn new(
            protected_zone_type: ProtectedZoneType,
            expiry_time: Option<TimestampIts>,
            protected_zone_latitude: Latitude,
            protected_zone_longitude: Longitude,
            protected_zone_radius: Option<ProtectedZoneRadius>,
            protected_zone_i_d: Option<ProtectedZoneID>,
        ) -> Self {
            Self {
                protected_zone_type,
                expiry_time,
                protected_zone_latitude,
                protected_zone_longitude,
                protected_zone_radius,
                protected_zone_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=134217727"))]
    pub struct ProtectedZoneID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=255", extensible))]
    pub struct ProtectedZoneRadius(pub Integer);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum ProtectedZoneType {
        permanentCenDsrcTolling = 0,
        #[rasn(extension_addition)]
        temporaryCenDsrcTolling = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ProtocolType(pub VarLengthNumber);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ProviderMacAddress(pub MACaddress);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ProviderPermissions(pub SequenceOf<ChannelSpecificProviderPermission>);
    #[doc = "ServiceInfo extension elements"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ProviderServiceContext {
        #[rasn(size("3"), identifier = "fillBit")]
        pub fill_bit: BitString,
        #[rasn(size("0..=31"))]
        pub psc: OctetString,
    }
    impl ProviderServiceContext {
        pub fn new(fill_bit: BitString, psc: OctetString) -> Self {
            Self { fill_bit, psc }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0.."))]
    pub struct Psid(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PsidSsp {
        pub psid: Psid,
        pub ssp: Option<ServiceSpecificPermissions>,
    }
    impl PsidSsp {
        pub fn new(psid: Psid, ssp: Option<ServiceSpecificPermissions>) -> Self {
            Self { psid, ssp }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PsidSspRange {
        pub psid: Psid,
        #[rasn(identifier = "sspRange")]
        pub ssp_range: Option<SspRange>,
    }
    impl PsidSspRange {
        pub fn new(psid: Psid, ssp_range: Option<SspRange>) -> Self {
            Self { psid, ssp_range }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=20"))]
    pub struct PtActivationData(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct PtActivationType(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PtvRequestType {
        preRequest = 0,
        mainRequest = 1,
        doorCloseRequest = 2,
        cancelRequest = 3,
        emergencyRequest = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PublicEncryptionKey {
        #[rasn(identifier = "supportedSymmAlg")]
        pub supported_symm_alg: SymmAlgorithm,
        #[rasn(identifier = "publicKey")]
        pub public_key: BasePublicEncryptionKey,
    }
    impl PublicEncryptionKey {
        pub fn new(supported_symm_alg: SymmAlgorithm, public_key: BasePublicEncryptionKey) -> Self {
            Self {
                supported_symm_alg,
                public_key,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum PublicFacilitiesPictogram {
        publicFacilities = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct PublicTransportContainer {
        #[rasn(identifier = "embarkationStatus")]
        pub embarkation_status: EmbarkationStatus,
        #[rasn(identifier = "ptActivation")]
        pub pt_activation: Option<PtActivation>,
    }
    impl PublicTransportContainer {
        pub fn new(
            embarkation_status: EmbarkationStatus,
            pt_activation: Option<PtActivation>,
        ) -> Self {
            Self {
                embarkation_status,
                pt_activation,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum PublicVerificationKey {
        ecdsaNistP256(EccP256CurvePoint),
        ecdsaBrainpoolP256r1(EccP256CurvePoint),
        #[rasn(extension_addition)]
        ecdsaBrainpoolP384r1(EccP384CurvePoint),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=32"))]
    pub struct ROI(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct RSCUnit(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RSUContainerHighFrequency {
        #[rasn(identifier = "protectedCommunicationZonesRSU")]
        pub protected_communication_zones_r_s_u: Option<ProtectedCommunicationZonesRSU>,
    }
    impl RSUContainerHighFrequency {
        pub fn new(
            protected_communication_zones_r_s_u: Option<ProtectedCommunicationZonesRSU>,
        ) -> Self {
            Self {
                protected_communication_zones_r_s_u,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated, identifier = "RTCM-Revision")]
    #[non_exhaustive]
    pub enum RTCMRevision {
        unknown = 0,
        rtcmRev2 = 1,
        rtcmRev3 = 2,
        reserved = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RTCMEM {
        pub header: ItsPduHeader,
        pub rtcmc: RTCMcorrections,
    }
    impl RTCMEM {
        pub fn new(header: ItsPduHeader, rtcmc: RTCMcorrections) -> Self {
            Self { header, rtcmc }
        }
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousRTCMcorrectionsRegional {
        #[rasn(value("0..=255"), identifier = "regionId")]
        pub region_id: u8,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousRTCMcorrectionsRegional {
        pub fn new(region_id: u8, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RTCMcorrectionsRegional(pub SequenceOf<AnonymousRTCMcorrectionsRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RTCMcorrections {
        #[rasn(identifier = "msgCnt")]
        pub msg_cnt: MsgCount,
        pub rev: RTCMRevision,
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        #[rasn(identifier = "anchorPoint")]
        pub anchor_point: Option<FullPositionVector>,
        #[rasn(identifier = "rtcmHeader")]
        pub rtcm_header: Option<RTCMheader>,
        pub msgs: RTCMmessageList,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<RTCMcorrectionsRegional>>,
    }
    impl RTCMcorrections {
        pub fn new(
            msg_cnt: MsgCount,
            rev: RTCMRevision,
            time_stamp: Option<MinuteOfTheYear>,
            anchor_point: Option<FullPositionVector>,
            rtcm_header: Option<RTCMheader>,
            msgs: RTCMmessageList,
            regional: Option<SequenceOf<RTCMcorrectionsRegional>>,
        ) -> Self {
            Self {
                msg_cnt,
                rev,
                time_stamp,
                anchor_point,
                rtcm_header,
                msgs,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RTCMheader {
        pub status: GNSSstatus,
        #[rasn(identifier = "offsetSet")]
        pub offset_set: AntennaOffsetSet,
    }
    impl RTCMheader {
        pub fn new(status: GNSSstatus, offset_set: AntennaOffsetSet) -> Self {
            Self { status, offset_set }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=1023"))]
    pub struct RTCMmessage(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=5"))]
    pub struct RTCMmessageList(pub SequenceOf<RTCMmessage>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Radius(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Range(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RcpiThreshold(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=150"))]
    pub struct RearOverhang(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RectangularRegion {
        #[rasn(identifier = "northWest")]
        pub north_west: Dot2TwoDLocation,
        #[rasn(identifier = "southEast")]
        pub south_east: Dot2TwoDLocation,
    }
    impl RectangularRegion {
        pub fn new(north_west: Dot2TwoDLocation, south_east: Dot2TwoDLocation) -> Self {
            Self {
                north_west,
                south_east,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RefExt(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RefPointId(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ReferenceDenms(pub SequenceOf<ActionID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegAdvisorySpeed_Type {}
    impl RegAdvisorySpeed_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegAdvisorySpeed_id {}
    impl RegAdvisorySpeed_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegComputedLane_Type {}
    impl RegComputedLane_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegComputedLane_id {}
    impl RegComputedLane_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegConnectionManeuverAssist_Type {
        AddGrpC(ConnectionManeuverAssistAddGrpC),
    }
    impl RegConnectionManeuverAssist_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegGenericLane_Type {
        AddGrpC(ConnectionTrajectoryAddGrpC),
    }
    impl RegGenericLane_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionGeometry_Type {}
    impl RegIntersectionGeometry_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionGeometry_id {}
    impl RegIntersectionGeometry_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegIntersectionState_Type {
        AddGrpC(IntersectionStateAddGrpC),
    }
    impl RegIntersectionState_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneAttributes_Type {
        AddGrpC(LaneAttributesAddGrpC),
    }
    impl RegLaneAttributes_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneDataAttribute_Type {}
    impl RegLaneDataAttribute_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegLaneDataAttribute_id {}
    impl RegLaneDataAttribute_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMapData_Type {
        AddGrpC(MapDataAddGrpC),
    }
    impl RegMapData_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementEvent_Type {
        AddGrpC(MovementEventAddGrpC),
    }
    impl RegMovementEvent_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementState_Type {}
    impl RegMovementState_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegMovementState_id {}
    impl RegMovementState_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeAttributeSetXY_Type {
        AddGrpC(NodeAttributeSetAddGrpC),
    }
    impl RegNodeAttributeSetXY_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeOffsetPointXY_Type {}
    impl RegNodeOffsetPointXY_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegNodeOffsetPointXY_id {}
    impl RegNodeOffsetPointXY_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegPosition3D_Type {
        AddGrpC(Position3DAddGrpC),
    }
    impl RegPosition3D_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRTCMcorrections_Type {}
    impl RegRTCMcorrections_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRTCMcorrections_id {}
    impl RegRTCMcorrections_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRestrictionUserType_Type {
        AddGrpC(RestrictionUserTypeAddGrpC),
    }
    impl RegRestrictionUserType_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRoadSegment_Type {}
    impl RegRoadSegment_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRoadSegment_id {}
    impl RegRoadSegment_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSPAT_Type {}
    impl RegSPAT_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSPAT_id {}
    impl RegSPAT_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalControlZone_Type {}
    impl RegSignalControlZone_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalControlZone_id {}
    impl RegSignalControlZone_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequest_Type {}
    impl RegSignalRequest_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequest_id {}
    impl RegSignalRequest_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestMessage_Type {}
    impl RegSignalRequestMessage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestMessage_id {}
    impl RegSignalRequestMessage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestPackage_Type {}
    impl RegSignalRequestPackage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalRequestPackage_id {}
    impl RegSignalRequestPackage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatus_Type {}
    impl RegSignalStatus_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatus_id {}
    impl RegSignalStatus_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusMessage_Type {}
    impl RegSignalStatusMessage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusMessage_id {}
    impl RegSignalStatusMessage_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegSignalStatusPackage_Type {
        AddGrpC(SignalStatusPackageAddGrpC),
    }
    impl RegSignalStatusPackage_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequesterDescription_Type {
        AddGrpC(RequesterDescriptionAddGrpC),
    }
    impl RegRequesterDescription_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ADD_GRP_C => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::AddGrpC)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::AddGrpC(inner), i) if i == &ADD_GRP_C => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequesterType_Type {}
    impl RegRequesterType_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RegRequesterType_id {}
    impl RegRequesterType_id {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RegionId,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RegionId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RegionAndSubregions {
        pub region: Uint8,
        pub subregions: SequenceOfUint16,
    }
    impl RegionAndSubregions {
        pub fn new(region: Uint8, subregions: SequenceOfUint16) -> Self {
            Self { region, subregions }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RegionId(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum RelevanceTrafficDirection {
        allTrafficDirections = 0,
        upstreamTraffic = 1,
        downstreamTraffic = 2,
        oppositeTraffic = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RepeatRate(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ReplyAddress(pub PortNumber);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RequestID(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum RequestResponseIndication {
        request = 0,
        response = 1,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RequesterDescriptionRegional(pub SequenceOf<AnonymousRequesterDescriptionRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<RequesterDescriptionRegional>>,
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
            regional: Option<SequenceOf<RequesterDescriptionRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RescueContainer {
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: LightBarSirenInUse,
    }
    impl RescueContainer {
        pub fn new(light_bar_siren_in_use: LightBarSirenInUse) -> Self {
            Self {
                light_bar_siren_in_use,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=3", extensible))]
    pub struct RestrictedTypes(pub SequenceOf<StationType>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RestrictionClassID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=254"))]
    pub struct RestrictionClassList(pub SequenceOf<RestrictionClassAssignment>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RestrictionUserTypeRegional(pub SequenceOf<AnonymousRestrictionUserTypeRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum RestrictionUserType {
        basicType(RestrictionAppliesTo),
        #[rasn(size("1..=4"))]
        regional(SequenceOf<RestrictionUserTypeRegional>),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16"))]
    pub struct RestrictionUserTypeList(pub SequenceOf<RestrictionUserType>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct RoadAngles(pub SequenceOf<Heading>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct RoadConfigurationContainer(pub SequenceOf<RccPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=255"))]
    pub struct RoadLaneSetList(pub SequenceOf<GenericLane>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadRegulatorID(pub u16);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct RoadSegmentRegional(pub SequenceOf<AnonymousRoadSegmentRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<RoadSegmentRegional>>,
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
            regional: Option<SequenceOf<RoadSegmentRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RoadSegmentID(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct RoadSegmentList(pub SequenceOf<RoadSegment>);
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum RoadType {
        urban_NoStructuralSeparationToOppositeLanes = 0,
        urban_WithStructuralSeparationToOppositeLanes = 1,
        nonUrban_NoStructuralSeparationToOppositeLanes = 2,
        nonUrban_WithStructuralSeparationToOppositeLanes = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RoadWorksContainerBasic {
        #[rasn(identifier = "roadworksSubCauseCode")]
        pub roadworks_sub_cause_code: Option<RoadworksSubCauseCode>,
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: LightBarSirenInUse,
        #[rasn(identifier = "closedLanes")]
        pub closed_lanes: Option<ClosedLanes>,
    }
    impl RoadWorksContainerBasic {
        pub fn new(
            roadworks_sub_cause_code: Option<RoadworksSubCauseCode>,
            light_bar_siren_in_use: LightBarSirenInUse,
            closed_lanes: Option<ClosedLanes>,
        ) -> Self {
            Self {
                roadworks_sub_cause_code,
                light_bar_siren_in_use,
                closed_lanes,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RoadWorksContainerExtended {
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: Option<LightBarSirenInUse>,
        #[rasn(identifier = "closedLanes")]
        pub closed_lanes: Option<ClosedLanes>,
        pub restriction: Option<RestrictedTypes>,
        #[rasn(identifier = "speedLimit")]
        pub speed_limit: Option<SpeedLimit>,
        #[rasn(identifier = "incidentIndication")]
        pub incident_indication: Option<CauseCode>,
        #[rasn(identifier = "recommendedPath")]
        pub recommended_path: Option<ItineraryPath>,
        #[rasn(identifier = "startingPointSpeedLimit")]
        pub starting_point_speed_limit: Option<DeltaReferencePosition>,
        #[rasn(identifier = "trafficFlowRule")]
        pub traffic_flow_rule: Option<TrafficRule>,
        #[rasn(identifier = "referenceDenms")]
        pub reference_denms: Option<ReferenceDenms>,
    }
    impl RoadWorksContainerExtended {
        pub fn new(
            light_bar_siren_in_use: Option<LightBarSirenInUse>,
            closed_lanes: Option<ClosedLanes>,
            restriction: Option<RestrictedTypes>,
            speed_limit: Option<SpeedLimit>,
            incident_indication: Option<CauseCode>,
            recommended_path: Option<ItineraryPath>,
            starting_point_speed_limit: Option<DeltaReferencePosition>,
            traffic_flow_rule: Option<TrafficRule>,
            reference_denms: Option<ReferenceDenms>,
        ) -> Self {
            Self {
                light_bar_siren_in_use,
                closed_lanes,
                restriction,
                speed_limit,
                incident_indication,
                recommended_path,
                starting_point_speed_limit,
                traffic_flow_rule,
                reference_denms,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-128..=127"))]
    pub struct RoadwayCrownAngle(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RoadworksSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RoutAdvertExt {
        #[rasn(identifier = "extensionId")]
        pub extension_id: RefExt,
        pub value: Any,
    }
    impl RoutAdvertExt {
        pub fn new(extension_id: RefExt, value: Any) -> Self {
            Self {
                extension_id,
                value,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum RoutAdvertExtTypes_ExtValue {
        CSecondaryDNS(SecondaryDns),
        CGatewayMACaddress(GatewayMacAddress),
    }
    impl RoutAdvertExtTypes_ExtValue {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &C__SECONDARY_D_N_S => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CSecondaryDNS)?),
                i if i == &C__GATEWAY_M_A_CADDRESS => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CGatewayMACaddress)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::CSecondaryDNS(inner), i) if i == &C__SECONDARY_D_N_S => {
                    inner.encode(encoder)
                }
                (Self::CGatewayMACaddress(inner), i) if i == &C__GATEWAY_M_A_CADDRESS => {
                    inner.encode(encoder)
                }
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct RoutAdvertExts(pub SequenceOf<RoutAdvertExt>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct RouterLifetime(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct RoutingAdvertisement {
        pub lifetime: RouterLifetime,
        #[rasn(identifier = "ipPrefix")]
        pub ip_prefix: IpV6Prefix,
        #[rasn(identifier = "ipPrefixLength")]
        pub ip_prefix_length: IpV6PrefixLength,
        #[rasn(identifier = "defaultGateway")]
        pub default_gateway: IPv6Address,
        #[rasn(identifier = "primaryDns")]
        pub primary_dns: IPv6Address,
        pub extensions: RoutAdvertExts,
    }
    impl RoutingAdvertisement {
        pub fn new(
            lifetime: RouterLifetime,
            ip_prefix: IpV6Prefix,
            ip_prefix_length: IpV6PrefixLength,
            default_gateway: IPv6Address,
            primary_dns: IPv6Address,
            extensions: RoutAdvertExts,
        ) -> Self {
            Self {
                lifetime,
                ip_prefix,
                ip_prefix_length,
                default_gateway,
                primary_dns,
                extensions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct RsvAdvPrtVersion(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=28800"))]
    pub struct SAEHeading(pub u16);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum SAEHeadingConfidence {
        unavailable = 0,
        prec10deg = 1,
        prec05deg = 2,
        prec01deg = 3,
        prec0_1deg = 4,
        prec0_05deg = 5,
        prec0_01deg = 6,
        prec0_0125deg = 7,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum SAESpeedConfidence {
        unavailable = 0,
        prec100ms = 1,
        prec10ms = 2,
        prec5ms = 3,
        prec1ms = 4,
        prec0_1ms = 5,
        prec0_05ms = 6,
        prec0_01ms = 7,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum SAEThrottleConfidence {
        unavailable = 0,
        prec10percent = 1,
        prec1percent = 2,
        prec0_5percent = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-4096..=61439"))]
    pub struct SAElevation(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SALatitude {
        #[rasn(size("1"), identifier = "fillBit")]
        pub fill_bit: BitString,
        #[rasn(value("-900000000..=900000001"))]
        pub lat: i32,
    }
    impl SALatitude {
        pub fn new(fill_bit: BitString, lat: i32) -> Self {
            Self { fill_bit, lat }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1800000000..=1800000001"))]
    pub struct SALongitude(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SAMapplicationData(pub OctetString);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SPATRegional(pub SequenceOf<AnonymousSPATRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SPAT {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub name: Option<DescriptiveName>,
        pub intersections: IntersectionStateList,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SPATRegional>>,
    }
    impl SPAT {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            name: Option<DescriptiveName>,
            intersections: IntersectionStateList,
            regional: Option<SequenceOf<SPATRegional>>,
        ) -> Self {
            Self {
                time_stamp,
                name,
                intersections,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SRMextension {
        #[rasn(identifier = "extensionId")]
        pub extension_id: RefExt,
        pub value: Any,
    }
    impl SRMextension {
        pub fn new(extension_id: RefExt, value: Any) -> Self {
            Self {
                extension_id,
                value,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SRMextensions(pub SequenceOf<SRMextension>);
    #[derive(Debug, Clone, PartialEq)]
    pub enum SRMexts_ExtValue {}
    impl SRMexts_ExtValue {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SRMexts_extRef {}
    impl SRMexts_extRef {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SafetyCarContainer {
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: LightBarSirenInUse,
        #[rasn(identifier = "incidentIndication")]
        pub incident_indication: Option<CauseCode>,
        #[rasn(identifier = "trafficRule")]
        pub traffic_rule: Option<TrafficRule>,
        #[rasn(identifier = "speedLimit")]
        pub speed_limit: Option<SpeedLimit>,
    }
    impl SafetyCarContainer {
        pub fn new(
            light_bar_siren_in_use: LightBarSirenInUse,
            incident_indication: Option<CauseCode>,
            traffic_rule: Option<TrafficRule>,
            speed_limit: Option<SpeedLimit>,
        ) -> Self {
            Self {
                light_bar_siren_in_use,
                incident_indication,
                traffic_rule,
                speed_limit,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Sam {
        pub version: RsvAdvPrtVersion,
        pub body: SamBody,
    }
    impl Sam {
        pub fn new(version: RsvAdvPrtVersion, body: SamBody) -> Self {
            Self { version, body }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SamBody {
        #[rasn(identifier = "changeCount")]
        pub change_count: SrvAdvChangeCount,
        pub extensions: Option<SrvAdvMsgHeaderExts>,
        #[rasn(identifier = "serviceInfos")]
        pub service_infos: Option<ServiceInfos>,
        #[rasn(identifier = "channelInfos")]
        pub channel_infos: Option<ChannelInfos>,
        #[rasn(identifier = "routingAdvertisement")]
        pub routing_advertisement: Option<RoutingAdvertisement>,
    }
    impl SamBody {
        pub fn new(
            change_count: SrvAdvChangeCount,
            extensions: Option<SrvAdvMsgHeaderExts>,
            service_infos: Option<ServiceInfos>,
            channel_infos: Option<ChannelInfos>,
            routing_advertisement: Option<RoutingAdvertisement>,
        ) -> Self {
            Self {
                change_count,
                extensions,
                service_infos,
                channel_infos,
                routing_advertisement,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SamContext {
        #[rasn(identifier = "itsaidCtxRef")]
        pub itsaid_ctx_ref: ItsAidCtxRef,
        pub context: Any,
    }
    impl SamContext {
        pub fn new(itsaid_ctx_ref: ItsAidCtxRef, context: Any) -> Self {
            Self {
                itsaid_ctx_ref,
                context,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-2048..=2047"), identifier = "Scale-B12")]
    pub struct ScaleB12(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SecondaryDns(pub IPv6Address);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8"))]
    pub struct SegmentAttributeXYList(pub SequenceOf<SegmentAttributeXY>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SegmentCount(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=4095"))]
    pub struct SemiAxisLength(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMajorAxisAccuracy(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SemiMajorAxisOrientation(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SemiMinorAxisAccuracy(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct SemiRangeLength(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-5000..=5000"))]
    pub struct SensorHeight(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorIdList(pub SequenceOf<Identifier>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorInformation {
        #[rasn(identifier = "sensorID")]
        pub sensor_i_d: Identifier,
        #[rasn(identifier = "type")]
        pub r_type: SensorType,
        #[rasn(identifier = "detectionArea")]
        pub detection_area: DetectionArea,
        #[rasn(identifier = "freeSpaceConfidence")]
        pub free_space_confidence: Option<FreeSpaceConfidence>,
    }
    impl SensorInformation {
        pub fn new(
            sensor_i_d: Identifier,
            r_type: SensorType,
            detection_area: DetectionArea,
            free_space_confidence: Option<FreeSpaceConfidence>,
        ) -> Self {
            Self {
                sensor_i_d,
                r_type,
                detection_area,
                free_space_confidence,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct SensorType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct SequenceNumber(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfEdcaIdentifier(pub SequenceOf<EdcaIdentifier>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfHashedId3(pub SequenceOf<HashedId3>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfIdentifiedRegion(pub SequenceOf<IdentifiedRegion>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, identifier = "OCTET_STRING")]
    pub struct AnonymousSequenceOfOctetString(pub OctetString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfOctetString(pub SequenceOf<AnonymousSequenceOfOctetString>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfPsid(pub SequenceOf<Psid>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfPsidSsp(pub SequenceOf<PsidSsp>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfPsidSspRange(pub SequenceOf<PsidSspRange>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfRectangularRegion(pub SequenceOf<RectangularRegion>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfRegionAndSubregions(pub SequenceOf<RegionAndSubregions>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8"))]
    pub struct SequenceOfTrajectoryInterceptionIndication(
        pub SequenceOf<TrajectoryInterceptionIndication>,
    );
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfUint16(pub SequenceOf<Uint16>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SequenceOfUint8(pub SequenceOf<Uint8>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=8"))]
    pub struct SequenceOfVruSafeDistanceIndication(pub SequenceOf<VruSafeDistanceIndication>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ServiceCategoryCode {
        trafficSignPictogram(TrafficSignPictogram),
        publicFacilitiesPictogram(PublicFacilitiesPictogram),
        ambientOrRoadConditionPictogram(AmbientOrRoadConditionPictogram),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ServiceInfo {
        #[rasn(identifier = "serviceID")]
        pub service_i_d: ITSaid,
        #[rasn(identifier = "channelIndex")]
        pub channel_index: ChannelIndex,
        #[rasn(identifier = "chOptions")]
        pub ch_options: ChannelOptions,
    }
    impl ServiceInfo {
        pub fn new(
            service_i_d: ITSaid,
            channel_index: ChannelIndex,
            ch_options: ChannelOptions,
        ) -> Self {
            Self {
                service_i_d,
                channel_index,
                ch_options,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ServiceInfoExt {
        #[rasn(identifier = "extensionId")]
        pub extension_id: RefExt,
        pub value: Any,
    }
    impl ServiceInfoExt {
        pub fn new(extension_id: RefExt, value: Any) -> Self {
            Self {
                extension_id,
                value,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ServiceInfoExtTypes_ExtValue {
        CProviderServContext(ProviderServiceContext),
        CIPv6Address(IPv6Address),
        CServicePort(ServicePort),
        CProviderMACaddress(ProviderMacAddress),
        CRCPIthreshold(RcpiThreshold),
        CWSAcountThreshold(WsaCountThreshold),
        CWSAcountThresInt(WsaCountThresholdInterval),
        CSAMapplicationData(SAMapplicationData),
        CProtocolType(ProtocolType),
    }
    impl ServiceInfoExtTypes_ExtValue {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &C__PROVIDER_SERV_CONTEXT => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CProviderServContext)?),
                i if i == &C__I_PV6ADDRESS => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CIPv6Address)?),
                i if i == &C_SERVICE_PORT => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CServicePort)?),
                i if i == &C__PROVIDER_M_A_CADDRESS => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CProviderMACaddress)?),
                i if i == &C__R_C_P_ITHRESHOLD => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CRCPIthreshold)?),
                i if i == &C__W_S_ACOUNT_THRESHOLD => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CWSAcountThreshold)?),
                i if i == &C__W_S_ACOUNT_THRES_INT => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CWSAcountThresInt)?),
                i if i == &C__S_A_MAPPLICATION_DATA => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CSAMapplicationData)?),
                i if i == &C__PROTOCOL_TYPE => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CProtocolType)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::CProviderServContext(inner), i) if i == &C__PROVIDER_SERV_CONTEXT => {
                    inner.encode(encoder)
                }
                (Self::CIPv6Address(inner), i) if i == &C__I_PV6ADDRESS => inner.encode(encoder),
                (Self::CServicePort(inner), i) if i == &C_SERVICE_PORT => inner.encode(encoder),
                (Self::CProviderMACaddress(inner), i) if i == &C__PROVIDER_M_A_CADDRESS => {
                    inner.encode(encoder)
                }
                (Self::CRCPIthreshold(inner), i) if i == &C__R_C_P_ITHRESHOLD => {
                    inner.encode(encoder)
                }
                (Self::CWSAcountThreshold(inner), i) if i == &C__W_S_ACOUNT_THRESHOLD => {
                    inner.encode(encoder)
                }
                (Self::CWSAcountThresInt(inner), i) if i == &C__W_S_ACOUNT_THRES_INT => {
                    inner.encode(encoder)
                }
                (Self::CSAMapplicationData(inner), i) if i == &C__S_A_MAPPLICATION_DATA => {
                    inner.encode(encoder)
                }
                (Self::CProtocolType(inner), i) if i == &C__PROTOCOL_TYPE => inner.encode(encoder),
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ServiceInfoExts(pub SequenceOf<ServiceInfoExt>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ServiceInfos(pub SequenceOf<ServiceInfo>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct ServicePort(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum ServiceSpecificPermissions {
        opaque(OctetString),
        #[rasn(extension_addition)]
        bitmapSsp(BitmapSsp),
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ShadowingApplies(pub bool);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalGroupID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalHeadLocation {
        #[rasn(identifier = "nodeXY")]
        pub node_x_y: NodeOffsetPointXY,
        #[rasn(identifier = "nodeZ")]
        pub node_z: DeltaAltitude,
        #[rasn(identifier = "signalGroupID")]
        pub signal_group_i_d: SignalGroupID,
    }
    impl SignalHeadLocation {
        pub fn new(
            node_x_y: NodeOffsetPointXY,
            node_z: DeltaAltitude,
            signal_group_i_d: SignalGroupID,
        ) -> Self {
            Self {
                node_x_y,
                node_z,
                signal_group_i_d,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=64"))]
    pub struct SignalHeadLocationList(pub SequenceOf<SignalHeadLocation>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestRegional(pub SequenceOf<AnonymousSignalRequestRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequest {
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "requestID")]
        pub request_i_d: RequestID,
        #[rasn(identifier = "requestType")]
        pub request_type: PriorityRequestType,
        #[rasn(identifier = "inBoundLane")]
        pub in_bound_lane: IntersectionAccessPoint,
        #[rasn(identifier = "outBoundLane")]
        pub out_bound_lane: Option<IntersectionAccessPoint>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalRequestRegional>>,
    }
    impl SignalRequest {
        pub fn new(
            id: IntersectionReferenceID,
            request_i_d: RequestID,
            request_type: PriorityRequestType,
            in_bound_lane: IntersectionAccessPoint,
            out_bound_lane: Option<IntersectionAccessPoint>,
            regional: Option<SequenceOf<SignalRequestRegional>>,
        ) -> Self {
            Self {
                id,
                request_i_d,
                request_type,
                in_bound_lane,
                out_bound_lane,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalRequestList(pub SequenceOf<SignalRequestPackage>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestMessageRegional(pub SequenceOf<AnonymousSignalRequestMessageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalRequestMessageRegional>>,
    }
    impl SignalRequestMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            requests: Option<SignalRequestList>,
            requester: RequesterDescription,
            regional: Option<SequenceOf<SignalRequestMessageRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalRequestPackageRegional(pub SequenceOf<AnonymousSignalRequestPackageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalRequestPackage {
        pub request: SignalRequest,
        pub minute: Option<MinuteOfTheYear>,
        pub second: Option<DSecond>,
        pub duration: Option<DSecond>,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalRequestPackageRegional>>,
    }
    impl SignalRequestPackage {
        pub fn new(
            request: SignalRequest,
            minute: Option<MinuteOfTheYear>,
            second: Option<DSecond>,
            duration: Option<DSecond>,
            regional: Option<SequenceOf<SignalRequestPackageRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusRegional(pub SequenceOf<AnonymousSignalStatusRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatus {
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: MsgCount,
        pub id: IntersectionReferenceID,
        #[rasn(identifier = "sigStatus")]
        pub sig_status: SignalStatusPackageList,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalStatusRegional>>,
    }
    impl SignalStatus {
        pub fn new(
            sequence_number: MsgCount,
            id: IntersectionReferenceID,
            sig_status: SignalStatusPackageList,
            regional: Option<SequenceOf<SignalStatusRegional>>,
        ) -> Self {
            Self {
                sequence_number,
                id,
                sig_status,
                regional,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusList(pub SequenceOf<SignalStatus>);
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusMessageRegional(pub SequenceOf<AnonymousSignalStatusMessageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SignalStatusMessage {
        #[rasn(identifier = "timeStamp")]
        pub time_stamp: Option<MinuteOfTheYear>,
        pub second: DSecond,
        #[rasn(identifier = "sequenceNumber")]
        pub sequence_number: Option<MsgCount>,
        pub status: SignalStatusList,
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalStatusMessageRegional>>,
    }
    impl SignalStatusMessage {
        pub fn new(
            time_stamp: Option<MinuteOfTheYear>,
            second: DSecond,
            sequence_number: Option<MsgCount>,
            status: SignalStatusList,
            regional: Option<SequenceOf<SignalStatusMessageRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct SignalStatusPackageRegional(pub SequenceOf<AnonymousSignalStatusPackageRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<SignalStatusPackageRegional>>,
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
            regional: Option<SequenceOf<SignalStatusPackageRegional>>,
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=32"))]
    pub struct SignalStatusPackageList(pub SequenceOf<SignalStatusPackage>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SignalViolationSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Signature {
        ecdsaNistP256Signature(EcdsaP256Signature),
        ecdsaBrainpoolP256r1Signature(EcdsaP256Signature),
        #[rasn(extension_addition)]
        ecdsaBrainpoolP384r1Signature(EcdsaP384Signature),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SituationContainer {
        #[rasn(identifier = "informationQuality")]
        pub information_quality: InformationQuality,
        #[rasn(identifier = "eventType")]
        pub event_type: CauseCode,
        #[rasn(identifier = "linkedCause")]
        pub linked_cause: Option<CauseCode>,
        #[rasn(identifier = "eventHistory")]
        pub event_history: Option<EventHistory>,
    }
    impl SituationContainer {
        pub fn new(
            information_quality: InformationQuality,
            event_type: CauseCode,
            linked_cause: Option<CauseCode>,
            event_history: Option<EventHistory>,
        ) -> Self {
            Self {
                information_quality,
                event_type,
                linked_cause,
                event_history,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SlowVehicleSubCauseCode(pub u8);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SpecialTransportContainer {
        #[rasn(identifier = "specialTransportType")]
        pub special_transport_type: SpecialTransportType,
        #[rasn(identifier = "lightBarSirenInUse")]
        pub light_bar_siren_in_use: LightBarSirenInUse,
    }
    impl SpecialTransportContainer {
        pub fn new(
            special_transport_type: SpecialTransportType,
            light_bar_siren_in_use: LightBarSirenInUse,
        ) -> Self {
            Self {
                special_transport_type,
                light_bar_siren_in_use,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("4"))]
    pub struct SpecialTransportType(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum SpecialVehicleContainer {
        publicTransportContainer(PublicTransportContainer),
        specialTransportContainer(SpecialTransportContainer),
        dangerousGoodsContainer(DangerousGoodsContainer),
        roadWorksContainerBasic(RoadWorksContainerBasic),
        rescueContainer(RescueContainer),
        emergencyContainer(EmergencyContainer),
        safetyCarContainer(SafetyCarContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=500"))]
    pub struct SpeedAdvice(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SpeedConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct SpeedLimit(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=9"))]
    pub struct SpeedLimitList(pub SequenceOf<RegulatorySpeedLimit>);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=16383"))]
    pub struct SpeedValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-16383..=16383"))]
    pub struct SpeedValueExtended(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SpotAvailability {
        #[rasn(value("0..=1400"), identifier = "maxWaitingTimeMinutes")]
        pub max_waiting_time_minutes: u16,
        pub blocking: bool,
    }
    impl SpotAvailability {
        pub fn new(max_waiting_time_minutes: u16, blocking: bool) -> Self {
            Self {
                max_waiting_time_minutes,
                blocking,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct Srm {
        pub header: RsvAdvPrtVersion,
        pub body: SrmBody,
    }
    impl Srm {
        pub fn new(header: RsvAdvPrtVersion, body: SrmBody) -> Self {
            Self { header, body }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SrmBody {
        pub extensions: Option<SRMextensions>,
        #[rasn(identifier = "prvChannelsRq")]
        pub prv_channels_rq: Option<SrmPrivateChannelsRq>,
        pub contexts: Option<SrmContexts>,
        #[rasn(identifier = "prvChannelsCf")]
        pub prv_channels_cf: Option<SrmPrvChAllocConf>,
    }
    impl SrmBody {
        pub fn new(
            extensions: Option<SRMextensions>,
            prv_channels_rq: Option<SrmPrivateChannelsRq>,
            contexts: Option<SrmContexts>,
            prv_channels_cf: Option<SrmPrvChAllocConf>,
        ) -> Self {
            Self {
                extensions,
                prv_channels_rq,
                contexts,
                prv_channels_cf,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SrmContext {
        pub context: SamContext,
        #[rasn(identifier = "clientPort")]
        pub client_port: PortNumber,
    }
    impl SrmContext {
        pub fn new(context: SamContext, client_port: PortNumber) -> Self {
            Self {
                context,
                client_port,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SrmContexts(pub SequenceOf<SrmContext>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SrmPrivateChannelsRq {
        #[rasn(identifier = "portDynSam")]
        pub port_dyn_sam: PortNumber,
        #[rasn(identifier = "allocReqs")]
        pub alloc_reqs: SrmPrvChAllocReq,
    }
    impl SrmPrivateChannelsRq {
        pub fn new(port_dyn_sam: PortNumber, alloc_reqs: SrmPrvChAllocReq) -> Self {
            Self {
                port_dyn_sam,
                alloc_reqs,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SrmPrvChAllocConf(pub SequenceOf<ITSaid>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SrmPrvChAllocReq(pub SequenceOf<ITSaid>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SrvAdvChangeCount {
        #[rasn(identifier = "saID")]
        pub sa_i_d: SrvAdvID,
        #[rasn(identifier = "contentCount")]
        pub content_count: SrvAdvContentCount,
    }
    impl SrvAdvChangeCount {
        pub fn new(sa_i_d: SrvAdvID, content_count: SrvAdvContentCount) -> Self {
            Self {
                sa_i_d,
                content_count,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct SrvAdvContentCount(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct SrvAdvID(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct SrvAdvMsgHeaderExt {
        #[rasn(identifier = "extensionId")]
        pub extension_id: RefExt,
        pub value: Any,
    }
    impl SrvAdvMsgHeaderExt {
        pub fn new(extension_id: RefExt, value: Any) -> Self {
            Self {
                extension_id,
                value,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum SrvAdvMsgHeaderExtTypes_ExtValue {
        CRepeatRate(RepeatRate),
        C2Dlocation(TwoDLocation),
        C3Dlocation(ThreeDLocation),
        CAdvertiserID(AdvertiserIdentifier),
        CExtendedChannelInfos(ExtendedChannelInfos),
    }
    impl SrvAdvMsgHeaderExtTypes_ExtValue {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &RefExt,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &C__REPEAT_RATE => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CRepeatRate)?),
                i if i == &C_2DLOCATION => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::C2Dlocation)?),
                i if i == &C_3DLOCATION => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::C3Dlocation)?),
                i if i == &C_ADVERTISER_I_D => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CAdvertiserID)?),
                i if i == &C__EXTENDED_CHANNEL_INFOS => Ok(decoder
                    .codec()
                    .decode_from_binary(
                        open_type_payload
                            .ok_or_else(|| {
                                rasn::error::DecodeError::from_kind(
                                    rasn::error::DecodeErrorKind::Custom {
                                        msg: "Failed to decode open type! No input data given."
                                            .into(),
                                    },
                                    decoder.codec(),
                                )
                                .into()
                            })?
                            .as_bytes(),
                    )
                    .map(Self::CExtendedChannelInfos)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                )
                .into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &RefExt,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::CRepeatRate(inner), i) if i == &C__REPEAT_RATE => inner.encode(encoder),
                (Self::C2Dlocation(inner), i) if i == &C_2DLOCATION => inner.encode(encoder),
                (Self::C3Dlocation(inner), i) if i == &C_3DLOCATION => inner.encode(encoder),
                (Self::CAdvertiserID(inner), i) if i == &C_ADVERTISER_I_D => inner.encode(encoder),
                (Self::CExtendedChannelInfos(inner), i) if i == &C__EXTENDED_CHANNEL_INFOS => {
                    inner.encode(encoder)
                }
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                )
                .into()),
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SrvAdvMsgHeaderExts(pub SequenceOf<SrvAdvMsgHeaderExt>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum SspRange {
        opaque(SequenceOfOctetString),
        all(()),
        #[rasn(extension_addition)]
        bitmapSspRange(BitmapSspRange),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct StabilityChangeIndication {
        #[rasn(identifier = "lossProbability")]
        pub loss_probability: StabilityLossProbability,
        #[rasn(identifier = "actionDeltaTime")]
        pub action_delta_time: ActionDeltaTime,
    }
    impl StabilityChangeIndication {
        pub fn new(
            loss_probability: StabilityLossProbability,
            action_delta_time: ActionDeltaTime,
        ) -> Self {
            Self {
                loss_probability,
                action_delta_time,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=63"))]
    pub struct StabilityLossProbability(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum StationDataContainer {
        originatingVehicleContainer(OriginatingVehicleContainer),
        originatingRSUContainer(OriginatingRSUContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct StationID(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationType(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum StationarySince {
        lessThan1Minute = 0,
        lessThan2Minutes = 1,
        lessThan15Minutes = 2,
        equalOrGreater15Minutes = 3,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct StationaryVehicleContainer {
        #[rasn(identifier = "stationarySince")]
        pub stationary_since: Option<StationarySince>,
        #[rasn(identifier = "stationaryCause")]
        pub stationary_cause: Option<CauseCode>,
        #[rasn(identifier = "carryingDangerousGoods")]
        pub carrying_dangerous_goods: Option<DangerousGoodsExtended>,
        #[rasn(identifier = "numberOfOccupants")]
        pub number_of_occupants: Option<NumberOfOccupants>,
        #[rasn(identifier = "vehicleIdentification")]
        pub vehicle_identification: Option<VehicleIdentification>,
        #[rasn(identifier = "energyStorageType")]
        pub energy_storage_type: Option<EnergyStorageType>,
    }
    impl StationaryVehicleContainer {
        pub fn new(
            stationary_since: Option<StationarySince>,
            stationary_cause: Option<CauseCode>,
            carrying_dangerous_goods: Option<DangerousGoodsExtended>,
            number_of_occupants: Option<NumberOfOccupants>,
            vehicle_identification: Option<VehicleIdentification>,
            energy_storage_type: Option<EnergyStorageType>,
        ) -> Self {
            Self {
                stationary_since,
                stationary_cause,
                carrying_dangerous_goods,
                number_of_occupants,
                vehicle_identification,
                energy_storage_type,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct StationaryVehicleSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SteeringWheelAngleConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-511..=512"))]
    pub struct SteeringWheelAngleValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct SubCauseCodeType(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1"))]
    pub struct SubjectAssurance(pub OctetString);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum SymmAlgorithm {
        aes128Ccm = 0,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum SymmetricEncryptionKey {
        #[rasn(size("16"))]
        aes128Ccm(OctetString),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SystemService(pub SequenceOf<SystemServiceAndContext>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct SystemServiceAndContext(pub SamContext);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-128..=127"))]
    pub struct TXpower80211(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-60..=67"))]
    pub struct Temperature(pub i8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("4"))]
    pub struct TemporaryID(pub OctetString);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum Termination {
        isCancellation = 0,
        isNegation = 1,
    }
    #[doc = " Anonymous SEQUENCE OF member "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags, identifier = "SEQUENCE")]
    pub struct AnonymousTestDataRegional {
        #[rasn(identifier = "regionId")]
        pub region_id: RegionId,
        #[rasn(identifier = "regExtValue")]
        pub reg_ext_value: Any,
    }
    impl AnonymousTestDataRegional {
        pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
            Self {
                region_id,
                reg_ext_value,
            }
        }
    }
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=4"))]
    pub struct TestDataRegional(pub SequenceOf<AnonymousTestDataRegional>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct TestData {
        #[rasn(size("1..=4"))]
        pub regional: Option<SequenceOf<TestDataRegional>>,
    }
    impl TestData {
        pub fn new(regional: Option<SequenceOf<TestDataRegional>>) -> Self {
            Self { regional }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=16", extensible))]
    pub struct TextContainer(pub SequenceOf<TcPart>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ThreeDLocation {
        pub latitude: SALatitude,
        pub longitude: SALongitude,
        pub elevation: SAElevation,
    }
    impl ThreeDLocation {
        pub fn new(latitude: SALatitude, longitude: SALongitude, elevation: SAElevation) -> Self {
            Self {
                latitude,
                longitude,
                elevation,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Time32(pub Uint32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct Time64(pub Uint64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum TimeConfidence {
        unavailable = 0,
        time_100_000 = 1,
        time_050_000 = 2,
        time_020_000 = 3,
        time_010_000 = 4,
        time_002_000 = 5,
        time_001_000 = 6,
        time_000_500 = 7,
        time_000_200 = 8,
        time_000_100 = 9,
        time_000_050 = 10,
        time_000_020 = 11,
        time_000_010 = 12,
        time_000_005 = 13,
        time_000_002 = 14,
        time_000_001 = 15,
        time_000_000_5 = 16,
        time_000_000_2 = 17,
        time_000_000_1 = 18,
        time_000_000_05 = 19,
        time_000_000_02 = 20,
        time_000_000_01 = 21,
        time_000_000_005 = 22,
        time_000_000_002 = 23,
        time_000_000_001 = 24,
        time_000_000_000_5 = 25,
        time_000_000_000_2 = 26,
        time_000_000_000_1 = 27,
        time_000_000_000_05 = 28,
        time_000_000_000_02 = 29,
        time_000_000_000_01 = 30,
        time_000_000_000_005 = 31,
        time_000_000_000_002 = 32,
        time_000_000_000_001 = 33,
        time_000_000_000_000_5 = 34,
        time_000_000_000_000_2 = 35,
        time_000_000_000_000_1 = 36,
        time_000_000_000_000_05 = 37,
        time_000_000_000_000_02 = 38,
        time_000_000_000_000_01 = 39,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=15"))]
    pub struct TimeIntervalConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=36001"))]
    pub struct TimeMark(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1500..=1500"))]
    pub struct TimeOfMeasurement(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=60000"))]
    pub struct TimeReference(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=4398046511103"))]
    pub struct TimestampIts(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=7"))]
    pub struct Traces(pub SequenceOf<PathHistory>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct TrafficConditionSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct TrafficIslandPosition {
        #[rasn(identifier = "oneSide")]
        pub one_side: NonIslandLanePosition,
        #[rasn(identifier = "otherSide")]
        pub other_side: NonIslandLanePosition,
    }
    impl TrafficIslandPosition {
        pub fn new(one_side: NonIslandLanePosition, other_side: NonIslandLanePosition) -> Self {
            Self {
                one_side,
                other_side,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TrafficRule {
        noPassing = 0,
        noPassingForTrucks = 1,
        passToRight = 2,
        passToLeft = 3,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    pub enum TrafficSignPictogram {
        dangerWarning = 0,
        regulatory = 1,
        informative = 2,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=2"))]
    pub struct TrailerDataContainer(pub SequenceOf<TrailerData>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct TrainCharacteristics(pub TractorCharacteristics);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct TrajectoryInterceptionIndication {
        #[rasn(identifier = "subjectStation")]
        pub subject_station: Option<StationID>,
        #[rasn(identifier = "trajectoryInterceptionIndication")]
        pub trajectory_interception_indication: bool,
    }
    impl TrajectoryInterceptionIndication {
        pub fn new(
            subject_station: Option<StationID>,
            trajectory_interception_indication: bool,
        ) -> Self {
            Self {
                subject_station,
                trajectory_interception_indication,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct TransitVehicleStatus(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=10000"))]
    pub struct TransmissionInterval(pub u16);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct TurningRadius(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct TwoDLocation {
        pub latitude: SALatitude,
        pub longitude: SALongitude,
    }
    impl TwoDLocation {
        pub fn new(latitude: SALatitude, longitude: SALongitude) -> Self {
            Self {
                latitude,
                longitude,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct TypeOfReceptacle(pub BitString);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct URLOfMDUs {
        #[rasn(identifier = "startingMDU")]
        pub starting_m_d_u: Option<SequenceNumber>,
        #[rasn(identifier = "endingMDU")]
        pub ending_m_d_u: Option<SequenceNumber>,
        pub url: Ia5String,
    }
    impl URLOfMDUs {
        pub fn new(
            starting_m_d_u: Option<SequenceNumber>,
            ending_m_d_u: Option<SequenceNumber>,
            url: Ia5String,
        ) -> Self {
            Self {
                starting_m_d_u,
                ending_m_d_u,
                url,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct Uint16(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct Uint3(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=4294967295"))]
    pub struct Uint32(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=18446744073709551615"))]
    pub struct Uint64(pub u64);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Uint8(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum UnitType {
        mg_km = 0,
        mg_kWh = 1,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("900000001"))]
    pub struct UnknownLatitude(pub NinetyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("1800000001"))]
    pub struct UnknownLongitude(pub OneEightyDegreeInt);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct VAM {
        pub header: ItsPduHeader,
        pub vam: VruAwareness,
    }
    impl VAM {
        pub fn new(header: ItsPduHeader, vam: VruAwareness) -> Self {
            Self { header, vam }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("6"))]
    pub struct VDS(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=86400"))]
    pub struct ValidityDuration(pub u32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct ValidityPeriod {
        pub start: Time32,
        pub duration: Duration,
    }
    impl ValidityPeriod {
        pub fn new(start: Time32, duration: Duration) -> Self {
            Self { start, duration }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VamParameters {
        #[rasn(identifier = "basicContainer")]
        pub basic_container: BasicContainer,
        #[rasn(identifier = "vruHighFrequencyContainer")]
        pub vru_high_frequency_container: Option<VruHighFrequencyContainer>,
        #[rasn(identifier = "vruLowFrequencyContainer")]
        pub vru_low_frequency_container: Option<VruLowFrequencyContainer>,
        #[rasn(identifier = "vruClusterInformationContainer")]
        pub vru_cluster_information_container: Option<VruClusterInformationContainer>,
        #[rasn(identifier = "vruClusterOperationContainer")]
        pub vru_cluster_operation_container: Option<VruClusterOperationContainer>,
        #[rasn(identifier = "vruMotionPredictionContainer")]
        pub vru_motion_prediction_container: Option<VruMotionPredictionContainer>,
    }
    impl VamParameters {
        pub fn new(
            basic_container: BasicContainer,
            vru_high_frequency_container: Option<VruHighFrequencyContainer>,
            vru_low_frequency_container: Option<VruLowFrequencyContainer>,
            vru_cluster_information_container: Option<VruClusterInformationContainer>,
            vru_cluster_operation_container: Option<VruClusterOperationContainer>,
            vru_motion_prediction_container: Option<VruMotionPredictionContainer>,
        ) -> Self {
            Self {
                basic_container,
                vru_high_frequency_container,
                vru_low_frequency_container,
                vru_cluster_information_container,
                vru_cluster_operation_container,
                vru_motion_prediction_container,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum VarLengthNumber {
        #[rasn(value("0..=127"))]
        content(u8),
        extension(Ext1),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct VcClass(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=7"))]
    pub struct VcOption(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct VehicleBreakdownSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=127"))]
    pub struct VehicleHeight(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    pub enum VehicleID {
        entityID(TemporaryID),
        stationID(StationID),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VehicleIdentification {
        #[rasn(identifier = "wMInumber")]
        pub w_m_inumber: Option<WMInumber>,
        #[rasn(identifier = "vDS")]
        pub v_d_s: Option<VDS>,
    }
    impl VehicleIdentification {
        pub fn new(w_m_inumber: Option<WMInumber>, v_d_s: Option<VDS>) -> Self {
            Self { w_m_inumber, v_d_s }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VehicleLengthConfidenceIndication {
        noTrailerPresent = 0,
        trailerPresentWithKnownLength = 1,
        trailerPresentWithUnknownLength = 2,
        trailerPresenceIsUnknown = 3,
        unavailable = 4,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=1023"))]
    pub struct VehicleLengthValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=1024"))]
    pub struct VehicleMass(pub u16);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=10"))]
    pub struct VehicleSensorPropertyList(pub SequenceOf<VehicleSensorProperties>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    fn vehicle_subclass_r_type_default() -> VehicleSubclassType {
        VehicleSubclassType(0)
    }
    fn vehicle_subclass_confidence_default() -> ClassConfidence {
        ClassConfidence(0)
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct VehicleSubclassType(pub u8);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=62"))]
    pub struct VehicleWidth(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=8191"))]
    pub struct Velocity(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-160..=161"))]
    pub struct VerticalAccelerationValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct VruAwareness {
        #[rasn(identifier = "generationDeltaTime")]
        pub generation_delta_time: GenerationDeltaTime,
        #[rasn(identifier = "vamParameters")]
        pub vam_parameters: VamParameters,
    }
    impl VruAwareness {
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
            vam_parameters: VamParameters,
        ) -> Self {
            Self {
                generation_delta_time,
                vam_parameters,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruClusterInformationContainer {
        #[rasn(identifier = "clusterId")]
        pub cluster_id: ClusterId,
        #[rasn(identifier = "clusterBoundingBoxShape")]
        pub cluster_bounding_box_shape: ClusterBoundingBoxShape,
        #[rasn(value("0..=255"), identifier = "clusterCardinalitySize")]
        pub cluster_cardinality_size: u8,
        #[rasn(identifier = "clusterProfiles")]
        pub cluster_profiles: ClusterProfiles,
    }
    impl VruClusterInformationContainer {
        pub fn new(
            cluster_id: ClusterId,
            cluster_bounding_box_shape: ClusterBoundingBoxShape,
            cluster_cardinality_size: u8,
            cluster_profiles: ClusterProfiles,
        ) -> Self {
            Self {
                cluster_id,
                cluster_bounding_box_shape,
                cluster_cardinality_size,
                cluster_profiles,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=255"))]
    pub struct VruClusterOpTimestamp(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruClusterOperationContainer {
        #[rasn(identifier = "clusterJoinInfo")]
        pub cluster_join_info: Option<ClusterJoinInfo>,
        #[rasn(identifier = "clusterLeaveInfo")]
        pub cluster_leave_info: Option<ClusterLeaveInfo>,
        #[rasn(identifier = "clusterBreakupInfo")]
        pub cluster_breakup_info: Option<ClusterBreakupInfo>,
        #[rasn(identifier = "clusterIdChangeInfo")]
        pub cluster_id_change_info: Option<VruClusterOpTimestamp>,
    }
    impl VruClusterOperationContainer {
        pub fn new(
            cluster_join_info: Option<ClusterJoinInfo>,
            cluster_leave_info: Option<ClusterLeaveInfo>,
            cluster_breakup_info: Option<ClusterBreakupInfo>,
            cluster_id_change_info: Option<VruClusterOpTimestamp>,
        ) -> Self {
            Self {
                cluster_join_info,
                cluster_leave_info,
                cluster_breakup_info,
                cluster_id_change_info,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruDeviceUsage {
        unavailable = 0,
        other = 1,
        idle = 2,
        listeningToAudio = 3,
        typing = 4,
        calling = 5,
        playingGames = 6,
        reading = 7,
        viewing = 8,
        max = 255,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruEnvironment {
        unavailable = 0,
        intersectionCrossing = 1,
        zebraCrossing = 2,
        sidewalk = 3,
        onVehicleRoad = 4,
        protectedGeographicArea = 5,
        max = 255,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct VruExteriorLights {
        #[rasn(identifier = "vruSpecific")]
        pub vru_specific: VruSpecificExteriorLights,
        pub generic: ExteriorLights,
    }
    impl VruExteriorLights {
        pub fn new(vru_specific: VruSpecificExteriorLights, generic: ExteriorLights) -> Self {
            Self {
                vru_specific,
                generic,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruHighFrequencyContainer {
        pub heading: Heading,
        pub speed: Speed,
        #[rasn(identifier = "longitudinalAcceleration")]
        pub longitudinal_acceleration: LongitudinalAcceleration,
        pub curvature: Option<Curvature>,
        #[rasn(identifier = "curvatureCalculationMode")]
        pub curvature_calculation_mode: Option<CurvatureCalculationMode>,
        #[rasn(identifier = "yawRate")]
        pub yaw_rate: Option<YawRate>,
        #[rasn(identifier = "lateralAcceleration")]
        pub lateral_acceleration: Option<LateralAcceleration>,
        #[rasn(identifier = "verticalAcceleration")]
        pub vertical_acceleration: Option<VerticalAcceleration>,
        #[rasn(identifier = "vruLanePosition")]
        pub vru_lane_position: Option<VruLanePosition>,
        pub environment: Option<VruEnvironment>,
        #[rasn(identifier = "movementControl")]
        pub movement_control: Option<VruMovementControl>,
        pub orientation: Option<VruOrientation>,
        #[rasn(identifier = "rollAngle")]
        pub roll_angle: Option<VruRollAngle>,
        #[rasn(identifier = "deviceUsage")]
        pub device_usage: Option<VruDeviceUsage>,
    }
    impl VruHighFrequencyContainer {
        pub fn new(
            heading: Heading,
            speed: Speed,
            longitudinal_acceleration: LongitudinalAcceleration,
            curvature: Option<Curvature>,
            curvature_calculation_mode: Option<CurvatureCalculationMode>,
            yaw_rate: Option<YawRate>,
            lateral_acceleration: Option<LateralAcceleration>,
            vertical_acceleration: Option<VerticalAcceleration>,
            vru_lane_position: Option<VruLanePosition>,
            environment: Option<VruEnvironment>,
            movement_control: Option<VruMovementControl>,
            orientation: Option<VruOrientation>,
            roll_angle: Option<VruRollAngle>,
            device_usage: Option<VruDeviceUsage>,
        ) -> Self {
            Self {
                heading,
                speed,
                longitudinal_acceleration,
                curvature,
                curvature_calculation_mode,
                yaw_rate,
                lateral_acceleration,
                vertical_acceleration,
                vru_lane_position,
                environment,
                movement_control,
                orientation,
                roll_angle,
                device_usage,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VruLanePosition {
        offRoadLanePosition(OffRoadLanePosition),
        vehicularLanePosition(LanePosition),
        trafficIslandPosition(TrafficIslandPosition),
        mapPosition(MapPosition),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruLowFrequencyContainer {
        #[rasn(identifier = "profileAndSubprofile")]
        pub profile_and_subprofile: Option<VruProfileAndSubprofile>,
        #[rasn(identifier = "exteriorLights")]
        pub exterior_lights: Option<VruExteriorLights>,
        #[rasn(identifier = "sizeClass")]
        pub size_class: Option<VruSizeClass>,
    }
    impl VruLowFrequencyContainer {
        pub fn new(
            profile_and_subprofile: Option<VruProfileAndSubprofile>,
            exterior_lights: Option<VruExteriorLights>,
            size_class: Option<VruSizeClass>,
        ) -> Self {
            Self {
                profile_and_subprofile,
                exterior_lights,
                size_class,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruMotionPredictionContainer {
        #[rasn(identifier = "pathHistory")]
        pub path_history: Option<PathHistory>,
        #[rasn(identifier = "pathPrediction")]
        pub path_prediction: Option<PathPrediction>,
        #[rasn(identifier = "safeDistance")]
        pub safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
        #[rasn(identifier = "trajectoryChangeIndication")]
        pub trajectory_change_indication: Option<SequenceOfTrajectoryInterceptionIndication>,
        #[rasn(identifier = "accelerationChangeIndication")]
        pub acceleration_change_indication: Option<AccelerationChangeIndication>,
        #[rasn(identifier = "headingChangeIndication")]
        pub heading_change_indication: Option<HeadingChangeIndication>,
        #[rasn(identifier = "stabilityChangeIndication")]
        pub stability_change_indication: Option<StabilityChangeIndication>,
    }
    impl VruMotionPredictionContainer {
        pub fn new(
            path_history: Option<PathHistory>,
            path_prediction: Option<PathPrediction>,
            safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
            trajectory_change_indication: Option<SequenceOfTrajectoryInterceptionIndication>,
            acceleration_change_indication: Option<AccelerationChangeIndication>,
            heading_change_indication: Option<HeadingChangeIndication>,
            stability_change_indication: Option<StabilityChangeIndication>,
        ) -> Self {
            Self {
                path_history,
                path_prediction,
                safe_distance,
                trajectory_change_indication,
                acceleration_change_indication,
                heading_change_indication,
                stability_change_indication,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruMovementControl {
        unavailable = 0,
        braking = 1,
        hardBraking = 2,
        stopPedaling = 3,
        noReaction = 4,
        max = 255,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct VruOrientation(pub Heading);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruProfile {
        unavailable = 0,
        pedestrian = 1,
        cyclist = 2,
        motorcyclist = 3,
        animal = 4,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum VruProfileAndSubprofile {
        pedestrian(VruSubProfilePedestrian),
        bicyclist(VruSubProfileBicyclist),
        motorcylist(VruSubProfileMotorcyclist),
        animal(VruSubProfileAnimal),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct VruRollAngle(pub SteeringWheelAngle);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct VruSafeDistanceIndication {
        #[rasn(identifier = "subjectStation")]
        pub subject_station: Option<StationID>,
        #[rasn(identifier = "stationSafeDistanceIndication")]
        pub station_safe_distance_indication: bool,
        #[rasn(identifier = "timeToCollision")]
        pub time_to_collision: Option<ActionDeltaTime>,
    }
    impl VruSafeDistanceIndication {
        pub fn new(
            subject_station: Option<StationID>,
            station_safe_distance_indication: bool,
            time_to_collision: Option<ActionDeltaTime>,
        ) -> Self {
            Self {
                subject_station,
                station_safe_distance_indication,
                time_to_collision,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruSizeClass {
        unavailable = 0,
        low = 1,
        medium = 2,
        high = 3,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("8"))]
    pub struct VruSpecificExteriorLights(pub BitString);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruSubProfileAnimal {
        unavailable = 0,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruSubProfileBicyclist {
        unavailable = 0,
        bicyclist = 1,
        wheelchair_user = 2,
        horse_and_rider = 3,
        rollerskater = 4,
        e_scooter = 5,
        personal_transporter = 6,
        pedelec = 7,
        speed_pedelec = 8,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruSubProfileMotorcyclist {
        unavailable = 0,
        moped = 1,
        motorcycle = 2,
        motorcycle_and_sidecar_right = 3,
        motorcycle_and_sidecar_left = 4,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum VruSubProfilePedestrian {
        unavailable = 0,
        ordinary_pedestrian = 1,
        road_worker = 2,
        first_responder = 3,
        max = 15,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=3601"))]
    pub struct WGS84AngleValue(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=3"))]
    pub struct WMInumber(pub Ia5String);
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct WaitOnStopline(pub bool);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct WheelBaseVehicle(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct WrongWayDrivingSubCauseCode(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct WsaChInfoDataRate {
        #[rasn(size("1"))]
        pub adaptable: BitString,
        #[rasn(value("0..=127"), identifier = "dataRate")]
        pub data_rate: u8,
    }
    impl WsaChInfoDataRate {
        pub fn new(adaptable: BitString, data_rate: u8) -> Self {
            Self {
                adaptable,
                data_rate,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct WsaCountThreshold(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct WsaCountThresholdInterval(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct WsaSsp {
        pub version: Uint8,
        #[rasn(identifier = "advertiserPermissions")]
        pub advertiser_permissions: Option<AdvertiserPermissions>,
        #[rasn(identifier = "providerPermissions")]
        pub provider_permissions: Option<ProviderPermissions>,
    }
    impl WsaSsp {
        pub fn new(
            version: Uint8,
            advertiser_permissions: Option<AdvertiserPermissions>,
            provider_permissions: Option<ProviderPermissions>,
        ) -> Self {
            Self {
                version,
                advertiser_permissions,
                provider_permissions,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-5000..=0"))]
    pub struct XSensorOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-1000..=1000"))]
    pub struct YSensorOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(enumerated)]
    pub enum YawRateConfidence {
        degSec_000_01 = 0,
        degSec_000_05 = 1,
        degSec_000_10 = 2,
        degSec_001_00 = 3,
        degSec_005_00 = 4,
        degSec_010_00 = 5,
        degSec_100_00 = 6,
        outOfRange = 7,
        unavailable = 8,
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("-32766..=32767"))]
    pub struct YawRateValue(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=1000"))]
    pub struct ZSensorOffset(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=32", extensible))]
    pub struct Zid(pub Integer);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum Zone {
        segment(Segment),
        area(PolygonalLine),
        computedSegment(ComputedSegment),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct ZoneLength(pub u16);
    pub const ADD_GRP_A: RegionId = RegionId(1);
    pub const ADD_GRP_B: RegionId = RegionId(2);
    pub const ADD_GRP_C: RegionId = RegionId(3);
    pub const C_2DLOCATION: RefExt = RefExt(5);
    pub const C_3DLOCATION: RefExt = RefExt(6);
    pub const C__CHANNEL_ACCESS: RefExt = RefExt(21);
    pub const C__CHANNEL_LOAD: RefExt = RefExt(23);
    pub const C__CHANNEL_NUMBER80211: RefExt = RefExt(15);
    pub const C__COUNTRY_STRING: RefExt = RefExt(18);
    pub const C__DATA_RATE80211: RefExt = RefExt(16);
    pub const C__E_D_C_APARAMETER_SET: RefExt = RefExt(12);
    pub const C__EXTENDED_CHANNEL_INFOS: RefExt = RefExt(84);
    pub const C__GATEWAY_M_A_CADDRESS: RefExt = RefExt(14);
    pub const C__I_PV6ADDRESS: RefExt = RefExt(9);
    pub const C__L_MCHANNEL_BUSY_RATIO: RefExt = RefExt(82);
    pub const C__L_MPACKET_I_D: RefExt = RefExt(83);
    pub const C__L_MRX_CIP: RefExt = RefExt(81);
    pub const C__L_MTX_CIP: RefExt = RefExt(80);
    pub const C__PROTOCOL_TYPE: RefExt = RefExt(24);
    pub const C__PROVIDER_M_A_CADDRESS: RefExt = RefExt(11);
    pub const C__PROVIDER_SERV_CONTEXT: RefExt = RefExt(8);
    pub const C__R_C_P_ITHRESHOLD: RefExt = RefExt(19);
    pub const C__REPEAT_RATE: RefExt = RefExt(17);
    pub const C__RESERVED: RefExt = RefExt(0);
    pub const C__S_A_MAPPLICATION_DATA: RefExt = RefExt(85);
    pub const C__SECONDARY_D_N_S: RefExt = RefExt(13);
    pub const C__TX_POWER_USED80211: RefExt = RefExt(4);
    pub const C__W_S_ACOUNT_THRES_INT: RefExt = RefExt(22);
    pub const C__W_S_ACOUNT_THRESHOLD: RefExt = RefExt(20);
    pub const C_ADVERTISER_I_D: RefExt = RefExt(7);
    pub const C_CTX_REF_MAND_APP: CtxRef = CtxRef(1);
    pub const C_CTX_REF_NULL: CtxRef = CtxRef(0);
    pub const C_SERVICE_PORT: RefExt = RefExt(10);
    pub const CH_INFO_TYPE_15628: MedType = MedType(128);
    pub const CH_INFO_TYPE_2G: MedType = MedType(2);
    pub const CH_INFO_TYPE_3G: MedType = MedType(3);
    pub const CH_INFO_TYPE_6LOW_PAN: MedType = MedType(11);
    pub const CH_INFO_TYPE_80216E: MedType = MedType(7);
    pub const CH_INFO_TYPE_80220: MedType = MedType(9);
    pub const CH_INFO_TYPE__C_A_N: MedType = MedType(254);
    pub const CH_INFO_TYPE__ETHERNET: MedType = MedType(255);
    pub const CH_INFO_TYPE__H_C__S_D_M_A: MedType = MedType(8);
    pub const CH_INFO_TYPE__I_R: MedType = MedType(4);
    pub const CH_INFO_TYPE__L_T_E: MedType = MedType(10);
    pub const CH_INFO_TYPE__M5: MedType = MedType(5);
    pub const CH_INFO_TYPE__M_M: MedType = MedType(6);
    pub const CH_INFO_TYPE_ANY: MedType = MedType(1);
    pub const CH_INFO_TYPE_UNKNOWN: MedType = MedType(0);
    lazy_static! {
        pub static ref DEFAULT_VALIDITY: ValidityDuration = ValidityDuration(600);
    }
    pub const DIESEL: FuelType = FuelType(3);
    pub const ELECTRIC: FuelType = FuelType(4);
    pub const ETHANOL: FuelType = FuelType(2);
    pub const GASOLINE: FuelType = FuelType(1);
    pub const HYBRID: FuelType = FuelType(5);
    pub const HYDROGEN: FuelType = FuelType(6);
    pub const MAP_DATA: DSRCmsgID = DSRCmsgID(18);
    pub const NAT_GAS_COMP: FuelType = FuelType(8);
    pub const NAT_GAS_LIQUID: FuelType = FuelType(7);
    pub const NO_REGION: RegionId = RegionId(0);
    pub const PROPANE: FuelType = FuelType(9);
    pub const RTCM_CORRECTIONS: DSRCmsgID = DSRCmsgID(28);
    pub const SIGNAL_PHASE_AND_TIMING_MESSAGE: DSRCmsgID = DSRCmsgID(19);
    pub const SIGNAL_REQUEST_MESSAGE: DSRCmsgID = DSRCmsgID(29);
    pub const SIGNAL_STATUS_MESSAGE: DSRCmsgID = DSRCmsgID(30);
    pub const UNKNOWN_FUEL: FuelType = FuelType(0);
