#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod denm_pdu_descriptions {
    extern crate alloc;
    use core::borrow::Borrow;
    use std::sync::LazyLock;

    use rasn::prelude::*;

    use super::super::cdd_1_3_1_1::its_container::{
        ActionID,
        CauseCode,
        ClosedLanes,
        DangerousGoodsExtended,
        DeltaReferencePosition,
        EnergyStorageType,
        EventHistory,
        Heading,
        HeightLonCarr,
        InformationQuality,
        ItineraryPath,
        ItsPduHeader,
        LanePosition,
        LightBarSirenInUse,
        NumberOfOccupants,
        PosCentMass,
        PosFrontAx,
        PosLonCarr,
        PositionOfOccupants,
        PositionOfPillars,
        PositioningSolutionType,
        ReferencePosition,
        RelevanceDistance,
        RelevanceTrafficDirection,
        RequestResponseIndication,
        RestrictedTypes,
        RoadType,
        Speed,
        SpeedLimit,
        StationType,
        StationarySince,
        Temperature,
        TimestampIts,
        Traces,
        TrafficRule,
        TransmissionInterval,
        TurningRadius,
        ValidityDuration,
        VehicleIdentification,
        VehicleMass,
        WheelBaseVehicle,
    };
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ManagementContainer {
        #[rasn(identifier = "actionID")]
        pub action_id: ActionID,
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
            action_id: ActionID,
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
                action_id,
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
        DEFAULT_VALIDITY.clone()
    }
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=8", extensible))]
    pub struct ReferenceDenms(pub SequenceOf<ActionID>);
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Termination {
        isCancellation = 0,
        isNegation = 1,
    }
    pub static DEFAULT_VALIDITY: ValidityDuration = ValidityDuration(600);
}
