#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_originating_station_containers {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::{
        CartesianAngle, MapReference, Speed, StationType, TrailerData, Wgs84Angle,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "This DF  represents the Originating RSU Container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field mapReference: identifies the MAPEM containing the topology information reference in the Perceived Object Container"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OriginatingRsuContainer {
        #[rasn(identifier = "mapReference")]
        pub map_reference: Option<MapReference>,
    }
    impl OriginatingRsuContainer {
        pub fn new(map_reference: Option<MapReference>) -> Self {
            Self { map_reference }
        }
    }

    #[doc = "This DF  represents the Originating Vehicle Container"]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field orientationAngle: the angle and angle accuracy of the absolute orientation of the disseminating vehicle in the WGS84 coordinate system with respect to true North."]
    #[doc = ""]
    #[doc = "\n@field pitchAngle: the optional angle and angle accuracy between the ground plane and the current orientation of the vehicle's x-axis with respect to the ground plane about the y-axis according to the ISO 8855."]
    #[doc = ""]
    #[doc = "\n@field rollAngle: the optional angle and angle accuracy between the ground plane and the current orientation of a vehicle's y-axis with respect to the ground plane about the x-axis according to the ISO 8855"]
    #[doc = ""]
    #[doc = "\n@field trailerData: information about the trailer dimensions and orientation in case a trailer is present."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OriginatingVehicleContainer {
        #[rasn(identifier = "orientationAngle")]
        pub orientation_angle: Wgs84Angle,
        #[rasn(identifier = "pitchAngle")]
        pub pitch_angle: Option<CartesianAngle>,
        #[rasn(identifier = "rollAngle")]
        pub roll_angle: Option<CartesianAngle>,
        #[rasn(identifier = "trailerDataSet")]
        pub trailer_data_set: Option<TrailerDataSet>,
    }
    impl OriginatingVehicleContainer {
        pub fn new(
            orientation_angle: Wgs84Angle,
            pitch_angle: Option<CartesianAngle>,
            roll_angle: Option<CartesianAngle>,
            trailer_data_set: Option<TrailerDataSet>,
        ) -> Self {
            Self {
                orientation_angle,
                pitch_angle,
                roll_angle,
                trailer_data_set,
            }
        }
    }

    #[doc = "This DF  represents a list of trailer data."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct TrailerDataSet(pub SequenceOf<TrailerData>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_pdu_descriptions {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::{
        ItsPduHeader, MessageRateHz, MessageSegmentationInfo, OrdinalNumber1B, ReferencePosition,
        StationType, TimestampIts,
    };
    use super::cpm_originating_station_containers::{
        OriginatingRsuContainer, OriginatingVehicleContainer,
    };
    use super::cpm_perceived_object_container::PerceivedObjectContainer;
    use super::cpm_perception_region_container::PerceptionRegionContainer;
    use super::cpm_sensor_information_container::SensorInformationContainer;
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "This DF  represents the Collective Perception Message (CPM) and is the top level Protocol Data Unit. "]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field header: the common message header for the facilities layer message. "]
    #[doc = ""]
    #[doc = "\n@field payload: the payload of the message. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct CollectivePerceptionMessage {
        #[rasn(value("0.."))]
        pub header: ItsPduHeader,
        pub payload: CpmPayload,
    }
    impl CollectivePerceptionMessage {
        pub fn new(header: ItsPduHeader, payload: CpmPayload) -> Self {
            Self { header, payload }
        }
    }

    #[doc = "This DF represents a list of CPM containers, each with their type identifier with an additional constraint. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate)]
    pub struct ConstraintWrappedCpmContainers(pub WrappedCpmContainers);

    #[doc = "This DE represents the identifier of the container type. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("1..=16"))]
    pub struct CpmContainerId(pub u8);

    #[doc = "This DF  represents the payload of the CPM. "]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field managementContainer: the management container. "]
    #[doc = ""]
    #[doc = "\n@field cpmContainers: the list of CPM containers, including its container type identifier and including either one or none of originatingVehicleContainer and/or originatingRsuContainer. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CpmPayload {
        #[rasn(identifier = "managementContainer")]
        pub management_container: ManagementContainer,
        #[rasn(identifier = "cpmContainers")]
        pub cpm_containers: ConstraintWrappedCpmContainers,
    }
    impl CpmPayload {
        pub fn new(
            management_container: ManagementContainer,
            cpm_containers: ConstraintWrappedCpmContainers,
        ) -> Self {
            Self {
                management_container,
                cpm_containers,
            }
        }
    }

    #[doc = "This DF  represents the management container of the CPM. "]
    #[doc = "The management container provides basic information about the originating ITS-S, which are not specific to a specific type of station."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field referenceTime: the reference time for all time related information in the CPM."]
    #[doc = ""]
    #[doc = "\n@field messageRateRange: the planned or expected range of the CPM generation rate."]
    #[doc = ""]
    #[doc = "\n@field segmentationInfo: information regarding the message segmentation on facility layer."]
    #[doc = ""]
    #[doc = "\n@field referencePosition: the reference position for all position related information in the CPM."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ManagementContainer {
        #[rasn(identifier = "referenceTime")]
        pub reference_time: TimestampIts,
        #[rasn(identifier = "referencePosition")]
        pub reference_position: ReferencePosition,
        #[rasn(identifier = "segmentationInfo")]
        pub segmentation_info: Option<MessageSegmentationInfo>,
        #[rasn(identifier = "messageRateRange")]
        pub message_rate_range: Option<MessageRateRange>,
    }
    impl ManagementContainer {
        pub fn new(
            reference_time: TimestampIts,
            reference_position: ReferencePosition,
            segmentation_info: Option<MessageSegmentationInfo>,
            message_rate_range: Option<MessageRateRange>,
        ) -> Self {
            Self {
                reference_time,
                reference_position,
                segmentation_info,
                message_rate_range,
            }
        }
    }

    #[doc = "This DF  represents the planned or expected range of the message generation rate."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field messageRateMin: the minimum planned or expected message rate."]
    #[doc = ""]
    #[doc = "\n@field messageRateMax: the maximum planned or expected message rate."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct MessageRateRange {
        #[rasn(identifier = "messageRateMin")]
        pub message_rate_min: MessageRateHz,
        #[rasn(identifier = "messageRateMax")]
        pub message_rate_max: MessageRateHz,
    }
    impl MessageRateRange {
        pub fn new(message_rate_min: MessageRateHz, message_rate_max: MessageRateHz) -> Self {
            Self {
                message_rate_min,
                message_rate_max,
            }
        }
    }

    #[doc = "This DF represents a CPM container preceded by its type identifier and a lenght indicator."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field containerId: the identifier of the container type."]
    #[doc = ""]
    #[doc = "\n@field containerData: the container content consistent with the container type."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct WrappedCpmContainer {
        #[rasn(identifier = "containerId")]
        pub container_id: CpmContainerId,
        #[rasn(identifier = "containerData")]
        pub container_data: Any,
    }
    impl WrappedCpmContainer {
        pub fn new(container_id: CpmContainerId, container_data: Any) -> Self {
            Self {
                container_id,
                container_data,
            }
        }
    }

    #[doc = "This DF represents a list of CPM containers, each with their type identifier. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct WrappedCpmContainers(pub SequenceOf<WrappedCpmContainer>);
    pub const ORIGINATING_RSU_CONTAINER: CpmContainerId = CpmContainerId(2);

    #[doc = "These value assignements represent specific values of the container type identifier. "]

    pub const ORIGINATING_VEHICLE_CONTAINER: CpmContainerId = CpmContainerId(1);
    pub const PERCEIVED_OBJECT_CONTAINER: CpmContainerId = CpmContainerId(5);
    pub const PERCEPTION_REGION_CONTAINER: CpmContainerId = CpmContainerId(4);
    pub const SENSOR_INFORMATION_CONTAINER: CpmContainerId = CpmContainerId(3);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_perceived_object_container {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::{CardinalNumber1B, PerceivedObject};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "This DF  represents the Perceived Object Container "]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field numberOfPerceivedObjects: the total number of perceived objects at the time of generating the message. "]
    #[doc = ""]
    #[doc = "\n@field perceivedObjects: the list of perceived objects."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceivedObjectContainer {
        #[rasn(identifier = "numberOfPerceivedObjects")]
        pub number_of_perceived_objects: CardinalNumber1B,
        #[rasn(identifier = "perceivedObjects")]
        pub perceived_objects: PerceivedObjects,
    }
    impl PerceivedObjectContainer {
        pub fn new(
            number_of_perceived_objects: CardinalNumber1B,
            perceived_objects: PerceivedObjects,
        ) -> Self {
            Self {
                number_of_perceived_objects,
                perceived_objects,
            }
        }
    }
    #[doc = "\n@brief Perceived Objects"]
    #[doc = "This DF provides a list of perceived objects represented in the coordinate system in which the y-axis corresponds to the North direction, the x-axis to the East direction, and the z- axis to the vertical direction."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=255", extensible))]
    pub struct PerceivedObjects(pub SequenceOf<PerceivedObject>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_perception_region_container {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::{
        CardinalNumber1B, ConfidenceLevel, DeltaTimeMilliSecondSigned, Identifier2B, SensorType,
        SequenceOfIdentifier1B, Shape,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "This DF  represents a list of identifiers of perceived objects. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=255", extensible))]
    pub struct PerceivedObjectIds(pub SequenceOf<Identifier2B>);

    #[doc = "This DF represents the actual perception capabilities available to the transmitting ITS-S, offering additional (often dynamic) details to the information provided in the sensor information container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = "\n@field measurementDeltaTime: difference between the time of estimation of the perception region and the reference time. Positive values indicates that the provided information refers to a point in time after the reference time."]
    #[doc = ""]
    #[doc = "\n@field perceptionRegionConfidence: the perception confidence."]
    #[doc = ""]
    #[doc = "\n@field perceptionRegionShape: specification of the shape of the perception region."]
    #[doc = ""]
    #[doc = "\n@field shadowingApplies: indicates if the standard shadowing approach applies to the described perception region."]
    #[doc = ""]
    #[doc = "\n@field sensorIdList: the optional list of identifiers of the sensors which are involved in perceiving the region."]
    #[doc = ""]
    #[doc = "\n@field numberOfPerceivedObjects: the optional number of perceived objects contained in the perception region specified in the component perceptionRegionShape. "]
    #[doc = ""]
    #[doc = "\n@field perceivedObjectIds: the optional list of identifiers of the objects specified in the Perceived Object Container that are contained in the perception region specified in the component perceptionRegionShape."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceptionRegion {
        #[rasn(identifier = "measurementDeltaTime")]
        pub measurement_delta_time: DeltaTimeMilliSecondSigned,
        #[rasn(identifier = "perceptionRegionConfidence")]
        pub perception_region_confidence: ConfidenceLevel,
        #[rasn(identifier = "perceptionRegionShape")]
        pub perception_region_shape: Shape,
        #[rasn(identifier = "shadowingApplies")]
        pub shadowing_applies: bool,
        #[rasn(identifier = "sensorIdList")]
        pub sensor_id_list: Option<SequenceOfIdentifier1B>,
        #[rasn(identifier = "numberOfPerceivedObjects")]
        pub number_of_perceived_objects: Option<CardinalNumber1B>,
        #[rasn(identifier = "perceivedObjectIds")]
        pub perceived_object_ids: Option<PerceivedObjectIds>,
    }
    impl PerceptionRegion {
        pub fn new(
            measurement_delta_time: DeltaTimeMilliSecondSigned,
            perception_region_confidence: ConfidenceLevel,
            perception_region_shape: Shape,
            shadowing_applies: bool,
            sensor_id_list: Option<SequenceOfIdentifier1B>,
            number_of_perceived_objects: Option<CardinalNumber1B>,
            perceived_object_ids: Option<PerceivedObjectIds>,
        ) -> Self {
            Self {
                measurement_delta_time,
                perception_region_confidence,
                perception_region_shape,
                shadowing_applies,
                sensor_id_list,
                number_of_perceived_objects,
                perceived_object_ids,
            }
        }
    }

    #[doc = "This DF  represents the Perception Region Container as a list of perception region information objects. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=256", extensible))]
    pub struct PerceptionRegionContainer(pub SequenceOf<PerceptionRegion>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cpm_sensor_information_container {
    extern crate alloc;
    use super::super::cdd_2_2_1::etsi_its_cdd::{ConfidenceLevel, Identifier1B, SensorType, Shape};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    use std::sync::LazyLock;

    #[doc = "This DF  represents the characteristics of a single sensor or data fusion system."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field sensorId: identifier of the sensor or data fusion system used to relate the perceived object in the Perceived Object Container to the sensor that detected it."]
    #[doc = "this identifier shall uniquely identify the sensor inside the CPM, but may be changed from one CPM to the next."]
    #[doc = ""]
    #[doc = "\n@field sensorType: the type of the sensor."]
    #[doc = ""]
    #[doc = "\n@field perceptionRegionShape: the perception region of the sensor."]
    #[doc = ""]
    #[doc = "\n@field perceptionRegionConfidence: the homogeneous perception region confidence that can be assumed for the entire perception region shape of this sensor. "]
    #[doc = ""]
    #[doc = "\n@field shadowingApplies: indicates if the standard shadowing approach applies to the described perception region."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorInformation {
        #[rasn(identifier = "sensorId")]
        pub sensor_id: Identifier1B,
        #[rasn(identifier = "sensorType")]
        pub sensor_type: SensorType,
        #[rasn(identifier = "perceptionRegionShape")]
        pub perception_region_shape: Option<Shape>,
        #[rasn(identifier = "perceptionRegionConfidence")]
        pub perception_region_confidence: Option<ConfidenceLevel>,
        #[rasn(identifier = "shadowingApplies")]
        pub shadowing_applies: bool,
    }
    impl SensorInformation {
        pub fn new(
            sensor_id: Identifier1B,
            sensor_type: SensorType,
            perception_region_shape: Option<Shape>,
            perception_region_confidence: Option<ConfidenceLevel>,
            shadowing_applies: bool,
        ) -> Self {
            Self {
                sensor_id,
                sensor_type,
                perception_region_shape,
                perception_region_confidence,
                shadowing_applies,
            }
        }
    }

    #[doc = "This DF  represents the Sensor Information Container as a list of information objects about the sensors or data fusion systems from which the station provides information about detected objects."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);
}
