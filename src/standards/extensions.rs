// Copyright (c) 2026 consider it GmbH

//! Manual implementation of things missing from code generated from ASN.1
//!
//! - rasn-compiler v0.14.3 does not generate a way to access named BIT STRING bits

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
pub enum ItsMessageId {
    Denm = 1,
    Cam = 2,
    Poi = 3,
    Spatem = 4,
    Mapem = 5,
    Ivim = 6,
    EvRsr = 7,
    Tistpgtransaction = 8,
    Srem = 9,
    Ssem = 10,
    Evcsn = 11,
    Saem = 12,
    Rtcmem = 13,
    Cpm = 14,
    Imzm = 15,
    Vam = 16,
    Dsm = 17,
    Pcim = 18,
    Pcvm = 19,
    Mcm = 20,
    Pam = 21,
}

impl ItsMessageId {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(feature = "_cdd_2_2_1")]
macro_rules! itsmessageid_conv {
    ($t:ty) => {
        impl From<crate::standards::extensions::ItsMessageId> for $t {
            fn from(value: crate::standards::extensions::ItsMessageId) -> Self {
                Self(value as u8)
            }
        }

        impl TryInto<crate::standards::extensions::ItsMessageId> for $t {
            type Error = String;

            fn try_into(self) -> Result<crate::standards::extensions::ItsMessageId, Self::Error> {
                match self.0 {
                    1 => Ok(crate::standards::extensions::ItsMessageId::Denm),
                    2 => Ok(crate::standards::extensions::ItsMessageId::Cam),
                    3 => Ok(crate::standards::extensions::ItsMessageId::Poi),
                    4 => Ok(crate::standards::extensions::ItsMessageId::Spatem),
                    5 => Ok(crate::standards::extensions::ItsMessageId::Mapem),
                    6 => Ok(crate::standards::extensions::ItsMessageId::Ivim),
                    7 => Ok(crate::standards::extensions::ItsMessageId::EvRsr),
                    8 => Ok(crate::standards::extensions::ItsMessageId::Tistpgtransaction),
                    9 => Ok(crate::standards::extensions::ItsMessageId::Srem),
                    10 => Ok(crate::standards::extensions::ItsMessageId::Ssem),
                    11 => Ok(crate::standards::extensions::ItsMessageId::Evcsn),
                    12 => Ok(crate::standards::extensions::ItsMessageId::Saem),
                    13 => Ok(crate::standards::extensions::ItsMessageId::Rtcmem),
                    14 => Ok(crate::standards::extensions::ItsMessageId::Cpm),
                    15 => Ok(crate::standards::extensions::ItsMessageId::Imzm),
                    16 => Ok(crate::standards::extensions::ItsMessageId::Vam),
                    17 => Ok(crate::standards::extensions::ItsMessageId::Dsm),
                    18 => Ok(crate::standards::extensions::ItsMessageId::Pcim),
                    19 => Ok(crate::standards::extensions::ItsMessageId::Pcvm),
                    20 => Ok(crate::standards::extensions::ItsMessageId::Mcm),
                    21 => Ok(crate::standards::extensions::ItsMessageId::Pam),
                    _ => Err(format!("MessageId {} not a known value", self.0)),
                }
            }
        }
    };
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
pub enum ItsStationType {
    Unknown = 0,
    Pedestrian = 1,
    Cyclist = 2,
    Moped = 3,
    Motorcycle = 4,
    Passengercar = 5,
    Bus = 6,
    Lighttruck = 7,
    Heavytruck = 8,
    Trailer = 9,
    Specialvehicles = 10,
    Tram = 11,
    LightVruVehicle = 12,
    Animal = 13,
    Roadsideunit = 15,
}

impl ItsStationType {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! itsstationtype_conv {
    ($t:ty) => {
        impl From<crate::standards::extensions::ItsStationType> for $t {
            fn from(value: crate::standards::extensions::ItsStationType) -> Self {
                Self(value as u8)
            }
        }

        impl TryInto<crate::standards::extensions::ItsStationType> for $t {
            type Error = String;

            fn try_into(self) -> Result<crate::standards::extensions::ItsStationType, Self::Error> {
                match self.0 {
                    0 => Ok(crate::standards::extensions::ItsStationType::Unknown),
                    1 => Ok(crate::standards::extensions::ItsStationType::Pedestrian),
                    2 => Ok(crate::standards::extensions::ItsStationType::Cyclist),
                    3 => Ok(crate::standards::extensions::ItsStationType::Moped),
                    4 => Ok(crate::standards::extensions::ItsStationType::Motorcycle),
                    5 => Ok(crate::standards::extensions::ItsStationType::Passengercar),
                    6 => Ok(crate::standards::extensions::ItsStationType::Bus),
                    7 => Ok(crate::standards::extensions::ItsStationType::Lighttruck),
                    8 => Ok(crate::standards::extensions::ItsStationType::Heavytruck),
                    9 => Ok(crate::standards::extensions::ItsStationType::Trailer),
                    10 => Ok(crate::standards::extensions::ItsStationType::Specialvehicles),
                    11 => Ok(crate::standards::extensions::ItsStationType::Tram),
                    12 => Ok(crate::standards::extensions::ItsStationType::LightVruVehicle),
                    13 => Ok(crate::standards::extensions::ItsStationType::Animal),
                    15 => Ok(crate::standards::extensions::ItsStationType::Roadsideunit),
                    _ => Err(format!("StationType {} not a known value", self.0)),
                }
            }
        }
    };
}

pub mod its_scc {
    /// Common conversions for manual ETSI Enums
    macro_rules! scc_conv_part {
        ($t:ty, $etsi:ty) => {
            impl $t {
                pub fn as_u8(self) -> u8 {
                    self as u8
                }
            }

            impl From<$t> for $etsi {
                fn from(value: $t) -> Self {
                    Self(value as u8)
                }
            }
        };
    }

    /// SCC 1, ASN.1 `TrafficConditionSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum TrafficCondition {
        Unavailable = 0,
        IncreasedVolumeOfTraffic = 1,
        TrafficJamSlowlyIncreasing = 2,
        TrafficJamIncreasing = 3,
        TrafficJamStronglyIncreasing = 4,
        TrafficStationary = 5,
        TrafficJamSlightlyDecreasing = 6,
        TrafficJamDecreasing = 7,
        TrafficJamStronglyDecreasing = 8,
    }
    scc_conv_part!(
        TrafficCondition,
        crate::standards::cdd_2_2_1::etsi_its_cdd::TrafficConditionSubCauseCode
    );
    impl TryInto<TrafficCondition>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::TrafficConditionSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<TrafficCondition, Self::Error> {
            match self.0 {
                0 => Ok(TrafficCondition::Unavailable),
                1 => Ok(TrafficCondition::IncreasedVolumeOfTraffic),
                2 => Ok(TrafficCondition::TrafficJamSlowlyIncreasing),
                3 => Ok(TrafficCondition::TrafficJamIncreasing),
                4 => Ok(TrafficCondition::TrafficJamStronglyIncreasing),
                5 => Ok(TrafficCondition::TrafficStationary),
                6 => Ok(TrafficCondition::TrafficJamSlightlyDecreasing),
                7 => Ok(TrafficCondition::TrafficJamDecreasing),
                8 => Ok(TrafficCondition::TrafficJamStronglyDecreasing),
                _ => Err(format!(
                    "TrafficConditionSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 2, ASN.1 `AccidentSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum Accident {
        Unavailable = 0,
        MultiVehicleAccident = 1,
        HeavyAccident = 2,
        AccidentInvolvingLorry = 3,
        AccidentInvolvingBus = 4,
        AccidentInvolvingHazardousMaterials = 5,
        AccidentOnOppositeLane = 6,
        UnsecuredAccident = 7,
        AssistanceRequested = 8,
    }
    scc_conv_part!(
        Accident,
        crate::standards::cdd_2_2_1::etsi_its_cdd::AccidentSubCauseCode
    );
    impl TryInto<Accident> for crate::standards::cdd_2_2_1::etsi_its_cdd::AccidentSubCauseCode {
        type Error = String;

        fn try_into(self) -> Result<Accident, Self::Error> {
            match self.0 {
                0 => Ok(Accident::Unavailable),
                1 => Ok(Accident::MultiVehicleAccident),
                2 => Ok(Accident::HeavyAccident),
                3 => Ok(Accident::AccidentInvolvingLorry),
                4 => Ok(Accident::AccidentInvolvingBus),
                5 => Ok(Accident::AccidentInvolvingHazardousMaterials),
                6 => Ok(Accident::AccidentOnOppositeLane),
                7 => Ok(Accident::UnsecuredAccident),
                8 => Ok(Accident::AssistanceRequested),
                _ => Err(format!("AccidentSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 3, ASN.1 `RoadworksSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum Roadworks {
        Unavailable = 0,
        MajorRoadworks = 1,
        RoadMarkingWork = 2,
        SlowMovingRoadMaintenance = 3,
        ShortTermStationaryRoadworks = 4,
        StreetCleaning = 5,
        WinterService = 6,
    }
    scc_conv_part!(
        Roadworks,
        crate::standards::cdd_2_2_1::etsi_its_cdd::RoadworksSubCauseCode
    );
    impl TryInto<Roadworks> for crate::standards::cdd_2_2_1::etsi_its_cdd::RoadworksSubCauseCode {
        type Error = String;

        fn try_into(self) -> Result<Roadworks, Self::Error> {
            match self.0 {
                0 => Ok(Roadworks::Unavailable),
                1 => Ok(Roadworks::MajorRoadworks),
                2 => Ok(Roadworks::RoadMarkingWork),
                3 => Ok(Roadworks::SlowMovingRoadMaintenance),
                4 => Ok(Roadworks::ShortTermStationaryRoadworks),
                5 => Ok(Roadworks::StreetCleaning),
                6 => Ok(Roadworks::WinterService),
                _ => Err(format!(
                    "RoadworksSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 12, ASN.1 `HumanPresenceOnTheRoadSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HumanPresenceOnTheRoad {
        Unavailable = 0,
        ChildrenOnRoadway = 1,
        CyclistOnRoadway = 2,
        MotorcyclistOnRoadway = 3,
    }
    scc_conv_part!(
        HumanPresenceOnTheRoad,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HumanPresenceOnTheRoadSubCauseCode
    );
    impl TryInto<HumanPresenceOnTheRoad>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::HumanPresenceOnTheRoadSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<HumanPresenceOnTheRoad, Self::Error> {
            match self.0 {
                0 => Ok(HumanPresenceOnTheRoad::Unavailable),
                1 => Ok(HumanPresenceOnTheRoad::ChildrenOnRoadway),
                2 => Ok(HumanPresenceOnTheRoad::CyclistOnRoadway),
                3 => Ok(HumanPresenceOnTheRoad::MotorcyclistOnRoadway),
                _ => Err(format!(
                    "HumanPresenceOnTheRoadSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 14, ASN.1 `WrongWayDrivingSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum WrongWayDriving {
        Unavailable = 0,
        WrongLane = 1,
        WrongDirection = 2,
    }
    scc_conv_part!(
        WrongWayDriving,
        crate::standards::cdd_2_2_1::etsi_its_cdd::WrongWayDrivingSubCauseCode
    );
    impl TryInto<WrongWayDriving>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::WrongWayDrivingSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<WrongWayDriving, Self::Error> {
            match self.0 {
                0 => Ok(WrongWayDriving::Unavailable),
                1 => Ok(WrongWayDriving::WrongLane),
                2 => Ok(WrongWayDriving::WrongDirection),
                _ => Err(format!(
                    "WrongWayDrivingSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 17, ASN.1 `AdverseWeatherCondition-ExtremeWeatherConditionSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum AdverseWeatherConditionExtremeWeatherCondition {
        Unavailable = 0,
        StrongWinds = 1,
        DamagingHail = 2,
        Hurricane = 3,
        Thunderstorm = 4,
        Tornado = 5,
        Blizzard = 6,
    }
    scc_conv_part!(
        AdverseWeatherConditionExtremeWeatherCondition,
        crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionExtremeWeatherConditionSubCauseCode
    );
    impl TryInto<AdverseWeatherConditionExtremeWeatherCondition>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionExtremeWeatherConditionSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<AdverseWeatherConditionExtremeWeatherCondition, Self::Error> {
            match self.0 {
                0 => Ok(AdverseWeatherConditionExtremeWeatherCondition::Unavailable),
                1 => Ok(AdverseWeatherConditionExtremeWeatherCondition::StrongWinds),
                2 => Ok(AdverseWeatherConditionExtremeWeatherCondition::DamagingHail),
                3 => Ok(AdverseWeatherConditionExtremeWeatherCondition::Hurricane),
                4 => Ok(AdverseWeatherConditionExtremeWeatherCondition::Thunderstorm),
                5 => Ok(AdverseWeatherConditionExtremeWeatherCondition::Tornado),
                6 => Ok(AdverseWeatherConditionExtremeWeatherCondition::Blizzard),
                _ => Err(format!("ExtremeWeatherConditionSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 6, ASN.1 `AdverseWeatherCondition-AdhesionSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum AdverseWeatherConditionAdhesion {
        Unavailable = 0,
        HeavyFrostOnRoad = 1,
        FuelOnRoad = 2,
        MudOnRoad = 3,
        SnowOnRoad = 4,
        IceOnRoad = 5,
        BlackIceOnRoad = 6,
        OilOnRoad = 7,
        LooseChippings = 8,
        InstantBlackIce = 9,
        RoadsSalted = 10,
    }
    scc_conv_part!(
        AdverseWeatherConditionAdhesion,
        crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionAdhesionSubCauseCode
    );
    impl TryInto<AdverseWeatherConditionAdhesion>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionAdhesionSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<AdverseWeatherConditionAdhesion, Self::Error> {
            match self.0 {
                0 => Ok(AdverseWeatherConditionAdhesion::Unavailable),
                1 => Ok(AdverseWeatherConditionAdhesion::HeavyFrostOnRoad),
                2 => Ok(AdverseWeatherConditionAdhesion::FuelOnRoad),
                3 => Ok(AdverseWeatherConditionAdhesion::MudOnRoad),
                4 => Ok(AdverseWeatherConditionAdhesion::SnowOnRoad),
                5 => Ok(AdverseWeatherConditionAdhesion::IceOnRoad),
                6 => Ok(AdverseWeatherConditionAdhesion::BlackIceOnRoad),
                7 => Ok(AdverseWeatherConditionAdhesion::OilOnRoad),
                8 => Ok(AdverseWeatherConditionAdhesion::LooseChippings),
                9 => Ok(AdverseWeatherConditionAdhesion::InstantBlackIce),
                10 => Ok(AdverseWeatherConditionAdhesion::RoadsSalted),
                _ => Err(format!("AdhesionSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 18, ASN.1 `AdverseWeatherCondition-VisibilitySubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum AdverseWeatherConditionVisibility {
        Unavailable = 0,
        Fog = 1,
        Smoke = 2,
        HeavySnowfall = 3,
        HeavyRain = 4,
        HeavyHail = 5,
        LowSunGlare = 6,
        Sandstorms = 7,
        SwarmsOfInsects = 8,
    }
    scc_conv_part!(
        AdverseWeatherConditionVisibility,
        crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionVisibilitySubCauseCode
    );
    impl TryInto<AdverseWeatherConditionVisibility>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionVisibilitySubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<AdverseWeatherConditionVisibility, Self::Error> {
            match self.0 {
                0 => Ok(AdverseWeatherConditionVisibility::Unavailable),
                1 => Ok(AdverseWeatherConditionVisibility::Fog),
                2 => Ok(AdverseWeatherConditionVisibility::Smoke),
                3 => Ok(AdverseWeatherConditionVisibility::HeavySnowfall),
                4 => Ok(AdverseWeatherConditionVisibility::HeavyRain),
                5 => Ok(AdverseWeatherConditionVisibility::HeavyHail),
                6 => Ok(AdverseWeatherConditionVisibility::LowSunGlare),
                7 => Ok(AdverseWeatherConditionVisibility::Sandstorms),
                8 => Ok(AdverseWeatherConditionVisibility::SwarmsOfInsects),
                _ => Err(format!(
                    "VisibilitySubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 19, ASN.1 `AdverseWeatherCondition-PrecipitationSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum AdverseWeatherConditionPrecipitation {
        Unavailable = 0,
        HeavyRain = 1,
        HeavySnowfall = 2,
        SoftHail = 3,
    }
    scc_conv_part!(
        AdverseWeatherConditionPrecipitation,
        crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionPrecipitationSubCauseCode
    );
    impl TryInto<AdverseWeatherConditionPrecipitation>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::AdverseWeatherConditionPrecipitationSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<AdverseWeatherConditionPrecipitation, Self::Error> {
            match self.0 {
                0 => Ok(AdverseWeatherConditionPrecipitation::Unavailable),
                1 => Ok(AdverseWeatherConditionPrecipitation::HeavyRain),
                2 => Ok(AdverseWeatherConditionPrecipitation::HeavySnowfall),
                3 => Ok(AdverseWeatherConditionPrecipitation::SoftHail),
                _ => Err(format!("PrecipitationSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 26, ASN.1 `SlowVehicleSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum SlowVehicle {
        Unavailable = 0,
        MaintenanceVehicle = 1,
        VehiclesSlowingToLookAtAccident = 2,
        AbnormalLoad = 3,
        AbnormalWideLoad = 4,
        Convoy = 5,
        Snowplough = 6,
        Deicing = 7,
        SaltingVehicles = 8,
    }
    scc_conv_part!(
        SlowVehicle,
        crate::standards::cdd_2_2_1::etsi_its_cdd::SlowVehicleSubCauseCode
    );
    impl TryInto<SlowVehicle> for crate::standards::cdd_2_2_1::etsi_its_cdd::SlowVehicleSubCauseCode {
        type Error = String;

        fn try_into(self) -> Result<SlowVehicle, Self::Error> {
            match self.0 {
                0 => Ok(SlowVehicle::Unavailable),
                1 => Ok(SlowVehicle::MaintenanceVehicle),
                2 => Ok(SlowVehicle::VehiclesSlowingToLookAtAccident),
                3 => Ok(SlowVehicle::AbnormalLoad),
                4 => Ok(SlowVehicle::AbnormalWideLoad),
                5 => Ok(SlowVehicle::Convoy),
                6 => Ok(SlowVehicle::Snowplough),
                7 => Ok(SlowVehicle::Deicing),
                8 => Ok(SlowVehicle::SaltingVehicles),
                _ => Err(format!(
                    "SlowVehicleSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 94, ASN.1 `StationaryVehicleSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum StationaryVehicle {
        Unavailable = 0,
        HumanProblem = 1,
        VehicleBreakdown = 2,
        PostCrash = 3,
        PublicTransportStop = 4,
        CarryingDangerousGoods = 5,
    }
    scc_conv_part!(
        StationaryVehicle,
        crate::standards::cdd_2_2_1::etsi_its_cdd::StationaryVehicleSubCauseCode
    );
    impl TryInto<StationaryVehicle>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::StationaryVehicleSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<StationaryVehicle, Self::Error> {
            match self.0 {
                0 => Ok(StationaryVehicle::Unavailable),
                1 => Ok(StationaryVehicle::HumanProblem),
                2 => Ok(StationaryVehicle::VehicleBreakdown),
                3 => Ok(StationaryVehicle::PostCrash),
                4 => Ok(StationaryVehicle::PublicTransportStop),
                5 => Ok(StationaryVehicle::CarryingDangerousGoods),
                _ => Err(format!(
                    "StationaryVehicleSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 93, ASN.1 `HumanProblemSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HumanProblem {
        Unavailable = 0,
        GlycemiaProblem = 1,
        HeartProblem = 2,
    }
    scc_conv_part!(
        HumanProblem,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HumanProblemSubCauseCode
    );
    impl TryInto<HumanProblem> for crate::standards::cdd_2_2_1::etsi_its_cdd::HumanProblemSubCauseCode {
        type Error = String;

        fn try_into(self) -> Result<HumanProblem, Self::Error> {
            match self.0 {
                0 => Ok(HumanProblem::Unavailable),
                1 => Ok(HumanProblem::GlycemiaProblem),
                2 => Ok(HumanProblem::HeartProblem),
                _ => Err(format!(
                    "HumanProblemSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 95, ASN.1 `EmergencyVehicleApproachingSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum EmergencyVehicleApproaching {
        Unavailable = 0,
        EmergencyVehicleApproaching = 1,
        PrioritizedVehicleApproaching = 2,
    }
    scc_conv_part!(
        EmergencyVehicleApproaching,
        crate::standards::cdd_2_2_1::etsi_its_cdd::EmergencyVehicleApproachingSubCauseCode
    );
    impl TryInto<EmergencyVehicleApproaching>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::EmergencyVehicleApproachingSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<EmergencyVehicleApproaching, Self::Error> {
            match self.0 {
                0 => Ok(EmergencyVehicleApproaching::Unavailable),
                1 => Ok(EmergencyVehicleApproaching::EmergencyVehicleApproaching),
                2 => Ok(EmergencyVehicleApproaching::PrioritizedVehicleApproaching),
                _ => Err(format!(
                    "EmergencyVehicleApproachingSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 96, ASN.1 `HazardousLocation-DangerousCurveSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HazardousLocationDangerousCurve {
        Unavailable = 0,
        DangerousLeftTurnCurve = 1,
        DangerousRightTurnCurve = 2,
        MultipleCurvesStartingWithUnknownTurningDirection = 3,
        MultipleCurvesStartingWithLeftTurn = 4,
        MultipleCurvesStartingWithRightTurn = 5,
    }
    scc_conv_part!(
        HazardousLocationDangerousCurve,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationDangerousCurveSubCauseCode
    );
    impl TryInto<HazardousLocationDangerousCurve>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationDangerousCurveSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<HazardousLocationDangerousCurve, Self::Error> {
            match self.0 {
                0 => Ok(HazardousLocationDangerousCurve::Unavailable),
                1 => Ok(HazardousLocationDangerousCurve::DangerousLeftTurnCurve),
                2 => Ok(HazardousLocationDangerousCurve::DangerousRightTurnCurve),
                3 => Ok(HazardousLocationDangerousCurve::MultipleCurvesStartingWithUnknownTurningDirection),
                4 => Ok(HazardousLocationDangerousCurve::MultipleCurvesStartingWithLeftTurn),
                5 => Ok(HazardousLocationDangerousCurve::MultipleCurvesStartingWithRightTurn),
                _ => Err(format!("DangerousCurveSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 9, ASN.1 `HazardousLocation-SurfaceConditionSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HazardousLocationSurfaceCondition {
        Unavailable = 0,
        Rockfalls = 1,
        EarthquakeDamage = 2,
        SewerCollapse = 3,
        Subsidence = 4,
        SnowDrifts = 5,
        StormDamage = 6,
        BurstPipe = 7,
        VolcanoEruption = 8,
        FallingIce = 9,
    }
    scc_conv_part!(
        HazardousLocationSurfaceCondition,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationSurfaceConditionSubCauseCode
    );
    impl TryInto<HazardousLocationSurfaceCondition>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationSurfaceConditionSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<HazardousLocationSurfaceCondition, Self::Error> {
            match self.0 {
                0 => Ok(HazardousLocationSurfaceCondition::Unavailable),
                1 => Ok(HazardousLocationSurfaceCondition::Rockfalls),
                2 => Ok(HazardousLocationSurfaceCondition::EarthquakeDamage),
                3 => Ok(HazardousLocationSurfaceCondition::SewerCollapse),
                4 => Ok(HazardousLocationSurfaceCondition::Subsidence),
                5 => Ok(HazardousLocationSurfaceCondition::SnowDrifts),
                6 => Ok(HazardousLocationSurfaceCondition::StormDamage),
                7 => Ok(HazardousLocationSurfaceCondition::BurstPipe),
                8 => Ok(HazardousLocationSurfaceCondition::VolcanoEruption),
                9 => Ok(HazardousLocationSurfaceCondition::FallingIce),
                _ => Err(format!(
                    "SurfaceConditionSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 10, ASN.1 `HazardousLocation-ObstacleOnTheRoadSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HazardousLocationObstacleOnTheRoad {
        Unavailable = 0,
        ShedLoad = 1,
        PartsOfVehicles = 2,
        PartsOfTyres = 3,
        BigObjects = 4,
        FallenTrees = 5,
        HubCaps = 6,
        WaitingVehicles = 7,
    }
    scc_conv_part!(
        HazardousLocationObstacleOnTheRoad,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationObstacleOnTheRoadSubCauseCode
    );
    impl TryInto<HazardousLocationObstacleOnTheRoad>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationObstacleOnTheRoadSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<HazardousLocationObstacleOnTheRoad, Self::Error> {
            match self.0 {
                0 => Ok(HazardousLocationObstacleOnTheRoad::Unavailable),
                1 => Ok(HazardousLocationObstacleOnTheRoad::ShedLoad),
                2 => Ok(HazardousLocationObstacleOnTheRoad::PartsOfVehicles),
                3 => Ok(HazardousLocationObstacleOnTheRoad::PartsOfTyres),
                4 => Ok(HazardousLocationObstacleOnTheRoad::BigObjects),
                5 => Ok(HazardousLocationObstacleOnTheRoad::FallenTrees),
                6 => Ok(HazardousLocationObstacleOnTheRoad::HubCaps),
                7 => Ok(HazardousLocationObstacleOnTheRoad::WaitingVehicles),
                _ => Err(format!("ObstacleOnTheRoadSubCauseCode {} not a known value", self.0)),
            }
        }
    }

    /// SCC 11, ASN.1 `HazardousLocation-AnimalOnTheRoadSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum HazardousLocationAnimalOnTheRoad {
        Unavailable = 0,
        WildAnimals = 1,
        HerdOfAnimals = 2,
        SmallAnimals = 3,
        LargeAnimals = 4,
    }
    scc_conv_part!(
        HazardousLocationAnimalOnTheRoad,
        crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationAnimalOnTheRoadSubCauseCode
    );
    impl TryInto<HazardousLocationAnimalOnTheRoad>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::HazardousLocationAnimalOnTheRoadSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<HazardousLocationAnimalOnTheRoad, Self::Error> {
            match self.0 {
                0 => Ok(HazardousLocationAnimalOnTheRoad::Unavailable),
                1 => Ok(HazardousLocationAnimalOnTheRoad::WildAnimals),
                2 => Ok(HazardousLocationAnimalOnTheRoad::HerdOfAnimals),
                3 => Ok(HazardousLocationAnimalOnTheRoad::SmallAnimals),
                4 => Ok(HazardousLocationAnimalOnTheRoad::LargeAnimals),
                _ => Err(format!(
                    "AnimalOnTheRoadSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 97, ASN.1 `CollisionRiskSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum CollisionRisk {
        Unavailable = 0,
        LongitudinalCollisionRisk = 1,
        CrossingCollisionRisk = 2,
        LateralCollisionRisk = 3,
        VulnerableRoadUser = 4,
    }
    scc_conv_part!(
        CollisionRisk,
        crate::standards::cdd_2_2_1::etsi_its_cdd::CollisionRiskSubCauseCode
    );
    impl TryInto<CollisionRisk>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::CollisionRiskSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<CollisionRisk, Self::Error> {
            match self.0 {
                0 => Ok(CollisionRisk::Unavailable),
                1 => Ok(CollisionRisk::LongitudinalCollisionRisk),
                2 => Ok(CollisionRisk::CrossingCollisionRisk),
                3 => Ok(CollisionRisk::LateralCollisionRisk),
                4 => Ok(CollisionRisk::VulnerableRoadUser),
                _ => Err(format!(
                    "CollisionRiskSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 98, ASN.1 `SignalViolationSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum SignalViolation {
        Unavailable = 0,
        StopSignViolation = 1,
        TrafficLightViolation = 2,
        TurningRegulationViolation = 3,
    }
    scc_conv_part!(
        SignalViolation,
        crate::standards::cdd_2_2_1::etsi_its_cdd::SignalViolationSubCauseCode
    );
    impl TryInto<SignalViolation>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::SignalViolationSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<SignalViolation, Self::Error> {
            match self.0 {
                0 => Ok(SignalViolation::Unavailable),
                1 => Ok(SignalViolation::StopSignViolation),
                2 => Ok(SignalViolation::TrafficLightViolation),
                3 => Ok(SignalViolation::TurningRegulationViolation),
                _ => Err(format!(
                    "SignalViolationSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 15, ASN.1 `RescueAndRecoveryWorkInProgressSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum RescueAndRecoveryWorkInProgress {
        Unavailable = 0,
        EmergencyVehicles = 1,
        RescueHelicopterLanding = 2,
        PoliceActivityOngoing = 3,
        MedicalEmergencyOngoing = 4,
        ChildAbductionInProgress = 5,
    }
    scc_conv_part!(
        RescueAndRecoveryWorkInProgress,
        crate::standards::cdd_2_2_1::etsi_its_cdd::RescueAndRecoveryWorkInProgressSubCauseCode
    );
    impl TryInto<RescueAndRecoveryWorkInProgress>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::RescueAndRecoveryWorkInProgressSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<RescueAndRecoveryWorkInProgress, Self::Error> {
            match self.0 {
                0 => Ok(RescueAndRecoveryWorkInProgress::Unavailable),
                1 => Ok(RescueAndRecoveryWorkInProgress::EmergencyVehicles),
                2 => Ok(RescueAndRecoveryWorkInProgress::RescueHelicopterLanding),
                3 => Ok(RescueAndRecoveryWorkInProgress::PoliceActivityOngoing),
                4 => Ok(RescueAndRecoveryWorkInProgress::MedicalEmergencyOngoing),
                5 => Ok(RescueAndRecoveryWorkInProgress::ChildAbductionInProgress),
                _ => Err(format!(
                    "RescueAndRecoveryWorkInProgressSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 27, ASN.1 `DangerousEndOfQueueSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum DangerousEndOfQueue {
        Unavailable = 0,
        SuddenEndOfQueue = 1,
        QueueOverHill = 2,
        QueueAroundBend = 3,
        QueueInTunnel = 4,
    }
    scc_conv_part!(
        DangerousEndOfQueue,
        crate::standards::cdd_2_2_1::etsi_its_cdd::DangerousEndOfQueueSubCauseCode
    );
    impl TryInto<DangerousEndOfQueue>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::DangerousEndOfQueueSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<DangerousEndOfQueue, Self::Error> {
            match self.0 {
                0 => Ok(DangerousEndOfQueue::Unavailable),
                1 => Ok(DangerousEndOfQueue::SuddenEndOfQueue),
                2 => Ok(DangerousEndOfQueue::QueueOverHill),
                3 => Ok(DangerousEndOfQueue::QueueAroundBend),
                4 => Ok(DangerousEndOfQueue::QueueInTunnel),
                _ => Err(format!(
                    "DangerousEndOfQueueSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 99, ASN.1 `DangerousSituationSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum DangerousSituation {
        Unavailable = 0,
        EmergencyElectronicBrakeEngaged = 1,
        PreCrashSystemEngaged = 2,
        EspEngaged = 3,
        AbsEngaged = 4,
        AebEngaged = 5,
        BrakeWarningEngaged = 6,
        CollisionRiskWarningEngaged = 7,
    }
    scc_conv_part!(
        DangerousSituation,
        crate::standards::cdd_2_2_1::etsi_its_cdd::DangerousSituationSubCauseCode
    );
    impl TryInto<DangerousSituation>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::DangerousSituationSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<DangerousSituation, Self::Error> {
            match self.0 {
                0 => Ok(DangerousSituation::Unavailable),
                1 => Ok(DangerousSituation::EmergencyElectronicBrakeEngaged),
                2 => Ok(DangerousSituation::PreCrashSystemEngaged),
                3 => Ok(DangerousSituation::EspEngaged),
                4 => Ok(DangerousSituation::AbsEngaged),
                5 => Ok(DangerousSituation::AebEngaged),
                6 => Ok(DangerousSituation::BrakeWarningEngaged),
                7 => Ok(DangerousSituation::CollisionRiskWarningEngaged),
                _ => Err(format!(
                    "DangerousSituationSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 91, ASN.1 `VehicleBreakdownSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum VehicleBreakdown {
        Unavailable = 0,
        LackOfFuel = 1,
        LackOfBatteryPower = 2,
        EngineProblem = 3,
        TransmissionProblem = 4,
        EngineCoolingProblem = 5,
        BrakingSystemProblem = 6,
        SteeringProblem = 7,
        TyrePuncture = 8,
        TyrePressureProblem = 9,
    }
    scc_conv_part!(
        VehicleBreakdown,
        crate::standards::cdd_2_2_1::etsi_its_cdd::VehicleBreakdownSubCauseCode
    );
    impl TryInto<VehicleBreakdown>
        for crate::standards::cdd_2_2_1::etsi_its_cdd::VehicleBreakdownSubCauseCode
    {
        type Error = String;

        fn try_into(self) -> Result<VehicleBreakdown, Self::Error> {
            match self.0 {
                0 => Ok(VehicleBreakdown::Unavailable),
                1 => Ok(VehicleBreakdown::LackOfFuel),
                2 => Ok(VehicleBreakdown::LackOfBatteryPower),
                3 => Ok(VehicleBreakdown::EngineProblem),
                4 => Ok(VehicleBreakdown::TransmissionProblem),
                5 => Ok(VehicleBreakdown::EngineCoolingProblem),
                6 => Ok(VehicleBreakdown::BrakingSystemProblem),
                7 => Ok(VehicleBreakdown::SteeringProblem),
                8 => Ok(VehicleBreakdown::TyrePuncture),
                9 => Ok(VehicleBreakdown::TyrePressureProblem),
                _ => Err(format!(
                    "VehicleBreakdownSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }

    /// SCC 92, ASN.1 `PostCrashSubCauseCode`
    #[repr(u8)]
    #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
    pub enum PostCrash {
        Unavailable = 0,
        AccidentWithoutECallTriggered = 1,
        AccidentWithECallManuallyTriggered = 2,
        AccidentWithECallAutomaticallyTriggered = 3,
        AccidentWithECallTriggeredWithoutAccessToCellularNetwork = 4,
    }
    scc_conv_part!(
        PostCrash,
        crate::standards::cdd_2_2_1::etsi_its_cdd::PostCrashSubCauseCode
    );
    impl TryInto<PostCrash> for crate::standards::cdd_2_2_1::etsi_its_cdd::PostCrashSubCauseCode {
        type Error = String;

        fn try_into(self) -> Result<PostCrash, Self::Error> {
            match self.0 {
                0 => Ok(PostCrash::Unavailable),
                1 => Ok(PostCrash::AccidentWithoutECallTriggered),
                2 => Ok(PostCrash::AccidentWithECallManuallyTriggered),
                3 => Ok(PostCrash::AccidentWithECallAutomaticallyTriggered),
                4 => Ok(PostCrash::AccidentWithECallTriggeredWithoutAccessToCellularNetwork),
                _ => Err(format!(
                    "PostCrashSubCauseCode {} not a known value",
                    self.0
                )),
            }
        }
    }
}

#[cfg(feature = "_cdd_1_3_1_1")]
pub mod cdd_1_3_1_1 {
    use crate::standards::cdd_1_3_1_1::its_container::{
        AccelerationControl, EmergencyPriority, ExteriorLights, LightBarSirenInUse,
        SpecialTransportType,
    };

    // used in CAM 1.4.1 via BasicVehicleContainerHighFrequency
    impl Default for AccelerationControl {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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
    impl Default for ExteriorLights {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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
    impl Default for EmergencyPriority {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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
    impl Default for LightBarSirenInUse {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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
    impl Default for SpecialTransportType {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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

    itsstationtype_conv!(crate::standards::cdd_1_3_1_1::its_container::StationType);
}

#[cfg(feature = "_cdd_2_2_1")]
pub mod cdd_2_2_1 {
    use crate::standards::cdd_2_2_1::etsi_its_cdd::EnergyStorageType;

    impl Default for EnergyStorageType {
        fn default() -> Self {
            Self(Default::default())
        }
    }
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

    itsmessageid_conv!(crate::standards::cdd_2_2_1::etsi_its_cdd::MessageId);

    itsstationtype_conv!(crate::standards::cdd_2_2_1::etsi_its_cdd::StationType);
}

#[cfg(feature = "_dsrc_2_2_1")]
mod dsrc_2_2_1 {
    use rasn::types::Ia5String;

    use crate::standards::dsrc_2_2_1::etsi_its_dsrc::{
        AllowedManeuvers, IntersectionStatusObject, LaneAttributes, LaneAttributesBarrier,
        LaneAttributesBike, LaneAttributesCrosswalk, LaneAttributesParking, LaneAttributesSidewalk,
        LaneAttributesStriping, LaneAttributesTrackedVehicle, LaneAttributesVehicle, LaneDirection,
        LaneSharing, LaneTypeAttributes, OcitRequestorDescriptionContainer, TransitVehicleStatus,
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

    impl Default for LaneSharing {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for AllowedManeuvers {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for IntersectionStatusObject {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesBarrier {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesBike {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesCrosswalk {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesParking {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesSidewalk {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesStriping {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesTrackedVehicle {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneAttributesVehicle {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for LaneDirection {
        fn default() -> Self {
            Self(Default::default())
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

    impl Default for TransitVehicleStatus {
        fn default() -> Self {
            Self(Default::default())
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

    impl TryFrom<String> for crate::standards::dsrc_2_2_1::etsi_its_dsrc::DescriptiveName {
        type Error = String;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Ok(Self(
                Ia5String::from_iso646_bytes(value.as_bytes())
                    .map_err(|err| format!("Failed to create DescriptiveName: {err}"))?,
            ))
        }
    }

    impl Default for OcitRequestorDescriptionContainer {
        fn default() -> Self {
            Self::new(None, None, None, None, None, None, None, None)
        }
    }
}

#[cfg(feature = "ivim_2_2_1")]
mod ivim_2_2_1 {
    use crate::standards::ivim_2_2_1::gdd::DayOfWeek;

    impl Default for DayOfWeek {
        fn default() -> Self {
            Self(Default::default())
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

    mod gdd {
        use crate::standards::ivim_2_2_1::gdd::RepeatingPeriodDayTypes;

        impl Default for RepeatingPeriodDayTypes {
            fn default() -> Self {
                Self(Default::default())
            }
        }
        impl RepeatingPeriodDayTypes {
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
        impl std::fmt::Display for RepeatingPeriodDayTypes {
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
    }
}
