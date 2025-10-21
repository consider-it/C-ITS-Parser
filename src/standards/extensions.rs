// Copyright (c) 2024 consider it GmbH

//! Manual implementation of things missing from code generated from ASN.1
//!
//! - rasn-compiler v0.14.3 does not generate a way to access named BIT STRING bits

pub mod cdd_1_3_1_1 {
    use crate::standards::cdd_1_3_1_1::its_container::{
        AccelerationControl, EmergencyPriority, ExteriorLights, LightBarSirenInUse,
        SpecialTransportType,
    };

    // used in CAM 1.4.1 via BasicVehicleContainerHighFrequency
    impl AccelerationControl {
        pub fn get_brake_pedal_engaged(&self) -> bool {
            self.0[0]
        }
        pub fn get_gas_pedal_engaged(&self) -> bool {
            self.0[1]
        }
        pub fn get_emergency_brake_engaged(&self) -> bool {
            self.0[2]
        }
        pub fn get_collision_warning_engaged(&self) -> bool {
            self.0[3]
        }
        pub fn get_acc_engaged(&self) -> bool {
            self.0[4]
        }
        pub fn get_cruise_control_engaged(&self) -> bool {
            self.0[5]
        }
        pub fn get_speed_limiter_engaged(&self) -> bool {
            self.0[6]
        }

