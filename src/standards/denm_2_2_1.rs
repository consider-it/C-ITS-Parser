#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::too_many_arguments
)]
pub mod denm_pdu_description {
    extern crate alloc;
    use core::borrow::Borrow;
    use std::sync::LazyLock;

    use rasn::prelude::*;

    use super::super::cdd_2_2_1::etsi_its_cdd::{
        ActionId,
        ActionIdList,
        CauseCodeV2,
        ClosedLanes,
        DangerousGoodsExtended,
        DeltaReferencePosition,
        DeltaTimeMilliSecondPositive,
        DeltaTimeSecond,
        EnergyStorageType,
        EventZone,
        GeneralizedLanePositions,
        HeightLonCarr,
        InformationQuality,
        ItineraryPath,
        ItsPduHeader,
        IvimReferences,
        LanePosition,
        LightBarSirenInUse,
        MapReferences,
        MetaInformation,
        NumberOfOccupants,
        ObjectFace,
        OccupiedLanesWithConfidence,
        PathPredictedList,
        PerceivedObject,
        PosCentMass,
        PosFrontAx,
        PosLonCarr,
        Position1d,
        PositionOfOccupants,
        PositionOfPillars,
        PositioningSolutionType,
        ReferencePosition,
        RequestResponseIndication,
        RestrictedTypes,
        RoadConfigurationSectionList,
        RoadType,
        Speed,
        SpeedLimit,
        StandardLength12b,
        StandardLength3b,
        StationId,
        StationType,
        StationarySince,
        Temperature,
        TimestampIts,
        Traces,
        TracesExtended,
        TrafficDirection,
        TrafficRule,
        TurningRadius,
        VehicleIdentification,
        VehicleMass,
        Wgs84Angle,
        WheelBaseVehicle,
    };
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct AlacarteContainerExtGroupRoadConfiguration {
        #[rasn(identifier = "roadConfiguration")]
        pub road_configuration: Option<RoadConfigurationContainer>,
        #[rasn(identifier = "preCrash")]
        pub pre_crash: Option<PreCrashContainer>,
    }
    impl AlacarteContainerExtGroupRoadConfiguration {
        pub fn new(
            road_configuration: Option<RoadConfigurationContainer>,
            pre_crash: Option<PreCrashContainer>,
        ) -> Self {
            Self {
                road_configuration,
                pre_crash,
            }
        }
    }

