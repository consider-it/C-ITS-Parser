#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_pdu_descriptions {
    extern crate alloc;
    use core::borrow::Borrow;

    use rasn::prelude::*;

    use super::super::cam_1_4_1::cam_pdu_descriptions::GenerationDeltaTime;
    use super::super::cdd_1_3_1_1::its_container::{
        DriveDirection,
        Heading,
        ItsPduHeader,
        LateralAcceleration,
        LongitudinalAcceleration,
        ReferencePosition,
        Speed,
        SpeedConfidence,
        StationType,
        VehicleLength,
        VehicleWidth,
        VerticalAcceleration,
        YawRate,
    };
    use super::super::dsrc_2_2_1::etsi_its_dsrc::{
        IntersectionReferenceID,
        LaneID,
        NodeOffsetPointXY,
        OffsetB10,
        OffsetB11,
        OffsetB12,
        OffsetB13,
        OffsetB14,
        OffsetB16,
        RoadSegmentReferenceID,
        VehicleHeight,
    };
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
    #[doc = " The root data frame for collective perception message"]
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
    #[rasn(delegate, value("0..=102"))]
    pub struct DistanceConfidence(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-132768..=132767"))]
    pub struct DistanceValue(pub i32);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=2"))]
    pub struct DynamicStatus(pub u8);
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
    #[rasn(delegate, value("0..=100"))]
    pub struct HitchPointOffset(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct Identifier(pub u8);
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
    #[rasn(delegate, value("0..=255"))]
    pub struct NumberOfPerceivedObjects(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1500"))]
    pub struct ObjectAge(pub u16);
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    pub enum ObjectClassClass {
        vehicle(VehicleSubclass),
        person(PersonSubclass),
        animal(AnimalSubclass),
        other(OtherSubclass),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct ObjectClass {
        pub confidence: ClassConfidence,
        pub class: ObjectClassClass,
    }
    impl ObjectClass {
        pub fn new(confidence: ClassConfidence, class: ObjectClassClass) -> Self {
            Self { confidence, class }
        }
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
    #[rasn(automatic_tags)]
    pub struct OffsetPoint {
        #[rasn(value("0.."), identifier = "nodeOffsetPointxy")]
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
    #[rasn(delegate, size("3..=16", extensible))]
    pub struct PolyPointList(pub SequenceOf<OffsetPoint>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Radius(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=10000"))]
    pub struct Range(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=150"))]
    pub struct RearOverhang(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=255"))]
    pub struct RefPointId(pub u8);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=127"))]
    pub struct SegmentCount(pub u8);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(delegate)]
    pub struct ShadowingApplies(pub bool);
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
    #[rasn(delegate, value("-16383..=16383"))]
    pub struct SpeedValueExtended(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum StationDataContainer {
        originatingVehicleContainer(OriginatingVehicleContainer),
        originatingRSUContainer(OriginatingRSUContainer),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1500..=1500"))]
    pub struct TimeOfMeasurement(pub i16);
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-5000..=0"))]
    pub struct XSensorOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("-1000..=1000"))]
    pub struct YSensorOffset(pub i16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=1000"))]
    pub struct ZSensorOffset(pub u16);
}
