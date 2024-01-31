#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]
pub mod c_p_m__originating_station_containers {
    extern crate alloc;
    use crate::standards::cdd_2_2_1::{
        CartesianAngle, MapReference, Speed, StationType, TrailerData, Wgs84Angle,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = " * This DF  represents the Originating RSU Container."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field mapReference: identifies the MAPEM containing the topology information reference in the Perceived Object Container"]
    #[doc = " * "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OriginatingRsuContainer {
        pub map_reference: Option<MapReference>,
    }
    impl OriginatingRsuContainer {
        pub fn new(map_reference: Option<MapReference>) -> Self {
            Self { map_reference }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF  represents the Originating Vehicle Container"]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field orientationAngle: the angle and angle accuracy of the absolute orientation of the disseminating vehicle in the WGS84 coordinate system with respect to true North."]
    #[doc = " *"]
    #[doc = " * @field pitchAngle: the optional angle and angle accuracy between the ground plane and the current orientation of the vehicle's x-axis with respect to the ground plane about the y-axis according to the ISO 8855."]
    #[doc = " *"]
    #[doc = " * @field rollAngle: the optional angle and angle accuracy between the ground plane and the current orientation of a vehicle's y-axis with respect to the ground plane about the x-axis according to the ISO 8855"]
    #[doc = " *"]
    #[doc = " * @field trailerData: information about the trailer dimensions and orientation in case a trailer is present."]
    #[doc = " * "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct OriginatingVehicleContainer {
        pub orientation_angle: Wgs84Angle,
        pub pitch_angle: Option<CartesianAngle>,
        pub roll_angle: Option<CartesianAngle>,
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
    #[doc = "*"]
    #[doc = " * This DF  represents a list of trailer data."]
    #[doc = " * "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct TrailerDataSet(pub SequenceOf<TrailerData>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]
pub mod c_p_m__p_d_u__descriptions {
    extern crate alloc;
    use super::c_p_m__originating_station_containers::{
        OriginatingRsuContainer, OriginatingVehicleContainer,
    };
    use super::c_p_m__perceived_object_container::PerceivedObjectContainer;
    use super::c_p_m__perception_region_container::PerceptionRegionContainer;
    use super::c_p_m__sensor_information_container::SensorInformationContainer;
    use crate::standards::cdd_2_2_1::{
        ItsPduHeader, MessageRateHz, MessageSegmentationInfo, OrdinalNumber1B, ReferencePosition,
        StationType, TimestampIts,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = " * This DF  represents the Collective Perception Message (CPM) and is the top level Protocol Data Unit. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field header: the common message header for the facilities layer message. "]
    #[doc = " *"]
    #[doc = " * @field payload: the payload of the message. "]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
    #[doc = "*"]
    #[doc = " * This DF represents a list of CPM containers, each with their type identifier with an additional constraint. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate)]
    pub struct ConstraintWrappedCpmContainers(pub WrappedCpmContainers);
    #[doc = "*"]
    #[doc = " * This DE represents the identifier of the container type. "]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
    #[rasn(delegate, value("1..=16"))]
    pub struct CpmContainerId(pub u8);
    #[derive(Debug, Clone, PartialEq)]
    pub enum CpmContainers_Type {
        OriginatingVehicleContainer(OriginatingVehicleContainer),
        OriginatingRsuContainer(OriginatingRsuContainer),
        SensorInformationContainer(SensorInformationContainer),
        PerceptionRegionContainer(PerceptionRegionContainer),
        PerceivedObjectContainer(PerceivedObjectContainer),
    }
    impl CpmContainers_Type {
        pub fn decode<D: Decoder>(
            decoder: &mut D,
            open_type_payload: Option<&Any>,
            identifier: &CpmContainerId,
        ) -> Result<Self, D::Error> {
            match identifier {
                i if i == &ORIGINATING_VEHICLE_CONTAINER => Ok(decoder
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
                                ).into()
                            })?.as_bytes(),
                    ).map(Self::OriginatingVehicleContainer)?),
                i if i == &ORIGINATING_RSU_CONTAINER => Ok(decoder
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
                                ).into()
                            })?.as_bytes(),
                    ).map(Self::OriginatingRsuContainer)?),
                i if i == &SENSOR_INFORMATION_CONTAINER => Ok(decoder
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
                                ).into()
                            })?.as_bytes(),
                    ).map(Self::SensorInformationContainer)?),
                i if i == &PERCEPTION_REGION_CONTAINER => Ok(decoder
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
                                ).into()
                            })?.as_bytes(),
                    ).map(Self::PerceptionRegionContainer)?),
                i if i == &PERCEIVED_OBJECT_CONTAINER => Ok(decoder
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
                                ).into()
                            })?.as_bytes(),
                    ).map(Self::PerceivedObjectContainer)?),
                _ => Err(rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    decoder.codec(),
                ).into()),
            }
        }
        pub fn encode<E: Encoder>(
            &self,
            encoder: &mut E,
            identifier: &CpmContainerId,
        ) -> Result<(), E::Error> {
            match (self, identifier) {
                (Self::OriginatingVehicleContainer(inner), i)
                    if i == &ORIGINATING_VEHICLE_CONTAINER =>
                {
                    inner.encode(encoder)
                }
                (Self::OriginatingRsuContainer(inner), i) if i == &ORIGINATING_RSU_CONTAINER => {
                    inner.encode(encoder)
                }
                (Self::SensorInformationContainer(inner), i)
                    if i == &SENSOR_INFORMATION_CONTAINER =>
                {
                    inner.encode(encoder)
                }
                (Self::PerceptionRegionContainer(inner), i)
                    if i == &PERCEPTION_REGION_CONTAINER =>
                {
                    inner.encode(encoder)
                }
                (Self::PerceivedObjectContainer(inner), i) if i == &PERCEIVED_OBJECT_CONTAINER => {
                    inner.encode(encoder)
                }
                _ => Err(rasn::error::EncodeError::from_kind(
                    rasn::error::EncodeErrorKind::Custom {
                        msg: alloc::format!(
                            "Unknown unique identifier for information object class instance."
                        ),
                    },
                    encoder.codec(),
                ).into()),
            }
        }
    }
    #[doc = "*"]
    #[doc = " * This DF  represents the payload of the CPM. "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field managementContainer: the management container. "]
    #[doc = " *"]
    #[doc = " * @field cpmContainers: the list of CPM containers, including its container type identifier and including either one or none of originatingVehicleContainer and/or originatingRsuContainer. "]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct CpmPayload {
        pub management_container: ManagementContainer,
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
    #[doc = "*"]
    #[doc = " * This DF  represents the management container of the CPM. "]
    #[doc = " * The management container provides basic information about the originating ITS-S, which are not specific to a specific type of station."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field referenceTime: the reference time for all time related information in the CPM."]
    #[doc = " *"]
    #[doc = " * @field messageRateRange: the planned or expected range of the CPM generation rate."]
    #[doc = " *"]
    #[doc = " * @field segmentationInfo: information regarding the message segmentation on facility layer."]
    #[doc = " *"]
    #[doc = " * @field referencePosition: the reference position for all position related information in the CPM."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ManagementContainer {
        pub reference_time: TimestampIts,
        pub reference_position: ReferencePosition,
        pub segmentation_info: Option<MessageSegmentationInfo>,
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
    #[doc = "*"]
    #[doc = " * This DF  represents the planned or expected range of the message generation rate."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field messageRateMin: the minimum planned or expected message rate."]
    #[doc = " *"]
    #[doc = " * @field messageRateMax: the maximum planned or expected message rate."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct MessageRateRange {
        pub message_rate_min: MessageRateHz,
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
    #[doc = "*"]
    #[doc = " * This DF represents a CPM container preceded by its type identifier and a lenght indicator."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field containerId: the identifier of the container type."]
    #[doc = " *"]
    #[doc = " * @field containerData: the container content consistent with the container type."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    pub struct WrappedCpmContainer {
        pub container_id: CpmContainerId,
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
    impl WrappedCpmContainer {
        pub fn decode_container_data<D: Decoder>(
            &self,
            decoder: &mut D,
        ) -> Result<CpmContainers_Type, D::Error> {
            CpmContainers_Type::decode(decoder, Some(&self.container_data), &self.container_id)
        }
    }
    #[doc = "*"]
    #[doc = " * This DF represents a list of CPM containers, each with their type identifier. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct WrappedCpmContainers(pub SequenceOf<WrappedCpmContainer>);
    pub const ORIGINATING_RSU_CONTAINER: CpmContainerId = CpmContainerId(2);
    #[doc = "*"]
    #[doc = " * These value assignements represent specific values of the container type identifier. "]
    #[doc = ""]
    pub const ORIGINATING_VEHICLE_CONTAINER: CpmContainerId = CpmContainerId(1);
    pub const PERCEIVED_OBJECT_CONTAINER: CpmContainerId = CpmContainerId(5);
    pub const PERCEPTION_REGION_CONTAINER: CpmContainerId = CpmContainerId(4);
    pub const SENSOR_INFORMATION_CONTAINER: CpmContainerId = CpmContainerId(3);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]
