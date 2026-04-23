#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod cam_pdu_descriptions {
    extern crate alloc;
    use core::borrow::Borrow;

    use rasn::prelude::*;

    use super::super::cdd_1_3_1_1::its_container::{
        AccelerationControl,
        CauseCode,
        CenDsrcTollingZone,
        ClosedLanes,
        Curvature,
        CurvatureCalculationMode,
        DangerousGoodsBasic,
        DriveDirection,
        EmbarkationStatus,
        EmergencyPriority,
        ExteriorLights,
        Heading,
        ItsPduHeader,
        LanePosition,
        LateralAcceleration,
        Latitude,
        LightBarSirenInUse,
        Longitude,
        LongitudinalAcceleration,
        PathHistory,
        PerformanceClass,
        ProtectedCommunicationZone,
        ProtectedCommunicationZonesRSU,
        PtActivation,
        ReferencePosition,
        RoadworksSubCauseCode,
        SpecialTransportType,
        Speed,
        SpeedLimit,
        StationType,
        SteeringWheelAngle,
        TrafficRule,
        VehicleLength,
        VehicleRole,
        VehicleWidth,
        VerticalAcceleration,
        YawRate,
    };
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[doc = "\tThe root data frame for cooperative awareness messages"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, value("0..=65535"))]
    pub struct GenerationDeltaTime(pub u16);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum HighFrequencyContainer {
        basicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
        rsuContainerHighFrequency(RSUContainerHighFrequency),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(choice, automatic_tags)]
    #[non_exhaustive]
    pub enum LowFrequencyContainer {
        basicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RSUContainerHighFrequency {
        #[rasn(identifier = "protectedCommunicationZonesRSU")]
        pub protected_communication_zones_rsu: Option<ProtectedCommunicationZonesRSU>,
    }
    impl RSUContainerHighFrequency {
        pub fn new(
            protected_communication_zones_rsu: Option<ProtectedCommunicationZonesRSU>,
        ) -> Self {
            Self {
                protected_communication_zones_rsu,
            }
        }
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
}
