#![allow(unused, non_upper_case_globals, non_snake_case)]
extern crate alloc;

use super::cdd_1_3_1_1::*;
use rasn::prelude::*;
// =====================================================
// CAM-PDU-Descriptions
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) en(302637) cam(2) version(2) }
// =====================================================

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct BasicContainer {
    pub station_type: StationType,
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
    pub drive_direction: DriveDirection,
    pub vehicle_length: VehicleLength,
    pub vehicle_width: VehicleWidth,
    pub longitudinal_acceleration: LongitudinalAcceleration,
    pub curvature: Curvature,
    pub curvature_calculation_mode: CurvatureCalculationMode,
    pub yaw_rate: YawRate,
    pub acceleration_control: Option<AccelerationControl>,
    pub lane_position: Option<LanePosition>,
    pub steering_wheel_angle: Option<SteeringWheelAngle>,
    pub lateral_acceleration: Option<LateralAcceleration>,
    pub vertical_acceleration: Option<VerticalAcceleration>,
    pub performance_class: Option<PerformanceClass>,
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
    pub vehicle_role: VehicleRole,
    pub exterior_lights: ExteriorLights,
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

///	The root data frame for cooperative awareness messages

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
#[non_exhaustive]
pub struct CamParameters {
    pub basic_container: BasicContainer,
    pub high_frequency_container: HighFrequencyContainer,
    pub low_frequency_container: Option<LowFrequencyContainer>,
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
pub struct CoopAwareness {
    pub generation_delta_time: GenerationDeltaTime,
    pub cam_parameters: CamParameters,
}

impl CoopAwareness {
    pub fn new(generation_delta_time: GenerationDeltaTime, cam_parameters: CamParameters) -> Self {
        Self {
            generation_delta_time,
            cam_parameters,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DangerousGoodsContainer {
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
pub struct EmergencyContainer {
    pub light_bar_siren_in_use: LightBarSirenInUse,
    pub incident_indication: Option<CauseCode>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct GenerationDeltaTime(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum HighFrequencyContainer {
    BasicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
    RsuContainerHighFrequency(RSUContainerHighFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LowFrequencyContainer {
    BasicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PublicTransportContainer {
    pub embarkation_status: EmbarkationStatus,
    pub pt_activation: Option<PtActivation>,
}

impl PublicTransportContainer {
    pub fn new(embarkation_status: EmbarkationStatus, pt_activation: Option<PtActivation>) -> Self {
        Self {
            embarkation_status,
            pt_activation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RSUContainerHighFrequency {
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RescueContainer {
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
#[rasn(automatic_tags)]
pub struct RoadWorksContainerBasic {
    pub roadworks_sub_cause_code: Option<RoadworksSubCauseCode>,
    pub light_bar_siren_in_use: LightBarSirenInUse,
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
pub struct SafetyCarContainer {
    pub light_bar_siren_in_use: LightBarSirenInUse,
    pub incident_indication: Option<CauseCode>,
    pub traffic_rule: Option<TrafficRule>,
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
pub struct SpecialTransportContainer {
    pub special_transport_type: SpecialTransportType,
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
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SpecialVehicleContainer {
    PublicTransportContainer(PublicTransportContainer),
    SpecialTransportContainer(SpecialTransportContainer),
    DangerousGoodsContainer(DangerousGoodsContainer),
    RoadWorksContainerBasic(RoadWorksContainerBasic),
    RescueContainer(RescueContainer),
    EmergencyContainer(EmergencyContainer),
    SafetyCarContainer(SafetyCarContainer),
}