pub mod c_p_m__perceived_object_container {
    extern crate alloc;
    use crate::standards::cdd_2_2_1::{CardinalNumber1B, PerceivedObject};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = " * This DF  represents the Perceived Object Container "]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field numberOfPerceivedObjects: the total number of perceived objects at the time of generating the message. "]
    #[doc = " *"]
    #[doc = " * @field perceivedObjects: the list of perceived objects."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceivedObjectContainer {
        pub number_of_perceived_objects: CardinalNumber1B,
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
    #[doc = "* @brief Perceived Objects"]
    #[doc = " * This DF provides a list of perceived objects represented in the coordinate system in which the y-axis corresponds to the North direction, the x-axis to the East direction, and the z- axis to the vertical direction."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=255", extensible))]
    pub struct PerceivedObjects(pub SequenceOf<PerceivedObject>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]
pub mod c_p_m__perception_region_container {
    extern crate alloc;
    use crate::standards::cdd_2_2_1::{
        CardinalNumber1B, ConfidenceLevel, DeltaTimeMilliSecondSigned, Identifier2B, SensorType,
        SequenceOfIdentifier1B, Shape,
    };
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = " * This DF  represents a list of identifiers of perceived objects. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("0..=255", extensible))]
    pub struct PerceivedObjectIds(pub SequenceOf<Identifier2B>);
    #[doc = "*"]
    #[doc = " * This DF represents the actual perception capabilities available to the transmitting ITS-S, offering additional (often dynamic) details to the information provided in the sensor information container."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = ""]
    #[doc = " * @field measurementDeltaTime: difference between the time of estimation of the perception region and the reference time. Positive values indicates that the provided information refers to a point in time after the reference time."]
    #[doc = " *"]
    #[doc = " * @field perceptionRegionConfidence: the perception confidence."]
    #[doc = " *"]
    #[doc = " * @field perceptionRegionShape: specification of the shape of the perception region."]
    #[doc = " *"]
    #[doc = " * @field shadowingApplies: indicates if the standard shadowing approach applies to the described perception region."]
    #[doc = " *"]
    #[doc = " * @field sensorIdList: the optional list of identifiers of the sensors which are involved in perceiving the region."]
    #[doc = " *"]
    #[doc = " * @field numberOfPerceivedObjects: the optional number of perceived objects contained in the perception region specified in the component perceptionRegionShape. "]
    #[doc = " *"]
    #[doc = " * @field perceivedObjectIds: the optional list of identifiers of the objects specified in the Perceived Object Container that are contained in the perception region specified in the component perceptionRegionShape."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PerceptionRegion {
        pub measurement_delta_time: DeltaTimeMilliSecondSigned,
        pub perception_region_confidence: ConfidenceLevel,
        pub perception_region_shape: Shape,
        pub shadowing_applies: bool,
        pub sensor_id_list: Option<SequenceOfIdentifier1B>,
        pub number_of_perceived_objects: Option<CardinalNumber1B>,
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
    #[doc = "*"]
    #[doc = " * This DF  represents the Perception Region Container as a list of perception region information objects. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=256", extensible))]
    pub struct PerceptionRegionContainer(pub SequenceOf<PerceptionRegion>);
}
#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused
)]
pub mod c_p_m__sensor_information_container {
    extern crate alloc;
    use crate::standards::cdd_2_2_1::{ConfidenceLevel, Identifier1B, SensorType, Shape};
    use core::borrow::Borrow;
    use rasn::prelude::*;
    #[doc = "*"]
    #[doc = " * This DF  represents the characteristics of a single sensor or data fusion system."]
    #[doc = " *"]
    #[doc = " * It shall include the following components: "]
    #[doc = " *"]
    #[doc = " * @field sensorId: identifier of the sensor or data fusion system used to relate the perceived object in the Perceived Object Container to the sensor that detected it."]
    #[doc = " * this identifier shall uniquely identify the sensor inside the CPM, but may be changed from one CPM to the next."]
    #[doc = " *"]
    #[doc = " * @field sensorType: the type of the sensor."]
    #[doc = " *"]
    #[doc = " * @field perceptionRegionShape: the perception region of the sensor."]
    #[doc = " *"]
    #[doc = " * @field perceptionRegionConfidence: the homogeneous perception region confidence that can be assumed for the entire perception region shape of this sensor. "]
    #[doc = " *"]
    #[doc = " * @field shadowingApplies: indicates if the standard shadowing approach applies to the described perception region."]
    #[doc = " *"]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SensorInformation {
        pub sensor_id: Identifier1B,
        pub sensor_type: SensorType,
        pub perception_region_shape: Option<Shape>,
        pub perception_region_confidence: Option<ConfidenceLevel>,
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
    #[doc = "*"]
    #[doc = " * This DF  represents the Sensor Information Container as a list of information objects about the sensors or data fusion systems from which the station provides information about detected objects."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("1..=128", extensible))]
    pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);
}