        pub fn set_brake_pedal_engaged(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_gas_pedal_engaged(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_emergency_brake_engaged(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_collision_warning_engaged(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_acc_engaged(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_cruise_control_engaged(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_speed_limiter_engaged(&mut self, value: bool) {
            self.0.set(6, value)
        }
    }
    impl std::fmt::Display for AccelerationControl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_brake_pedal_engaged() {
                items.push("brakePedalEngaged: 1".into());
            }
            if self.get_gas_pedal_engaged() {
                items.push("gasPedalEngaged: 1".into());
            }
            if self.get_emergency_brake_engaged() {
                items.push("emergencyBrakeEngaged: 1".into());
            }
            if self.get_collision_warning_engaged() {
                items.push("collisionWarningEngaged: 1".into());
            }
            if self.get_acc_engaged() {
                items.push("accEngaged: 1".into());
            }
            if self.get_cruise_control_engaged() {
                items.push("cruiseControlEngaged: 1".into());
            }
            if self.get_speed_limiter_engaged() {
                items.push("speedLimiterEngaged: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    // used in CAM 1.4.1 via BasicVehicleContainerLowFrequency
    impl ExteriorLights {
        pub fn get_low_beam_headlights_on(&self) -> bool {
            self.0[0]
        }
        pub fn get_high_beam_headlights_on(&self) -> bool {
            self.0[1]
        }
        pub fn get_left_turn_signal_on(&self) -> bool {
            self.0[2]
        }
        pub fn get_right_turn_signal_on(&self) -> bool {
            self.0[3]
        }
        pub fn get_daytime_running_lights_on(&self) -> bool {
            self.0[4]
        }
        pub fn get_reverse_light_on(&self) -> bool {
            self.0[5]
        }
        pub fn get_fog_light_on(&self) -> bool {
            self.0[6]
        }
        pub fn get_parking_lights_on(&self) -> bool {
            self.0[7]
        }

        pub fn set_low_beam_headlights_on(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_high_beam_headlights_on(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_left_turn_signal_on(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_right_turn_signal_on(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_daytime_running_lights_on(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_reverse_light_on(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_fog_light_on(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_parking_lights_on(&mut self, value: bool) {
            self.0.set(7, value)
        }
    }
    impl std::fmt::Display for ExteriorLights {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_low_beam_headlights_on() {
                items.push("lowBeamHeadlightsOn: 1".into());
            }
            if self.get_high_beam_headlights_on() {
                items.push("highBeamHeadlightsOn: 1".into());
            }
            if self.get_left_turn_signal_on() {
                items.push("leftTurnSignalOn: 1".into());
            }
            if self.get_right_turn_signal_on() {
                items.push("rightTurnSignalOn: 1".into());
            }
            if self.get_daytime_running_lights_on() {
                items.push("daytimeRunningLightsOn: 1".into());
            }
            if self.get_reverse_light_on() {
                items.push("reverseLightOn: 1".into());
            }
            if self.get_fog_light_on() {
                items.push("fogLightOn: 1".into());
            }
            if self.get_parking_lights_on() {
                items.push("parkingLightsOn: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    // used in CAM 1.4.1 via SpecialVehicleContainer -> EmergencyContainer
    impl EmergencyPriority {
        pub fn get_request_for_right_of_way(&self) -> bool {
            self.0[0]
        }
        pub fn get_request_for_free_crossing_at_atraffic_light(&self) -> bool {
            self.0[1]
        }

        pub fn set_request_for_right_of_way(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_request_for_free_crossing_at_atraffic_light(&mut self, value: bool) {
            self.0.set(1, value)
        }
    }
    impl std::fmt::Display for EmergencyPriority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "requestForRightOfWay: {}, requestForFreeCrossingAtATrafficLight: {}",
                self.get_request_for_right_of_way(),
                self.get_request_for_free_crossing_at_atraffic_light()
            )
        }
    }

    // used in CAM 1.4.1 via SpecialVehicleContainer -> (multiple containers)
    impl LightBarSirenInUse {
        pub fn get_light_bar_activated(&self) -> bool {
            self.0[0]
        }
        pub fn get_siren_activated(&self) -> bool {
            self.0[1]
        }

        pub fn set_light_bar_activated(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_siren_activated(&mut self, value: bool) {
            self.0.set(1, value)
        }
    }
    impl std::fmt::Display for LightBarSirenInUse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "lightBarActivated: {}, sirenActivated: {}",
                self.get_light_bar_activated(),
                self.get_siren_activated()
            )
        }
    }

    // used in CAM 1.4.1 via SpecialVehicleContainer -> SpecialTransportContainer
    impl SpecialTransportType {
        pub fn get_heavy_load(&self) -> bool {
            self.0[0]
        }
        pub fn get_excess_width(&self) -> bool {
            self.0[1]
        }
        pub fn get_excess_length(&self) -> bool {
            self.0[2]
        }
        pub fn get_excess_height(&self) -> bool {
            self.0[3]
        }

        pub fn set_heavy_load(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_excess_width(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_excess_length(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_excess_height(&mut self, value: bool) {
            self.0.set(3, value)
        }
    }
    impl std::fmt::Display for SpecialTransportType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_heavy_load() {
                items.push("heavyLoad: 1".into());
            }
            if self.get_excess_width() {
                items.push("excessWidth: 1".into());
            }
            if self.get_excess_length() {
                items.push("excessLength: 1".into());
            }
            if self.get_excess_height() {
                items.push("excessHeight: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }
}

pub mod denm_2_1_1 {
    use crate::standards::denm_2_1_1::etsi_its_cdd::EnergyStorageType;

    impl EnergyStorageType {
        pub fn get_hydrogen_storage(&self) -> bool {
            self.0[0]
        }
        pub fn get_electric_energy_storage(&self) -> bool {
            self.0[1]
        }
        pub fn get_liquid_propane_gas(&self) -> bool {
            self.0[2]
        }
        pub fn get_compressed_natural_gas(&self) -> bool {
            self.0[3]
        }
        pub fn get_diesel(&self) -> bool {
            self.0[4]
        }
        pub fn get_gasoline(&self) -> bool {
            self.0[5]
        }
        pub fn get_ammonia(&self) -> bool {
            self.0[6]
        }

        pub fn set_hydrogen_storage(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_electric_energy_storage(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_liquid_propane_gas(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_compressed_natural_gas(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_diesel(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_gasoline(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_ammonia(&mut self, value: bool) {
            self.0.set(6, value)
        }
    }
    impl std::fmt::Display for EnergyStorageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_hydrogen_storage() {
                items.push("hydrogenStorage: 1".into());
            }
            if self.get_electric_energy_storage() {
                items.push("electricEnergyStorage: 1".into());
            }
            if self.get_liquid_propane_gas() {
                items.push("liquidPropaneGas: 1".into());
            }
            if self.get_compressed_natural_gas() {
                items.push("compressedNaturalGas: 1".into());
            }
            if self.get_diesel() {
                items.push("diesel: 1".into());
            }
            if self.get_gasoline() {
                items.push("gasoline: 1".into());
            }
            if self.get_ammonia() {
                items.push("ammonia: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }
}

pub mod is_1_3_1 {
    use crate::standards::is_1_3_1::etsi_schema::{
        AllowedManeuvers, DayOfWeek, IntersectionStatusObject, LaneAttributes,
        LaneAttributesBarrier, LaneAttributesBike, LaneAttributesCrosswalk, LaneAttributesParking,
        LaneAttributesSidewalk, LaneAttributesStriping, LaneAttributesTrackedVehicle,
        LaneAttributesVehicle, LaneDirection, LaneSharing, LaneTypeAttributes,
        TransitVehicleStatus, PMD,
    };

    // MAPEM/ SPATEM

    impl std::fmt::Display for LaneAttributes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "LaneTypeAttributes{{{}}}, LaneDirection{{{}}}, LaneSharing{{{}}}",
                self.lane_type, self.directional_use, self.shared_with
            )
            // regional value left out for now
        }
    }

    impl std::fmt::Display for LaneTypeAttributes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LaneTypeAttributes::vehicle(attrs) => write!(f, "vehicle({attrs})"),
                LaneTypeAttributes::crosswalk(attrs) => write!(f, "crosswalk({attrs})"),
                LaneTypeAttributes::bikeLane(attrs) => write!(f, "bikeLane({attrs})"),
                LaneTypeAttributes::sidewalk(attrs) => write!(f, "sidewalk({attrs})"),
                LaneTypeAttributes::median(attrs) => write!(f, "median({attrs})"),
                LaneTypeAttributes::striping(attrs) => write!(f, "striping({attrs})"),
                LaneTypeAttributes::trackedVehicle(attrs) => write!(f, "trackedVehicle({attrs})"),
                LaneTypeAttributes::parking(attrs) => write!(f, "parking({attrs})"),
            }
        }
    }

    impl LaneSharing {
        pub fn get_overlapping_lane_description_provided(&self) -> bool {
            self.0[0]
        }
        pub fn get_multiple_lanes_treated_as_one_lane(&self) -> bool {
            self.0[1]
        }
        pub fn get_other_non_motorized_traffic_types(&self) -> bool {
            self.0[2]
        }
        pub fn get_individual_motorized_vehicle_traffic(&self) -> bool {
            self.0[3]
        }
        pub fn get_bus_vehicle_traffic(&self) -> bool {
            self.0[4]
        }
        pub fn get_taxi_vehicle_traffic(&self) -> bool {
            self.0[5]
        }
        pub fn get_pedestrians_traffic(&self) -> bool {
            self.0[6]
        }
        pub fn get_cyclist_vehicle_traffic(&self) -> bool {
            self.0[7]
        }
        pub fn get_tracked_vehicle_traffic(&self) -> bool {
            self.0[8]
        }
        pub fn get_pedestrian_traffic(&self) -> bool {
            self.0[9]
        }

        pub fn set_overlapping_lane_description_provided(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_multiple_lanes_treated_as_one_lane(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_other_non_motorized_traffic_types(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_individual_motorized_vehicle_traffic(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_bus_vehicle_traffic(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_taxi_vehicle_traffic(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_pedestrians_traffic(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_cyclist_vehicle_traffic(&mut self, value: bool) {
            self.0.set(7, value)
        }
        pub fn set_tracked_vehicle_traffic(&mut self, value: bool) {
            self.0.set(8, value)
        }
        pub fn set_pedestrian_traffic(&mut self, value: bool) {
            self.0.set(9, value)
        }
    }
    impl std::fmt::Display for LaneSharing {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_overlapping_lane_description_provided() {
                items.push("overlappingLaneDescriptionProvided: 1".into());
            }
            if self.get_multiple_lanes_treated_as_one_lane() {
                items.push("multipleLanesTreatedAsOneLane: 1".into());
            }
            if self.get_other_non_motorized_traffic_types() {
                items.push("otherNonMotorizedTrafficTypes: 1".into());
            }
            if self.get_individual_motorized_vehicle_traffic() {
                items.push("individualMotorizedVehicleTraffic: 1".into());
            }
            if self.get_bus_vehicle_traffic() {
                items.push("busVehicleTraffic: 1".into());
            }
            if self.get_taxi_vehicle_traffic() {
                items.push("taxiVehicleTraffic: 1".into());
            }
            if self.get_pedestrians_traffic() {
                items.push("pedestriansTraffic: 1".into());
            }
            if self.get_cyclist_vehicle_traffic() {
                items.push("cyclistVehicleTraffic: 1".into());
            }
            if self.get_tracked_vehicle_traffic() {
                items.push("trackedVehicleTraffic: 1".into());
            }
            if self.get_pedestrian_traffic() {
                items.push("pedestrianTraffic: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl AllowedManeuvers {
        pub fn get_maneuver_straight_allowed(&self) -> bool {
            self.0[0]
        }
        pub fn get_maneuver_left_allowed(&self) -> bool {
            self.0[1]
        }
        pub fn get_maneuver_right_allowed(&self) -> bool {
            self.0[2]
        }
        pub fn get_maneuver_uturn_allowed(&self) -> bool {
            self.0[3]
        }
        pub fn get_maneuver_left_turn_on_red_allowed(&self) -> bool {
            self.0[4]
        }
        pub fn get_maneuver_right_turn_on_red_allowed(&self) -> bool {
            self.0[5]
        }
        pub fn get_maneuver_lane_change_allowed(&self) -> bool {
            self.0[6]
        }
        pub fn get_maneuver_no_stopping_allowed(&self) -> bool {
            self.0[7]
        }
        pub fn get_yield_allways_required(&self) -> bool {
            self.0[8]
        }
        pub fn get_go_with_halt(&self) -> bool {
            self.0[9]
        }
        pub fn get_caution(&self) -> bool {
            self.0[10]
        }
        pub fn get_reserved1(&self) -> bool {
            self.0[11]
        }

        pub fn set_maneuver_straight_allowed(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_maneuver_left_allowed(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_maneuver_right_allowed(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_maneuver_uturn_allowed(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_maneuver_left_turn_on_red_allowed(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_maneuver_right_turn_on_red_allowed(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_maneuver_lane_change_allowed(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_maneuver_no_stopping_allowed(&mut self, value: bool) {
            self.0.set(7, value)
        }
        pub fn set_yield_allways_required(&mut self, value: bool) {
            self.0.set(8, value)
        }
        pub fn set_go_with_halt(&mut self, value: bool) {
            self.0.set(9, value)
        }
        pub fn set_caution(&mut self, value: bool) {
            self.0.set(10, value)
        }
        pub fn set_reserved1(&mut self, value: bool) {
            self.0.set(11, value)
        }
    }
    impl std::fmt::Display for AllowedManeuvers {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_maneuver_straight_allowed() {
                items.push("maneuverStraightAllowed: 1".into());
            }
            if self.get_maneuver_left_allowed() {
                items.push("maneuverLeftAllowed: 1".into());
            }
            if self.get_maneuver_right_allowed() {
                items.push("maneuverRightAllowed: 1".into());
            }
            if self.get_maneuver_uturn_allowed() {
                items.push("maneuverUTurnAllowed: 1".into());
            }
            if self.get_maneuver_left_turn_on_red_allowed() {
                items.push("maneuverLeftTurnOnRedAllowed: 1".into());
            }
            if self.get_maneuver_right_turn_on_red_allowed() {
                items.push("maneuverRightTurnOnRedAllowed: 1".into());
            }
            if self.get_maneuver_lane_change_allowed() {
                items.push("maneuverLaneChangeAllowed: 1".into());
            }
            if self.get_maneuver_no_stopping_allowed() {
                items.push("maneuverNoStoppingAllowed: 1".into());
            }
            if self.get_yield_allways_required() {
                items.push("yieldAllwaysRequired: 1".into());
            }
            if self.get_go_with_halt() {
                items.push("goWithHalt: 1".into());
            }
            if self.get_caution() {
                items.push("caution: 1".into());
            }
            if self.get_reserved1() {
                items.push("reserved1: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl IntersectionStatusObject {
        pub fn get_manual_control_is_enabled(&self) -> bool {
            self.0[0]
        }
        pub fn get_stop_time_is_activated(&self) -> bool {
            self.0[1]
        }
        pub fn get_failure_flash(&self) -> bool {
            self.0[2]
        }
        pub fn get_preempt_is_active(&self) -> bool {
            self.0[3]
        }
        pub fn get_signal_priority_is_active(&self) -> bool {
            self.0[4]
        }
        pub fn get_fixed_time_operation(&self) -> bool {
            self.0[5]
        }
        pub fn get_traffic_dependent_operation(&self) -> bool {
            self.0[6]
        }
        pub fn get_standby_operation(&self) -> bool {
            self.0[7]
        }
        pub fn get_failure_mode(&self) -> bool {
            self.0[8]
        }
        pub fn get_off(&self) -> bool {
            self.0[9]
        }
        pub fn get_recent_map_message_update(&self) -> bool {
            self.0[10]
        }
        pub fn get_recent_change_in_map_assigned_lanes_ids_used(&self) -> bool {
            self.0[11]
        }
        pub fn get_no_valid_map_is_available_at_this_time(&self) -> bool {
            self.0[12]
        }
        pub fn get_no_valid_spat_is_available_at_this_time(&self) -> bool {
            self.0[13]
        }

        pub fn set_manual_control_is_enabled(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_stop_time_is_activated(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_failure_flash(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_preempt_is_active(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_signal_priority_is_active(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_fixed_time_operation(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_traffic_dependent_operation(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_standby_operation(&mut self, value: bool) {
            self.0.set(7, value)
        }
        pub fn set_failure_mode(&mut self, value: bool) {
            self.0.set(8, value)
        }
        pub fn set_off(&mut self, value: bool) {
            self.0.set(9, value)
        }
        pub fn set_recent_map_message_update(&mut self, value: bool) {
            self.0.set(10, value)
        }
        pub fn set_recent_change_in_map_assigned_lanes_ids_used(&mut self, value: bool) {
            self.0.set(11, value)
        }
        pub fn set_no_valid_map_is_available_at_this_time(&mut self, value: bool) {
            self.0.set(12, value)
        }
        pub fn set_no_valid_spat_is_available_at_this_time(&mut self, value: bool) {
            self.0.set(13, value)
        }
    }
    impl std::fmt::Display for IntersectionStatusObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_manual_control_is_enabled() {
                items.push("manualControlIsEnabled: 1".into());
            }
            if self.get_stop_time_is_activated() {
                items.push("stopTimeIsActivated: 1".into());
            }
            if self.get_failure_flash() {
                items.push("failureFlash: 1".into());
            }
            if self.get_preempt_is_active() {
                items.push("preemptIsActive: 1".into());
            }
            if self.get_signal_priority_is_active() {
                items.push("signalPriorityIsActive: 1".into());
            }
            if self.get_fixed_time_operation() {
                items.push("fixedTimeOperation: 1".into());
            }
            if self.get_traffic_dependent_operation() {
                items.push("trafficDependentOperation: 1".into());
            }
            if self.get_standby_operation() {
                items.push("standbyOperation: 1".into());
            }
            if self.get_failure_mode() {
                items.push("failureMode: 1".into());
            }
            if self.get_off() {
                items.push("off: 1".into());
            }
            if self.get_recent_map_message_update() {
                items.push("recentMAPmessageUpdate: 1".into());
            }
            if self.get_recent_change_in_map_assigned_lanes_ids_used() {
                items.push("recentChangeInMAPassignedLanesIDsUsed: 1".into());
            }
            if self.get_no_valid_map_is_available_at_this_time() {
                items.push("noValidMAPisAvailableAtThisTime: 1".into());
            }
            if self.get_no_valid_spat_is_available_at_this_time() {
                items.push("noValidSPATisAvailableAtThisTime: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesBarrier {
        pub fn get_median_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_median(&self) -> bool {
            self.0[1]
        }
        pub fn get_white_line_hashing(&self) -> bool {
            self.0[2]
        }
        pub fn get_striped_lines(&self) -> bool {
            self.0[3]
        }
        pub fn get_double_striped_lines(&self) -> bool {
            self.0[4]
        }
        pub fn get_traffic_cones(&self) -> bool {
            self.0[5]
        }
        pub fn get_construction_barrier(&self) -> bool {
            self.0[6]
        }
        pub fn get_traffic_channels(&self) -> bool {
            self.0[7]
        }
        pub fn get_low_curbs(&self) -> bool {
            self.0[8]
        }
        pub fn get_high_curbs(&self) -> bool {
            self.0[9]
        }

        pub fn set_median_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_median(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_white_line_hashing(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_striped_lines(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_double_striped_lines(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_traffic_cones(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_construction_barrier(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_traffic_channels(&mut self, value: bool) {
            self.0.set(7, value)
        }
        pub fn set_low_curbs(&mut self, value: bool) {
            self.0.set(8, value)
        }
        pub fn set_high_curbs(&mut self, value: bool) {
            self.0.set(9, value)
        }
    }
    impl std::fmt::Display for LaneAttributesBarrier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_median_revocable_lane() {
                items.push("median-RevocableLane: 1".into());
            }
            if self.get_median() {
                items.push("median: 1".into());
            }
            if self.get_white_line_hashing() {
                items.push("whiteLineHashing: 1".into());
            }
            if self.get_striped_lines() {
                items.push("stripedLines: 1".into());
            }
            if self.get_double_striped_lines() {
                items.push("doubleStripedLines: 1".into());
            }
            if self.get_traffic_cones() {
                items.push("trafficCones: 1".into());
            }
            if self.get_construction_barrier() {
                items.push("constructionBarrier: 1".into());
            }
            if self.get_traffic_channels() {
                items.push("trafficChannels: 1".into());
            }
            if self.get_low_curbs() {
                items.push("lowCurbs: 1".into());
            }
            if self.get_high_curbs() {
                items.push("highCurbs: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesBike {
        pub fn get_bike_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_pedestrian_use_allowed(&self) -> bool {
            self.0[1]
        }
        pub fn get_is_bike_fly_over_lane(&self) -> bool {
            self.0[2]
        }
        pub fn get_fixed_cycle_time(&self) -> bool {
            self.0[3]
        }
        pub fn get_bi_directional_cycle_times(&self) -> bool {
            self.0[4]
        }
        pub fn get_isolated_by_barrier(&self) -> bool {
            self.0[5]
        }
        pub fn get_unsignalized_segments_present(&self) -> bool {
            self.0[6]
        }

        pub fn set_bike_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_pedestrian_use_allowed(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_is_bike_fly_over_lane(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_fixed_cycle_time(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_bi_directional_cycle_times(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_isolated_by_barrier(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_unsignalized_segments_present(&mut self, value: bool) {
            self.0.set(6, value)
        }
    }
    impl std::fmt::Display for LaneAttributesBike {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_bike_revocable_lane() {
                items.push("bikeRevocableLane: 1".into());
            }
            if self.get_pedestrian_use_allowed() {
                items.push("pedestrianUseAllowed: 1".into());
            }
            if self.get_is_bike_fly_over_lane() {
                items.push("isBikeFlyOverLane: 1".into());
            }
            if self.get_fixed_cycle_time() {
                items.push("fixedCycleTime: 1".into());
            }
            if self.get_bi_directional_cycle_times() {
                items.push("biDirectionalCycleTimes: 1".into());
            }
            if self.get_isolated_by_barrier() {
                items.push("isolatedByBarrier: 1".into());
            }
            if self.get_unsignalized_segments_present() {
                items.push("unsignalizedSegmentsPresent: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesCrosswalk {
        pub fn get_crosswalk_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_bicycle_use_allowed(&self) -> bool {
            self.0[1]
        }
        pub fn get_is_x_walk_fly_over_lane(&self) -> bool {
            self.0[2]
        }
        pub fn get_fixed_cycle_time(&self) -> bool {
            self.0[3]
        }
        pub fn get_bi_directional_cycle_times(&self) -> bool {
            self.0[4]
        }
        pub fn get_has_push_to_walk_button(&self) -> bool {
            self.0[5]
        }
        pub fn get_audio_support(&self) -> bool {
            self.0[6]
        }
        pub fn get_rf_signal_request_present(&self) -> bool {
            self.0[7]
        }
        pub fn get_unsignalized_segments_present(&self) -> bool {
            self.0[8]
        }

        pub fn set_crosswalk_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_bicycle_use_allowed(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_is_x_walk_fly_over_lane(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_fixed_cycle_time(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_bi_directional_cycle_times(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_has_push_to_walk_button(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_audio_support(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_rf_signal_request_present(&mut self, value: bool) {
            self.0.set(7, value)
        }
        pub fn set_unsignalized_segments_present(&mut self, value: bool) {
            self.0.set(8, value)
        }
    }
    impl std::fmt::Display for LaneAttributesCrosswalk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_crosswalk_revocable_lane() {
                items.push("crosswalkRevocableLane: 1".into());
            }
            if self.get_bicycle_use_allowed() {
                items.push("bicyleUseAllowed: 1".into());
            }
            if self.get_is_x_walk_fly_over_lane() {
                items.push("isXwalkFlyOverLane: 1".into());
            }
            if self.get_fixed_cycle_time() {
                items.push("fixedCycleTime: 1".into());
            }
            if self.get_bi_directional_cycle_times() {
                items.push("biDirectionalCycleTimes: 1".into());
            }
            if self.get_has_push_to_walk_button() {
                items.push("hasPushToWalkButton: 1".into());
            }
            if self.get_audio_support() {
                items.push("audioSupport: 1".into());
            }
            if self.get_rf_signal_request_present() {
                items.push("rfSignalRequestPresent: 1".into());
            }
            if self.get_unsignalized_segments_present() {
                items.push("unsignalizedSegmentsPresent: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesParking {
        pub fn get_parking_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_parallel_parking_in_use(&self) -> bool {
            self.0[1]
        }
        pub fn get_head_in_parking_in_use(&self) -> bool {
            self.0[2]
        }
        pub fn get_do_not_park_zone(&self) -> bool {
            self.0[3]
        }
        pub fn get_parking_for_bus_use(&self) -> bool {
            self.0[4]
        }
        pub fn get_parking_for_taxi_use(&self) -> bool {
            self.0[5]
        }
        pub fn get_no_public_parking_use(&self) -> bool {
            self.0[6]
        }

        pub fn set_parking_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_parallel_parking_in_use(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_head_in_parking_in_use(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_do_not_park_zone(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_parking_for_bus_use(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_parking_for_taxi_use(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_no_public_parking_use(&mut self, value: bool) {
            self.0.set(6, value)
        }
    }
    impl std::fmt::Display for LaneAttributesParking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_parking_revocable_lane() {
                items.push("parkingRevocableLane: 1".into());
            }
            if self.get_parallel_parking_in_use() {
                items.push("parallelParkingInUse: 1".into());
            }
            if self.get_head_in_parking_in_use() {
                items.push("headInParkingInUse: 1".into());
            }
            if self.get_do_not_park_zone() {
                items.push("doNotParkZone: 1".into());
            }
            if self.get_parking_for_bus_use() {
                items.push("parkingForBusUse: 1".into());
            }
            if self.get_parking_for_taxi_use() {
                items.push("parkingForTaxiUse: 1".into());
            }
            if self.get_no_public_parking_use() {
                items.push("noPublicParkingUse: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesSidewalk {
        pub fn get_sidewalk_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_bicycle_use_allowed(&self) -> bool {
            self.0[1]
        }
        pub fn get_is_sidewalk_fly_over_lane(&self) -> bool {
            self.0[2]
        }
        pub fn get_walk_bikes(&self) -> bool {
            self.0[3]
        }

        pub fn set_sidewalk_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_bicycle_use_allowed(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_is_sidewalk_fly_over_lane(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_walk_bikes(&mut self, value: bool) {
            self.0.set(3, value)
        }
    }
    impl std::fmt::Display for LaneAttributesSidewalk {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_sidewalk_revocable_lane() {
                items.push("sidewalk-RevocableLane: 1".into());
            }
            if self.get_bicycle_use_allowed() {
                items.push("bicyleUseAllowed: 1".into());
            }
            if self.get_is_sidewalk_fly_over_lane() {
                items.push("isSidewalkFlyOverLane: 1".into());
            }
            if self.get_walk_bikes() {
                items.push("walkBikes: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesStriping {
        pub fn get_stripe_to_connecting_lanes_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_stripe_draw_on_left(&self) -> bool {
            self.0[1]
        }
        pub fn get_stripe_draw_on_right(&self) -> bool {
            self.0[2]
        }
        pub fn get_stripe_to_connecting_lanes_left(&self) -> bool {
            self.0[3]
        }
        pub fn get_stripe_to_connecting_lanes_right(&self) -> bool {
            self.0[4]
        }
        pub fn get_stripe_to_connecting_lanes_ahead(&self) -> bool {
            self.0[5]
        }

        pub fn set_stripe_to_connecting_lanes_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_stripe_draw_on_left(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_stripe_draw_on_right(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_stripe_to_connecting_lanes_left(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_stripe_to_connecting_lanes_right(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_stripe_to_connecting_lanes_ahead(&mut self, value: bool) {
            self.0.set(5, value)
        }
    }
    impl std::fmt::Display for LaneAttributesStriping {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_stripe_to_connecting_lanes_revocable_lane() {
                items.push("stripeToConnectingLanesRevocableLane: 1".into());
            }
            if self.get_stripe_draw_on_left() {
                items.push("stripeDrawOnLeft: 1".into());
            }
            if self.get_stripe_draw_on_right() {
                items.push("stripeDrawOnRight: 1".into());
            }
            if self.get_stripe_to_connecting_lanes_left() {
                items.push("stripeToConnectingLanesLeft: 1".into());
            }
            if self.get_stripe_to_connecting_lanes_right() {
                items.push("stripeToConnectingLanesRight: 1".into());
            }
            if self.get_stripe_to_connecting_lanes_ahead() {
                items.push("stripeToConnectingLanesAhead: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesTrackedVehicle {
        pub fn get_spec_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_spec_commuter_rail_road_track(&self) -> bool {
            self.0[1]
        }
        pub fn get_spec_light_rail_road_track(&self) -> bool {
            self.0[2]
        }
        pub fn get_spec_heavy_rail_road_track(&self) -> bool {
            self.0[3]
        }
        pub fn get_spec_other_rail_type(&self) -> bool {
            self.0[4]
        }

        pub fn set_spec_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_spec_commuter_rail_road_track(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_spec_light_rail_road_track(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_spec_heavy_rail_road_track(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_spec_other_rail_type(&mut self, value: bool) {
            self.0.set(4, value)
        }
    }
    impl std::fmt::Display for LaneAttributesTrackedVehicle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_spec_revocable_lane() {
                items.push("spec-RevocableLane: 1".into());
            }
            if self.get_spec_commuter_rail_road_track() {
                items.push("spec-commuterRailRoadTrack: 1".into());
            }
            if self.get_spec_light_rail_road_track() {
                items.push("spec-lightRailRoadTrack: 1".into());
            }
            if self.get_spec_heavy_rail_road_track() {
                items.push("spec-heavyRailRoadTrack: 1".into());
            }
            if self.get_spec_other_rail_type() {
                items.push("spec-otherRailType: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneAttributesVehicle {
        pub fn get_is_vehicle_revocable_lane(&self) -> bool {
            self.0[0]
        }
        pub fn get_is_vehicle_fly_over_lane(&self) -> bool {
            self.0[1]
        }
        pub fn get_hov_lane_use_only(&self) -> bool {
            self.0[2]
        }
        pub fn get_restricted_to_bus_use(&self) -> bool {
            self.0[3]
        }
        pub fn get_restricted_to_taxi_use(&self) -> bool {
            self.0[4]
        }
        pub fn get_restricted_from_public_use(&self) -> bool {
            self.0[5]
        }
        pub fn get_has_irbeacon_coverage(&self) -> bool {
            self.0[6]
        }
        pub fn get_permission_on_request(&self) -> bool {
            self.0[7]
        }

        pub fn set_is_vehicle_revocable_lane(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_is_vehicle_fly_over_lane(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_hov_lane_use_only(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_restricted_to_bus_use(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_restricted_to_taxi_use(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_restricted_from_public_use(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_has_irbeacon_coverage(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_permission_on_request(&mut self, value: bool) {
            self.0.set(7, value)
        }
    }
    impl std::fmt::Display for LaneAttributesVehicle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_is_vehicle_revocable_lane() {
                items.push("isVehicleRevocableLane: 1".into());
            }
            if self.get_is_vehicle_fly_over_lane() {
                items.push("isVehicleFlyOverLane: 1".into());
            }
            if self.get_hov_lane_use_only() {
                items.push("hovLaneUseOnly: 1".into());
            }
            if self.get_restricted_to_bus_use() {
                items.push("restrictedToBusUse: 1".into());
            }
            if self.get_restricted_to_taxi_use() {
                items.push("restrictedToTaxiUse: 1".into());
            }
            if self.get_restricted_from_public_use() {
                items.push("restrictedFromPublicUse: 1".into());
            }
            if self.get_has_irbeacon_coverage() {
                items.push("hasIRbeaconCoverage: 1".into());
            }
            if self.get_permission_on_request() {
                items.push("permissionOnRequest: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl LaneDirection {
        pub fn get_ingress_path(&self) -> bool {
            self.0[0]
        }
        pub fn get_egress_path(&self) -> bool {
            self.0[1]
        }

        pub fn set_ingress_path(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_egress_path(&mut self, value: bool) {
            self.0.set(1, value)
        }
    }
    impl std::fmt::Display for LaneDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "ingressPath: {}, egressPath: {}",
                self.get_ingress_path(),
                self.get_egress_path()
            )
        }
    }

    impl TransitVehicleStatus {
        pub fn get_loading(&self) -> bool {
            self.0[0]
        }
        pub fn get_an_ada_use(&self) -> bool {
            self.0[1]
        }
        pub fn get_a_bike_load(&self) -> bool {
            self.0[2]
        }
        pub fn get_door_open(&self) -> bool {
            self.0[3]
        }
        pub fn get_charging(&self) -> bool {
            self.0[4]
        }

        pub fn set_loading(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_an_ada_use(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_a_bike_load(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_door_open(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_charging(&mut self, value: bool) {
            self.0.set(4, value)
        }
    }
    impl std::fmt::Display for TransitVehicleStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_loading() {
                items.push("loading: 1".into());
            }
            if self.get_an_ada_use() {
                items.push("anADAuse: 1".into());
            }
            if self.get_a_bike_load() {
                items.push("aBikeLoad: 1".into());
            }
            if self.get_door_open() {
                items.push("doorOpen: 1".into());
            }
            if self.get_charging() {
                items.push("charging: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl PMD {
        pub fn get_national_holiday(&self) -> bool {
            self.0[0]
        }
        pub fn get_even_days(&self) -> bool {
            self.0[1]
        }
        pub fn get_odd_days(&self) -> bool {
            self.0[2]
        }
        pub fn get_market_day(&self) -> bool {
            self.0[3]
        }

        pub fn set_national_holiday(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_even_days(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_odd_days(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_market_day(&mut self, value: bool) {
            self.0.set(3, value)
        }
    }
    impl std::fmt::Display for PMD {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_national_holiday() {
                items.push("national-holiday: 1".into());
            }
            if self.get_even_days() {
                items.push("even-days: 1".into());
            }
            if self.get_odd_days() {
                items.push("odd-days: 1".into());
            }
            if self.get_market_day() {
                items.push("market-day: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }

    impl DayOfWeek {
        pub fn get_unused(&self) -> bool {
            self.0[0]
        }
        pub fn get_monday(&self) -> bool {
            self.0[1]
        }
        pub fn get_tuesday(&self) -> bool {
            self.0[2]
        }
        pub fn get_wednesday(&self) -> bool {
            self.0[3]
        }
        pub fn get_thursday(&self) -> bool {
            self.0[4]
        }
        pub fn get_friday(&self) -> bool {
            self.0[5]
        }
        pub fn get_saturday(&self) -> bool {
            self.0[6]
        }
        pub fn get_sunday(&self) -> bool {
            self.0[7]
        }

        pub fn set_unused(&mut self, value: bool) {
            self.0.set(0, value)
        }
        pub fn set_monday(&mut self, value: bool) {
            self.0.set(1, value)
        }
        pub fn set_tuesday(&mut self, value: bool) {
            self.0.set(2, value)
        }
        pub fn set_wednesday(&mut self, value: bool) {
            self.0.set(3, value)
        }
        pub fn set_thursday(&mut self, value: bool) {
            self.0.set(4, value)
        }
        pub fn set_friday(&mut self, value: bool) {
            self.0.set(5, value)
        }
        pub fn set_saturday(&mut self, value: bool) {
            self.0.set(6, value)
        }
        pub fn set_sunday(&mut self, value: bool) {
            self.0.set(7, value)
        }
    }
    impl std::fmt::Display for DayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut items = Vec::<String>::new();

            if self.get_unused() {
                items.push("unused: 1".into());
            }
            if self.get_monday() {
                items.push("monday: 1".into());
            }
            if self.get_tuesday() {
                items.push("tuesday: 1".into());
            }
            if self.get_wednesday() {
                items.push("wednesday: 1".into());
            }
            if self.get_thursday() {
                items.push("thursday: 1".into());
            }
            if self.get_friday() {
                items.push("friday: 1".into());
            }
            if self.get_saturday() {
                items.push("saturday: 1".into());
            }
            if self.get_sunday() {
                items.push("sunday: 1".into());
            }

            write!(f, "{}", items.join(", "))
        }
    }
}
