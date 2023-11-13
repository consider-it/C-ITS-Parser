#![allow(unused, non_upper_case_globals, non_snake_case)]
extern crate alloc;

use super::cdd_2_2_1::*;
use rasn::prelude::*;
// =====================================================
// DENM-PDU-Description
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) denmPduRelease2(103831) major-version-2(2) minor-version-1(1) }
// =====================================================

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AlacarteContainer {
    pub lane_position: Option<LanePosition>,
    pub impact_reduction: Option<ImpactReductionContainer>,
    pub external_temperature: Option<Temperature>,
    pub road_works: Option<RoadWorksContainerExtended>,
    pub positioning_solution: Option<PositioningSolutionType>,
    pub stationary_vehicle: Option<StationaryVehicleContainer>,
}

impl AlacarteContainer {
    pub fn new(
        lane_position: Option<LanePosition>,
        impact_reduction: Option<ImpactReductionContainer>,
        external_temperature: Option<Temperature>,
        road_works: Option<RoadWorksContainerExtended>,
        positioning_solution: Option<PositioningSolutionType>,
        stationary_vehicle: Option<StationaryVehicleContainer>,
    ) -> Self {
        Self {
            lane_position,
            impact_reduction,
            external_temperature,
            road_works,
            positioning_solution,
            stationary_vehicle,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DENM {
    #[rasn(value("0.."))]
    pub header: ItsPduHeader,
    pub denm: DenmPayload,
}

impl DENM {
    pub fn new(header: ItsPduHeader, denm: DenmPayload) -> Self {
        Self { header, denm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DenmPayload {
    pub management: ManagementContainer,
    pub situation: Option<SituationContainer>,
    pub location: Option<LocationContainer>,
    pub alacarte: Option<AlacarteContainer>,
}

impl DenmPayload {
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ImpactReductionContainer {
    pub height_lon_carr_left: HeightLonCarr,
    pub height_lon_carr_right: HeightLonCarr,
    pub pos_lon_carr_left: PosLonCarr,
    pub pos_lon_carr_right: PosLonCarr,
    pub position_of_pillars: PositionOfPillars,
    pub pos_cent_mass: PosCentMass,
    pub wheel_base_vehicle: WheelBaseVehicle,
    pub turning_radius: TurningRadius,
    pub pos_front_ax: PosFrontAx,
    pub position_of_occupants: PositionOfOccupants,
    pub vehicle_mass: VehicleMass,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LocationContainer {
    pub event_speed: Option<Speed>,
    pub event_position_heading: Option<Wgs84Angle>,
    pub traces: Traces,
    pub road_type: Option<RoadType>,
}

impl LocationContainer {
    pub fn new(
        event_speed: Option<Speed>,
        event_position_heading: Option<Wgs84Angle>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ManagementContainer {
    pub action_id: ActionId,
    pub detection_time: TimestampIts,
    pub reference_time: TimestampIts,
    pub termination: Option<Termination>,
    pub event_position: ReferencePosition,
    pub awareness_distance: Option<StandardLength3b>,
    pub awareness_traffic_direction: Option<TrafficDirection>,
    #[rasn(default = "management_container_validity_duration_default")]
    pub validity_duration: DeltaTimeSecond,
    pub transmission_interval: Option<DeltaTimeMilliSecondPositive>,
    pub station_type: StationType,
}

impl ManagementContainer {
    pub fn new(
        action_id: ActionId,
        detection_time: TimestampIts,
        reference_time: TimestampIts,
        termination: Option<Termination>,
        event_position: ReferencePosition,
        awareness_distance: Option<StandardLength3b>,
        awareness_traffic_direction: Option<TrafficDirection>,
        validity_duration: DeltaTimeSecond,
        transmission_interval: Option<DeltaTimeMilliSecondPositive>,
        station_type: StationType,
    ) -> Self {
        Self {
            action_id,
            detection_time,
            reference_time,
            termination,
            event_position,
            awareness_distance,
            awareness_traffic_direction,
            validity_duration,
            transmission_interval,
            station_type,
        }
    }
}

fn management_container_validity_duration_default() -> DeltaTimeSecond {
    DeltaTimeSecond(600).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadWorksContainerExtended {
    pub light_bar_siren_in_use: Option<LightBarSirenInUse>,
    pub closed_lanes: Option<ClosedLanes>,
    pub restriction: Option<RestrictedTypes>,
    pub speed_limit: Option<SpeedLimit>,
    pub incident_indication: Option<CauseCodeV2>,
    pub recommended_path: Option<ItineraryPath>,
    pub starting_point_speed_limit: Option<DeltaReferencePosition>,
    pub traffic_flow_rule: Option<TrafficRule>,
    pub reference_denms: Option<ActionIdList>,
}

impl RoadWorksContainerExtended {
    pub fn new(
        light_bar_siren_in_use: Option<LightBarSirenInUse>,
        closed_lanes: Option<ClosedLanes>,
        restriction: Option<RestrictedTypes>,
        speed_limit: Option<SpeedLimit>,
        incident_indication: Option<CauseCodeV2>,
        recommended_path: Option<ItineraryPath>,
        starting_point_speed_limit: Option<DeltaReferencePosition>,
        traffic_flow_rule: Option<TrafficRule>,
        reference_denms: Option<ActionIdList>,
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SituationContainer {
    pub information_quality: InformationQuality,
    pub event_type: CauseCodeV2,
    pub linked_cause: Option<CauseCodeV2>,
    pub event_zone: Option<EventZone>,
}

impl SituationContainer {
    pub fn new(
        information_quality: InformationQuality,
        event_type: CauseCodeV2,
        linked_cause: Option<CauseCodeV2>,
        event_zone: Option<EventZone>,
    ) -> Self {
        Self {
            information_quality,
            event_type,
            linked_cause,
            event_zone,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct StationaryVehicleContainer {
    pub stationary_since: Option<StationarySince>,
    pub stationary_cause: Option<CauseCodeV2>,
    pub carrying_dangerous_goods: Option<DangerousGoodsExtended>,
    pub number_of_occupants: Option<NumberOfOccupants>,
    pub vehicle_identification: Option<VehicleIdentification>,
    pub energy_storage_type: Option<EnergyStorageType>,
}

impl StationaryVehicleContainer {
    pub fn new(
        stationary_since: Option<StationarySince>,
        stationary_cause: Option<CauseCodeV2>,
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

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum Termination {
    IsCancellation = 0,
    IsNegation = 1,
}

pub const DEFAULT_VALIDITY: u16 = 600;