    #[doc = "This type represents the A La Carte Container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field lanePosition: the optional lane position of the event."]
    #[doc = ""]
    #[doc = "\n@field impactReduction: optional vehicle data for collision mitigation."]
    #[doc = ""]
    #[doc = "\n@field externalTemperature: optional the ambient temperature at the event position."]
    #[doc = ""]
    #[doc = "\n@field roadWorks: optional information of the roadwork zone."]
    #[doc = ""]
    #[doc = "\n@field positioningSolution: optionally indicates the technical solution being used by the originating ITS-S to estimate the event position."]
    #[doc = ""]
    #[doc = "\n@field stationaryVehicle: optional information about a stationary vehicle."]
    #[doc = ""]
    #[doc = "\n@field roadConfiguration: optional information about the configuration of the road."]
    #[doc = ""]
    #[doc = "\n@field precrash: the optional information about perceived objects that represent hazards and/or could be subject of collisions. "]
    #[doc = ""]
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
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_road_configuration: Option<AlacarteContainerExtGroupRoadConfiguration>,
    }
    impl AlacarteContainer {
        pub fn new(
            lane_position: Option<LanePosition>,
            impact_reduction: Option<ImpactReductionContainer>,
            external_temperature: Option<Temperature>,
            road_works: Option<RoadWorksContainerExtended>,
            positioning_solution: Option<PositioningSolutionType>,
            stationary_vehicle: Option<StationaryVehicleContainer>,
            ext_group_road_configuration: Option<AlacarteContainerExtGroupRoadConfiguration>,
        ) -> Self {
            Self {
                lane_position,
                impact_reduction,
                external_temperature,
                road_works,
                positioning_solution,
                stationary_vehicle,
                ext_group_road_configuration,
            }
        }
    }

    #[doc = "This type represents the DENM PDU."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field header: the header of the DENM PDU."]
    #[doc = ""]
    #[doc = "\n@field denm: the payload of the DENM PDU."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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

    #[doc = "This type represents the DENM payload."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field management: the Management Container."]
    #[doc = ""]
    #[doc = "\n@field situation: the optional Situation Container."]
    #[doc = ""]
    #[doc = "\n@field location: the optional Location Container."]
    #[doc = ""]
    #[doc = "\n@field alacarte: the optional ALaCarte Container ."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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

    #[doc = "This type contains detailed information about the vehicle in which the originating ITS-S is mounted, for mitigating the consequences "]
    #[doc = "of a collision."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field heightLonCarrLeft: the height of the left longitudinal carrier of the vehicle from base to top."]
    #[doc = ""]
    #[doc = "\n@field heightLonCarrRight: the height of the right longitudinal carrier of the vehicle from base to top."]
    #[doc = ""]
    #[doc = "\n@field posLonCarrLeft: the position of the left longitudinal carrier of vehicle."]
    #[doc = ""]
    #[doc = "\n@field posLonCarrRight: the position of the right longitudinal carrier of vehicle."]
    #[doc = ""]
    #[doc = "\n@field positionOfPillars: information about the vertical support of the vehicle in which the originating ITS-S is mounted. It shall be "]
    #[doc = "included for passenger vehicles only."]
    #[doc = ""]
    #[doc = "\n@field posCentMass: the position of the centre of mass of the vehicle."]
    #[doc = ""]
    #[doc = "\n@field wheelBaseVehicle: the wheel base of the vehicle."]
    #[doc = ""]
    #[doc = "\n@field turningRadius: the turning radius of the vehicle."]
    #[doc = ""]
    #[doc = "\n@field posFrontAx: the position of the front axle of the vehicle."]
    #[doc = ""]
    #[doc = "\n@field positionOfOccupants: indicates whether an in-vehicle seat is occupied at the moment of generation of the message."]
    #[doc = ""]
    #[doc = "\n@field vehicleMass: the mass of the unloaded vehicle"]
    #[doc = ""]
    #[doc = "\n@field requestResponseIndication: indicates whether the originating ITS-S transmitting the impactReduction component is requesting "]
    #[doc = "the receiving ITS-S to provide also its impactReduction component."]
    #[doc = ""]
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
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct LocationContainerExtGroupLanePositions {
        #[rasn(identifier = "lanePositions")]
        pub lane_positions: Option<GeneralizedLanePositions>,
        #[rasn(identifier = "occupiedLanes")]
        pub occupied_lanes: Option<OccupiedLanesWithConfidence>,
        #[rasn(identifier = "linkedIvims")]
        pub linked_ivims: Option<IvimReferences>,
        #[rasn(identifier = "linkedMapems")]
        pub linked_mapems: Option<MapReferences>,
        #[rasn(identifier = "detectionZonesToSpecifiedEventPoint")]
        pub detection_zones_to_specified_event_point: Option<TracesExtended>,
        #[rasn(identifier = "predictedPaths")]
        pub predicted_paths: Option<PathPredictedList>,
    }
    impl LocationContainerExtGroupLanePositions {
        pub fn new(
            lane_positions: Option<GeneralizedLanePositions>,
            occupied_lanes: Option<OccupiedLanesWithConfidence>,
            linked_ivims: Option<IvimReferences>,
            linked_mapems: Option<MapReferences>,
            detection_zones_to_specified_event_point: Option<TracesExtended>,
            predicted_paths: Option<PathPredictedList>,
        ) -> Self {
            Self {
                lane_positions,
                occupied_lanes,
                linked_ivims,
                linked_mapems,
                detection_zones_to_specified_event_point,
                predicted_paths,
            }
        }
    }

    #[doc = "This type represents the Location Container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field eventSpeed: optional speed of a detected dynamic event and the confidence of the speed information. "]
    #[doc = ""]
    #[doc = "\n@field eventPositionHeading: the optional heading of a dynamic event and the confidence of the heading information."]
    #[doc = ""]
    #[doc = "\n@field detectionZonesToEventPosition: the detection zone information approaching the event position, see clause 6.1.3.3."]
    #[doc = ""]
    #[doc = "\n@field roadType: the optional road type information at the event position. "]
    #[doc = ""]
    #[doc = "\n@field lanePositions: the optional lane(s) where the event is located, at the position indicated by the component eventPosition "]
    #[doc = "of the Management container and for a given reference direction."]
    #[doc = ""]
    #[doc = "\n@field occupiedLanes: the optional lane(s) that are fully or partially occupied by the event, at the position indicated by the "]
    #[doc = "component eventPosition of the Management container and for a given reference direction."]
    #[doc = ""]
    #[doc = "\n@field linkedIvims: the optional list of DF IvimReference, pointing to IVIMs that are semantically connected because providing information "]
    #[doc = "applying to the road segment(s) covered by the components detectionZonesToEventPosition, detectionZonesToSpecifiedEventPoint and "]
    #[doc = "the SituationContainer component eventZone."]
    #[doc = ""]
    #[doc = "\n@field linkedMapem: the optional list of DF Mapreference, pointing to MAPEMs that are semantically connected because providing information "]
    #[doc = "applying to the road segment(s) covered by the component detectionZonesToEventPosition, detectionZonesToSpecifiedEventPoint and "]
    #[doc = "the SituationContainer component eventZone."]
    #[doc = ""]
    #[doc = "\n@field detectionZonesToSpecifiedEventPoint: the optional detection zone information approaching towards a "]
    #[doc = "specified event point, see clause 6.1.3.3. "]
    #[doc = ""]
    #[doc = "\n@field predictedPaths: the optional list of future paths or trajectories that the event may move along or zones that the event may occupy. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct LocationContainer {
        #[rasn(identifier = "eventSpeed")]
        pub event_speed: Option<Speed>,
        #[rasn(identifier = "eventPositionHeading")]
        pub event_position_heading: Option<Wgs84Angle>,
        #[rasn(identifier = "detectionZonesToEventPosition")]
        pub detection_zones_to_event_position: Traces,
        #[rasn(identifier = "roadType")]
        pub road_type: Option<RoadType>,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_lane_positions: Option<LocationContainerExtGroupLanePositions>,
    }
    impl LocationContainer {
        pub fn new(
            event_speed: Option<Speed>,
            event_position_heading: Option<Wgs84Angle>,
            detection_zones_to_event_position: Traces,
            road_type: Option<RoadType>,
            ext_group_lane_positions: Option<LocationContainerExtGroupLanePositions>,
        ) -> Self {
            Self {
                event_speed,
                event_position_heading,
                detection_zones_to_event_position,
                road_type,
                ext_group_lane_positions,
            }
        }
    }

    #[doc = "This type represents the Management Container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field actionId: the identifier of the DENM."]
    #[doc = ""]
    #[doc = "\n@field detectionTime: the time at which the event is detected."]
    #[doc = ""]
    #[doc = "\n@field referenceTime: the time at which a new DENM, an update DENM or a cancellation DENM is generated"]
    #[doc = ""]
    #[doc = "\n@field termination: the optional termination type of the DENM."]
    #[doc = ""]
    #[doc = "\n@field eventPosition: the geographical position used in the definition of the awareness area / relevance zone, see clause 6.1.3."]
    #[doc = ""]
    #[doc = "\n@field awarenessDistance: the optional radius of the circular awareness area, with centre at the event position or at any of the "]
    #[doc = "event history points as defined in clause 6.1.3.1."]
    #[doc = ""]
    #[doc = "\n@field trafficDirection: the optional traffic direction along which the receiving ITS-S may encounter the event, "]
    #[doc = "as defined in clause 6.1.3."]
    #[doc = ""]
    #[doc = "\n@field validityDuration: the validity duration of a DENM. This component represents a time offset in the unit of second since detectionTime."]
    #[doc = ""]
    #[doc = "\n@field transmissionInterval: the optional time interval for DENM transmission as defined by the originating ITS-S. "]
    #[doc = "If the component is not present in DENM, a default value defaultValidity is assumed."]
    #[doc = ""]
    #[doc = "\n@field stationType: the station type information of the originating ITS-S."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct ManagementContainer {
        #[rasn(identifier = "actionId")]
        pub action_id: ActionId,
        #[rasn(identifier = "detectionTime")]
        pub detection_time: TimestampIts,
        #[rasn(identifier = "referenceTime")]
        pub reference_time: TimestampIts,
        pub termination: Option<Termination>,
        #[rasn(identifier = "eventPosition")]
        pub event_position: ReferencePosition,
        #[rasn(identifier = "awarenessDistance")]
        pub awareness_distance: Option<StandardLength3b>,
        #[rasn(identifier = "trafficDirection")]
        pub traffic_direction: Option<TrafficDirection>,
        #[rasn(
            default = "management_container_validity_duration_default",
            identifier = "validityDuration"
        )]
        pub validity_duration: DeltaTimeSecond,
        #[rasn(identifier = "transmissionInterval")]
        pub transmission_interval: Option<DeltaTimeMilliSecondPositive>,
        #[rasn(identifier = "stationType")]
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
            traffic_direction: Option<TrafficDirection>,
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
                traffic_direction,
                validity_duration,
                transmission_interval,
                station_type,
            }
        }
    }
    fn management_container_validity_duration_default() -> DeltaTimeSecond {
        DEFAULT_VALIDITY.clone()
    }

    #[doc = "This type contains detailed information about an object with which a vehicle and/or the traffic is likely to collide."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field perceivedPreCrashObject: information about a perceived object in the East, North, Up reference frame."]
    #[doc = ""]
    #[doc = "\n@field objectStationId: the optional station Id of the object for which the information is provided."]
    #[doc = ""]
    #[doc = "\n@field timeToCollision: the optional estimated time to collision of a vehicle with the object. "]
    #[doc = ""]
    #[doc = "\n@field impactSection: indication of the object's section where the impact will most likely occur. "]
    #[doc = "When the target object is likely to be a vehicle, then this component should be present, otherwise it should not be provided.  "]
    #[doc = ""]
    #[doc = "\n@field estimatedBrakingDistance: the optional estimated distance in which the vehicle would need to come to a complete hold, "]
    #[doc = "if no obstruction was in the way. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct PreCrashContainer {
        #[rasn(identifier = "perceivedPreCrashObject")]
        pub perceived_pre_crash_object: PerceivedObject,
        #[rasn(identifier = "objectStationId")]
        pub object_station_id: Option<StationId>,
        #[rasn(identifier = "timeToCollision")]
        pub time_to_collision: Option<DeltaTimeMilliSecondPositive>,
        #[rasn(identifier = "impactSection")]
        pub impact_section: Option<ObjectFace>,
        #[rasn(identifier = "estimatedBrakingDistance")]
        pub estimated_braking_distance: Option<StandardLength12b>,
    }
    impl PreCrashContainer {
        pub fn new(
            perceived_pre_crash_object: PerceivedObject,
            object_station_id: Option<StationId>,
            time_to_collision: Option<DeltaTimeMilliSecondPositive>,
            impact_section: Option<ObjectFace>,
            estimated_braking_distance: Option<StandardLength12b>,
        ) -> Self {
            Self {
                perceived_pre_crash_object,
                object_station_id,
                time_to_collision,
                impact_section,
                estimated_braking_distance,
            }
        }
    }

    #[doc = "This type contains detailed information about the configuration of road section(s) that are geographically related to the event."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field roadConfigurationConfidence: information about the source of the road configuration and the confidence in the information."]
    #[doc = ""]
    #[doc = "\n@field roadConfigurationSectionList: a list of road configuration information per applicable road section. "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct RoadConfigurationContainer {
        #[rasn(identifier = "roadConfigurationConfidence")]
        pub road_configuration_confidence: MetaInformation,
        #[rasn(identifier = "roadConfigurationSectionList")]
        pub road_configuration_section_list: RoadConfigurationSectionList,
    }
    impl RoadConfigurationContainer {
        pub fn new(
            road_configuration_confidence: MetaInformation,
            road_configuration_section_list: RoadConfigurationSectionList,
        ) -> Self {
            Self {
                road_configuration_confidence,
                road_configuration_section_list,
            }
        }
    }

    #[doc = "This type contains detailed information of a roadwork zone and specific access conditions."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field lightBarSirenInUse: optionally indicates whether a roadwork vehicle has switched on the light bar or siren. "]
    #[doc = "It is used when the roadwork involves a specific roadwork vehicle"]
    #[doc = ""]
    #[doc = "\n@field closedLanes: optionally indicates whether the roadwork has caused the closure of one or several driving lanes. "]
    #[doc = "Optionally, it may indicate whether a hard shoulder lane is closed to traffic or can be used for specific usage (e.g. for stopping)."]
    #[doc = ""]
    #[doc = "\n@field restriction: the optional type(s) of vehicles that are restricted to access the road work zone. "]
    #[doc = "More than one vehicle types may be provided by this component if the restriction apply to multiple vehicle types. "]
    #[doc = ""]
    #[doc = "\n@field speedLimit: the optional speed limitation applied to the roadwork zone."]
    #[doc = ""]
    #[doc = "\n@field incidentIndication: the optional incident related to the roadworks to provide additional information of the roadworks zone."]
    #[doc = ""]
    #[doc = "\n@field recommendedPath: the optional recommended itinerary in order to contour the roadworks zone. If present, a recommended path "]
    #[doc = "shall be a list of path points in the order from the starting point closest to the roadworks zone to the end point of the recommended path. "]
    #[doc = ""]
    #[doc = "\n@field startingPointSpeedLimit: the optional effective starting position of a speed limit being applied to the roadwork zone, "]
    #[doc = "with respect to the component eventPosition on the Management Container."]
    #[doc = "This component shall be present if the speed limit is applied at a certain distance prior to the roadwork zone starting position."]
    #[doc = ""]
    #[doc = "\n@field trafficFlowRule: optionally indicates the side of the road to which the traffic should flow around a roadwork."]
    #[doc = ""]
    #[doc = "\n@field referenceDenms: an optional sequence of actionIds for different DENMs that describe the same event. "]
    #[doc = "If it is available, it indicates the actionIds of all other DENMs describing this event."]
    #[doc = ""]
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
        pub incident_indication: Option<CauseCodeV2>,
        #[rasn(identifier = "recommendedPath")]
        pub recommended_path: Option<ItineraryPath>,
        #[rasn(identifier = "startingPointSpeedLimit")]
        pub starting_point_speed_limit: Option<DeltaReferencePosition>,
        #[rasn(identifier = "trafficFlowRule")]
        pub traffic_flow_rule: Option<TrafficRule>,
        #[rasn(identifier = "referenceDenms")]
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
    #[doc = " Inner type "]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct SituationContainerExtGroupLinkedDenms {
        #[rasn(identifier = "linkedDenms")]
        pub linked_denms: Option<ActionIdList>,
        #[rasn(identifier = "eventEnd")]
        pub event_end: Option<Position1d>,
    }
    impl SituationContainerExtGroupLinkedDenms {
        pub fn new(linked_denms: Option<ActionIdList>, event_end: Option<Position1d>) -> Self {
            Self {
                linked_denms,
                event_end,
            }
        }
    }

    #[doc = "This type represents the situation container."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field informationQuality: the quality level of the information provided by the ITS-S application of the originating ITS-S. "]
    #[doc = "It indicates the probability of the detected event being truly existent at the event position."]
    #[doc = ""]
    #[doc = "\n@field eventType: the event type, including direct cause and sub cause."]
    #[doc = ""]
    #[doc = "\n@field linkedCause: the optional type of a linked event co-existing at the same time and the same place (same event zone), "]
    #[doc = "including direct cause and sub cause of the linked event, for which no other DENM is sent out."]
    #[doc = ""]
    #[doc = "\n@field eventZone: an optional list of EventPoint, using the position indicated in the component eventPosition of the Management container "]
    #[doc = "as the reference position for the first EventPoint."]
    #[doc = ""]
    #[doc = "\n@field linkedDenms: the optional list of DF ActionId, pointing to DENMs that are semantically connected because applying to consecutive "]
    #[doc = "event zones at the same time."]
    #[doc = ""]
    #[doc = "\n@field eventEnd: the end position of the event along the road that is affected by the event w.r.t. the component eventPosition of the "]
    #[doc = "Management container. This end position is represented by the length of the event along the road. "]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    pub struct SituationContainer {
        #[rasn(identifier = "informationQuality")]
        pub information_quality: InformationQuality,
        #[rasn(identifier = "eventType")]
        pub event_type: CauseCodeV2,
        #[rasn(identifier = "linkedCause")]
        pub linked_cause: Option<CauseCodeV2>,
        #[rasn(identifier = "eventZone")]
        pub event_zone: Option<EventZone>,
        #[rasn(extension_addition_group, identifier = "SEQUENCE")]
        pub ext_group_linked_denms: Option<SituationContainerExtGroupLinkedDenms>,
    }
    impl SituationContainer {
        pub fn new(
            information_quality: InformationQuality,
            event_type: CauseCodeV2,
            linked_cause: Option<CauseCodeV2>,
            event_zone: Option<EventZone>,
            ext_group_linked_denms: Option<SituationContainerExtGroupLinkedDenms>,
        ) -> Self {
            Self {
                information_quality,
                event_type,
                linked_cause,
                event_zone,
                ext_group_linked_denms,
            }
        }
    }

    #[doc = "This type contains detailed information about a stationary vehicle."]
    #[doc = ""]
    #[doc = "It shall include the following components: "]
    #[doc = ""]
    #[doc = "\n@field stationarySince: the optional time duration of the stationary vehicle being stationary."]
    #[doc = ""]
    #[doc = "\n@field stationaryCause: optional additional information to describe causes of the stationary vehicle event such as human problem."]
    #[doc = ""]
    #[doc = "\n@field carryingDangerousGoods: optional information on the type of dangerous goods, the required emergency action and other information."]
    #[doc = ""]
    #[doc = "\n@field numberOfOccupants: the optional estimated number of occupants involved in the stationary vehicle event."]
    #[doc = ""]
    #[doc = "\n@field vehicleIdentification: the optional identification of the stationary vehicle."]
    #[doc = ""]
    #[doc = "\n@field energyStorageType: the optional vehicle energy storage type information of the stationary vehicle, such as electric, diesel, etc."]
    #[doc = ""]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags)]
    pub struct StationaryVehicleContainer {
        #[rasn(identifier = "stationarySince")]
        pub stationary_since: Option<StationarySince>,
        #[rasn(identifier = "stationaryCause")]
        pub stationary_cause: Option<CauseCodeV2>,
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

    #[doc = "  * This indicates the termination type of generated DENM, i.e. if it is a cancellation DENM or a negation DENM"]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
    #[rasn(enumerated)]
    pub enum Termination {
        isCancellation = 0,
        isNegation = 1,
    }

    #[doc = "This type specifies the default value for DENM validity duration used for DENM protocol operation."]

    pub static DEFAULT_VALIDITY: DeltaTimeSecond = DeltaTimeSecond(600);
}
