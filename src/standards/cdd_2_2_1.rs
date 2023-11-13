#![allow(unused, non_upper_case_globals, non_snake_case)]
extern crate alloc;

use rasn::prelude::*;
// =====================================================
// ETSI-ITS-CDD
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) 102894 cdd(2) major-version-4(4) minor-version-1(1) }
// =====================================================

///
///
///
///
///
///
///
///
///
///
/// Specification of CDD Data Frames:
///
///
///
///
///
///
///
///
///
///
///*
/// * This DF represents an acceleration vector with associated confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field polarAcceleration: the representation of the acceleration vector in a polar or cylindrical coordinate system.
/// *
/// * @field cartesianAcceleration: the representation of the acceleration vector in a cartesian coordinate system.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Acceleration3dWithConfidence {
    PolarAcceleration(AccelerationPolarWithZ),
    CartesianAcceleration(AccelerationCartesian),
}

///*
/// * This DF represents a acceleration vector in a cartesian coordinate system.
///
/// * It shall include the following components:
/// *
/// * @field xAcceleration: the x component of the acceleration vector with the associated confidence value.
/// *
/// * @field yAcceleration: the y component of the acceleration vector with the associated confidence value.
/// *
/// * @field zAcceleration: the optional z component of the acceleration vector with the associated confidence value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AccelerationCartesian {
    pub x_acceleration: AccelerationComponent,
    pub y_acceleration: AccelerationComponent,
    pub z_acceleration: Option<AccelerationComponent>,
}

impl AccelerationCartesian {
    pub fn new(
        x_acceleration: AccelerationComponent,
        y_acceleration: AccelerationComponent,
        z_acceleration: Option<AccelerationComponent>,
    ) -> Self {
        Self {
            x_acceleration,
            y_acceleration,
            z_acceleration,
        }
    }
}

///
///
///
///
///
///
///
///
///
///
///
/// Specification of CDD Data Elements:
///
///
///
///
///
///
///
///
///
///
///
///*
/// * This DE indicates a change of acceleration.
/// *
/// * The value shall be set to:
/// * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.
/// * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AccelerationChange {
    Accelerate = 0,
    Decelerate = 1,
}

///*
/// * This DF represents information associated to changes in acceleration.
/// *
/// * It shall include the following components:
/// *
/// * @field accelOrDecel: the indication of an acceleration change.
/// *
/// * @field actionDeltaTime: the period over which the acceleration change action is performed.
/// *
/// * @category: Kinematic Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AccelerationChangeIndication {
    pub accel_or_decel: AccelerationChange,
    pub action_delta_time: DeltaTimeTenthOfSecond,
}

impl AccelerationChangeIndication {
    pub fn new(
        accel_or_decel: AccelerationChange,
        action_delta_time: DeltaTimeTenthOfSecond,
    ) -> Self {
        Self {
            accel_or_decel,
            action_delta_time,
        }
    }
}

///*
/// * This DF represents an acceleration component along with a confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field value: the value of the acceleration component which can be estimated as the mean of the current distribution.
/// *
/// * @field confidence: the confidence value associated to the provided value.
/// *
/// * @category: Kinematic Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AccelerationComponent {
    pub value: AccelerationValue,
    pub confidence: AccelerationConfidence,
}

impl AccelerationComponent {
    pub fn new(value: AccelerationValue, confidence: AccelerationConfidence) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy of an acceleration value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 101`) if the confidence value is equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `101` if the confidence value is out of range i.e. greater than 10 m/s^2,
/// * - `102` if the confidence value is unavailable.
/// *
/// * The value 0 shall not be used.
/// *
/// * @note: The fact that an acceleration value is received with confidence value set to `unavailable(102)` can be caused by several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the acceleration value may be valid and used by the application.
/// *
/// * @note: If an acceleration value is received and its confidence value is set to `outOfRange(101)`, it means that the value is not valid and therefore cannot be trusted. Such value is not useful for the application.
/// *
/// * @unit 0,1 m/s^2
/// * @category: Kinematic information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=102"))]
pub struct AccelerationConfidence(pub u8);

///*
/// * This DE indicates the current controlling mechanism for longitudinal movement of the vehicle.
/// * The data may be provided via the in-vehicle network. It indicates whether a specific in-vehicle
/// * acceleration control system is engaged or not. Currently, this DE includes the information of the
/// * vehicle brake pedal, gas pedal, emergency brake system, collision warning system, adaptive cruise
/// * control system, cruise control system and speed limiter system.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 - `brakePedalEngaged`      - Driver is stepping on the brake pedal,
/// * - 1 - `gasPedalEngaged`        - Driver is stepping on the gas pedal,
/// * - 2 - `emergencyBrakeEngaged`  - emergency brake system is engaged,
/// * - 3 - `collisionWarningEngaged`- collision warning system is engaged,
/// * - 4 - `accEngaged`             - ACC is engaged,
/// * - 5 - `cruiseControlEngaged`   - cruise control is engaged,
/// * - 6 - `speedLimiterEngaged`    - speed limiter is engaged.
/// *
/// * Otherwise (for example when the corresponding system is not available due to non equipped system
/// * or information is unavailable), the corresponding bit shall be set to 0.
/// *
/// * @note: The system engagement condition is OEM specific and therefore out of scope of the present document.
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("7"))]
pub struct AccelerationControl(pub BitString);

///*
/// * This DF represents the magnitude of the acceleration vector and associated confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field accelerationMagnitudeValue: the magnitude of the acceleration vector.
/// *
/// * @field accelerationConfidence: the confidence value of the magnitude value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AccelerationMagnitude {
    pub acceleration_magnitude_value: AccelerationMagnitudeValue,
    pub acceleration_confidence: AccelerationConfidence,
}

impl AccelerationMagnitude {
    pub fn new(
        acceleration_magnitude_value: AccelerationMagnitudeValue,
        acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            acceleration_magnitude_value,
            acceleration_confidence,
        }
    }
}

///*
/// * This DE represents the magnitude of the acceleration vector in a defined coordinate system.
/// *
/// * The value shall be set to:
/// * - `0` to indicate no acceleration,
/// * - `n` (`n > 0` and `n < 160`) to indicate acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `160` for acceleration values greater than 15,9 m/s^2,
/// * - `161` when the data is unavailable.
/// *
/// * @unit 0,1 m/s^2
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=161"))]
pub struct AccelerationMagnitudeValue(pub u8);

///*
/// * This DF represents an acceleration vector in a polar or cylindrical coordinate system.
///
/// * It shall include the following components:
/// *
/// * @field accelerationMagnitude: magnitude of the acceleration vector projected onto the reference plane, with the associated confidence value.
/// *
/// * @field accelerationDirection: polar angle of the acceleration vector projected onto the reference plane, with the associated confidence value.
/// *
/// * @field zAcceleration: the optional z component of the acceleration vector along the reference axis of the cylindrical coordinate system, with the associated confidence value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AccelerationPolarWithZ {
    pub acceleration_magnitude: AccelerationMagnitude,
    pub acceleration_direction: CartesianAngle,
    pub z_acceleration: Option<AccelerationComponent>,
}

impl AccelerationPolarWithZ {
    pub fn new(
        acceleration_magnitude: AccelerationMagnitude,
        acceleration_direction: CartesianAngle,
        z_acceleration: Option<AccelerationComponent>,
    ) -> Self {
        Self {
            acceleration_magnitude,
            acceleration_direction,
            z_acceleration,
        }
    }
}

///*
/// * This DE represents the value of an acceleration component in a defined coordinate system.
/// *
/// * The value shall be set to:
/// * - `-160` for acceleration values equal to or less than -16 m/s^2,
/// * - `n` (`n > -160` and `n <= 0`) to indicate negative acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `n` (`n > 0` and `n < 160`) to indicate positive acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `160` for acceleration values greater than 15,9 m/s^2,
/// * - `161` when the data is unavailable.
/// *
/// * @note: the formula for values > -160 and <160 results in rounding up to the next value. Zero acceleration is indicated using n=0.
/// * @unit 0,1 m/s^2
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct AccelerationValue(pub i16);

///*
/// * This DE indicates an access technology.
/// *
/// * The value shall be set to:
/// * - `0`: in case of any access technology class,
/// * - `1`: in case of ITS-G5 access technology class,
/// * - `2`: in case of LTE-V2X access technology class,
/// * - `3`: in case of NR-V2X access technology class.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum AccessTechnologyClass {
    Any = 0,
    Itsg5Class = 1,
    Ltev2xClass = 2,
    Nrv2xClass = 3,
}

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `accident`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                        - in case the information on the sub cause of the accident is unavailable,
/// * - 1 - `multiVehicleAccident`               - in case more than two vehicles are involved in accident,
/// * - 2 - `heavyAccident`                      - in case the airbag of the vehicle involved in the accident is triggered,
/// *                                              the accident requires important rescue and/or recovery work,
/// * - 3 - `accidentInvolvingLorry`             - in case the accident involves a lorry,
/// * - 4 - `accidentInvolvingBus`               - in case the accident involves a bus,
/// * - 5 - `accidentInvolvingHazardousMaterials`- in case the accident involves hazardous material,
/// * - 6 - `accidentOnOppositeLane`             - in case the accident happens on opposite lanes,
/// * - 7 - `unsecuredAccident`                  - in case the accident is not secured,
/// * - 8 - `assistanceRequested`                - in case rescue and assistance are requested,
/// * - 9-255                                    - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AccidentSubCauseCode(pub u8);

///*
/// * This DF represents an identifier used to describe a protocol action taken by an ITS-S.
/// *
/// * It shall include the following components:
/// *
/// * @field originatingStationId: Id of the ITS-S that takes the action.
/// *
/// * @field sequenceNumber: a sequence number.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref ActionId instead.
/// * @category: Communication information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ActionID {
    pub originating_station_id: StationID,
    pub sequence_number: SequenceNumber,
}

impl ActionID {
    pub fn new(originating_station_id: StationID, sequence_number: SequenceNumber) -> Self {
        Self {
            originating_station_id,
            sequence_number,
        }
    }
}

///*
/// * This DF represents an identifier used to describe a protocol action taken by an ITS-S.
/// *
/// * It shall include the following components:
/// *
/// * @field originatingStationId: Id of the ITS-S that takes the action.
/// *
/// * @field sequenceNumber: a sequence number.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1 based on @ref ActionID.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ActionId {
    pub originating_station_id: StationId,
    pub sequence_number: SequenceNumber,
}

impl ActionId {
    pub fn new(originating_station_id: StationId, sequence_number: SequenceNumber) -> Self {
        Self {
            originating_station_id,
            sequence_number,
        }
    }
}

///*
/// * This DF shall contain a list of @ref ActionId.
///
/// * @category: Communication Information
/// * @revision: Created in V2.1.1 based on ReferenceDenms from DENM Release 1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct ActionIdList(pub SequenceOf<ActionId>);

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `adverseWeatherCondition-Adhesion`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`     - in case information on the cause of the low road adhesion is unavailable,
/// * - 1 - `heavyFrostOnRoad`- in case the low road adhesion is due to heavy frost on the road,
/// * - 2 - `fuelOnRoad`      - in case the low road adhesion is due to fuel on the road,
/// * - 3 - `mudOnRoad`       - in case the low road adhesion is due to mud on the road,
/// * - 4 - `snowOnRoad`      - in case the low road adhesion is due to snow on the road,
/// * - 5 - `iceOnRoad`       - in case the low road adhesion is due to ice on the road,
/// * - 6 - `blackIceOnRoad`  - in case the low road adhesion is due to black ice on the road,
/// * - 7 - `oilOnRoad`       - in case the low road adhesion is due to oil on the road,
/// * - 8 - `looseChippings`  - in case the low road adhesion is due to loose gravel or stone fragments detached from a road surface or from a hazard,
/// * - 9 - `instantBlackIce` - in case the low road adhesion is due to instant black ice on the road surface,
/// * - 10 - `roadsSalted`    - when the low road adhesion is due to salted road,
/// * - 11-255                - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-ExtremeWeatherCondition`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable` - in case information on the type of extreme weather condition is unavailable,
/// * - 1 - `strongWinds` - in case the type of extreme weather condition is strong wind,
/// * - 2 - `damagingHail`- in case the type of extreme weather condition is damaging hail,
/// * - 3 - `hurricane`   - in case the type of extreme weather condition is hurricane,
/// * - 4 - `thunderstorm`- in case the type of extreme weather condition is thunderstorm,
/// * - 5 - `tornado`     - in case the type of extreme weather condition is tornado,
/// * - 6 - `blizzard`    - in case the type of extreme weather condition is blizzard.
/// * - 7-255             - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Precipitation`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`   - in case information on the type of precipitation is unavailable,
/// * - 1 - `heavyRain`     - in case the type of precipitation is heavy rain,
/// * - 2 - `heavySnowfall` - in case the type of precipitation is heavy snow fall,
/// * - 3 - `softHail`      - in case the type of precipitation is soft hail.
/// * - 4-255               - are reserved for future usage
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Visibility`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`    - in case information on the cause of low visibility is unavailable,
/// * - 1 - `fog`            - in case the cause of low visibility is fog,
/// * - 2 - `smoke`          - in case the cause of low visibility is smoke,
/// * - 3 - `heavySnowfall`  - in case the cause of low visibility is heavy snow fall,
/// * - 4 - `heavyRain`      - in case the cause of low visibility is heavy rain,
/// * - 5 - `heavyHail`      - in case the cause of low visibility is heavy hail,
/// * - 6 - `lowSunGlare`    - in case the cause of low visibility is sun glare,
/// * - 7 - `sandstorms`     - in case the cause of low visibility is sand storm,
/// * - 8 - `swarmsOfInsects`- in case the cause of low visibility is swarm of insects.
/// * - 9-255                - are reserved for future usage
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);

///*
/// * This DE represents the air humidity in tenths of percent.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 1001`) indicates that the applicable value is equal to or less than n x 0,1 percent and greater than (n-1) x 0,1 percent.
/// * - `1001` indicates that the air humidity is unavailable.
/// *
/// * @category: Basic information
/// * @unit: 0,1 %
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=1001"))]
pub struct AirHumidity(pub u16);

///*
/// * This DF provides the altitude and confidence level of an altitude information in a WGS84 coordinate system.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * It shall include the following components:
/// *
/// * @field altitudeValue: altitude of a geographical point.
/// *
/// * @field altitudeConfidence: confidence level of the altitudeValue.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref AltitudeWithConfidence instead.
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Altitude {
    pub altitude_value: AltitudeValue,
    pub altitude_confidence: AltitudeConfidence,
}

impl Altitude {
    pub fn new(altitude_value: AltitudeValue, altitude_confidence: AltitudeConfidence) -> Self {
        Self {
            altitude_value,
            altitude_confidence,
        }
    }
}

///*
/// * This DE indicates the altitude confidence value which represents the estimated absolute accuracy of an altitude value of a geographical point with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// *   - 0  - `alt-000-01`   - if the confidence value is equal to or less than 0,01 metre,
/// *   - 1  - `alt-000-02`   - if the confidence value is equal to or less than 0,02 metre and greater than 0,01 metre,
/// *   - 2  - `alt-000-05`   - if the confidence value is equal to or less than 0,05 metre and greater than 0,02 metre,            
/// *   - 3  - `alt-000-10`   - if the confidence value is equal to or less than 0,1 metre and greater than 0,05 metre,            
/// *   - 4  - `alt-000-20`   - if the confidence value is equal to or less than 0,2 metre and greater than 0,1 metre,            
/// *   - 5  - `alt-000-50`   - if the confidence value is equal to or less than 0,5 metre and greater than 0,2 metre,             
/// *   - 6  - `alt-001-00`   - if the confidence value is equal to or less than 1 metre and greater than 0,5 metre,             
/// *   - 7  - `alt-002-00`   - if the confidence value is equal to or less than 2 metres and greater than 1 metre,             
/// *   - 8  - `alt-005-00`   - if the confidence value is equal to or less than 5 metres and greater than 2 metres,              
/// *   - 9  - `alt-010-00`   - if the confidence value is equal to or less than 10 metres and greater than 5 metres,             
/// *   - 10 - `alt-020-00`   - if the confidence value is equal to or less than 20 metres and greater than 10 metres,            
/// *   - 11 - `alt-050-00`   - if the confidence value is equal to or less than 50 metres and greater than 20 metres,            
/// *   - 12 - `alt-100-00`   - if the confidence value is equal to or less than 100 metres and greater than 50 metres,           
/// *   - 13 - `alt-200-00`   - if the confidence value is equal to or less than 200 metres and greater than 100 metres,           
/// *   - 14 - `outOfRange`   - if the confidence value is out of range, i.e. greater than 200 metres,
/// *   - 15 - `unavailable`  - if the confidence value is unavailable.       
/// *
/// * @note: The fact that an altitude value is received with confidence value set to `unavailable(15)` can be caused
/// * by several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the altitude value may be valid and used by the application.
/// *
/// * @note: If an altitude value is received and its confidence value is set to `outOfRange(14)`, it means that the  
/// * altitude value is not valid and therefore cannot be trusted. Such value is not useful for the application.             
/// *
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AltitudeConfidence {
    Alt00001 = 0,
    Alt00002 = 1,
    Alt00005 = 2,
    Alt00010 = 3,
    Alt00020 = 4,
    Alt00050 = 5,
    Alt00100 = 6,
    Alt00200 = 7,
    Alt00500 = 8,
    Alt01000 = 9,
    Alt02000 = 10,
    Alt05000 = 11,
    Alt10000 = 12,
    Alt20000 = 13,
    OutOfRange = 14,
    Unavailable = 15,
}

///*
/// * This DE represents the altitude value in a WGS84 coordinate system.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `-100 000` if the altitude is equal to or less than -1 000 m,
/// * - `n` (`n > -100 000` and `n < 800 000`) if the altitude is equal to or less than n  x 0,01 metre and greater than (n-1) x 0,01 metre,
/// * - `800 000` if the altitude  greater than 7 999,99 m,
/// * - `800 001` if the information is not available.
/// *
/// * @note: the range of this DE does not use the full binary encoding range, but all reasonable values are covered. In order to cover all possible altitude ranges a larger encoding would be necessary.
/// * @unit: 0,01 metre
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1 (definition of 800 000 has slightly changed)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-100000..=800001"))]
pub struct AltitudeValue(pub i32);

///*
/// * This DE indicates the angle confidence value which represents the estimated absolute accuracy of an angle value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 126`)  if the accuracy is equal to or less than n * 0,1 degrees and greater than (n-1) x * 0,1 degrees,
/// * - `126` if the  accuracy is out of range, i.e. greater than 12,5 degrees,
/// * - `127` if the accuracy information is not available.
/// *
/// * @unit: 0,1 degrees
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct AngleConfidence(pub u8);

///*
/// * This DE indicates the angular acceleration confidence value which represents the estimated accuracy of an angular acceleration value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// * For correlation computation, maximum interval levels shall be assumed.
/// *
/// * The value shall be set to:
/// * - 0 - `degSecSquared-01` - if the accuracy is equal to or less than 1 degree/second^2,
/// * - 1 - `degSecSquared-02` - if the accuracy is equal to or less than 2 degrees/second^2 and greater than 1 degree/second^2,
/// * - 2 - `degSecSquared-05` - if the accuracy is equal to or less than 5 degrees/second^2 and greater than 1 degree/second^2,
/// * - 3 - `degSecSquared-10` - if the accuracy is equal to or less than 10 degrees/second^2 and greater than 5 degrees/second^2,
/// * - 4 - `degSecSquared-20` - if the accuracy is equal to or less than 20 degrees/second^2 and greater than 10 degrees/second^2,
/// * - 5 - `degSecSquared-50` - if the accuracy is equal to or less than 50 degrees/second^2 and greater than 20 degrees/second^2,
/// * - 6 - `outOfRange`       - if the accuracy is out of range, i.e. greater than 50 degrees/second^2,
/// * - 7 - `unavailable`      - if the accuracy information is unavailable.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AngularAccelerationConfidence {
    DegSecSquared01 = 0,
    DegSecSquared02 = 1,
    DegSecSquared05 = 2,
    DegSecSquared10 = 3,
    DegSecSquared20 = 4,
    DegSecSquared50 = 5,
    OutOfRange = 6,
    Unavailable = 7,
}

///*
/// * This DE indicates the angular speed confidence value which represents the estimated absolute accuracy of an angular speed value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// * For correlation computation, maximum interval levels can be assumed.
/// *
/// * The value shall be set to:
/// * - 0 - `degSec-01`   - if the accuracy is equal to or less than 1 degree/second,
/// * - 1 - `degSec-02`   - if the accuracy is equal to or less than 2 degrees/second and greater than 1 degree/second,
/// * - 2 - `degSec-05`   - if the accuracy is equal to or less than 5 degrees/second and greater than 2 degrees/second,
/// * - 3 - `degSec-10`   - if the accuracy is equal to or less than 10 degrees/second and greater than 5 degrees/second,
/// * - 4 - `degSec-20`   - if the accuracy is equal to or less than 20 degrees/second and greater than 10 degrees/second,
/// * - 5 - `degSec-50`   - if the accuracy is equal to or less than 50 degrees/second and greater than 20 degrees/second,
/// * - 6 - `outOfRange`  - if the accuracy is out of range, i.e. greater than 50 degrees/second,
/// * - 7 - `unavailable` - if the accuracy information is unavailable.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AngularSpeedConfidence {
    DegSec01 = 0,
    DegSec02 = 1,
    DegSec05 = 2,
    DegSec10 = 3,
    DegSec20 = 4,
    DegSec50 = 5,
    OutOfRange = 6,
    Unavailable = 7,
}

///*
/// * This DE indicates the number of axles of a passing train.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 2` and `n < 1001`) indicates that the train has n x axles,
/// * - `1001`indicates that the number of axles is out of range,
/// * - `1002` the information is unavailable.
/// *
/// *
/// * @unit: Number of axles
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2..=1002"))]
pub struct AxlesCount(pub u16);

///*
/// * This DE represents the measured uncompensated atmospheric pressure.
/// *
/// * The value shall be set to:
/// * - `2999` indicates that the applicable value is less than 29990 Pa,
/// * - `n` (`n > 2999` and `n <= 12000`) indicates that the applicable value is equal to or less than n x 10 Pa and greater than (n-1) x 10 Pa,
/// * - `12001` indicates that the values is greater than 120000 Pa,
/// * - `12002` indicates that the information is not available.
/// *
/// * @category: Basic information
/// * @unit: 10 Pascal
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2999..=12002"))]
pub struct BarometricPressure(pub u16);

///*
/// * This DE represents a general container for usage in various types of messages.
/// *
/// * It shall include the following components:
/// *
/// * @field stationType: the type of technical context in which the ITS-S that has generated the message is integrated in.
/// *
/// * @field referencePosition: the reference position of the station that has generated the message that contains the basic container.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct BasicContainer {
    pub station_type: TrafficParticipantType,
    pub reference_position: ReferencePositionWithConfidence,
}

impl BasicContainer {
    pub fn new(
        station_type: TrafficParticipantType,
        reference_position: ReferencePositionWithConfidence,
    ) -> Self {
        Self {
            station_type,
            reference_position,
        }
    }
}

///*
/// * This DF provides information about the configuration of a road section in terms of lanes using a list of @ref LanePositionAndType .
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct BasicLaneConfiguration(pub SequenceOf<BasicLaneInformation>);

///*
/// * This DF provides basic information about a single lane of a road segment.
/// * It includes the following components:
/// *
/// * @field laneNumber: the number associated to the lane that provides a transversal identification.
/// *
/// * @field direction: the direction of traffic flow allowed on the lane.
/// *
/// * @field laneWidth: the optional width of the lane.
/// *
/// * @field connectingLane: the number of the connecting lane in the next road section, i.e. the number of the lane which the vehicle will use when travelling from one section to the next,
/// * if it does not actively change lanes. If this component is absent, the lane name number remains the same in the next section.
/// *
/// * @field connectingRoadSection: the identifier of the next road section in direction of traffic, that is connecting to the current road section.
/// * If this component is absent, the connecting road section is the one following the instance where this DF is placed in the @ref RoadConfigurationSectionList.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct BasicLaneInformation {
    pub lane_number: LanePosition,
    pub direction: Direction,
    pub lane_width: Option<LaneWidth>,
    pub connecting_lane: Option<LanePosition>,
    pub connecting_road_section: Option<RoadSectionId>,
}

impl BasicLaneInformation {
    pub fn new(
        lane_number: LanePosition,
        direction: Direction,
        lane_width: Option<LaneWidth>,
        connecting_lane: Option<LanePosition>,
        connecting_road_section: Option<RoadSectionId>,
    ) -> Self {
        Self {
            lane_number,
            direction,
            lane_width,
            connecting_lane,
            connecting_road_section,
        }
    }
}

///*
/// * This DE indicates the cardinal number of bogies of a train.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 1` and `n < 100`) indicates that the train has n x bogies,
/// * - `100`indicates that the number of bogies is out of range,
/// * - `101` the information is unavailable.
/// *
/// * @unit: Number of bogies
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2..=101"))]
pub struct BogiesCount(pub u8);

///*
/// * The DE represents a cardinal number that counts the size of a set.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CardinalNumber1B(pub u8);

///*
/// * The DE represents a cardinal number that counts the size of a set.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=8"))]
pub struct CardinalNumber3b(pub u8);

///*
/// * This DF represents a general Data Frame to describe an angle component along with a confidence value in a cartesian coordinate system.
/// *
/// * It shall include the following components:
/// *
/// * @field value: The angle value which can be estimated as the mean of the current distribution.
/// *
/// * @field confidence: The confidence value associated to the provided value.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

///*
/// * This DE represents an angle value described in a local Cartesian coordinate system, per default counted positive in
/// * a right-hand local coordinate system from the abscissa.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 0` and `n < 3600`) if the angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees,
/// * - `3601` if the information is not available.
/// *
/// * The value 3600 shall not be used.
/// *
/// * @unit 0,1 degrees
/// * @category: Basic information
/// * @revision: Created in V2.1.1, description and value for 3601 corrected in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct CartesianAngleValue(pub u16);

///*
/// * This DF represents a general Data Frame to describe an angular acceleration component along with a confidence value in a cartesian coordinate system.
/// *
/// * It shall include the following components:
/// *
/// * @field value: The angular acceleration component value.
/// *
/// * @field confidence: The confidence value associated to the provided value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianAngularAccelerationComponent {
    pub value: CartesianAngularAccelerationComponentValue,
    pub confidence: AngularAccelerationConfidence,
}

impl CartesianAngularAccelerationComponent {
    pub fn new(
        value: CartesianAngularAccelerationComponentValue,
        confidence: AngularAccelerationConfidence,
    ) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DE represents an angular acceleration value described in a local Cartesian coordinate system, per default counted positive in
/// * a right-hand local coordinate system from the abscissa.
/// *
///  * The value shall be set to:
/// * - `-255` if the acceleration is equal to or less than -255 degrees/s^2,
/// * - `n` (`n > -255` and `n < 255`) if the acceleration is equal to or less than n x 1 degree/s^2,
///      and greater than `(n-1)` x 0,01 degree/s^2,
/// * - `255` if the acceleration is greater than 254 degrees/s^2,
/// * - `256` if the information is unavailable.
/// *
/// * @unit:  degree/s^2 (degrees per second squared)
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-255..=256"))]
pub struct CartesianAngularAccelerationComponentValue(pub i16);

///*
/// * This DF represents an angular velocity component along with a confidence value in a cartesian coordinate system.
/// *
/// * It shall include the following components:
/// *
/// * @field value: The angular velocity component.
/// *
/// * @field confidence: The confidence value associated to the provided value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianAngularVelocityComponent {
    pub value: CartesianAngularVelocityComponentValue,
    pub confidence: AngularSpeedConfidence,
}

impl CartesianAngularVelocityComponent {
    pub fn new(
        value: CartesianAngularVelocityComponentValue,
        confidence: AngularSpeedConfidence,
    ) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DE represents an angular velocity component described in a local Cartesian coordinate system, per default counted positive in
/// * a right-hand local coordinate system from the abscissa.
/// *
/// * The value shall be set to:
/// * - `-255` if the velocity is equal to or less than -255 degrees/s,
/// * - `n` (`n > -255` and `n < 255`) if the velocity is equal to or less than n x 1 degree/s, and greater than (n-1) x 1 degree/s,
/// * - `255` if the velocity is greater than 254 degrees/s,
/// * - `256` if the information is unavailable.
/// *
/// * @unit: degree/s
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-255..=256"))]
pub struct CartesianAngularVelocityComponentValue(pub i16);

///*
/// * This DF represents the value of a cartesian coordinate with a range of -327,68 metres to +327,66 metres.
/// *
/// * The value shall be set to:
/// * - `-32 768` if the longitudinal offset is out of range, i.e. less than or equal to -327,68 metres,
/// * - `n` (`n > -32 768` and `n < 32 767`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
/// * - `32 767` if the longitudinal offset is out of range, i.e. greater than + 327,66 metres.
/// *
/// * @unit 0,01 m
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-32768..=32767"))]
pub struct CartesianCoordinate(pub i16);

///*
/// * This DF represents the value of a cartesian coordinate with a range of -1 310,72 metres to +1 310,70 metres.
/// *
/// * The value shall be set to:
/// * - `-131072` if the longitudinal offset is out of range, i.e. less than or equal to -1 310,72 metres,
/// * - `n` (`n > 131 072` and `n < 131 071`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
/// * - `131 071` if the longitudinal offset is out of range, i.e. greater than + 1 310,70 metres.
/// *  
/// * @unit 0,01 m
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131072..=131071"))]
pub struct CartesianCoordinateLarge(pub i32);

///*
/// * This DF represents the value of a cartesian coordinate with a range of -30,94 metres to +10,00 metres.
/// *
/// * The value shall be set to:
/// * - `3094` if the longitudinal offset is out of range, i.e. less than or equal to -30,94 metres,
/// * - `n` (`n > -3 094` and `n < 1 001`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
/// * - `1001` if the longitudinal offset is out of range, i.e. greater than 10 metres.
/// *
/// * @unit 0,01 m
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-3094..=1001"))]
pub struct CartesianCoordinateSmall(pub i16);

///*
/// * This DF represents a coordinate along with a confidence value in a cartesian reference system.
/// *
/// * It shall include the following components:
/// *
/// * @field value: the coordinate value, which can be estimated as the mean of the current distribution.
/// *
/// * @field confidence: the coordinate confidence value associated to the provided value.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianCoordinateWithConfidence {
    pub value: CartesianCoordinateLarge,
    pub confidence: CoordinateConfidence,
}

impl CartesianCoordinateWithConfidence {
    pub fn new(value: CartesianCoordinateLarge, confidence: CoordinateConfidence) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DF represents a  position in a two- or three-dimensional cartesian coordinate system.
/// *
/// * It shall include the following components:
/// *
/// * @field xCoordinate: the X coordinate value.
/// *
/// * @field yCoordinate: the Y coordinate value.
/// *
/// * @field zCoordinate: the optional Z coordinate value.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianPosition3d {
    pub x_coordinate: CartesianCoordinate,
    pub y_coordinate: CartesianCoordinate,
    pub z_coordinate: Option<CartesianCoordinate>,
}

impl CartesianPosition3d {
    pub fn new(
        x_coordinate: CartesianCoordinate,
        y_coordinate: CartesianCoordinate,
        z_coordinate: Option<CartesianCoordinate>,
    ) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            z_coordinate,
        }
    }
}

///*
/// * This DF represents a  position in a two- or three-dimensional cartesian coordinate system with an associated confidence level for each coordinate.
/// *
/// * It shall include the following components:
/// *
/// * @field xCoordinate: the X coordinate value with the associated confidence level.
/// *
/// * @field yCoordinate: the Y coordinate value with the associated confidence level.
/// *
/// * @field zCoordinate: the optional Z coordinate value with the associated confidence level.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianPosition3dWithConfidence {
    pub x_coordinate: CartesianCoordinateWithConfidence,
    pub y_coordinate: CartesianCoordinateWithConfidence,
    pub z_coordinate: Option<CartesianCoordinateWithConfidence>,
}

impl CartesianPosition3dWithConfidence {
    pub fn new(
        x_coordinate: CartesianCoordinateWithConfidence,
        y_coordinate: CartesianCoordinateWithConfidence,
        z_coordinate: Option<CartesianCoordinateWithConfidence>,
    ) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            z_coordinate,
        }
    }
}

///*
/// * This DF is a representation of the cause code value of a traffic event.
/// *
/// * It shall include the following components:
/// *
/// * @field causeCode: the main cause of a detected event.
/// *
/// * @field subCauseCode: the subordinate cause of a detected event.
/// *
/// * The semantics of the entire DF are completely defined by the component causeCode. The interpretation of the subCauseCode may
/// * provide additional information that is not strictly necessary to understand the causeCode itself, and is therefore optional.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref CauseCodeV2 instead.
/// *
/// * @category: Traffic information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CauseCode {
    pub cause_code: CauseCodeType,
    pub sub_cause_code: SubCauseCodeType,
}

impl CauseCode {
    pub fn new(cause_code: CauseCodeType, sub_cause_code: SubCauseCodeType) -> Self {
        Self {
            cause_code,
            sub_cause_code,
        }
    }
}

///*
/// * This DF is a representation of the cause code value and associated sub cause code value of a traffic event.
/// *
/// * The following options are available:
/// * - 0                                                        - reserved for future use,
/// * - 1  - `trafficCondition1`                                 - in case the type of event is an abnormal traffic condition,
/// * - 2  - `accident2`                                         - in case the type of event is a road accident,
/// * - 3  - `roadworks3`                                        - in case the type of event is roadwork,
/// * - 4                                                        - reserved for future usage,
/// * - 5  - `impassability5`                                    - in case the  type of event is unmanaged road blocking, referring to any
/// *                                                              blocking of a road, partial or total, which has not been adequately secured and signposted,
/// * - 6  - `adverseWeatherCondition-Adhesion6`                 - in case the  type of event is low adhesion,
/// * - 7  - `aquaplaning7`                                      - danger of aquaplaning on the road,
/// * - 8                                                        - reserved for future usage,
/// * - 9  - `hazardousLocation-SurfaceCondition9`               - in case the type of event is abnormal road surface condition,
/// * - 10 - `hazardousLocation-ObstacleOnTheRoad10`             - in case the type of event is obstacle on the road,
/// * - 11 - `hazardousLocation-AnimalOnTheRoad11`               - in case the type of event is animal on the road,
/// * - 12 - `humanPresenceOnTheRoad`                            - in case the type of event is presence of human vulnerable road user on the road,
/// * - 13                                                       - reserved for future usage,
/// * - 14 - `wrongWayDriving14`                                 - in case the type of the event is vehicle driving in wrong way,
/// * - 15 - `rescueAndRecoveryWorkInProgress15`                 - in case the type of event is rescue and recovery work for accident or for a road hazard in progress,
/// * - 16                                                       - reserved for future usage,
/// * - 17 - `adverseWeatherCondition-ExtremeWeatherCondition17` - in case the type of event is extreme weather condition,
/// * - 18 - `adverseWeatherCondition-Visibility18`              - in case the type of event is low visibility,
/// * - 19 - `adverseWeatherCondition-Precipitation19`           - in case the type of event is precipitation,
/// * - 20 - `violence20`                                        - in case the the type of event is human violence on or near the road,
/// * - 21-25                                                    - reserved for future usage,
/// * - 26 - `slowVehicle26`                                     - in case the type of event is slow vehicle driving on the road,
/// * - 27 - `dangerousEndOfQueue27`                             - in case the type of event is dangerous end of vehicle queue,
/// * - 28 - `publicTransportVehicleApproaching                  - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,
/// * - 29-90                                                    - are reserved for future usage,
/// * - 91 - `vehicleBreakdown91`                                - in case the type of event is break down vehicle on the road,
/// * - 92 - `postCrash92`                                       - in case the type of event is a detected crash,
/// * - 93 - `humanProblem93`                                    - in case the type of event is human health problem in vehicles involved in traffic,
/// * - 94 - `stationaryVehicle94`                               - in case the type of event is stationary vehicle,
/// * - 95 - `emergencyVehicleApproaching95`                     - in case the type of event is an approaching vehicle operating on a mission for which the
///                                                                applicable traffic regulations provide it with defined priority rights in traffic.
/// * - 96 - `hazardousLocation-DangerousCurve96`                - in case the type of event is dangerous curve,
/// * - 97 - `collisionRisk97`                                   - in case the type of event is a collision risk,
/// * - 98 - `signalViolation98`                                 - in case the type of event is signal violation,
/// * - 99 - `dangerousSituation99`                              - in case the type of event is dangerous situation in which autonomous safety system in vehicle
/// *                                                              is activated,
/// * - 100 - `railwayLevelCrossing100`                          - in case the type of event is a railway level crossing.
/// * - 101-255                                                  - are reserved for future usage.
/// *
/// * @note: this DF is defined for use as part of CauseCodeV2. It is recommended to use CauseCodeV2.
/// * @category: Traffic information
/// * @revision: Created in V2.1.1, the type of impassability5 changed to ImpassabilitySubCauseCode in V2.2.1, value 28 added in V2.2.1, definition of value 12 and 95 changed in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum CauseCodeChoice {
    Reserved0(SubCauseCodeType),
    TrafficCondition1(TrafficConditionSubCauseCode),
    Accident2(AccidentSubCauseCode),
    Roadworks3(RoadworksSubCauseCode),
    Reserved4(SubCauseCodeType),
    Impassability5(ImpassabilitySubCauseCode),
    AdverseWeatherConditionAdhesion6(AdverseWeatherConditionAdhesionSubCauseCode),
    Aquaplaning7(SubCauseCodeType),
    Reserved8(SubCauseCodeType),
    HazardousLocationSurfaceCondition9(HazardousLocationSurfaceConditionSubCauseCode),
    HazardousLocationObstacleOnTheRoad10(HazardousLocationObstacleOnTheRoadSubCauseCode),
    HazardousLocationAnimalOnTheRoad11(HazardousLocationAnimalOnTheRoadSubCauseCode),
    HumanPresenceOnTheRoad12(HumanPresenceOnTheRoadSubCauseCode),
    Reserved13(SubCauseCodeType),
    WrongWayDriving14(WrongWayDrivingSubCauseCode),
    RescueAndRecoveryWorkInProgress15(RescueAndRecoveryWorkInProgressSubCauseCode),
    Reserved16(SubCauseCodeType),
    AdverseWeatherConditionExtremeWeatherCondition17(
        AdverseWeatherConditionExtremeWeatherConditionSubCauseCode,
    ),
    AdverseWeatherConditionVisibility18(AdverseWeatherConditionVisibilitySubCauseCode),
    AdverseWeatherConditionPrecipitation19(AdverseWeatherConditionPrecipitationSubCauseCode),
    Violence20(SubCauseCodeType),
    Reserved21(SubCauseCodeType),
    Reserved22(SubCauseCodeType),
    Reserved23(SubCauseCodeType),
    Reserved24(SubCauseCodeType),
    Reserved25(SubCauseCodeType),
    SlowVehicle26(SlowVehicleSubCauseCode),
    DangerousEndOfQueue27(DangerousEndOfQueueSubCauseCode),
    PublicTransportVehicleApproaching28(SubCauseCodeType),
    Reserved29(SubCauseCodeType),
    Reserved30(SubCauseCodeType),
    Reserved31(SubCauseCodeType),
    Reserved32(SubCauseCodeType),
    Reserved33(SubCauseCodeType),
    Reserved34(SubCauseCodeType),
    Reserved35(SubCauseCodeType),
    Reserved36(SubCauseCodeType),
    Reserved37(SubCauseCodeType),
    Reserved38(SubCauseCodeType),
    Reserved39(SubCauseCodeType),
    Reserved40(SubCauseCodeType),
    Reserved41(SubCauseCodeType),
    Reserved42(SubCauseCodeType),
    Reserved43(SubCauseCodeType),
    Reserved44(SubCauseCodeType),
    Reserved45(SubCauseCodeType),
    Reserved46(SubCauseCodeType),
    Reserved47(SubCauseCodeType),
    Reserved48(SubCauseCodeType),
    Reserved49(SubCauseCodeType),
    Reserved50(SubCauseCodeType),
    Reserved51(SubCauseCodeType),
    Reserved52(SubCauseCodeType),
    Reserved53(SubCauseCodeType),
    Reserved54(SubCauseCodeType),
    Reserved55(SubCauseCodeType),
    Reserved56(SubCauseCodeType),
    Reserved57(SubCauseCodeType),
    Reserved58(SubCauseCodeType),
    Reserved59(SubCauseCodeType),
    Reserved60(SubCauseCodeType),
    Reserved61(SubCauseCodeType),
    Reserved62(SubCauseCodeType),
    Reserved63(SubCauseCodeType),
    Reserved64(SubCauseCodeType),
    Reserved65(SubCauseCodeType),
    Reserved66(SubCauseCodeType),
    Reserved67(SubCauseCodeType),
    Reserved68(SubCauseCodeType),
    Reserved69(SubCauseCodeType),
    Reserved70(SubCauseCodeType),
    Reserved71(SubCauseCodeType),
    Reserved72(SubCauseCodeType),
    Reserved73(SubCauseCodeType),
    Reserved74(SubCauseCodeType),
    Reserved75(SubCauseCodeType),
    Reserved76(SubCauseCodeType),
    Reserved77(SubCauseCodeType),
    Reserved78(SubCauseCodeType),
    Reserved79(SubCauseCodeType),
    Reserved80(SubCauseCodeType),
    Reserved81(SubCauseCodeType),
    Reserved82(SubCauseCodeType),
    Reserved83(SubCauseCodeType),
    Reserved84(SubCauseCodeType),
    Reserved85(SubCauseCodeType),
    Reserved86(SubCauseCodeType),
    Reserved87(SubCauseCodeType),
    Reserved88(SubCauseCodeType),
    Reserved89(SubCauseCodeType),
    Reserved90(SubCauseCodeType),
    VehicleBreakdown91(VehicleBreakdownSubCauseCode),
    PostCrash92(PostCrashSubCauseCode),
    HumanProblem93(HumanProblemSubCauseCode),
    StationaryVehicle94(StationaryVehicleSubCauseCode),
    EmergencyVehicleApproaching95(EmergencyVehicleApproachingSubCauseCode),
    HazardousLocationDangerousCurve96(HazardousLocationDangerousCurveSubCauseCode),
    CollisionRisk97(CollisionRiskSubCauseCode),
    SignalViolation98(SignalViolationSubCauseCode),
    DangerousSituation99(DangerousSituationSubCauseCode),
    RailwayLevelCrossing100(RailwayLevelCrossingSubCauseCode),
    Reserved101(SubCauseCodeType),
    Reserved102(SubCauseCodeType),
    Reserved103(SubCauseCodeType),
    Reserved104(SubCauseCodeType),
    Reserved105(SubCauseCodeType),
    Reserved106(SubCauseCodeType),
    Reserved107(SubCauseCodeType),
    Reserved108(SubCauseCodeType),
    Reserved109(SubCauseCodeType),
    Reserved110(SubCauseCodeType),
    Reserved111(SubCauseCodeType),
    Reserved112(SubCauseCodeType),
    Reserved113(SubCauseCodeType),
    Reserved114(SubCauseCodeType),
    Reserved115(SubCauseCodeType),
    Reserved116(SubCauseCodeType),
    Reserved117(SubCauseCodeType),
    Reserved118(SubCauseCodeType),
    Reserved119(SubCauseCodeType),
    Reserved120(SubCauseCodeType),
    Reserved121(SubCauseCodeType),
    Reserved122(SubCauseCodeType),
    Reserved123(SubCauseCodeType),
    Reserved124(SubCauseCodeType),
    Reserved125(SubCauseCodeType),
    Reserved126(SubCauseCodeType),
    Reserved127(SubCauseCodeType),
    Reserved128(SubCauseCodeType),
}

///*
/// *The DE represents the value of the cause code of an event.
/// *
/// * The value shall be set to:
/// * - 0                                                     - reserved for future use,
/// * - 1  - `trafficCondition`                               - in case the type of event is an abnormal traffic condition,
/// * - 2  - `accident`                                       - in case the type of event is a road accident,
/// * - 3  - `roadworks`                                      - in case the type of event is roadwork,
/// * - 4                                                     - reserved for future usage,
/// * - 5  - `impassability`                                  - in case the  type of event is unmanaged road blocking, referring to any
/// *                                                           blocking of a road, partial or total, which has not been adequately
/// *                                                           secured and signposted,
/// * - 6  - `adverseWeatherCondition-Adhesion`               - in case the  type of event is low adhesion,
/// * - 7  - `aquaplaning`                                    - danger of aquaplaning on the road,
/// * - 8                                                     - reserved for future usage,
/// * - 9  - `hazardousLocation-SurfaceCondition`             - in case the type of event is abnormal road surface condition,
/// * - 10 - `hazardousLocation-ObstacleOnTheRoad`            - in case the type of event is obstacle on the road,
/// * - 11 - `hazardousLocation-AnimalOnTheRoad`              - in case the type of event is animal on the road,
/// * - 12 - `humanPresenceOnTheRoad`                         - in case the type of event is presence of human vulnerable road user on the road,
/// * - 13                                                    - reserved for future usage,
/// * - 14 - `wrongWayDriving`                                - in case the type of the event is vehicle driving in wrong way,
/// * - 15 - `rescueAndRecoveryWorkInProgress`                - in case the type of event is rescue and recovery work for accident or for a road hazard in progress,
/// * - 16                                                    - reserved for future usage,
/// * - 17 - `adverseWeatherCondition-ExtremeWeatherCondition`- in case the type of event is extreme weather condition,
/// * - 18 - `adverseWeatherCondition-Visibility`             - in case the type of event is low visibility,
/// * - 19 - `adverseWeatherCondition-Precipitation`          - in case the type of event is precipitation,
/// * - 20 - `violence`                                       - in case the the type of event is human violence on or near the road,
/// * - 21-25                                                 - reserved for future usage,
/// * - 26 - `slowVehicle`                                    - in case the type of event is slow vehicle driving on the road,
/// * - 27 - `dangerousEndOfQueue`                            - in case the type of event is dangerous end of vehicle queue,
/// * - 28 - `publicTransportVehicleApproaching               - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,
/// * - 29-90                                                 - are reserved for future usage,
/// * - 91 - `vehicleBreakdown`                               - in case the type of event is break down vehicle on the road,
/// * - 92 - `postCrash`                                      - in case the type of event is a detected crash,
/// * - 93 - `humanProblem`                                   - in case the type of event is human health problem in vehicles involved in traffic,
/// * - 94 - `stationaryVehicle`                              - in case the type of event is stationary vehicle,
/// * - 95 - `emergencyVehicleApproaching`                    - in case the type of event is an approaching vehicle operating on a mission for which the applicable
///                                                             traffic regulations provide it with defined priority rights in traffic.
/// * - 96 - `hazardousLocation-DangerousCurve`               - in case the type of event is dangerous curve,
/// * - 97 - `collisionRisk`                                  - in case the type of event is a collision risk,
/// * - 98 - `signalViolation`                                - in case the type of event is signal violation,
/// * - 99 - `dangerousSituation`                             - in case the type of event is dangerous situation in which autonomous safety system in vehicle
/// *                                                             is activated,
/// * - 100 - `railwayLevelCrossing`                          - in case the type of event is a railway level crossing.
/// * - 101-255                                               - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1, value 28 added in V2.2.1, definition of values 12 and 95 changed in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CauseCodeType(pub u8);

///*
/// * This DF is an alternative representation of the cause code value of a traffic event.
/// *
/// * It shall include the following components:
/// *
/// * @field ccAndScc: the main cause of a detected event. Each entry is of a different type and represents the sub cause code.
///
/// * The semantics of the entire DF are completely defined by the choice value which represents the cause code value.
/// * The interpretation of the sub cause code value may provide additional information that is not strictly necessary to understand
/// * the cause code itself, and is therefore optional.
/// *
/// * @category: Traffic information
/// * @revision: Created in V2.1.1, description amended in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CauseCodeV2 {
    pub cc_and_scc: CauseCodeChoice,
}

impl CauseCodeV2 {
    pub fn new(cc_and_scc: CauseCodeChoice) -> Self {
        Self { cc_and_scc }
    }
}

///*
/// * The DF describes the position of a CEN DSRC road side equipment.
/// *
/// * It shall include the following components:
/// *
/// * @field protectedZoneLatitude: the latitude of the CEN DSRC road side equipment.
/// *
/// * @field protectedZoneLongitude: the latitude of the CEN DSRC road side equipment.
/// *
/// * @field cenDsrcTollingZoneID: the optional ID of the CEN DSRC road side equipment.
/// *
/// * @category: Infrastructure information, Communication information
/// * @revision: revised in V2.1.1 (cenDsrcTollingZoneId is directly of type ProtectedZoneId)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CenDsrcTollingZone {
    pub protected_zone_latitude: Latitude,
    pub protected_zone_longitude: Longitude,
    pub cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
}

impl CenDsrcTollingZone {
    pub fn new(
        protected_zone_latitude: Latitude,
        protected_zone_longitude: Longitude,
        cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
    ) -> Self {
        Self {
            protected_zone_latitude,
            protected_zone_longitude,
            cen_dsrc_tolling_zone_id,
        }
    }
}

///*
/// * This DE represents the ID of a CEN DSRC tolling zone.
/// *
/// * @category: Communication information
/// * @revision: V1.3.1
/// * @note: this DE is deprecated and shall not be used anymore.  
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CenDsrcTollingZoneID(pub ProtectedZoneId);

///*
/// *
/// * This DF represents the shape of a circular area or a right cylinder that is centred on the shape's reference point.
/// *
/// * It shall include the following components:
/// *
/// * @field shapeReferencePoint: optional reference point that represents the centre of the circle, relative to an externally specified reference position.
/// * If this component is absent, the externally specified reference position represents the shape's reference point.
/// *
/// * @field radius: the radius of the circular area.
/// *
/// * @field height: the optional height, present if the shape is a right cylinder extending in the positive z-axis.
/// *
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CircularShape {
    pub shape_reference_point: Option<CartesianPosition3d>,
    pub radius: StandardLength12b,
    pub height: Option<StandardLength12b>,
}

impl CircularShape {
    pub fn new(
        shape_reference_point: Option<CartesianPosition3d>,
        radius: StandardLength12b,
        height: Option<StandardLength12b>,
    ) -> Self {
        Self {
            shape_reference_point,
            radius,
            height,
        }
    }
}

///*
/// * This DF indicates the opening/closure status of the lanes of a carriageway.
/// *
/// * It shall include the following components:
/// *
/// * @field innerhardShoulderStatus: this information is optional and shall be included if an inner hard shoulder is present and the information is known.
/// * It indicates the open/closing status of inner hard shoulder lanes.
/// *
/// * @field outerhardShoulderStatus: this information is optional and shall be included if an outer hard shoulder is present and the information is known.
/// * It indicates the open/closing status of outer hard shoulder lanes.
/// *
/// * @field drivingLaneStatus: this information is optional and shall be included if the information is known.
/// * It indicates the open/closing status of driving lanes.
/// * For carriageways with more than 13 driving lanes, the drivingLaneStatus component shall not be present.
/// *
/// * @category: Infrastructure information, Road topology information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClosedLanes {
    pub innerhard_shoulder_status: Option<HardShoulderStatus>,
    pub outerhard_shoulder_status: Option<HardShoulderStatus>,
    pub driving_lane_status: Option<DrivingLaneStatus>,
}

impl ClosedLanes {
    pub fn new(
        innerhard_shoulder_status: Option<HardShoulderStatus>,
        outerhard_shoulder_status: Option<HardShoulderStatus>,
        driving_lane_status: Option<DrivingLaneStatus>,
    ) -> Self {
        Self {
            innerhard_shoulder_status,
            outerhard_shoulder_status,
            driving_lane_status,
        }
    }
}

///*
/// * This DF provides information about the breakup of a cluster.
/// *
/// * It shall include the following components:
/// *
/// * @field clusterBreakupReason: indicates the reason for breakup.
/// *
/// * @field breakupTime: indicates the time of breakup.
/// *
/// * @category: Cluster Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterBreakupInfo {
    pub cluster_breakup_reason: ClusterBreakupReason,
    pub breakup_time: DeltaTimeQuarterSecond,
}

impl ClusterBreakupInfo {
    pub fn new(
        cluster_breakup_reason: ClusterBreakupReason,
        breakup_time: DeltaTimeQuarterSecond,
    ) -> Self {
        Self {
            cluster_breakup_reason,
            breakup_time,
        }
    }
}

///*
/// * This DE indicates the reason why a cluster leader intends to break up the cluster.
/// *
/// * The value shall be set to:
/// * - 0 - `notProvided`                          - if the information is not provided,
/// * - 1 - `clusteringPurposeCompleted`           - if the cluster purpose has been completed,
/// * - 2 - `leaderMovedOutOfClusterBoundingBox`   - if the leader moved out of the cluster's bounding box,
/// * - 3 - `joiningAnotherCluster`                - if the cluster leader is about to join another cluster,
/// * - 4 - `enteringLowRiskAreaBasedOnMaps`       - if the cluster is entering an area idenrified as low risk based on the use of maps,
/// * - 5 - `receptionOfCpmContainingCluster`      - if the leader received a Collective Perception Message containing information about the same cluster.
/// * - 6 to 15                                    - are reserved for future use.                                    
/// *
/// * @category: Cluster information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct ClusterBreakupReason(pub u8);

///*
/// * This DF provides information about the joining of a cluster.
/// *
/// * It shall include the following components:
/// *
/// * @field clusterId: indicates the identifier of the cluster.
/// *
/// * @field joinTime: indicates the time of joining.
/// *
/// * @category: Cluster Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterJoinInfo {
    pub cluster_id: Identifier1B,
    pub join_time: DeltaTimeQuarterSecond,
}

impl ClusterJoinInfo {
    pub fn new(cluster_id: Identifier1B, join_time: DeltaTimeQuarterSecond) -> Self {
        Self {
            cluster_id,
            join_time,
        }
    }
}

///*
/// * The DF provides information about the leaving of a cluster.
/// *
/// * It shall include the following components:
/// *
/// * @field clusterId: indicates the cluster.
/// *
/// * @field clusterLeaveReason: indicates the reason for leaving.
/// *
/// * @category: Cluster Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterLeaveInfo {
    pub cluster_id: Identifier1B,
    pub cluster_leave_reason: ClusterLeaveReason,
}

impl ClusterLeaveInfo {
    pub fn new(cluster_id: Identifier1B, cluster_leave_reason: ClusterLeaveReason) -> Self {
        Self {
            cluster_id,
            cluster_leave_reason,
        }
    }
}

///*
/// * This DE indicates the reason why a cluster participant is leaving the cluster.
/// *
/// * The value shall be set to:
/// * - 0 - `notProvided `                 - if the information is not provided,
/// * - 1 - `clusterLeaderLost`            - if the cluster leader cannot be found anymore,   
/// * - 2 - `clusterDisbandedByLeader`     - if the cluster has been disbanded by the leader,
/// * - 3 - `outOfClusterBoundingBox`      - if the participants moved out of the cluster's bounding box,
/// * - 4 - `outOfClusterSpeedRange`       - if the cluster speed moved out of a defined range,
/// * - 5 - `joiningAnotherCluster`        - if the participant is joining another cluster,
/// * - 6 - `cancelledJoin`                - if the participant is cancelling a joining procedure,
/// * - 7 - `failedJoin`                   - if the participant failed to join the cluster,
/// * - 8 - `safetyCondition`              - if a safety condition applies.
/// * - 9 to 15                            - are reserved for future use                             
/// *
/// * @category: Cluster information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct ClusterLeaveReason(pub u8);

///*
/// * This DE represents the sub cause codes of the @ref CauseCode `collisionRisk`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                    - in case information on the type of collision risk is unavailable,
/// * - 1 - `longitudinalCollisionRisk`      - in case the type of detected collision risk is longitudinal collision risk,
/// *                                          e.g. forward collision or face to face collision,
/// * - 2 - `crossingCollisionRisk`          - in case the type of detected collision risk is crossing collision risk,
/// * - 3 - `lateralCollisionRisk`           - in case the type of detected collision risk is lateral collision risk,
/// * - 4 - `vulnerableRoadUser`             - in case the type of detected collision risk involves vulnerable road users
/// *                                          e.g. pedestrians or bicycles.
/// * - 5 - `collisionRiskWithPedestrian`    - in case the type of detected collision risk involves at least one pedestrian,
/// * - 6 - `collisionRiskWithCyclist`       - in case the type of detected collision risk involves at least one cyclist (and no pedestrians),
/// * - 7 - `collisionRiskWithMotorVehicle`  - in case the type of detected collision risk involves at least one motor vehicle (and no pedestrians or cyclists),
/// * - 8-255                                - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1, values 5-7 assigned in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CollisionRiskSubCauseCode(pub u8);

///*
/// * This DE represents a confidence level in percentage.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 101`) : for the confidence level in %,
/// * - `101`                   : in case the confidence level is not available.
/// *
/// * @unit Percent
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=101"))]
pub struct ConfidenceLevel(pub u8);

///*
/// * This DE indicates the coordinate confidence value which represents the estimated absolute accuracy of a position coordinate with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 4095`) if the confidence value is is equal to or less than n x 0,01 metre, and greater than (n-1) x 0,01 metre,
/// * - `4095` if the confidence value is greater than 40,94 metres,
/// * - `4096` if the confidence value is not available.
/// *
/// * @unit 0,01 m
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=4096"))]
pub struct CoordinateConfidence(pub u16);

///*
/// * This DE represents the Bravais-Pearson correlation value for each cell of a lower triangular correlation matrix.
/// *
/// * The value shall be set to:
/// * - `-100` in case of full negative correlation,
/// * - `n` (`n > -100` and `n < 0`) if the correlation is negative and equal to n x 100,
/// * - `0` in case of no correlation,
/// * - `n` (`n > 0` and `n < 100`) if the correlation is positive and equal to n x 100,
/// * - `100` in case of full positive correlation,
/// * - `101` in case the correlation information is unavailable.
/// *
/// * @unit: the value is scaled by 100
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-100..=101"))]
pub struct CorrelationCellValue(pub i8);

///*
/// * This DF represents a column of a lower triangular positive semi-definite matrix and consists of a list of correlation cell values ordered by rows.
/// * Given a matrix "A" of size n x n, the number of columns to be included in the lower triangular matrix is k=n-1.
/// * Each column "i" of the lower triangular matrix then contains k-(i-1) values (ordered by rows from 1 to n-1), where "i" refers to the column number count
/// * starting at 1 from the left.
/// *
/// * @category: Sensing Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=13", extensible))]
pub struct CorrelationColumn(pub SequenceOf<CorrelationCellValue>);

///*
/// * This DE represents an ISO 3166-1 [25] country code encoded using ITA-2 encoding.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1 based on ISO 14816 [23]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct CountryCode(pub BitString);

///*
/// * This DF represents the curvature of the vehicle trajectory and the associated confidence value.
/// * The curvature detected by a vehicle represents the curvature of actual vehicle trajectory.
/// *
/// * It shall include the following components:
/// *
/// * @field curvatureValue: Detected curvature of the vehicle trajectory.
/// *
/// * @field curvatureConfidence: along with a confidence value of the curvature value with a predefined confidence level.
/// *
/// * @category: Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Curvature {
    pub curvature_value: CurvatureValue,
    pub curvature_confidence: CurvatureConfidence,
}

impl Curvature {
    pub fn new(curvature_value: CurvatureValue, curvature_confidence: CurvatureConfidence) -> Self {
        Self {
            curvature_value,
            curvature_confidence,
        }
    }
}

///*
/// * The DE describes whether the yaw rate is used to calculate the curvature for a curvature value.
/// *
/// * The value shall be set to:
/// * - 0 - `yawRateUsed`    - if the yaw rate is used,
/// * - 1 - `yawRateNotUsed` - if the yaw rate is not used,
/// * - 2 - `unavailable`    - if the information of curvature calculation mode is unknown.
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CurvatureCalculationMode {
    YawRateUsed = 0,
    YawRateNotUsed = 1,
    Unavailable = 2,
}

///*
/// * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy range of a curvature value with a confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - 0 - `onePerMeter-0-00002` - if the confidence value is less than or equal to 0,00002 m-1,
/// * - 1 - `onePerMeter-0-0001`  - if the confidence value is less than or equal to 0,0001 m-1 and greater than 0,00002 m-1,
/// * - 2 - `onePerMeter-0-0005`  - if the confidence value is less than or equal to 0,0005 m-1 and greater than 0,0001 m-1,
/// * - 3 - `onePerMeter-0-002`   - if the confidence value is less than or equal to 0,002 m-1 and greater than 0,0005 m-1,
/// * - 4 - `nePerMeter-0-01`     - if the confidence value is less than or equal to 0,01 m-1 and greater than 0,002 m-1,
/// * - 5 - `nePerMeter-0-1`      - if the confidence value is less than or equal to 0,1 m-1  and greater than 0,01 m-1,
/// * - 6 - `outOfRange`          - if the confidence value is out of range, i.e. greater than 0,1 m-1,
/// * - 7 - `unavailable`         - if the confidence value is not available.
/// *
/// * @note:	The fact that a curvature value is received with confidence value set to `unavailable(7)` can be caused by
/// * several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the curvature value may be valid and used by the application.
/// *
/// * @note: If a curvature value is received and its confidence value is set to `outOfRange(6)`, it means that the curvature value is not valid
/// * and therefore cannot be trusted. Such value is not useful for the application.
/// *
/// * @category: Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum CurvatureConfidence {
    OnePerMeter000002 = 0,
    OnePerMeter00001 = 1,
    OnePerMeter00005 = 2,
    OnePerMeter0002 = 3,
    OnePerMeter001 = 4,
    OnePerMeter01 = 5,
    OutOfRange = 6,
    Unavailable = 7,
}

///*
/// * This DE describes vehicle turning curve with the following information:
/// * ```
/// *     Value = 1 / Radius * 10000
/// * ```
/// * wherein radius is the vehicle turning curve radius in metres.
/// *
/// * Positive values indicate a turning curve to the left hand side of the driver.
/// * It corresponds to the vehicle coordinate system as defined in ISO 8855 [21].
/// *
/// * The value shall be set to:
/// * - `-1023` for  values smaller than -1023,
/// * - `n` (`n > -1023` and `n < 0`) for negative values equal to or less than `n`, and greater than `(n-1)`,
/// * - `0` when the vehicle is moving straight,
/// * - `n` (`n > 0` and `n < 1022`) for positive values equal to or less than `n`, and greater than `(n-1)`,
/// * - `1022`, for values  greater than 1021,
/// * - `1023`, if the information is not available.
/// *
/// * @note: The present DE is limited to vehicle types as defined in ISO 8855 [21].
/// *
/// * @unit: 1 over 10 000 metres
/// * @category: Vehicle information
/// * @revision: description revised in V2.1.1 (the definition of value 1022 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1023..=1023"))]
pub struct CurvatureValue(pub i16);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousEndOfQueue`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`     - in case information on the type of dangerous queue is unavailable,
/// * - 1 - `suddenEndOfQueue`- in case a sudden end of queue is detected, e.g. due to accident or obstacle,
/// * - 2 - `queueOverHill`   - in case the dangerous end of queue is detected on the road hill,
/// * - 3 - `queueAroundBend` - in case the dangerous end of queue is detected around the road bend,
/// * - 4 - `queueInTunnel`   - in case queue is detected in tunnel,
/// * - 5-255                 - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousEndOfQueueSubCauseCode(pub u8);

///*
/// * This DE indicates the type of the dangerous goods being carried by a heavy vehicle.
/// * The value is assigned according to `class` and `division` definitions of dangerous goods as specified in part II,
/// * chapter 2.1.1.1 of European Agreement concerning the International Carriage of Dangerous Goods by Road [3].
/// *
/// *
/// * @category Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum DangerousGoodsBasic {
    Explosives1 = 0,
    Explosives2 = 1,
    Explosives3 = 2,
    Explosives4 = 3,
    Explosives5 = 4,
    Explosives6 = 5,
    FlammableGases = 6,
    NonFlammableGases = 7,
    ToxicGases = 8,
    FlammableLiquids = 9,
    FlammableSolids = 10,
    SubstancesLiableToSpontaneousCombustion = 11,
    SubstancesEmittingFlammableGasesUponContactWithWater = 12,
    OxidizingSubstances = 13,
    OrganicPeroxides = 14,
    ToxicSubstances = 15,
    InfectiousSubstances = 16,
    RadioactiveMaterial = 17,
    CorrosiveSubstances = 18,
    MiscellaneousDangerousSubstances = 19,
}

///*
/// * This DF provides a description of dangerous goods being carried by a heavy vehicle.
/// *
/// * It shall include the following components:
/// *
/// * @field dangerousGoodsType: Type of dangerous goods.
/// *
/// * @field unNumber: a 4-digit number that identifies the substance of the dangerous goods as specified in
/// * United Nations Recommendations on the Transport of Dangerous Goods - Model Regulations [4],
/// *
/// * @field elevatedTemperature: whether the carried dangerous goods are transported at high temperature.
/// * If yes, the value shall be set to TRUE,
/// *
/// * @field tunnelsRestricted: whether the heavy vehicle carrying dangerous goods is restricted to enter tunnels.
/// * If yes, the value shall be set to TRUE,
/// *
/// * @field limitedQuantity: whether the carried dangerous goods are packed with limited quantity.
/// * If yes, the value shall be set to TRUE,
/// *
/// * @field emergencyActionCode: physical signage placard at the vehicle that carries information on how an emergency
/// * service should deal with an incident. This component is optional; it shall be present if the information is available.
/// *
/// * @field phoneNumber: contact phone number of assistance service in case of incident or accident.
/// * This component is optional, it shall be present if the information is available.
/// *
/// * @field companyName: name of company that manages the transportation of the dangerous goods.
/// * This component is optional; it shall be present if the information is available.
/// *
/// * @category Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DangerousGoodsExtended {
    pub dangerous_goods_type: DangerousGoodsBasic,
    #[rasn(value("0..=9999"))]
    pub un_number: u16,
    pub elevated_temperature: bool,
    pub tunnels_restricted: bool,
    pub limited_quantity: bool,
    #[rasn(size("1..=24"))]
    pub emergency_action_code: Option<Ia5String>,
    pub phone_number: Option<PhoneNumber>,
    #[rasn(size("1..=24"))]
    pub company_name: Option<Utf8String>,
}

impl DangerousGoodsExtended {
    pub fn new(
        dangerous_goods_type: DangerousGoodsBasic,
        un_number: u16,
        elevated_temperature: bool,
        tunnels_restricted: bool,
        limited_quantity: bool,
        emergency_action_code: Option<Ia5String>,
        phone_number: Option<PhoneNumber>,
        company_name: Option<Utf8String>,
    ) -> Self {
        Self {
            dangerous_goods_type,
            un_number,
            elevated_temperature,
            tunnels_restricted,
            limited_quantity,
            emergency_action_code,
            phone_number,
            company_name,
        }
    }
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousSituation`
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                      - in case information on the type of dangerous situation is unavailable,
/// * - 1 - `emergencyElectronicBrakeEngaged`  - in case emergency electronic brake is engaged,
/// * - 2 - `preCrashSystemEngaged`            - in case pre-crash system is engaged,
/// * - 3 - `espEngaged`                       - in case Electronic Stability Program (ESP) system is engaged,
/// * - 4 - `absEngaged`                       - in case Anti-lock Braking System (ABS) is engaged,
/// * - 5 - `aebEngaged`                       - in case Autonomous Emergency Braking (AEB) system is engaged,
/// * - 6 - `brakeWarningEngaged`              - in case brake warning is engaged,
/// * - 7 - `collisionRiskWarningEngaged`      - in case collision risk warning is engaged,
/// * - 8-255                                  - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousSituationSubCauseCode(pub u8);

///*
/// * This DE represents an offset altitude with regards to a defined altitude value.
/// * It may be used to describe a geographical point with regards to a specific reference geographical position.
/// *
/// * The value shall be set to:
/// * - `-12 700` for values equal to or lower than -127 metres,
/// * - `n` (`n > -12 700` and `n <= 0`) for altitude offset n x 0,01 metre below the reference position,
/// * - `0` for no altitudinal offset,
/// * - `n` (`n > 0` and `n < 12799`) for altitude offset n x 0,01 metre above the reference position,
/// * - `12 799` for values equal to or greater than 127,99 metres,
/// * - `12 800` when the information is unavailable.
/// *
/// * @unit: 0,01 metre
/// * @category: GeoReference information
/// * @revision: editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-12700..=12800"))]
pub struct DeltaAltitude(pub i16);

///*
/// * This DE represents an offset latitude with regards to a defined latitude value.
/// * It may be used to describe a geographical point with regards to a specific reference geographical position.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the south from the reference position,
/// * - `0` for no latitudinal offset,
/// * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the north from the reference position,
/// * - `131 072` when the information is unavailable.
/// *
/// * @unit: 10^-7 degree
/// * @category: GeoReference information
/// * @revision: editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLatitude(pub i32);

///*
/// * This DE represents an offset longitude with regards to a defined longitude value.
/// * It may be used to describe a geographical point with regards to a specific reference geographical position.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the west from the reference position,
/// * - `0` for no longitudinal offset,
/// * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the east from the reference position,
/// * - `131 072` when the information is unavailable.
/// *
/// * @unit: 10^-7 degree
/// * @category: GeoReference information
/// * @revision: editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLongitude(pub i32);

///*
/// * This DF defines a geographical point position as a 3 dimensional offset position to a geographical reference point.
/// *
/// * It shall include the following components:
/// *
/// * @field deltaLatitude: A delta latitude offset with regards to the latitude value of the reference position.
/// *
/// * @field deltaLongitude: A delta longitude offset with regards to the longitude value of the reference position.
/// *
/// * @field deltaAltitude: A delta altitude offset with regards to the altitude value of the reference position.
/// *
/// * @category: GeoReference information
/// * @revision:  V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DeltaReferencePosition {
    pub delta_latitude: DeltaLatitude,
    pub delta_longitude: DeltaLongitude,
    pub delta_altitude: DeltaAltitude,
}

impl DeltaReferencePosition {
    pub fn new(
        delta_latitude: DeltaLatitude,
        delta_longitude: DeltaLongitude,
        delta_altitude: DeltaAltitude,
    ) -> Self {
        Self {
            delta_latitude,
            delta_longitude,
            delta_altitude,
        }
    }
}

///*
/// * This DE represents a difference in time with respect to a reference time.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 10001`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,
/// *
/// * Example: a time interval between two consecutive message transmissions.
/// *
/// * @unit: 0,001 s
/// * @category: Basic information
/// * @revision: Created in V2.1.1 from the DE TransmissionInterval in [2]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=10000"))]
pub struct DeltaTimeMilliSecondPositive(pub u16);

///*
/// * This DE represents a signed difference in time with respect to a reference time.
/// *
/// * The value shall be set to:
/// * - `-2048` for time values equal to or less than -2,048 s,
/// * - `n` (`n > -2048` and `n < 2047`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,
/// * - `2047` for time values greater than 2,046 s
/// *
/// * @unit: 0,001 s
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-2048..=2047"))]
pub struct DeltaTimeMilliSecondSigned(pub i16);

///*
/// * This DE represents a difference in time with respect to a reference time.
/// * It can be interpreted as the first 8 bits of a GenerationDeltaTime. To convert it to a @ref GenerationDeltaTime,
/// * multiply by 256 (i.e. append a `00` byte)
/// *
/// * @unit: 256 * 0,001 s
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct DeltaTimeQuarterSecond(pub u8);

///*
/// * This DE represents a  difference in time with respect to a reference time.
/// *
/// * The value shall be set to:
/// * - `-0` for a difference in time of 0 seconds.
/// * - `n` (`n > 0` and `n <= 86400`) to indicate a time value equal to or less than n x 1 s, and greater than (n-1) x 1 s,
/// *
/// * @unit: 1 s
/// * @category: Basic information
/// * @revision: Created in V2.1.1 from ValidityDuration
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=86400"))]
pub struct DeltaTimeSecond(pub u32);

///*
/// * This DE represents a difference in time with respect to a reference time.
/// *
/// * The value shall be set to:
/// * - `-0` for a difference in time of 0 seconds.
/// * - `n` (`n > 0` and `n < 128`) to indicate a time value equal to or less than n x 10 s, and greater than (n-1) x 10 s,
/// *
/// * @unit: 10 s
/// * @category: Basic information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct DeltaTimeTenSeconds(pub u8);

///*
/// * This DE represents a difference in time with respect to a reference time.
/// *
/// * The value shall be set to:
/// * - `0` for a difference in time of 0 seconds.
/// * - `n` (`n > 0` and `n < 128`) to indicate a time value equal to or less than n x 0,1 s, and greater than (n-1) x 0,1 s,
/// *
/// * @unit: 0,1 s
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct DeltaTimeTenthOfSecond(pub u8);

///*
/// * This DF represents a portion of digital map. It shall contain a list of waypoints @ref ReferencePosition.
/// *
/// * @category: GeoReference information
/// * @revision:  V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=256"))]
pub struct DigitalMap(pub SequenceOf<ReferencePosition>);

///*
/// * This DE indicates a direction with respect to a defined reference direction.
/// * Example: a reference direction may be implicitly defined by the definition of a geographical zone.
/// *
/// * The value shall be set to:
/// * - 0 - `sameDirection`     - to indicate the same direction as the reference direction,
/// * - 1 - `oppositeDirection` - to indicate opposite direction as the reference direction,
/// * - 2 - `bothDirections`    - to indicate both directions, i.e. the same and the opposite direction,
/// * - 3 - `unavailable`       - to indicate that the information is unavailable.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct Direction(pub u8);

///*
/// * This DE indicates in which direction something is moving.
/// *
/// * The value shall be set to:
/// * - 0 - `forward`     - to indicate it is moving forward,
/// * - 1 - `backwards`   - to indicate it is moving backwards,
/// * - 2 - `unavailable` - to indicate that the information is unavailable.
/// *
/// * @category: Kinematic information
/// * @revision: editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum DriveDirection {
    Forward = 0,
    Backward = 1,
    Unavailable = 2,
}

///*
/// * This DE indicates whether a driving lane is open to traffic.
/// *
/// * A lane is counted from inside border of the road excluding the hard shoulder. The size of the bit string shall
/// * correspond to the total number of the driving lanes in the carriageway.
/// *
/// * The numbering is matched to @ref LanePosition.
/// * The bit `0` is used to indicate the innermost lane, bit `1` is used to indicate the second lane from inside border.
/// *
/// * If a lane is closed to traffic, the corresponding bit shall be set to `1`. Otherwise, it shall be set to `0`.
/// *
/// * @note: hard shoulder status is not provided by this DE but in @ref HardShoulderStatus.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=13"))]
pub struct DrivingLaneStatus(pub BitString);

///*
/// *
/// * This DF represents the shape of an elliptical area or right elliptical cylinder that is centred
/// * on the shape's reference point defined outside of the context of this DF and oriented w.r.t. a  
/// * cartesian coordinate system defined outside of the context of this DF.
/// *
/// * It shall include the following components:
/// *
/// * @field shapeReferencePoint: optional reference point which represents the centre of the ellipse,
/// * relative to an externally specified reference position. If this component is absent, the
/// * externally specified reference position represents the shape's reference point.
/// *
/// * @field semiMajorAxisLength: half length of the major axis of the ellipse located in the X-Y Plane.
/// *
/// * @field semiMinorAxisLength: half length of the minor axis of the ellipse located in the X-Y Plane.
/// *
/// * @field orientation: the optional orientation of the major axis of the ellipse, measured with
/// * positive values turning around the z-axis using the right-hand rule, starting from the X-axis.
/// *
/// * @field height: the optional height, present if the shape is a right elliptical cylinder extending
/// * in the positive Z-axis.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1, the type of the field orientation changed and the description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EllipticalShape {
    pub shape_reference_point: Option<CartesianPosition3d>,
    pub semi_major_axis_length: StandardLength12b,
    pub semi_minor_axis_length: StandardLength12b,
    pub orientation: Option<CartesianAngleValue>,
    pub height: Option<StandardLength12b>,
}

impl EllipticalShape {
    pub fn new(
        shape_reference_point: Option<CartesianPosition3d>,
        semi_major_axis_length: StandardLength12b,
        semi_minor_axis_length: StandardLength12b,
        orientation: Option<CartesianAngleValue>,
        height: Option<StandardLength12b>,
    ) -> Self {
        Self {
            shape_reference_point,
            semi_major_axis_length,
            semi_minor_axis_length,
            orientation,
            height,
        }
    }
}

///*
/// * This DE indicates whether a vehicle (e.g. public transport vehicle, truck) is under the embarkation process.
/// * If that is the case, the value is *TRUE*, otherwise *FALSE*.
/// *
/// * @category: Vehicle information
/// * @revision: editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct EmbarkationStatus(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct EmergencyPriority(pub BitString);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode "emergencyVehicleApproaching".
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                   - in case further detailed information on the emergency vehicle approaching event
/// *                                         is unavailable,
/// * - 1 - `emergencyVehicleApproaching`   - in case an operating emergency vehicle is approaching,
/// * - 2 - `prioritizedVehicleApproaching` - in case a prioritized vehicle is approaching,
/// * - 3-255                               - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);

///*
/// * This DE indicated the type of energy being used and stored in vehicle.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 - `hydrogenStorage`       - when hydrogen is being used and stored in vehicle,
/// * - 1 - `electricEnergyStorage` - when electric energy is being used and stored in vehicle,
/// * - 2 - `liquidPropaneGas`      - when liquid Propane Gas (LPG) is being used and stored in vehicle,   
/// * - 3 - `compressedNaturalGas ` - when compressedNaturalGas (CNG) is being used and stored in vehicle,
/// * - 4 - `diesel`                - when diesel is being used and stored in vehicle,
/// * - 5 - `gasoline`              - when gasoline is being used and stored in vehicle,
/// * - 6 - `ammonia`               - when ammonia is being used and stored in vehicle.
/// *
/// * - Otherwise, the corresponding bit shall be set to `0`.
/// *
/// * @category: Vehicle information
/// * @revision: editorial revision in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("7"))]
pub struct EnergyStorageType(pub BitString);

///*
/// *
/// * This DF represents a vehicle category according to the UNECE/TRANS/WP.29/78/Rev.4 [16].
/// * The following options are available:
/// *
/// * @field euVehicleCategoryL: indicates a vehicle in the L category.
/// *
/// * @field euVehicleCategoryM: indicates a vehicle in the M category.
/// *
/// * @field euVehicleCategoryN: indicates a vehicle in the N category.
/// *
/// * @field euVehicleCategoryO: indicates a vehicle in the O category.
/// *
/// * @field euVehicleCategoryT: indicates a vehicle in the T category.
/// *
/// * @field euVehicleCategoryG: indicates a vehicle in the G category.
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EuVehicleCategoryCode {
    EuVehicleCategoryL(EuVehicleCategoryL),
    EuVehicleCategoryM(EuVehicleCategoryM),
    EuVehicleCategoryN(EuVehicleCategoryN),
    EuVehicleCategoryO(EuVehicleCategoryO),
    EuVehicleCategoryT(()),
    EuVehicleCategoryG(()),
}

///*
/// * This DE represents one of the specific categories in the L category: L1, L2, L3, L4, L5, L6, or L7 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
/// *
/// *
/// * @category: Vehicle information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryL {
    L1 = 0,
    L2 = 1,
    L3 = 2,
    L4 = 3,
    L5 = 4,
    L6 = 5,
    L7 = 6,
}

///*
/// * This DE represents one of the specific categories in the M category: M1, M2, or M3 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
/// *
/// *
/// * @category: Vehicle information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryM {
    M1 = 0,
    M2 = 1,
    M3 = 2,
}

///*
/// * This DE represents one of the specific categories in the N category: N1, N2, or N3 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
/// *
/// *
/// * @category: Vehicle information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryN {
    N1 = 0,
    N2 = 1,
    N3 = 2,
}

///*
/// * This DE represents one of the specific categories in the O category: O1, O2, O3 or O4 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
/// *
/// *
/// * @category: Vehicle information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryO {
    O1 = 0,
    O2 = 1,
    O3 = 2,
    O4 = 3,
}

///*
/// * This DF represents the Euler angles which describe the orientation of an object bounding box in a Cartesian coordinate system with an associated confidence level for each angle.
/// *
/// * It shall include the following components:
/// *
/// * @field zAngle: z-angle of object bounding box at the time of measurement, with the associated confidence.
/// * The angle is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule, starting from the x-axis.
/// * This extrinsic rotation shall be applied around the centre point of the object bounding box before all other rotations.
/// *
/// * @field yAngle: optional y-angle of object bounding box at the time of measurement, with the associated confidence.
/// * The angle is measured with positive values considering the object orientation turning around the y-axis using the right-hand rule, starting from the z-axis.
/// * This extrinsic rotation shall be applied around the centre point of the object bounding box after the rotation by zAngle and before the rotation by xAngle.
/// *
/// * @field xAngle: optional x-angle of object bounding box at the time of measurement, with the associated confidence.
/// * The angle is measured with positive values considering the object orientation turning around the x-axis using the right-hand rule, starting from the z-axis.
/// * This extrinsic rotation shall be applied around the centre point of the object bounding box after all other rotations.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EulerAnglesWithConfidence {
    pub z_angle: CartesianAngle,
    pub y_angle: Option<CartesianAngle>,
    pub x_angle: Option<CartesianAngle>,
}

impl EulerAnglesWithConfidence {
    pub fn new(
        z_angle: CartesianAngle,
        y_angle: Option<CartesianAngle>,
        x_angle: Option<CartesianAngle>,
    ) -> Self {
        Self {
            z_angle,
            y_angle,
            x_angle,
        }
    }
}

///*
/// * The DF shall contain a list of @ref EventPoint.  
/// *
/// * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list.
/// * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF.
/// *
/// * @category: GeoReference information, Traffic information
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref EventZone instead.
/// * @revision: Generalized the semantics in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=23"))]
pub struct EventHistory(pub SequenceOf<EventPoint>);

///*
/// * This DF provides information related to an event at a defined position.
/// *
/// * It shall include the following components:
/// *
/// * @field eventPosition: offset position of a detected event point to a defined position.
/// *
/// * @field eventDeltaTime: optional time travelled by the detecting ITS-S since the previous detected event point.
/// *
/// * @field informationQuality: Information quality of the detection for this event point.
/// *
/// * @category: GeoReference information, Traffic information
/// * @revision: generalized the semantics in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EventPoint {
    pub event_position: DeltaReferencePosition,
    pub event_delta_time: Option<PathDeltaTime>,
    pub information_quality: InformationQuality,
}

impl EventPoint {
    pub fn new(
        event_position: DeltaReferencePosition,
        event_delta_time: Option<PathDeltaTime>,
        information_quality: InformationQuality,
    ) -> Self {
        Self {
            event_position,
            event_delta_time,
            information_quality,
        }
    }
}

///*
/// * The DF shall contain a list of @ref EventPoint, where all @ref EventPoint either contain the COMPONENT eventDeltaTime
/// * or do not contain the COMPONENT eventDeltaTime.  
/// *
/// * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list.
/// * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF.
/// *
/// * @category: GeoReference information, Traffic information
/// * @revision: created in V2.1.1 based on EventHistory
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct EventZone(pub EventHistory);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext1 {
    #[rasn(value("128..=16511"), tag(context, 0))]
    Content(u16),
    #[rasn(tag(context, 1))]
    Extension(Ext2),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext2 {
    #[rasn(value("16512..=2113663"), tag(context, 0))]
    Content(u32),
    #[rasn(tag(context, 1))]
    Extension(Ext3),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2113664..=270549119", extensible))]
pub struct Ext3(pub Integer);

///*
/// * This DE describes the status of the exterior light switches of a vehicle incl. VRU vehicles.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 - `lowBeamHeadlightsOn`    - when the low beam head light switch is on,
/// * - 1 - `highBeamHeadlightsOn`   - when the high beam head light switch is on,
/// * - 2 - `leftTurnSignalOn`       - when the left turnSignal switch is on,
/// * - 3 - `rightTurnSignalOn`      - when the right turn signal switch is on,
/// * - 4 - `daytimeRunningLightsOn` - when the daytime running light switch is on,
/// * - 5 - `reverseLightOn`         - when the reverse light switch is on,
/// * - 6 - `fogLightOn`             - when the tail fog light switch is on,
/// * - 7 - `parkingLightsOn`        - when the parking light switch is on.
/// *
/// * @note: The value of each bit indicates the state of the switch, which commands the corresponding light.
/// * The bit corresponding to a specific light is set to `1`, when the corresponding switch is turned on,
/// * either manually by the driver or automatically by a vehicle system. The bit value does not indicate
/// * if the corresponding lamps are alight or not.
/// *
/// * If a vehicle is not equipped with a certain light or if the light switch status information is not available,
/// * the corresponding bit shall be set to `0`.
/// *
/// * As the bit value indicates only the state of the switch, the turn signal and hazard signal bit values shall not
/// * alternate with the blinking interval.
/// *
/// * For hazard indicator, the `leftTurnSignalOn (2)` and `rightTurnSignalOn (3)` shall be both set to `1`.
/// *
/// * @category Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct ExteriorLights(pub BitString);

///*
/// * This DF represents the top-level DF to represent a lane position. A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway.
/// *
/// * @note: This DF is the most general way to represent a lane position: it provides a complete set of information regarding a transversal (dimensionless) position on the carriageway at a specific
/// * reference position, i.e. it provides different options and synonyms to represent the lane at which the reference position (the point) is located. A confidence is used to describe the probability
/// * that the object is located in the provided lane. The dimension of the object or extension of an area are not considered: See @ref OccupiedLanesWithConfidence for describing the occupation of lanes,
/// * where the dimensions of an object or the extension of an area is considered.
/// *
/// * It shall include the following components:
/// *
/// * @field lanePositionBased: lane position information for a defined reference position.
/// *
/// * @field mapBased: optional lane position information described in the context of a MAPEM as specified in ETSI TS 103 301 [15].
/// * If present, it shall describe the same reference position using the lane identification in the MAPEM. This component can be used only if a MAPEM is available for the reference position
/// * (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased.
/// *
/// * @field confidence: confidence information for expressing the probability that the object is located at the indicated lane.  
/// * If the value of the component lanePositionBased is generated directly from the absolute reference position and reference topology information,
/// * no sensor shall be indicated in the component usedDetectionInformation of the @ref MetaInformation.
/// *
/// * @category: Road Topology information
/// * @revision: newly created in V2.2.1. The previous DF GeneralizedLanePosition is now renamed to @ref LanePositionOptions.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GeneralizedLanePosition {
    pub lane_position_based: LanePositionOptions,
    pub map_based: Option<MapPosition>,
    pub confidence: MetaInformation,
}

impl GeneralizedLanePosition {
    pub fn new(
        lane_position_based: LanePositionOptions,
        map_based: Option<MapPosition>,
        confidence: MetaInformation,
    ) -> Self {
        Self {
            lane_position_based,
            map_based,
            confidence,
        }
    }
}

///*
/// * This DF represents transversal position information with respect to the road, at an externally defined reference position. It shall contain a set of up to `4` @ref GeneralizedLanePosition.
/// * Multiple entries can be used to describe several lane positions with the associated confidence, in cases where the reference position cannot be mapped to a single lane.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct GeneralizedLanePositions(pub SequenceOf<GeneralizedLanePosition>);

///*
/// * This DE represents a timestamp based on TimestampIts modulo 65 536.
/// * This means that generationDeltaTime = TimestampIts mod 65 536.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1 based on ETSI TS 103 900 [1]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct GenerationDeltaTime(pub u16);

///*
/// * This DF indicates a geographical position.
/// *
/// * It shall include the following components:
/// *
/// * @field latitude: the latitude of the geographical position.
/// *
/// * @field longitude: the longitude of the geographical position.
/// *
/// * @field altitude: the altitude of the geographical position with default value unavailable.
/// *
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GeoPosition {
    pub latitude: Latitude,
    pub longitude: Longitude,
    #[rasn(default = "geo_position_altitude_default")]
    pub altitude: AltitudeValue,
}

impl GeoPosition {
    pub fn new(latitude: Latitude, longitude: Longitude, altitude: AltitudeValue) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }
}

fn geo_position_altitude_default() -> AltitudeValue {
    AltitudeValue(800001).into()
}

///*
/// * This DE indicates the current status of a hard shoulder: whether it is available for special usage
/// * (e.g. for stopping or for driving) or closed for all vehicles.
/// *
/// * The value shall be set to:
/// * - 0 - `availableForStopping` - if the hard shoulder is available for stopping in e.g. emergency situations,
/// * - 1 - `closed`               - if the hard shoulder is closed and cannot be occupied in any case,
/// * - 2 - `availableForDriving`  - if the hard shoulder is available for regular driving.
/// *
/// * @category: Traffic information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum HardShoulderStatus {
    AvailableForStopping = 0,
    Closed = 1,
    AvailableForDriving = 2,
}

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-AnimalOnTheRoad`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`          - in case further detailed information on the animal(s) on the road is unavailable,
/// * - 1 - `wildAnimals`          - in case wild animals of unknown size are present on the road,
/// * - 2 - `herdOfAnimals`        - in case a herd of animals is present on the road,
/// * - 3 - `smallAnimals`         - in case small size animals of unknown type are present on the road,
/// * - 4 - `largeAnimals`         - in case large size animals of unknown type are present on the road,
/// * - 5 - `wildAnimalsSmall`     - in case small size wild animal(s) are present on the road,
/// * - 6 - `wildAnimalsLarge`     - in case large size wild animal(s) are present on the road,
/// * - 7 - `domesticAnimals`      - in case domestic animal(s) of unknown size are detected on the road,
/// * - 8 - `domesticAnimalsSmall` - in case small size domestic animal(s) are present on the road,
/// * - 9 - `domesticAnimalsLarge` - in case large size domestic animal(s) are present on the road.
/// * - 10-255                     - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1, named values 5 to 9 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);

///*
/// * This DE represents the sub cause code of the @ref CauseCode  `hazardousLocation-DangerousCurve`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                                        - in case further detailed information on the dangerous curve is unavailable,
/// * - 1 - `dangerousLeftTurnCurve`                             - in case the dangerous curve is a left turn curve,
/// * - 2 - `dangerousRightTurnCurve`                            - in case the dangerous curve is a right turn curve,
/// * - 3 - `multipleCurvesStartingWithUnknownTurningDirection`  - in case of multiple curves for which the starting curve turning direction is not known,
/// * - 4 - `multipleCurvesStartingWithLeftTurn`                 - in case of multiple curves starting with a left turn curve,
/// * - 5 - `multipleCurvesStartingWithRightTurn`                - in case of multiple curves starting with a right turn curve.
/// * - 6-255                                                    - are reserved for future usage.
/// *
/// * The definition of whether a curve is dangerous may vary according to region and according to vehicle types/mass
/// * and vehicle speed driving on the curve. This definition is out of scope of the present document.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-ObstacleOnTheRoad`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`    - in case further detailed information on the detected obstacle is unavailable,
/// * - 1 - `shedLoad`       - in case detected obstacle is large amount of obstacles (shedload),
/// * - 2 - `partsOfVehicles`- in case detected obstacles are parts of vehicles,
/// * - 3 - `partsOfTyres`   - in case the detected obstacles are parts of tyres,
/// * - 4 - `bigObjects`     - in case the detected obstacles are big objects,
/// * - 5 - `fallenTrees`    - in case the detected obstacles are fallen trees,
/// * - 6 - `hubCaps`        - in case the detected obstacles are hub caps,
/// * - 7 - `waitingVehicles`- in case the detected obstacles are waiting vehicles.
/// * - 8-255                - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-SurfaceCondition`.
/// *
///The value shall be set to:
/// * - 0  - `unavailable`     - in case further detailed information on the road surface condition is unavailable,
/// * - 1  - `rockfalls`       - in case rock falls are detected on the road surface,
/// * - 2  - `earthquakeDamage`- in case the road surface is damaged by earthquake,
/// * - 3  - `sewerCollapse`   - in case of sewer collapse on the road surface,
/// * - 4  - `subsidence`      - in case road surface is damaged by subsidence,
/// * - 5  - `snowDrifts`      - in case road surface is damaged due to snow drift,
/// * - 6  - `stormDamage`     - in case road surface is damaged by strong storm,
/// * - 7  - `burstPipe`       - in case road surface is damaged due to pipe burst,
/// * - 8  - `volcanoEruption` - in case road surface is damaged due to volcano eruption,
/// * - 9  - `fallingIce`      - in case road surface damage is due to falling ice,
/// * - 10 - `fire`            - in case there is fire on or near to the road surface.
/// * - 11-255                 - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);

///*
/// * This DF represents the Heading in a WGS84 co-ordinates system.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * It shall include the following components:
/// *
/// * @field headingValue: the heading value.
/// *
/// * @field headingConfidence: the confidence value of the heading value with a predefined confidence level.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84Angle instead.
/// * @category: Kinematic Information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Heading {
    pub heading_value: HeadingValue,
    pub heading_confidence: HeadingConfidence,
}

impl Heading {
    pub fn new(heading_value: HeadingValue, heading_confidence: HeadingConfidence) -> Self {
        Self {
            heading_value,
            heading_confidence,
        }
    }
}

///*
/// * This DF  provides  information  associated to heading  change indicators  such as  a  change  of  direction.
/// *
/// * It shall include the following components:
/// *
/// * @field direction: the direction of heading change value.
/// *
/// * @field actionDeltaTime: the period over which a direction change action is performed.
/// *
/// * @category: Kinematic Information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct HeadingChangeIndication {
    pub direction: TurningDirection,
    pub action_delta_time: DeltaTimeTenthOfSecond,
}

impl HeadingChangeIndication {
    pub fn new(direction: TurningDirection, action_delta_time: DeltaTimeTenthOfSecond) -> Self {
        Self {
            direction,
            action_delta_time,
        }
    }
}

///*
/// * This DE indicates the heading confidence value which represents the estimated absolute accuracy of a heading value with a confidence level of 95 %.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degree and more than (n-1) x 0,1 degree,
/// * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,
/// * - `127` if the confidence value information is not available.
/// *
/// * @note:	The fact that a value is received with confidence value set to `unavailable(127)` can be caused by several reasons,
/// * such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the heading value may be valid and used by the application.
/// *
/// * @note: If a heading value is received and its confidence value is set to `outOfRange(126)`, it means that the
/// * heading value is not valid and therefore cannot be trusted. Such value is not useful for the application.
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleConfidence instead.
/// *
/// * @unit: 0,1 degree
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct HeadingConfidence(pub u8);

///*
/// * This DE represents the orientation of the horizontal velocity vector with regards to the WGS84 north.
/// * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleValue instead.
/// *
/// * Unit: 0,1 degree
/// * Categories: GeoReference information
/// * @revision: Description revised in V2.1.1 (usage of value 3600 specified)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct HeadingValue(pub u16);

///*
/// * This DE represents the height of the left or right longitude carrier of vehicle from base to top (left or right carrier seen from vehicle
/// * rear to front).
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 1` and `n < 99`) if the height information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
/// * - `99` if the height is out of range, i.e. equal to or greater than 0,98 m,
/// * - `100` if the height information is not available.
/// *
/// * @unit 0,01 metre
/// * @category Vehicle information
/// * @revision: Description revised in V2.1.1 (the definition of 99 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=100"))]
pub struct HeightLonCarr(pub u8);

///*
/// * This DE represents the value of the sub cause code of the @ref CauseCode `humanPresenceOnTheRoad`.
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`                    - in case further detailed information abou the human presence on the road is unavailable,
/// * - 1 - `childrenOnRoadway`              - in case children are present on the road,
/// * - 2 - `cyclistOnRoadway`               - in case cyclist(s) are present on the road,
/// * - 3 - `motorcyclistOnRoadway`          - in case motorcyclist(s) are present on the road,
/// * - 4 - `pedestrian`                     - in case pedestrian(s) of any type are present on the road,
/// * - 5 - `ordinary-pedestrian`            - in case pedestrian(s) to which no more-specific profile applies are present on the road,
/// * - 6 - `road-worker`                    - in case pedestrian(s) with the role of a road worker applies are present on the road,
/// * - 7 - `first-responder`                - in case pedestrian(s) with the role of a first responder applies are present on the road,  
/// * - 8 - `lightVruVehicle                 - in case light vru vehicle(s) of any type are present on the road,
/// * - 9 - `bicyclist `                     - in case cycle(s) and their bicyclist(s) are present on the road,
/// * - 10 - `wheelchair-user`               - in case wheelchair(s) and their user(s) are present on the road,
/// * - 11 - `horse-and-rider`               - in case horse(s) and rider(s) are present on the road,
/// * - 12 - `rollerskater`                  - in case rolleskater(s) and skater(s) are present on the road,
/// * - 13 - `e-scooter`                     - in case e-scooter(s) and rider(s) are present on the road,
/// * - 14 - `personal-transporter`          - in case personal-transporter(s) and rider(s) are present on the road,
/// * - 15 - `pedelec`                       - in case pedelec(s) and rider(s) are present on the road,
/// * - 16 - `speed-pedelec`                 - in case speed-pedelec(s) and rider(s) are present on the road,
/// * - 17 - `ptw`                           - in case powered-two-wheeler(s) of any type are present on the road,
/// * - 18 - `moped`                         - in case moped(s) and rider(s) are present on the road,
/// * - 19 - `motorcycle`                    - in case motorcycle(s) and rider(s) are present on the road,
/// * - 20 - `motorcycle-and-sidecar-right`  - in case motorcycle(s) with sidecar(s) on the right and rider are present on the road,
/// * - 21 - `motorcycle-and-sidecar-left`   - in case motorcycle(s) with sidecar(s) on the left and rider are present on the road.
/// * - 22-255                               - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: editorial revision in V2.1.1, named values 4-21 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode "humanProblem".
/// *
/// * The value shall be set to:
/// * - 0 - `unavailable`    - in case further detailed information on human health problem is unavailable,
/// * - 1 - `glycemiaProblem`- in case human problem is due to glycaemia problem,
/// * - 2 - `heartProblem`   - in case human problem is due to heart problem.
/// * - 3-255                - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanProblemSubCauseCode(pub u8);

///*
/// * This DE is a general identifier.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Identifier1B(pub u8);

///*
/// * This DE is a general identifier.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct Identifier2B(pub u16);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `impassability`
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`              - in case further detailed information about the unmanaged road blockage is unavailable,
/// * - 1 `flooding          `       - in case the road is affected by flooding,
/// * - 2 `dangerOfAvalanches`       - in case the road is at risk of being affected or blocked by avalanches,
/// * - 3 `blastingOfAvalanches`     - in case there is an active blasting of avalanches on or near the road,
/// * - 4 `landslips`                - in case the road is affected by landslips,
/// * - 5 `chemicalSpillage`         - in case the road is affected by chemical spillage,
/// * - 6 `winterClosure`            - in case the road is impassable due to a winter closure.
/// * - 7 `sinkhole`                 - in case the road is impassable due to large holes in the road surface.
/// * - 8 `earthquakeDamage`         - in case the road is obstructed or partially obstructed because of damage caused by an earthquake.
/// * - 9 `fallenTrees`              - in case the road is obstructed or partially obstructed by one or more fallen trees.
/// * - 10 `rockfalls`               - in case the road is obstructed or partially obstructed due to fallen rocks.
/// * - 11 `sewerOverflow`           - in case the road is obstructed or partially obstructed by overflows from one or more sewers.
/// * - 12 `stormDamage`             - in case the road is obstructed or partially obstructed by debris caused by strong winds.
/// * - 13 `subsidence`              - in case the road surface has sunken or collapsed in places.
/// * - 14 `burstPipe`               - in case the road surface has sunken or collapsed in places due to burst pipes.
/// * - 15 `burstWaterMain`          - in case the road is obstructed due to local flooding and/or subsidence.
/// * - 16 `fallenPowerCables`       - in case the road is obstructed or partly obstructed by one or more fallen power cables.
/// * - 17 `snowDrifts`              - in case the road is obstructed or partially obstructed by snow drifting in progress or patches of deep snow due to earlier drifting.
/// * - 15-255                       - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ImpassabilitySubCauseCode(pub u8);

///*
/// * This DE represents the quality level of provided information.
/// *
/// * The value shall be set to:
/// * - `0` if the information is unavailable,
/// * - `1` if the quality level is lowest,
/// * - `n` (`n > 1` and `n < 7`) if the quality level is n,
/// * - `7` if the quality level is highest.
/// *
/// * @note: Definition of quality level is out of scope of the present document.
/// * @category: Basic information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct InformationQuality(pub u8);

///*
/// * This DF represents a frequency channel
/// *
/// * It shall include the following components:
/// *
/// * @field centreFrequency: the centre frequency of the channel in 10^(exp+2) Hz (where exp is exponent)
/// *
/// * @field channelWidth: width of the channel in 10^exp Hz (where exp is exponent)
/// *
/// * @field exponent: exponent of the power of 10 used in the calculation of the components above.
/// *
/// * @category: Communication information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InterferenceManagementChannel {
    #[rasn(value("1..=99999"))]
    pub centre_frequency: u32,
    #[rasn(value("0..=9999"))]
    pub channel_width: u16,
    #[rasn(value("0..=15"))]
    pub exponent: u8,
}

impl InterferenceManagementChannel {
    pub fn new(centre_frequency: u32, channel_width: u16, exponent: u8) -> Self {
        Self {
            centre_frequency,
            channel_width,
            exponent,
        }
    }
}

///*
/// * This DF shall contain a list of up to 16 definitions containing interference management information, per affected frequency channels.
/// *  
/// * @category: Communication information.
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct InterferenceManagementInfo(pub SequenceOf<InterferenceManagementInfoPerChannel>);

///*
/// * This DF contains interference management information for one affected frequency channel.
/// *
/// * It shall include the following components:
/// *
/// * @field interferenceManagementChannel: frequency channel for which the zone should be applied interference management
/// *
/// * @field interferenceManagementZoneType: type of the interference management zone.
/// *
/// * @field interferenceManagementMitigationType: optional type of the mitigation to be used in the interference management zone.
/// * In the case where no mitigation should be applied by the ITS-S, this is indicated by the field interferenceManagementMitigationType being absent.
/// *
/// * @field expiryTime: optional time at which the validity of the interference management communication zone will expire.
/// * This component is present when the interference management is temporarily valid
/// *
/// * @category: Communication information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct InterferenceManagementInfoPerChannel {
    pub interference_management_channel: InterferenceManagementChannel,
    pub interference_management_zone_type: InterferenceManagementZoneType,
    pub interference_management_mitigation_type: Option<MitigationForTechnologies>,
    pub expiry_time: Option<TimestampIts>,
}

impl InterferenceManagementInfoPerChannel {
    pub fn new(
        interference_management_channel: InterferenceManagementChannel,
        interference_management_zone_type: InterferenceManagementZoneType,
        interference_management_mitigation_type: Option<MitigationForTechnologies>,
        expiry_time: Option<TimestampIts>,
    ) -> Self {
        Self {
            interference_management_channel,
            interference_management_zone_type,
            interference_management_mitigation_type,
            expiry_time,
        }
    }
}

///*
/// *
/// * This DF represents a zone  inside which the ITS communication should be restricted in order to manage interference.
/// *
/// * It shall include the following components:
/// *
/// * @field zoneDefinition: contains the geographical definition of the zone.
/// *
/// * @field managementInfo: contains interference management information applicable in the zone defined in the component zoneDefinition.
/// *
/// * @category: Communication information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct InterferenceManagementZone {
    pub zone_definition: InterferenceManagementZoneDefinition,
    pub management_info: InterferenceManagementInfo,
}

impl InterferenceManagementZone {
    pub fn new(
        zone_definition: InterferenceManagementZoneDefinition,
        management_info: InterferenceManagementInfo,
    ) -> Self {
        Self {
            zone_definition,
            management_info,
        }
    }
}

///*
/// * This DF represents the geographical definition of the zone where band sharing occurs.
/// *
/// * It shall include the following components:
/// *
/// * @field interferenceManagementZoneLatitude: Latitude of the centre point of the interference management zone.
/// *
/// * @field interferenceManagementZoneLongitude: Longitude of the centre point of the interference management zone.
/// *
/// * @field interferenceManagementZoneId: optional identification of the interference management zone.
/// *
/// * @field interferenceManagementZoneShape: shape of the interference management zone placed at the centre point.
/// *
/// * @category: Communication information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct InterferenceManagementZoneDefinition {
    pub interference_management_zone_latitude: Latitude,
    pub interference_management_zone_longitude: Longitude,
    pub interference_management_zone_id: Option<ProtectedZoneId>,
    #[rasn(value("0.."))]
    pub interference_management_zone_shape: Option<Shape>,
}

impl InterferenceManagementZoneDefinition {
    pub fn new(
        interference_management_zone_latitude: Latitude,
        interference_management_zone_longitude: Longitude,
        interference_management_zone_id: Option<ProtectedZoneId>,
        interference_management_zone_shape: Option<Shape>,
    ) -> Self {
        Self {
            interference_management_zone_latitude,
            interference_management_zone_longitude,
            interference_management_zone_id,
            interference_management_zone_shape,
        }
    }
}

///*
/// * This DE defines the type of an interference management zone, so that an ITS-S can
/// * assert the actions to do while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station).
/// * It is an extension of the type @ref ProtectedZoneType.
/// *
/// * The value shall be set to:
/// * - 0 - `permanentCenDsrcTolling` - as specified in ETSI TS 102 792 [14],
/// * - 1 - `temporaryCenDsrcTolling` - as specified in ETSI TS 102 792 [14],
/// * - 2 - `unavailable`             - default value. Set to 2 for backwards compatibility with DSRC tolling,
/// * - 3 - `urbanRail`               - as specified in ETSI TS 103 724 [13], clause 7,
/// * - 4 - `satelliteStation`        - as specified in ETSI TS 103 724 [13], clause 7,
/// * - 5 - `fixedLinks`              - as specified in ETSI TS 103 724 [13], clause 7.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum InterferenceManagementZoneType {
    PermanentCenDsrcTolling = 0,
    TemporaryCenDsrcTolling = 1,
    Unavailable = 2,
    UrbanRail = 3,
    SatelliteStation = 4,
    FixedLinks = 5,
}

///*
/// * This DF shall contain a list of up to 16 interference  management zones.  
/// *
/// * **EXAMPLE**: An interference management communication zone may be defined around a CEN DSRC road side equipment or an urban rail operational area.
/// *
/// * @category: Communication information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct InterferenceManagementZones(pub SequenceOf<InterferenceManagementZone>);

///*
/// * This DF represents a unique id for an intersection, in accordance with ETSI TS 103 301 [15].
/// *
/// * It shall include the following components:
/// *
/// * @field region: the optional identifier of the entity that is responsible for the region in which the intersection is placed.
/// * It is the duty of that entity to guarantee that the @ref Id is unique within the region.
/// *
/// * @field id: the identifier of the intersection
/// *
/// * @note: when the component region is present, the IntersectionReferenceId is guaranteed to be globally unique.
/// * @category: Road topology information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IntersectionReferenceId {
    pub region: Option<Identifier2B>,
    pub id: Identifier2B,
}

impl IntersectionReferenceId {
    pub fn new(region: Option<Identifier2B>, id: Identifier2B) -> Self {
        Self { region, id }
    }
}

///*
/// * This DE represents the vehicle type according to ISO 3833 [22].
/// * A "term No" refers to the number of the corresponding term and its definition in ISO 3833.
/// *
/// * The value shall be set to:
/// * - 0	- `passengerCar`              - term No 3.1.1
/// * - 1	- `saloon`                    - term No 3.1.1.1 (sedan)
/// * - 2	- `convertibleSaloon`         - term No 3.1.1.2
/// * - 3	- `pullmanSaloon`             - term No 3.1.1.3
/// * - 4	- `stationWagon`              - term No 3.1.1.4
/// * - 5	- `truckStationWagon`         - term No 3.1.1.4.1
/// * - 6	- `coupe`                     - term No 3.1.1.5 (coupe)
/// * - 7	- `convertible`               - term No 3.1.1.6 (open tourer, roadstar, spider)
/// * - 8	- `multipurposePassengerCar`  - term No 3.1.1.7
/// * - 9	- `forwardControlPassengerCar`- term No 3.1.1.8
/// * - 10	- `specialPassengerCar`       - term No 3.1.1.9
/// * - 11	- `bus`                       - term No 3.1.2
/// * - 12	- `minibus`                   - term No 3.1.2.1
/// * - 13	- `urbanBus`                  - term No 3.1.2.2
/// * - 14	- `interurbanCoach`           - term No 3.1.2.3
/// * - 15	- `longDistanceCoach`         - term No 3.1.2.4
/// * - 16	- `articulatedBus`            - term No 3.1.2.5
/// * - 17	- `trolleyBus	`             - term No 3.1.2.6
/// * - 18	- `specialBus`                - term No 3.1.2.7
/// * - 19	- `commercialVehicle`         - term No 3.1.3
/// * - 20	- `specialCommercialVehicle`  - term No 3.1.3.1
/// * - 21	- `specialVehicle`            - term No 3.1.4
/// * - 22	- `trailingTowingVehicle`     - term No 3.1.5 (draw-bar tractor)
/// * - 23	- `semiTrailerTowingVehicle`  - term No 3.1.6 (fifth wheel tractor)
/// * - 24	- `trailer`                   - term No 3.2.1
/// * - 25	- `busTrailer`                - term No 3.2.1.1
/// * - 26	- `generalPurposeTrailer`     - term No 3.2.1.2
/// * - 27	- `caravan`                   - term No 3.2.1.3
/// * - 28	- `specialTrailer`            - term No 3.2.1.4
/// * - 29	- `semiTrailer`               - term No 3.2.2
/// * - 30	- `busSemiTrailer`            - term No 3.2.2.1
/// * - 31	- `generalPurposeSemiTrailer` - term No 3.2.2.2
/// * - 32	- `specialSemiTrailer`        - term No 3.2.2.3
/// * - 33	- `roadTrain`                 - term No 3.3.1
/// * - 34	- `passengerRoadTrain`        - term No 3.3.2
/// * - 35	- `articulatedRoadTrain`      - term No 3.3.3
/// * - 36	- `doubleRoadTrain`           - term No 3.3.4
/// * - 37	- `compositeRoadTrain`        - term No 3.3.5
/// * - 38	- `specialRoadTrain`          - term No 3.3.6
/// * - 39	- `moped`                     - term No 3.4
/// * - 40	- `motorCycle`                - term No 3.5
/// * - 41-255                           - reserved for future use
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Iso3833VehicleType(pub u8);

///*
/// * This DE represent the identifier of an organization according to the applicable registry.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1 based on ISO 14816 [23]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct IssuerIdentifier(pub u16);

///*
/// * This DF shall contain  a list of waypoints @ref ReferencePosition.
/// *
/// * @category: GeoReference information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=40"))]
pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);

///*
/// * This DF represents a common message header for application and facilities layer messages.
/// * It is included at the beginning of an ITS message as the message header.
/// *
/// * It shall include the following components:
/// *
/// * @field protocolVersion: version of the ITS message.
/// *
/// * @field messageId: type of the ITS message.
/// *
/// * @field stationId: the identifier of the ITS-S that generated the ITS message.
/// *
/// * @category: Communication information
/// * @revision:  update in V2.1.1: messageID and stationID changed to messageId and stationId; messageId is of type MessageId.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsPduHeader {
    pub protocol_version: OrdinalNumber1B,
    pub message_id: MessageId,
    pub station_id: StationId,
}

impl ItsPduHeader {
    pub fn new(
        protocol_version: OrdinalNumber1B,
        message_id: MessageId,
        station_id: StationId,
    ) -> Self {
        Self {
            protocol_version,
            message_id,
            station_id,
        }
    }
}

///*
/// * This DE represents the identifier of the IVIM.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1 based on ETSI TS 103 301 [15]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32767", extensible))]
pub struct IviIdentificationNumber(pub Integer);

///*
/// * This DF provides the reference to the information contained in a IVIM according to ETSI TS 103 301 [15].
/// *
/// * It shall include the following components:
/// *
/// * @field serviceProviderId: identifier of the organization that provided the IVIM.
/// *
/// * @field iviIdentificationNumber: identifier of the IVIM, as assigned by the organization identified in serviceProviderId.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IvimReference {
    pub service_provider_id: Provider,
    pub ivi_identification_number: IviIdentificationNumber,
}

impl IvimReference {
    pub fn new(
        service_provider_id: Provider,
        ivi_identification_number: IviIdentificationNumber,
    ) -> Self {
        Self {
            service_provider_id,
            ivi_identification_number,
        }
    }
}

///*
/// * This DF shall contain a list of @ref IvimReference.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct IvimReferences(pub SequenceOf<IvimReference>);

///*
/// * This DE indicates a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway.
/// *
/// * For right-hand traffic roads, the value shall be set to:
/// * - `-1` if the position is off, i.e. besides the road,
/// * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the leftmost lane,
/// * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the leftmost lane to the rightmost lane of a specific traffic direction,
/// * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to rightmost lane (if present).
/// *
/// * For left-hand traffic roads, the value shall be set to:
/// * - `-1` if the position is off, i.e. besides the road,
/// * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the rightmost lane,
/// * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the rightmost lane to the leftmost lane of a specific traffic direction,
/// * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to leftmost lane (if present).
/// *
/// *  @note: in practice this means that the position is counted from "inside" to "outside" no matter which traffic practice is used.
/// *
/// * If the carriageway allows only traffic in one direction (e.g. in case of dual or multiple carriageway roads), the position is counted from the physical border of the carriageway.
/// * If the carriageway allows traffic in both directions and there is no physical delimitation between traffic directions (e.g. on a single carrriageway road),
/// * the position is counted from the legal (i.e. optical) separation between traffic directions (horizontal marking).
/// *
/// * If not indicated otherwise (by lane markings or traffic signs), the legal separation on carriageways allowing traffic on both directions is identified as follows:
/// * - If the total number of lanes N is even, the lanes are divided evenly between the traffic directions starting from the outside of the carriageway on both sides and the
/// *   imaginary separation between traffic directions is on the border between the even number of lanes N/2.
/// * - If the total number of lanes N is odd, the lanes are divided evenly between traffic direction starting from the outside of the carriageway on both sides.
/// *   The remaining middle lane is assigned to both traffic directions as innermost lane.
/// *
/// * @category: Road topology information
/// * @revision: Description of the legal separation of carriageways added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1..=14"))]
pub struct LanePosition(pub i8);

///*
/// * This DF indicates a transversal position in resolution of lanes and other associated details.
/// *
/// * It shall include the following components:
/// *
/// * @field transversalPosition: the transversal position.
/// *
/// * @field laneType: the type of the lane identified in the component transversalPosition. By default set to `traffic`.
/// *
/// * @field direction: the traffic direction for the lane position relative to a defined reference direction. By default set to `sameDirection`, i.e. following the reference direction.
/// *
/// * @category Road topology information
/// * @revision: direction added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LanePositionAndType {
    pub transversal_position: LanePosition,
    #[rasn(default = "lane_position_and_type_lane_type_default")]
    pub lane_type: LaneType,
    #[rasn(default = "lane_position_and_type_direction_default")]
    pub direction: Direction,
}

impl LanePositionAndType {
    pub fn new(
        transversal_position: LanePosition,
        lane_type: LaneType,
        direction: Direction,
    ) -> Self {
        Self {
            transversal_position,
            lane_type,
            direction,
        }
    }
}

fn lane_position_and_type_lane_type_default() -> LaneType {
    LaneType(0).into()
}

fn lane_position_and_type_direction_default() -> Direction {
    Direction(0).into()
}

///*
/// * This DF represents a set of options to describe a lane position and is the second level DF to represent a lane position. The top-level DFs are @ref GeneralizedLanePosition or @ref OccupiedLanesWithConfidence.
/// * A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway.
/// *
/// * The following options are available:
/// *
/// * @field simplelanePosition: a single lane position without any additional context information.
/// *
/// * @field simpleLaneType: a lane type, to be used when the lane position is unknown but the type of lane is known. This can be used in scenarios where a certain confidence about the used lane type is given
/// * but no or limited knowledge about the absolute lane number is available. For example, a cyclist on a cycle-lane or vehicles on a specific lane that is unique for the part of the road (e.g. a bus lane).
/// *
/// * @field detailedlanePosition: a single lane position with additional lane details.
/// *
/// * @field lanePositionWithLateralDetails: a single lane position with additional details and the lateral position within the lane.
/// *
/// * @field trafficIslandPosition: a position on a traffic island, i.e. between two lanes.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.2.1 from the DF GeneralizedLanePosition of V2.1.1.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LanePositionOptions {
    SimplelanePosition(LanePosition),
    SimpleLaneType(LaneType),
    DetailedlanePosition(LanePositionAndType),
    LanePositionWithLateralDetails(LanePositionWithLateralDetails),
    TrafficIslandPosition(TrafficIslandPosition),
}

///*
/// * This DF is a third-level DF that represents a lane position and is an extended version of @ref LanePositionAndType that adds the distances to the left and right lane border.
/// *
/// * It shall additionally include the following components:
/// *
/// * @field distanceToLeftBorder: the distance of the transversal position to the left lane border. The real value shall be rounded to the next lower encoding-value.
/// *
/// * @field distanceToRightBorder: the distance of the transversal position to the right lane border. The real value shall be rounded to the next lower encoding-value.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LanePositionWithLateralDetails {
    pub distance_to_left_border: StandardLength9b,
    pub distance_to_right_border: StandardLength9b,
    pub transversal_position: LanePosition,
    #[rasn(default = "lane_position_with_lateral_details_lane_type_default")]
    pub lane_type: LaneType,
    #[rasn(default = "lane_position_with_lateral_details_direction_default")]
    pub direction: Direction,
}

impl LanePositionWithLateralDetails {
    pub fn new(
        distance_to_left_border: StandardLength9b,
        distance_to_right_border: StandardLength9b,
        transversal_position: LanePosition,
        lane_type: LaneType,
        direction: Direction,
    ) -> Self {
        Self {
            distance_to_left_border,
            distance_to_right_border,
            transversal_position,
            lane_type,
            direction,
        }
    }
}

fn lane_position_with_lateral_details_lane_type_default() -> LaneType {
    LaneType(0).into()
}

fn lane_position_with_lateral_details_direction_default() -> Direction {
    Direction(0).into()
}

///*
/// * This DE represents the type of a lane.
/// *
/// * The value shall be set to:
/// * - 0	- `traffic`            - Lane dedicated to the movement of vehicles,
/// * - 1	- `through`            - Lane dedicated to the movement of vehicles travelling ahead and not turning,
/// * - 2	- `reversible`         - Lane where the direction of traffic can be changed to match the peak flow,
/// * - 3	- `acceleration`	   - Lane that allows vehicles entering a road to accelerate to the speed of through traffic before merging with it,
/// * - 4	- `deceleration`       - Lane that allows vehicles exiting a road to decelerate before leaving it,
/// * - 5	- `leftHandTurning`    - Lane reserved for slowing down and making a left turn, so as not to disrupt traffic,
/// * - 6	- `rightHandTurning`   - Lane reserved for slowing down and making a right turn so as not to disrupt traffic,
/// * - 7	- `dedicatedVehicle`   - Lane dedicated to movement of motor vehicles with specific characteristics, such as heavy goods vehicles, etc.,
/// * - 8	- `bus`                - Lane dedicated to movement of buses providing public transport,
/// * - 9	- `taxi`               - Lane dedicated to movement of taxis,
/// * - 10	- `hov`                - Carpooling lane or high occupancy vehicle lane,
/// * - 11	- `hot`                - High occupancy vehicle lanes that is allowed to be used without meeting the occupancy criteria by paying a toll,
/// * - 12	- `pedestrian`         - Lanes dedicated to pedestrians such as pedestrian sidewalk paths,
/// * - 13	- `cycleLane`	       - Lane dedicated to exclusive or preferred use by bicycles,
/// * - 14	- `median`             - Lane not dedicated to movement of vehicles but representing a median / central reservation  such as the central median,
///                                 separating the two directional carriageways of the highway,
/// * - 15	- `striping`	       - Lane not dedicated to movement of vehicles but covered with roadway markings,
/// * - 16	- `trackedVehicle`     - Lane dedicated to movement of trains, trams and trolleys,
/// * - 17	- `parking`            - Lanes dedicated to vehicles parking, stopping and loading lanes,
/// * - 18	- `emergency`          - Lane dedicated to vehicles in breakdown or to emergency vehicles also called hard shoulder,
/// * - 19	- `verge`              - Lane representing the verge, i.e. a narrow strip of grass or plants and sometimes also trees located between
///                                 the road surface edge and the boundary of a road,
/// * - 20	`minimumRiskManoeuvre` - Lane dedicated to automated vehicles making a minimum risk manoeuvre,
/// * - 21	`separatedCycleLane`   - Lane dedicated to exclusive or preferred use by bicycles that is phyisically separated from the vehicle-traffic lanes, e.g. by a verge.
/// * - values 22 to 30             reserved for future use.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.1.1, named value 21 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct LaneType(pub u8);

///*
/// * This DE represents the width of a lane measured at a defined position.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 1022`) if the lane width information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
/// * - `1022` if the lane width is out of range, i.e. greater than 10,21 m,
/// * - `1023` if the lane width information is not available.
/// *
/// * The value 0 shall not be used.
/// *
/// * @unit: 0,01 metre
/// * @category: Road topology information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct LaneWidth(pub u16);

///*
/// * This DF indicates the vehicle acceleration at lateral direction and the confidence value of the lateral acceleration.
/// *
/// * It shall include the following components:
/// *
/// * @field lateralAccelerationValue: lateral acceleration value at a point in time.
/// *
/// * @field lateralAccelerationConfidence: confidence value of the lateral acceleration value.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead.
/// * @category Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LateralAcceleration {
    pub lateral_acceleration_value: LateralAccelerationValue,
    pub lateral_acceleration_confidence: AccelerationConfidence,
}

impl LateralAcceleration {
    pub fn new(
        lateral_acceleration_value: LateralAccelerationValue,
        lateral_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            lateral_acceleration_value,
            lateral_acceleration_confidence,
        }
    }
}

///*
/// * This DE represents the vehicle acceleration at lateral direction in the centre of the mass of the empty vehicle.
/// * It corresponds to the vehicle coordinate system as specified in ISO 8855 [21].
/// *
/// * The value shall be set to:
/// * - `-160` for acceleration values equal to or less than -16 m/s^2,
/// * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is accelerating towards the right side with regards to the vehicle orientation
/// *                            with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,
/// * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating towards the left hand side with regards to the vehicle orientation
///						     with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,
/// * - `160` for acceleration values greater than 15,9 m/s^2,
/// * - `161` when the data is unavailable.
/// *
/// * @note: the empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead.
/// *  
/// * @unit: 0,1 m/s^2
/// * @category: Vehicle information
/// * @revision: Description updated in V2.1.1 (the meaning of 160 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct LateralAccelerationValue(pub i16);

///*
/// * This DE represents the absolute geographical latitude in a WGS84 coordinate system, providing a range of 90 degrees in north or
/// * in south hemisphere.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= -900 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for latitudes south of the Equator,
/// * - `0` is used for the latitude of the equator,
/// * - `n` (`n > 0` and `n < 900 000 001`) x 10^-7 degree, i.e. positive values for latitudes north of the Equator,
/// * - `900 000 001` when the information is unavailable.
/// *
/// * @unit: 10^-7 degree
/// * @category: GeoReference information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct Latitude(pub i32);

///*
/// * This DE indicates the status of light bar and any sort of audible alarm system besides the horn.
/// * This includes various common sirens as well as backup up beepers and other slow speed manoeuvring alerts.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 - `lightBarActivated`      - when the light bar is activated,
/// * - 1 - `sirenActivated`         - when the siren is activated.
/// *
/// * Otherwise, it shall be set to 0.
/// *
/// * @category Vehicle information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct LightBarSirenInUse(pub BitString);

///*
/// * This DE represents the absolute geographical longitude in a WGS84 coordinate system, providing a range of 180 degrees
/// * to the east or to the west of the prime meridian.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > -1 800 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for longitudes to the west,
/// * - `0` to indicate the prime meridian,
/// * - `n` (`n > 0` and `n < 1 800 000 001`) x 10^-7 degree, i.e. positive values for longitudes to the east,
/// * - `1 800 000 001` when the information is unavailable.
/// *
/// * The value -1 800 000 000 shall not be used.
/// *
/// * @unit: 10^-7 degree
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1800000000..=1800000001"))]
pub struct Longitude(pub i32);

///*
/// * This DF indicates the vehicle acceleration at longitudinal direction and the confidence value of the longitudinal acceleration.
/// *
/// * It shall include the following components:
/// *
/// * @field longitudinalAccelerationValue: longitudinal acceleration value at a point in time.
///
/// * @field longitudinalAccelerationConfidence: confidence value of the longitudinal acceleration value.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead.
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LongitudinalAcceleration {
    pub longitudinal_acceleration_value: LongitudinalAccelerationValue,
    pub longitudinal_acceleration_confidence: AccelerationConfidence,
}

impl LongitudinalAcceleration {
    pub fn new(
        longitudinal_acceleration_value: LongitudinalAccelerationValue,
        longitudinal_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            longitudinal_acceleration_value,
            longitudinal_acceleration_confidence,
        }
    }
}

///*
/// * This DE represents the vehicle acceleration at longitudinal direction in the centre of the mass of the empty vehicle.
/// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
/// *
/// * The value shall be set to:
/// * - `-160` for acceleration values equal to or less than -16 m/s^2,
/// * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is braking with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2
/// * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `160` for acceleration values greater than 15,9 m/s^2,
/// * - `161` when the data is unavailable.
/// *
/// * This acceleration is along the tangent plane of the road surface and does not include gravity components.
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead.
/// *
/// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// * @unit: 0,1 m/s^2
/// * @category: Vehicle information
/// * @revision: description revised in V2.1.1 (the meaning of 160 has changed slightly). T
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct LongitudinalAccelerationValue(pub i16);

///*
/// * This DF represents the estimated position along the longitudinal extension of a carriageway or lane.
/// *
/// * It shall include the following components:
/// *
/// * @field  longitudinalLanePositionValue: the mean value of the longitudinal position along the carriageway or lane w.r.t. an externally defined start position.
/// *
/// * @field  longitudinalLanePositionConfidence: The confidence value associated to the value.
/// *
/// * @category: Road topology information
/// * @revision: created in V2.1.1, description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LongitudinalLanePosition {
    pub longitudinal_lane_position_value: LongitudinalLanePositionValue,
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

///*
/// * This DE indicates the longitudinal lane position confidence value which represents the estimated accuracy of longitudinal lane position measurement with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 1 022`) if the  confidence value is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,
/// * - `1 022` if the confidence value is out of range i.e. greater than 102,1 m,
/// * - `1 023` if the confidence value is unavailable.
/// *
/// * @unit 0,1 metre
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct LongitudinalLanePositionConfidence(pub u16);

///*
/// * This DE represents the longitudinal offset of a map-matched position along a matched lane, beginning from the lane's starting point.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 0` and `n < 32766`) if the longitudinal offset information is equal to or less than n x 0,1 metre and more than (n-1) x 0,1 metre,
/// * - `32 766` if the longitudinal offset is out of range, i.e. greater than 3276,5 m,
/// * - `32 767` if the longitudinal offset information is not available.
/// *
/// * @unit 0,1 metre
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=32767"))]
pub struct LongitudinalLanePositionValue(pub u16);

///*
/// * This DF shall contain a list of a lower triangular positive semi-definite matrices.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct LowerTriangularPositiveSemidefiniteMatrices(
    pub SequenceOf<LowerTriangularPositiveSemidefiniteMatrix>,
);

///*
/// * This DF represents a lower triangular positive semi-definite matrix.
/// *
/// * It shall include the following components:
/// *
/// * @field componentsIncludedIntheMatrix: the indication of which components of a @ref PerceivedObject are included in the matrix.
/// * This component also implicitly indicates the number n of included components which defines the size (n x n) of the full correlation matrix "A".
/// *
/// * @field matrix: the list of cells of the lower triangular positive semi-definite matrix ordered by columns and by rows.
/// *
/// * The number of columns to be included "k" is equal to the number of included components "n" indicated by componentsIncludedIntheMatrix minus 1: k = n-1.
/// * These components shall be included in the order or their appearance in componentsIncludedIntheMatrix.
/// * Each column "i" of the lowerTriangularCorrelationMatrixColumns contains k-(i-1) values.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LowerTriangularPositiveSemidefiniteMatrix {
    pub components_included_inthe_matrix: MatrixIncludedComponents,
    pub matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
}

impl LowerTriangularPositiveSemidefiniteMatrix {
    pub fn new(
        components_included_inthe_matrix: MatrixIncludedComponents,
        matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
    ) -> Self {
        Self {
            components_included_inthe_matrix,
            matrix,
        }
    }
}

///*
/// * This DF represents the columns of a lower triangular positive semi-definite matrix, each column not including the main diagonal cell of the matrix.
/// * Given a matrix "A" of size n x n, the number of @ref CorrelationColumn to be included in the lower triangular matrix is k=n-1.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1, extension indicator added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=13", extensible))]
pub struct LowerTriangularPositiveSemidefiniteMatrixColumns(pub SequenceOf<CorrelationColumn>);

///*
/// * This DF indicates a position on a topology description transmitted in a MAPEM according to ETSI TS 103 301 [15].
/// *
/// * It shall include the following components:
/// *
/// * @field mapReference: optionally identifies the MAPEM containing the topology information.
/// * It is absent if the MAPEM topology is known from the context.
/// *
/// * @field laneId: optionally identifies the lane in the road segment or intersection topology on which the position is located.
/// *
/// * @field connectionId: optionally identifies the connection inside the conflict area of an intersection, i.e. it identifies a trajectory for travelling through the
/// * conflict area of an intersection which connects e.g an ingress with an egress lane.
/// *
/// * @field longitudinalLanePosition: optionally indicates the longitudinal offset of the map-matched position of the object along the lane or connection measured from the start of the lane/connection, along the lane.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.1.1, definition of longitudinalLanePosition amended in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MapPosition {
    pub map_reference: Option<MapReference>,
    pub lane_id: Option<Identifier1B>,
    pub connection_id: Option<Identifier1B>,
    pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
}

impl MapPosition {
    pub fn new(
        map_reference: Option<MapReference>,
        lane_id: Option<Identifier1B>,
        connection_id: Option<Identifier1B>,
        longitudinal_lane_position: Option<LongitudinalLanePosition>,
    ) -> Self {
        Self {
            map_reference,
            lane_id,
            connection_id,
            longitudinal_lane_position,
        }
    }
}

///*
/// * This DF provides the reference to the information contained in a MAPEM according to ETSI TS 103 301 [15].
/// *
/// * The following options are provided:
/// *
/// * @field roadsegment: option that identifies the description of a road segment contained in a MAPEM.
/// *
/// * @field intersection: option that identifies the description of an intersection contained in a MAPEM.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum MapReference {
    Roadsegment(RoadSegmentReferenceId),
    Intersection(IntersectionReferenceId),
}

///*
/// * This DF shall contain a list of @ref MapReference.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct MapReferences(pub SequenceOf<MapReference>);

///*
/// * This DF provides information about the configuration of a road section in terms of MAPEM lanes or connections using a list of @ref MapemExtractedElementReference.
///
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct MapemConfiguration(pub SequenceOf<MapemElementReference>);

///*
/// * This DF provides references to MAPEM connections using a list of @ref Identifier1B.
/// * Note: connections are  allowed �maneuvers� (e.g. an ingress / egress relation) on an intersection.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct MapemConnectionList(pub SequenceOf<Identifier1B>);

///*
/// * This DF provides references to an element described in a MAPEM according to ETSI TS 103 301 [i.15], such as a lane or connection at a specific intersection or road segment.
/// *
/// * It shall include the following components:
/// *
/// * @field mapReference: the optional reference to a MAPEM that describes the intersection or road segment. It is absent if the MAPEM topology is known from the context.
/// *
/// * @field laneIds: the optional list of the identifiers of the lanes to be referenced.
/// *
/// * @field connectionIds: the optional list of the identifiers of the connections to be referenced.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MapemElementReference {
    pub map_reference: Option<MapReference>,
    pub lane_ids: Option<MapemLaneList>,
    pub connection_ids: Option<MapemConnectionList>,
}

impl MapemElementReference {
    pub fn new(
        map_reference: Option<MapReference>,
        lane_ids: Option<MapemLaneList>,
        connection_ids: Option<MapemConnectionList>,
    ) -> Self {
        Self {
            map_reference,
            lane_ids,
            connection_ids,
        }
    }
}

///*
/// * This DF provides references to MAPEM lanes using a list of @ref Identifier1B.
/// *
/// * @category: Road topology information
/// * @revision: Created in 2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct MapemLaneList(pub SequenceOf<Identifier1B>);

///*
/// * This DE indicates the components of an @ref PerceivedObject that are included in the @ref LowerTriangularPositiveSemidefiniteMatrix.
/// *
/// * The corresponding bit shall be set to 1 if the component is included:
/// * - 0 - `xCoordinate`                   - when the component xCoordinate of the component @ref CartesianPosition3dWithConfidence is included,
/// * - 1 - `yCoordinate`                   - when the component yCoordinate of the component @ref CartesianPosition3dWithConfidence is included,   
/// * - 2 - `zCoordinate`                   - when the component zCoordinate of the component @ref CartesianPosition3dWithConfidence is included,
/// * - 3 - `xVelocityOrVelocityMagnitude`  - when the component xVelocity of the component @ref VelocityCartesian or the component VelocityMagnitude of the component @ref VelocityPolarWithZ is included,   
/// * - 4 - `yVelocityOrVelocityDirection`  - when the component yVelocity of the component @ref VelocityCartesian or the component VelocityDirection of the component @ref VelocityPolarWithZ is included,   
/// * - 5 - `zVelocity`                     - when the component zVelocity of the component @ref VelocityCartesian or of the component @ref VelocityPolarWithZ is included,
/// * - 6 - `xAccelOrAccelMagnitude`        - when the component xAcceleration of the component @ref AccelerationCartesian or the component AccelerationMagnitude of the component @ref AccelerationPolarWithZ is included,  
/// * - 7 - `yAccelOrAccelDirection`        - when the component yAcceleration of the component @ref AccelerationCartesian or the component AccelerationDirection of the component @ref AccelerationPolarWithZ is included,   
/// * - 8 - `zAcceleration`                 - when the component zAcceleration of the component @ref AccelerationCartesian or of the component @ref AccelerationPolarWithZ is included,
/// * - 9 - `zAngle`                        - when the component zAngle is included,
/// * - 10 - `yAngle`                       - when the component yAngle is included,   
/// * - 11 - `xAngle`                       - when the component xAngle is included,   
/// * - 12 - `zAngularVelocity`             - when the component zAngularVelocity is included.   
/// *
/// * Otherwise, it shall be set to 0.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("13", extensible))]
pub struct MatrixIncludedComponents(pub BitString);

///*
/// * This DE represents the type of facility layer message.
/// *
/// *  The value shall be set to:
/// *	- 1  - `denm`              - for Decentralized Environmental Notification Message (DENM) as specified in ETSI EN 302 637-3 [2],
/// *  - 2  - `cam`               - for Cooperative Awareness Message (CAM) as specified in ETSI EN 302 637-2 [1],
/// *  - 3  - `poim`              - for Point of Interest message as specified in ETSI TS 103 916 [9],
/// *  - 4  - `spatem`            - for Signal Phase And Timing Extended Message (SPATEM) as specified in ETSI TS 103 301 [15],
/// *  - 5  - `mapem`             - for MAP Extended Message (MAPEM) as specified in ETSI TS 103 301 [15],
/// *  - 6  - `ivim`              - for in Vehicle Information Message (IVIM) as specified in ETSI TS 103 301 [15],
/// *  - 7  - `rfu1`              - reserved for future usage,
/// *  - 8  - `rfu2`              - reserved for future usage,
/// *  - 9  - `srem`              - for Signal Request Extended Message as specified in ETSI TS 103 301 [15],
/// *  - 10 - `ssem`              - for Signal request Status Extended Message as specified in ETSI TS 103 301 [15],
/// *  - 11 - `evcsn`             - for Electrical Vehicle Charging Spot Notification message as specified in ETSI TS 101 556-1 [9],
/// *  - 12 - `saem`              - for Services Announcement Extended Message as specified in ETSI EN 302 890-1 [17],
/// *  - 13 - `rtcmem`            - for Radio Technical Commission for Maritime Services Extended Message (RTCMEM) as specified in ETSI TS 103 301 [15],
/// *  - 14 - `cpm`               - reserved for Collective Perception Message (CPM),
/// *  - 15 - `imzm`              - for Interference Management Zone Message (IMZM) as specified in ETSI TS 103 724 [13],
/// *  - 16 - `vam`               - for Vulnerable Road User Awareness Message as specified in ETSI TS 130 300-3 [12],
/// *  - 17 - `dsm`               - reserved for Diagnosis, logging and Status Message,
/// *  - 18 - `pcim`              - reserved for Parking Control Infrastructure Message,
/// *  - 19 - `pcvm`              - reserved for Parking Control Vehicle Message,
/// *  - 20 - `mcm`               - reserved for Manoeuvre Coordination Message,
/// *  - 21 - `pam`               - reserved for Parking Availability Message,
/// *  - 22-255                   - reserved for future usage.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1 from @ref ItsPduHeader. Value 3 re-assigned to poim and value 7 and 8 reserved in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct MessageId(pub u8);

///*
/// * This DE indicates a message rate.
/// *
/// * @field mantissa: indicates the mantissa.
/// *
/// * @field exponent: indicates the exponent.
/// *
/// * The specified message rate is: mantissa*(10^exponent)
/// *
/// * @unit: Hz
/// * @category: Communication information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MessageRateHz {
    #[rasn(value("1..=100"))]
    pub mantissa: u8,
    #[rasn(value("-5..=2"))]
    pub exponent: i8,
}

impl MessageRateHz {
    pub fn new(mantissa: u8, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }
}

///*
/// * This DF provides information about a message with respect to the segmentation process on facility layer at the sender.
/// *
/// * It shall include the following components:
/// *
/// * @field totalMsgNo: indicates the total number of messages that have been assembled on the transmitter side to encode the information
/// * during the same messsage generation process.
/// *
/// * @field thisMsgNo: indicates the position of the message within of the total set of messages generated during the same message generation process.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1, description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MessageSegmentationInfo {
    pub total_msg_no: CardinalNumber3b,
    pub this_msg_no: OrdinalNumber3b,
}

impl MessageSegmentationInfo {
    pub fn new(total_msg_no: CardinalNumber3b, this_msg_no: OrdinalNumber3b) -> Self {
        Self {
            total_msg_no,
            this_msg_no,
        }
    }
}

///*
/// * This DF provides information about the source of and confidence in information.
/// *
/// * It shall include the following components:
/// *
/// * @field usedDetectionInformation: the type of sensor(s) that is used to provide the detection information.
/// *
/// * @field usedStoredInformation: the type of source of the stored information.
/// *
/// * @field confidenceValue: an optional confidence value associated to the information.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MetaInformation {
    pub used_detection_information: SensorTypes,
    pub used_stored_information: StoredInformationType,
    pub confidence_value: Option<ConfidenceLevel>,
}

impl MetaInformation {
    pub fn new(
        used_detection_information: SensorTypes,
        used_stored_information: StoredInformationType,
        confidence_value: Option<ConfidenceLevel>,
    ) -> Self {
        Self {
            used_detection_information,
            used_stored_information,
            confidence_value,
        }
    }
}

///*
/// * This DF shall contain a list of @ref MitigationPerTechnologyClass.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct MitigationForTechnologies(pub SequenceOf<MitigationPerTechnologyClass>);

///*
/// * This DF represents a set of mitigation parameters for a specific technology, as specified in ETSI TS 103 724 [24], clause 7.
/// *
/// * It shall include the following components:
/// *
/// * @field accessTechnologyClass:  channel access technology to which this mitigation is intended to be applied.
/// *
/// * @field lowDutyCycle: duty cycle limit.
/// * @unit: 0,01 % steps
/// *
/// * @field powerReduction: the delta value of power to be reduced.
/// * @unit: dB
/// *
/// * @field dmcToffLimit: idle time limit as defined in ETSI TS 103 175 [19].
/// * @unit: ms
/// *
/// * @field dmcTonLimit: Transmission duration limit, as defined in ETSI EN 302 571 [20].
/// * @unit: ms
/// *
/// * @note: All parameters are optional, as they may not apply to some of the technologies or
/// * interference management zone types. Specification details are in ETSI TS 103 724 [24], clause 7.
/// *
/// * @category: Communication information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MitigationPerTechnologyClass {
    pub access_technology_class: AccessTechnologyClass,
    #[rasn(value("0..=10000"))]
    pub low_duty_cycle: Option<u16>,
    #[rasn(value("0..=30"))]
    pub power_reduction: Option<u8>,
    #[rasn(value("0..=1200"))]
    pub dmc_toff_limit: Option<u16>,
    #[rasn(value("0..=20"))]
    pub dmc_ton_limit: Option<u8>,
}

impl MitigationPerTechnologyClass {
    pub fn new(
        access_technology_class: AccessTechnologyClass,
        low_duty_cycle: Option<u16>,
        power_reduction: Option<u8>,
        dmc_toff_limit: Option<u16>,
        dmc_ton_limit: Option<u8>,
    ) -> Self {
        Self {
            access_technology_class,
            low_duty_cycle,
            power_reduction,
            dmc_toff_limit,
            dmc_ton_limit,
        }
    }
}

///*
/// * This DE represents the number of occupants in a vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 0` and `n < 126`) for the number n of occupants,
/// * - `126` for values equal to or higher than 125,
/// * - `127` if information is not available.
/// *
/// * @unit: 1 person
/// * @category: Vehicle information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct NumberOfOccupants(pub u8);

///*
/// * This DF indicates both the class and associated subclass that best describes an object.
/// *
/// * The following options are available:
/// *
/// * @field vehicleSubClass: the object is a road vehicle and the specific subclass is specified.
/// *
/// * @field vruSubClass: the object is a VRU and the specific subclass is specified.
/// *
/// * @field groupSubClass: the object is a VRU group or cluster and the cluster information is specified.
/// *
/// * @field otherSubClass: the object is of a different type than the above and the specific subclass is specified.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ObjectClass {
    #[rasn(value("0..=14"))]
    VehicleSubClass(TrafficParticipantType),
    VruSubClass(VruProfileAndSubprofile),
    #[rasn(value("0.."))]
    GroupSubClass(VruClusterInformation),
    OtherSubClass(OtherSubClass),
}

///*
/// * This DF shall contain a list of object classes.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct ObjectClassDescription(pub SequenceOf<ObjectClassWithConfidence>);

///*
/// * This DF represents the classification of a detected object together with a confidence level.
/// *
/// * It shall include the following components:
/// *
/// * @field objectClass: the class of the object.
/// *
/// * @field Confidence: the associated confidence level.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ObjectClassWithConfidence {
    pub object_class: ObjectClass,
    pub confidence: ConfidenceLevel,
}

impl ObjectClassWithConfidence {
    pub fn new(object_class: ObjectClass, confidence: ConfidenceLevel) -> Self {
        Self {
            object_class,
            confidence,
        }
    }
}

///*
/// * This DF represents a dimension of an object together with a confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field value: the object dimension value which can be estimated as the mean of the current distribution.
/// *
/// * @field confidence: the associated confidence value.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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

///*
/// * This DE indicates the object dimension confidence value which represents the estimated absolute accuracy of an object dimension value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 31`) if the confidence value is equal to or less than n x 0,1 metre, and more than (n-1) x 0,1 metre,
/// * - `31` if the confidence value is out of range i.e. greater than 3,0 m,
/// * - `32` if the confidence value is unavailable.
/// *
/// * @unit 0,1 m
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32"))]
pub struct ObjectDimensionConfidence(pub u8);

///*
/// * This DE represents a single dimension of an object.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 255`) if the  accuracy is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,
/// * - `255` if the accuracy is out of range i.e. greater than 25,4 m,
/// * - `256` if the data is unavailable.
/// *
/// * @unit 0,1 m
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=256"))]
pub struct ObjectDimensionValue(pub u16);

///*
/// * This DE indicates the face or part of a face of a solid object.
/// *
/// * The object is modelled  as a rectangular prism that has a length that is greater than its width, with the faces of the object being defined as:
/// * - front: the face defined by the prism's width and height, and which is the first face in direction of longitudinal movement of the object,
/// * - back: the face defined by the prism's width and height, and which is the last face in direction of longitudinal movement of the object,
/// * - side: the faces defined by the prism's length and height with "left" and "right" defined by looking at the front face and "front" and "back" defined w.r.t to the front and back faces.
/// *
/// * Note: It is permissible to derive the required object dimensions and orientation from models to provide a best guess.
/// *
/// * @category: Basic information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum ObjectFace {
    Front = 0,
    SideLeftFront = 1,
    SideLeftBack = 2,
    SideRightFront = 3,
    SideRightBack = 4,
    Back = 5,
}

///*
/// * This DE represents a single-value indication about the overall information quality of a perceived object.
/// *
/// * The value shall be set to:  
/// * - `0`                        : if there is no confidence in detected object, e.g. for "ghost"-objects or if confidence could not be computed,
/// * - `n` (`n > 0` and `n < 15`) : for the applicable confidence value,
/// * - `15`                       : if there is full confidence in the detected Object.
/// *
/// * @unit n/a
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct ObjectPerceptionQuality(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct OccupiedLanesWithConfidenceLanePositionBased(pub SequenceOf<LanePositionOptions>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct OccupiedLanesWithConfidenceMapBased(pub SequenceOf<MapPosition>);

///*
/// * This DF represents a set of lanes which are partially or fully occupied by an object or event at an externally defined reference position.
/// *
/// * @note: In contrast to @ref GeneralizedLanePosition, the dimension of the object or event area (width and length) is taken into account to determine the occupancy,
/// * i.e. this DF describes the lanes which are blocked by an object or event and not the position of the object / event itself. A confidence is used to describe the
/// * probability that exactly all the provided lanes are occupied.
/// *
/// * It shall include the following components:
/// *
/// * @field lanePositionBased: a set of up to `4` lanes that are partially or fully occupied by an object or event, ordered by increasing value of @ref LanePosition.
/// * Lanes that are partially occupied can be described using the component lanePositionWithLateralDetails of @ref  Options, with the following constraints:
/// * The distance to lane borders which are covered by the object / event shall be set to 0. Only the distances to the leftmost and/or rightmost border which are not covered by
/// * the object / event shall be provided with values > 0. Those values shall be added to the respective instances of @ref LanePositionOptions, i.e. the first entry shall contain the component distanceToLeftBorder > 0 ,
/// * and/or the last entry shall contain the component distanceToRightBorder > 0; the respective other components of these entries shall be set to 0.
/// *
/// * @field mapBased: optional lane information described in the context of a MAPEM as specified in ETSI TS 103 301 [15].
/// * If present, it shall describe the same lane(s) as listed in the component lanePositionBased, but using the lane identification of the MAPEM. This component can be used only if a
/// * MAPEM is available for the reference position (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased.
/// *
/// * @field confidence: mandatory confidence information for expressing the probability that all the provided lanes are occupied. It also provides information on how the lane
/// * information were generated. If none of the sensors were used, the lane information is assumed to be derived directly from the absolute reference position and the related dimension.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct OccupiedLanesWithConfidence {
    pub lane_position_based: OccupiedLanesWithConfidenceLanePositionBased,
    pub map_based: Option<OccupiedLanesWithConfidenceMapBased>,
    pub confidence: MetaInformation,
}

impl OccupiedLanesWithConfidence {
    pub fn new(
        lane_position_based: OccupiedLanesWithConfidenceLanePositionBased,
        map_based: Option<OccupiedLanesWithConfidenceMapBased>,
        confidence: MetaInformation,
    ) -> Self {
        Self {
            lane_position_based,
            map_based,
            confidence,
        }
    }
}

///*
/// * This DE represents a time period to describe the opening days and hours of a Point of Interest.
/// * (for example local commerce).
/// *
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct OpeningDaysHours(pub Utf8String);

///*
/// * The DE represents an ordinal number that indicates the position of an element in a set.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct OrdinalNumber1B(pub u8);

///*
/// * The DE represents an ordinal number that indicates the position of an element in a set.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=8"))]
pub struct OrdinalNumber3b(pub u8);

///*
/// * This DE indicates the subclass of a detected object for @ref ObjectClass "otherSubclass".
/// *
/// * The value shall be set to:
/// * - `0` - unknown          - if the subclass is unknown.
/// * - `1` - singleObject     - if the object is a single object.
/// * - `2` - multipleObjects  - if the object is a group of multiple objects.
/// * - `3` - bulkMaterial     - if the object is a bulk material.
/// *
/// * @category: Sensing information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct OtherSubClass(pub u8);

///*
/// * This DF represents a path with a set of path points.
/// * It shall contain up to `40` @ref PathPoint.
/// *
/// * The first PathPoint presents an offset delta position with regards to an external reference position.
/// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint.
/// *
/// * @category: GeoReference information, Vehicle information
/// * @revision: created in V2.1.1 based on PathHistory
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=40"))]
pub struct Path(pub SequenceOf<PathPoint>);

///*
/// * This DE represents the recorded or estimated travel time between a position and a predefined reference position.
/// *
/// * @unit 0,01 second
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=65535", extensible))]
pub struct PathDeltaTime(pub Integer);

///*
/// * This DF represents estimated/predicted travel time between a position and a predefined reference position.
/// *
/// * the following options are available:
/// *
/// * @field deltaTimeHighPrecision: delta time with precision of 0,1 s.
/// *
/// * @field deltaTimeBigRange: delta time with precision of 10 s.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PathDeltaTimeChoice {
    DeltaTimeHighPrecision(DeltaTimeTenthOfSecond),
    DeltaTimeBigRange(DeltaTimeTenSeconds),
}

///*
/// * This DF represents a path towards a specific point specified in the @ref EventZone.
/// *
/// * It shall include the following components:
/// *
/// * @field pointOfEventZone: the ordinal number of the point within the DF EventZone, i.e. within the list of EventPoints.
/// *
/// * @field path: the associated path towards the point specified in pointOfEventZone.
/// * The first PathPoint presents an offset delta position with regards to the position of that pointOfEventZone.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PathExtended {
    #[rasn(value("1..=23"))]
    pub point_of_event_zone: u8,
    pub path: Path,
}

impl PathExtended {
    pub fn new(point_of_event_zone: u8, path: Path) -> Self {
        Self {
            point_of_event_zone,
            path,
        }
    }
}

///*
/// * This DF represents a path history with a set of path points.
/// * It shall contain up to `40` @ref PathPoint.
/// *
/// * The first PathPoint presents an offset delta position with regards to an external reference position.
/// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref Path instead.
/// * @category: GeoReference information, Vehicle information
/// * @revision: semantics updated in V2.1.1, size corrected to 0..40 in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=40"))]
pub struct PathHistory(pub SequenceOf<PathPoint>);

///*
/// * This DE indicates an ordinal number that represents the position of a component in the list of @ref Traces or @ref TracesExtended.
/// *
/// * The value shall be set to:
/// * - `0` - noPath  - if no path is identified
/// * - `1..7`        - for instances 1..7 of @ref Traces
/// * - `8..14`       - for instances 1..7 of @ref TracesExtended.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=14"))]
pub struct PathId(pub u8);

///*
/// * This DF defines an offset waypoint position within a path.
/// *
/// * It shall include the following components:
/// *
/// * @field pathPosition: The waypoint position defined as an offset position with regards to a pre-defined reference position.
/// *
/// * @field pathDeltaTime: The optional travel time separated from a waypoint to the predefined reference position.
/// *
/// * @category GeoReference information
/// * @revision: semantics updated in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PathPoint {
    pub path_position: DeltaReferencePosition,
    pub path_delta_time: Option<PathDeltaTime>,
}

impl PathPoint {
    pub fn new(
        path_position: DeltaReferencePosition,
        path_delta_time: Option<PathDeltaTime>,
    ) -> Self {
        Self {
            path_position,
            path_delta_time,
        }
    }
}

///*
/// * This DF defines a predicted offset position that can be used within a predicted path or trajectory, together with optional data to describe a path zone shape.
/// *
/// * It shall include the following components:
/// *
/// * @field deltaLatitude: the offset latitude with regards to a pre-defined reference position.
/// *
/// * @field deltaLongitude: the offset longitude with regards to a pre-defined reference position.
/// *
/// * @field horizontalPositionConfidence: the optional confidence value associated to the horizontal geographical position.
/// *
/// * @field deltaAltitude: the optional offset altitude with regards to a pre-defined reference position, with default value unavailable.
/// *
/// * @field altitudeConfidence: the optional confidence value associated to the altitude value of the geographical position, with default value unavailable.
/// *
/// * @field pathDeltaTime: the optional travel time to the waypoint from the predefined reference position.
///
/// * @field symmetricAreaOffset: the optional symmetric offset to generate a shape, see Annex D for details.
/// *  
/// * @field asymmetricAreaOffset: the optional asymmetric offset to generate a shape, see Annex D for details.
/// *
/// * @category GeoReference information
/// * @revision: Created in V2.1.1, type of pathDeltaTime changed and optionality added, fields symmetricAreaOffset and asymmetricAreaOffset added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PathPointPredicted {
    pub delta_latitude: DeltaLatitude,
    pub delta_longitude: DeltaLongitude,
    pub horizontal_position_confidence: Option<PosConfidenceEllipse>,
    #[rasn(default = "path_point_predicted_delta_altitude_default")]
    pub delta_altitude: DeltaAltitude,
    #[rasn(default = "path_point_predicted_altitude_confidence_default")]
    pub altitude_confidence: AltitudeConfidence,
    pub path_delta_time: Option<PathDeltaTimeChoice>,
    pub symmetric_area_offset: Option<StandardLength9b>,
    pub asymmetric_area_offset: Option<StandardLength9b>,
}

impl PathPointPredicted {
    pub fn new(
        delta_latitude: DeltaLatitude,
        delta_longitude: DeltaLongitude,
        horizontal_position_confidence: Option<PosConfidenceEllipse>,
        delta_altitude: DeltaAltitude,
        altitude_confidence: AltitudeConfidence,
        path_delta_time: Option<PathDeltaTimeChoice>,
        symmetric_area_offset: Option<StandardLength9b>,
        asymmetric_area_offset: Option<StandardLength9b>,
    ) -> Self {
        Self {
            delta_latitude,
            delta_longitude,
            horizontal_position_confidence,
            delta_altitude,
            altitude_confidence,
            path_delta_time,
            symmetric_area_offset,
            asymmetric_area_offset,
        }
    }
}

fn path_point_predicted_delta_altitude_default() -> DeltaAltitude {
    DeltaAltitude(12800).into()
}

fn path_point_predicted_altitude_confidence_default() -> AltitudeConfidence {
    AltitudeConfidence::Unavailable.into()
}

///*
/// * This DF represents a predicted path or trajectory with a set of predicted points and optional information to generate a shape which is estimated to contain the real path.
/// * It shall contain up to `16` @ref PathPointPredicted.
/// *
/// * The first PathPoint presents an offset delta position with regards to an external reference position.
/// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint.
/// *
/// * @category: GeoReference information
/// * @revision: created in V2.1.1 , size constraint changed to SIZE(1..16, ...) in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct PathPredicted(pub SequenceOf<PathPointPredicted>);

///*
/// * This DF represents a predicted path, predicted trajectory or predicted path zone together with usage information and a prediction confidence.
/// *
/// * It shall include the following components:
/// *
/// * @field pathPredicted: the predicted path (pathDeltaTime ABSENT) or trajectory (pathDeltaTime PRESENT) and/or the path zone (symmetricAreaOffset PRESENT).
/// *
/// * @field usageIndication: an indication of how the predicted path will be used.
/// *
/// * @field confidenceLevel: the confidence that the path/trajectory in pathPredicted will occur as predicted.
/// *
/// * @category: GeoReference information
/// * @revision: created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PathPredicted2 {
    #[rasn(value("0.."))]
    pub path_predicted: PathPredicted,
    pub usage_indication: UsageIndication,
    pub confidence_level: ConfidenceLevel,
}

impl PathPredicted2 {
    pub fn new(
        path_predicted: PathPredicted,
        usage_indication: UsageIndication,
        confidence_level: ConfidenceLevel,
    ) -> Self {
        Self {
            path_predicted,
            usage_indication,
            confidence_level,
        }
    }
}

///*
/// * This DF represents one or more predicted paths, or trajectories or path zones (zones that include all possible paths/trajectories within its boundaries) using @ref PathPredicted2.
/// * It shall contain up to `16` @ref PathPredicted2.
/// *
/// * @category: GeoReference information
/// * @revision: V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct PathPredictedList(pub SequenceOf<PathPredicted2>);

///*
/// * This DF represents a list of references to the components of a @ref Traces or @ref TracesExtended DF using the @ref PathId.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=14"))]
pub struct PathReferences(pub SequenceOf<PathId>);

///*
/// * This DF contains information about a perceived object including its kinematic state and attitude vector in a pre-defined coordinate system and with respect to a reference time.
/// *
/// * It shall include the following components:
/// *
/// * @field objectId: optional identifier assigned to a detected object.
/// *
/// * @field measurementDeltaTime: the time difference from a reference time to the time of the  measurement of the object.
/// * Negative values indicate that the provided object state refers to a point in time before the reference time.
/// *
/// * @field position: the position of the geometric centre of the object's bounding box within the pre-defined coordinate system.
/// *
/// * @field velocity: the velocity vector of the object within the pre-defined coordinate system.
/// *
/// * @field acceleration: the acceleration vector of the object within the pre-defined coordinate system.
/// *
/// * @field angles: optional Euler angles of the object bounding box at the time of measurement.
/// *
/// * @field zAngularVelocity: optional angular velocity of the object around the z-axis at the time of measurement.
/// * The angular velocity is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule.
/// *
/// * @field lowerTriangularCorrelationMatrices: optional set of lower triangular correlation matrices for selected components of the provided kinematic state and attitude vector.
/// *
/// * @field objectDimensionZ: optional z-dimension of object bounding box.
/// * This dimension shall be measured along the direction of the z-axis after all the rotations have been applied.
/// *
/// * @field objectDimensionY: optional y-dimension of the object bounding box.
/// * This dimension shall be measured along the direction of the y-axis after all the rotations have been applied.
/// *
/// * @field objectDimensionX: optional x-dimension of object bounding box.
/// * This dimension shall be measured along the direction of the x-axis after all the rotations have been applied.
/// *
/// * @field objectAge: optional age of the detected and described object, i.e. the difference in time between the moment
/// * it has been first detected and the reference time of the message. Value `1500` indicates that the object has been observed for more than 1.5s.
/// *
/// * @field objectPerceptionQuality: optional confidence associated to the object.
/// *
/// * @field sensorIdList: optional list of sensor-IDs which provided the measurement data.
/// *
/// * @field classification: optional classification of the described object
/// *
/// * @field matchedPosition: optional map-matched position of an object.
/// *
/// * @category Sensing information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PerceivedObject {
    pub object_id: Option<Identifier2B>,
    pub measurement_delta_time: DeltaTimeMilliSecondSigned,
    pub position: CartesianPosition3dWithConfidence,
    pub velocity: Option<Velocity3dWithConfidence>,
    pub acceleration: Option<Acceleration3dWithConfidence>,
    pub angles: Option<EulerAnglesWithConfidence>,
    pub z_angular_velocity: Option<CartesianAngularVelocityComponent>,
    pub lower_triangular_correlation_matrices: Option<LowerTriangularPositiveSemidefiniteMatrices>,
    pub object_dimension_z: Option<ObjectDimension>,
    pub object_dimension_y: Option<ObjectDimension>,
    pub object_dimension_x: Option<ObjectDimension>,
    #[rasn(value("0..=2047"))]
    pub object_age: Option<DeltaTimeMilliSecondSigned>,
    pub object_perception_quality: Option<ObjectPerceptionQuality>,
    pub sensor_id_list: Option<SequenceOfIdentifier1B>,
    pub classification: Option<ObjectClassDescription>,
    pub map_position: Option<MapPosition>,
}

impl PerceivedObject {
    pub fn new(
        object_id: Option<Identifier2B>,
        measurement_delta_time: DeltaTimeMilliSecondSigned,
        position: CartesianPosition3dWithConfidence,
        velocity: Option<Velocity3dWithConfidence>,
        acceleration: Option<Acceleration3dWithConfidence>,
        angles: Option<EulerAnglesWithConfidence>,
        z_angular_velocity: Option<CartesianAngularVelocityComponent>,
        lower_triangular_correlation_matrices: Option<LowerTriangularPositiveSemidefiniteMatrices>,
        object_dimension_z: Option<ObjectDimension>,
        object_dimension_y: Option<ObjectDimension>,
        object_dimension_x: Option<ObjectDimension>,
        object_age: Option<DeltaTimeMilliSecondSigned>,
        object_perception_quality: Option<ObjectPerceptionQuality>,
        sensor_id_list: Option<SequenceOfIdentifier1B>,
        classification: Option<ObjectClassDescription>,
        map_position: Option<MapPosition>,
    ) -> Self {
        Self {
            object_id,
            measurement_delta_time,
            position,
            velocity,
            acceleration,
            angles,
            z_angular_velocity,
            lower_triangular_correlation_matrices,
            object_dimension_z,
            object_dimension_y,
            object_dimension_x,
            object_age,
            object_perception_quality,
            sensor_id_list,
            classification,
            map_position,
        }
    }
}

///*
/// * This DE denotes the ability of an ITS-S to provide up-to-date information.
/// * A performance class value is used to describe age of data. The exact values are out of scope of the present document.
/// *
/// *  The value shall be set to:
/// * - `0` if  the performance class is unknown,
/// * - `1` for performance class A as defined in ETSI TS 101 539-1 [5],
/// * - `2` for performance class B as defined in ETSI TS 101 539-1 [5],
/// * -  3-7 reserved for future use.
/// *
/// * @category: Vehicle information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct PerformanceClass(pub u8);

///*
/// * This DE represents a telephone number
/// *
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct PhoneNumber(pub NumericString);

///*
/// * This DF represents the shape of a polygonal area or of a right prism.
/// *
/// * It shall include the following components:
/// *
/// * @field shapeReferencePoint: the optional reference point used for the definition of the shape, relative to an externally specified reference position.
/// * If this component is absent, the externally specified reference position represents the shape's reference point.
/// *
/// * @field polygon: the polygonal area represented by a list of minimum `3` to maximum `16` @ref CartesianPosition3d.
/// * All nodes of the polygon shall be considered relative to the shape's reference point.
/// *
/// * @field height: the optional height, present if the shape is a right prism extending in the positive z-axis.
/// *
/// * @category GeoReference information
/// * @revision: created in V2.1.1
/// *
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PolygonalShape {
    pub shape_reference_point: Option<CartesianPosition3d>,
    #[rasn(size("3..=16", extensible))]
    pub polygon: SequenceOfCartesianPosition3d,
    pub height: Option<StandardLength12b>,
}

impl PolygonalShape {
    pub fn new(
        shape_reference_point: Option<CartesianPosition3d>,
        polygon: SequenceOfCartesianPosition3d,
        height: Option<StandardLength12b>,
    ) -> Self {
        Self {
            shape_reference_point,
            polygon,
            height,
        }
    }
}

///*
/// * This DE indicates the perpendicular distance from the centre of mass of an empty load vehicle to the front line of
/// * the vehicle bounding box of the empty load vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 62`) for any aplicable value n between 0,1 metre and 6,2 metres,
/// * - `62` for values equal to or higher than 6.1 metres,
/// * - `63`  if the information is unavailable.
/// *
/// * @note:	The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// *
/// * @unit 0,1 metre
/// * @category Vehicle information
/// * @revision: description revised in V2.1.1 (the meaning of 62 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=63"))]
pub struct PosCentMass(pub u8);

///*
/// * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a
/// * confidence level of 95  %. The centre of the ellipse shape corresponds to the reference
/// * position point for which the position accuracy is evaluated.
/// *
/// * It shall include the following components:
/// *
/// * @field semiMajorConfidence: half of length of the major axis, i.e. distance between the centre point
/// * and major axis point of the position accuracy ellipse.
/// *
/// * @field semiMinorConfidence: half of length of the minor axis, i.e. distance between the centre point
/// * and minor axis point of the position accuracy ellipse.
/// *
/// * @field semiMajorOrientation: orientation direction of the ellipse major axis of the position accuracy
/// * ellipse with regards to the WGS84 north.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// *
/// * @category GeoReference information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PosConfidenceEllipse {
    pub semi_major_confidence: SemiAxisLength,
    pub semi_minor_confidence: SemiAxisLength,
    pub semi_major_orientation: HeadingValue,
}

impl PosConfidenceEllipse {
    pub fn new(
        semi_major_confidence: SemiAxisLength,
        semi_minor_confidence: SemiAxisLength,
        semi_major_orientation: HeadingValue,
    ) -> Self {
        Self {
            semi_major_confidence,
            semi_minor_confidence,
            semi_major_orientation,
        }
    }
}

///*
/// * This DE indicates the perpendicular distance between the vehicle front line of the bounding box and the front wheel axle in 0,1 metre.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 19`) for any aplicable value between 0,1 metre and 1,9 metres,
/// * - `19` for values equal to or higher than 1.8 metres,
/// * - `20` if the information is unavailable.
/// *
/// * @category: Vehicle information
/// * @unit 0,1 metre
/// * @revision: description revised in V2.1.1 (the meaning of 19 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=20"))]
pub struct PosFrontAx(pub u8);

///*
/// * This DE represents the distance from the centre of vehicle front bumper to the right or left longitudinal carrier of vehicle.
/// * The left/right carrier refers to the left/right as seen from a passenger sitting in the vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 126`) for any aplicable value between 0,01 metre and 1,26 metres,
/// * - `126` for values equal to or higher than 1.25 metres,
/// * - `127` if the information is unavailable.
/// *
/// * @unit 0,01 metre
/// * @category Vehicle information
/// * @revision: description revised in V2.1.1 (the meaning of 126 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct PosLonCarr(pub u8);

///*
/// * This DE represents the perpendicular inter-distance of neighbouring pillar axis of vehicle starting from the
/// * middle point of the front line of the vehicle bounding box.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 29`) for any aplicable value between 0,1 metre and 2,9 metres,
/// * - `29` for values equal to or greater than 2.8 metres,
/// * - `30` if the information is unavailable.
/// *
/// * @unit 0,1 metre
/// * @category Vehicle information
/// * @revision: description revised in V2.1.1 (the meaning of 29 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=30"))]
pub struct PosPillar(pub u8);

///*
/// * This DE represents a position along a single dimension such as the middle of a road or lane, measured as an offset from an externally defined starting point,
/// * in direction of an externally defined reference direction.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= -8190` and `n < 0`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in opposite direction of the reference direction,
/// * - `0` if the position is at the starting point,
/// * - `n` (`n > 0` and `n < 8190`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in the same direction as the reference direction,
/// * - `8 190` if the position is out of range, i.e. equal to or greater than 8 189 m,
/// * - `8 191` if the position information is not available.
/// *
/// * @unit 1 metre
/// * @category: GeoReference information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-8190..=8191"))]
pub struct Position1d(pub i16);

///*
/// * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a
/// * confidence level of 95 %. The centre of the ellipse shape corresponds to the reference
/// * position point for which the position accuracy is evaluated.
/// *
/// * It shall include the following components:
/// *
/// * @field semiMajorAxisLength: half of length of the major axis, i.e. distance between the centre point
/// * and major axis point of the position accuracy ellipse.
/// *
/// * @field semiMinorAxisLength: half of length of the minor axis, i.e. distance between the centre point
/// * and minor axis point of the position accuracy ellipse.
/// *
/// * @field semiMajorAxisOrientation: orientation direction of the ellipse major axis of the position accuracy
/// * ellipse with regards to the WGS84 north.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * @category GeoReference information
/// * @revision: created in V2.1.1 based on @ref PosConfidenceEllipse
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PositionConfidenceEllipse {
    pub semi_major_axis_length: SemiAxisLength,
    pub semi_minor_axis_length: SemiAxisLength,
    pub semi_major_axis_orientation: Wgs84AngleValue,
}

impl PositionConfidenceEllipse {
    pub fn new(
        semi_major_axis_length: SemiAxisLength,
        semi_minor_axis_length: SemiAxisLength,
        semi_major_axis_orientation: Wgs84AngleValue,
    ) -> Self {
        Self {
            semi_major_axis_length,
            semi_minor_axis_length,
            semi_major_axis_orientation,
        }
    }
}

///*
/// * This DE indicates whether a passenger seat is occupied or whether the occupation status is detectable or not.
/// *
/// * The number of row in vehicle seats layout is counted in rows from the driver row backwards from front to the rear
/// * of the vehicle.
/// * The left side seat of a row refers to the left hand side seen from vehicle rear to front.
/// * Additionally, a bit is reserved for each seat row, to indicate if the seat occupation of a row is detectable or not,
/// * i.e. `row1NotDetectable (3)`, `row2NotDetectable(8)`, `row3NotDetectable(13)` and `row4NotDetectable(18)`.
/// * Finally, a bit is reserved for each row seat to indicate if the seat row is present or not in the vehicle,
/// * i.e. `row1NotPresent (4)`, `row2NotPresent (9)`, `row3NotPresent(14)`, `row4NotPresent(19)`.
/// *
/// * When a seat is detected to be occupied, the corresponding seat occupation bit shall be set to `1`.
/// * For example, when the row 1 left seat is occupied, `row1LeftOccupied(0)` bit shall be set to `1`.
/// * When a seat is detected to be not occupied, the corresponding seat occupation bit shall be set to `0`.
/// * Otherwise, the value of seat occupation bit shall be set according to the following conditions:
/// * - If the seat occupation of a seat row is not detectable, the corresponding bit shall be set to `1`.
/// *   When any seat row not detectable bit is set to `1`, all corresponding seat occupation bits of the same row
/// *   shall be set to `1`.
/// * - If the seat row is not present, the corresponding not present bit of the same row shall be set to `1`.
/// *   When any of the seat row not present bit is set to `1`, the corresponding not detectable bit for that row
/// *   shall be set to `1`, and all the corresponding seat occupation bits in that row shall be set to `0`.
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("20"))]
pub struct PositionOfOccupants(pub BitString);

///*
/// * This DF shall contain a list of distances @ref PosPillar that refer to the perpendicular distance between centre of vehicle front bumper
/// * and vehicle pillar A, between neighbour pillars until the last pillar of the vehicle.
/// *
/// * Vehicle pillars refer to the vertical or near vertical support of vehicle,
/// * designated respectively as the A, B, C or D and other pillars moving in side profile view from the front to rear.
/// *
/// * The first value of the DF refers to the perpendicular distance from the centre of vehicle front bumper to
/// * vehicle A pillar. The second value refers to the perpendicular distance from the centre position of A pillar
/// * to the B pillar of vehicle and so on until the last pillar.
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct PositionOfPillars(pub SequenceOf<PosPillar>);

///*
/// * This DE indicates the positioning technology being used to estimate a geographical position.
/// *
/// * The value shall be set to:
/// * - 0 `noPositioningSolution`  - no positioning solution used,
/// * - 1 `sGNSS`                  - Global Navigation Satellite System used,
/// * - 2 `dGNSS`                  - Differential GNSS used,
/// * - 3 `sGNSSplusDR`            - GNSS and dead reckoning used,
/// * - 4 `dGNSSplusDR`            - Differential GNSS and dead reckoning used,
/// * - 5 `dR`                     - dead reckoning used,
/// * - 6 `manuallyByOperator`     - position set manually by a human operator.
/// *
/// * @category: GeoReference information
/// * @revision: V1.3.1, extension with value 6 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PositioningSolutionType {
    NoPositioningSolution = 0,
    SGNSS = 1,
    DGNSS = 2,
    SGNSSplusDR = 3,
    DGNSSplusDR = 4,
    DR = 5,
    #[rasn(extension_addition)]
    ManuallyByOperator = 6,
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `postCrash` .
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`                                               - in case further detailed information on post crash event is unavailable,
/// * - 1 `accidentWithoutECallTriggered`                             - in case no eCall has been triggered for an accident,
/// * - 2 `accidentWithECallManuallyTriggered`                        - in case eCall has been manually triggered and transmitted to eCall back end,
/// * - 3 `accidentWithECallAutomaticallyTriggered`                   - in case eCall has been automatically triggered and transmitted to eCall back end,
/// * - 4 `accidentWithECallTriggeredWithoutAccessToCellularNetwork`  - in case eCall has been triggered but cellular network is not accessible from triggering vehicle.
/// * - 5-255                                                         - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PostCrashSubCauseCode(pub u8);

///*
///* This DE represent the total amount of rain falling during one hour. It is measured in mm per hour at an area of 1 square metre.  
///*
///* The following values are specified:
///* - `n` (`n > 0` and `n < 2000`) if the amount of rain falling is equal to or less than n x 0,1 mm/h and greater than (n-1) x 0,1 mm/h,
///* - `2000` if the amount of rain falling is greater than 199.9 mm/h,
///* - `2001` if the information is not available.
///*
///* @unit: 0,1 mm/h
///* @category: Basic Information
///* @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=2001"))]
pub struct PrecipitationIntensity(pub u16);

///*
/// * This DF describes a zone of protection inside which the ITS communication should be restricted.
/// *
/// * It shall include the following components:
/// *
/// * @field protectedZoneType: type of the protected zone.
/// *
/// * @field expiryTime: optional time at which the validity of the protected communication zone will expire.
/// *
/// * @field protectedZoneLatitude: latitude of the centre point of the protected communication zone.
/// *
/// * @field protectedZoneLongitude: longitude of the centre point of the protected communication zone.
/// *
/// * @field protectedZoneRadius: optional radius of the protected communication zone in metres.
/// *
/// * @field protectedZoneId: the optional ID of the protected communication zone.
/// *
/// * @note: A protected communication zone may be defined around a CEN DSRC road side equipment.
/// *
/// * @category: Infrastructure information, Communication information
/// * @revision: revised in V2.1.1 (changed protectedZoneID to protectedZoneId)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ProtectedCommunicationZone {
    pub protected_zone_type: ProtectedZoneType,
    pub expiry_time: Option<TimestampIts>,
    pub protected_zone_latitude: Latitude,
    pub protected_zone_longitude: Longitude,
    pub protected_zone_radius: Option<ProtectedZoneRadius>,
    pub protected_zone_id: Option<ProtectedZoneId>,
}

impl ProtectedCommunicationZone {
    pub fn new(
        protected_zone_type: ProtectedZoneType,
        expiry_time: Option<TimestampIts>,
        protected_zone_latitude: Latitude,
        protected_zone_longitude: Longitude,
        protected_zone_radius: Option<ProtectedZoneRadius>,
        protected_zone_id: Option<ProtectedZoneId>,
    ) -> Self {
        Self {
            protected_zone_type,
            expiry_time,
            protected_zone_latitude,
            protected_zone_longitude,
            protected_zone_radius,
            protected_zone_id,
        }
    }
}

///*
/// * This DF shall contain a list of @ref ProtectedCommunicationZone provided by a road side ITS-S (Road Side Unit RSU).
/// *
/// * It may provide up to 16 protected communication zones information.
/// *
/// * @category: Infrastructure information, Communication information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);

///*
/// * This DE represents the indentifier of a protected communication zone.
/// *
/// *
/// * @category: Infrastructure information, Communication information
/// * @revision: Revision in V2.1.1 (changed name from ProtectedZoneID to ProtectedZoneId)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=134217727"))]
pub struct ProtectedZoneId(pub u32);

///*
/// * This DE represents the radius of a protected communication zone.
/// *
/// *
/// * @unit: metre
/// * @category: Infrastructure information, Communication information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255", extensible))]
pub struct ProtectedZoneRadius(pub Integer);

///*
/// * This DE indicates the type of a protected communication zone, so that an ITS-S is aware of the actions to do
/// * while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station).
/// *
/// * The protected zone type is defined in ETSI TS 102 792 [14].
/// *
/// *
/// * @category: Communication information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ProtectedZoneType {
    PermanentCenDsrcTolling = 0,
    #[rasn(extension_addition)]
    TemporaryCenDsrcTolling = 1,
}

///*
/// * This DF identifies an organization.
/// *
/// * It shall include the following components:
/// *
/// * @field countryCode: represents the country code that identifies the country of the national registration administrator for issuers according to ISO 14816.
/// *
/// * @field providerIdentifier: identifies the organization according to the national ISO 14816 register for issuers.
/// *
/// * @note: See https://www.itsstandards.eu/registries/register-of-nra-i-cs1/ for a list of national registration administrators and their respective registers
/// *
/// * @category: Communication information
/// * @revision: Created in V2.2.1 based on ISO 17573-3 [24]
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Provider {
    pub country_code: CountryCode,
    pub provider_identifier: IssuerIdentifier,
}

impl Provider {
    pub fn new(country_code: CountryCode, provider_identifier: IssuerIdentifier) -> Self {
        Self {
            country_code,
            provider_identifier,
        }
    }
}

///*
/// * This DF represents activation data for real-time systems designed for operations control, traffic light priorities, track switches, barriers, etc.
/// * using a range of activation devices equipped in public transport vehicles.
/// *
/// * The activation of the corresponding equipment is triggered by the approach or passage of a public transport
/// * vehicle at a certain point (e.g. a beacon).
/// *
/// * @field ptActivationType: type of activation.
/// *
/// * @field ptActicationData: data of activation.
/// *
/// * Today there are different payload variants defined for public transport activation-data. The R09.x is one of
/// * the industry standard used by public transport vehicles (e.g. buses, trams) in Europe (e.g. Germany Austria)
/// * for controlling traffic lights, barriers, bollards, etc. This DF shall include information like route, course,
/// * destination, priority, etc.
/// *
/// * The R09.x content is defined in VDV recommendation 420 [7]. It includes following information:
/// * - Priority Request Information (pre-request, request, ready to start)
/// * - End of Prioritization procedure
/// * - Priority request direction
/// * - Public Transport line number
/// * - Priority of public transport
/// * - Route line identifier of the public transport
/// * - Route number identification
/// * - Destination of public transport vehicle
/// *
/// * Other countries may use different message sets defined by the local administration.
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PtActivation {
    pub pt_activation_type: PtActivationType,
    pub pt_activation_data: PtActivationData,
}

impl PtActivation {
    pub fn new(pt_activation_type: PtActivationType, pt_activation_data: PtActivationData) -> Self {
        Self {
            pt_activation_type,
            pt_activation_data,
        }
    }
}

///*
/// * This DE is used for various tasks in the public transportation environment, especially for controlling traffic
/// * signal systems to prioritize and speed up public transportation in urban area (e.g. intersection "_bottlenecks_").
/// * The traffic lights may be controlled by an approaching bus or tram automatically. This permits "_In Time_" activation
/// * of the green phase, will enable the individual traffic to clear a potential traffic jam in advance. Thereby the
/// * approaching bus or tram may pass an intersection with activated green light without slowing down the speed due to
/// * traffic congestion. Other usage of the DE is the provision of information like the public transport line number
/// * or the schedule delay of a public transport vehicle.
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=20"))]
pub struct PtActivationData(pub OctetString);

///*
/// * This DE indicates a certain coding type of the PtActivationData data.
/// *
/// * The folowing value are specified:
/// * - 0 `undefinedCodingType`  : undefined coding type,
/// * - 1 `r09-16CodingType`     : coding of PtActivationData conform to VDV recommendation 420 [7],
/// * - 2 `vdv-50149CodingType`  : coding of PtActivationData based on VDV recommendation 420 [7].
/// * - 3 - 255                  : reserved for alternative and future use.
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PtActivationType(pub u8);

///*
/// * This DF describes a radial shape. The circular or spherical sector is constructed by sweeping      
/// * the provided range about the reference position specified outside of the context of this DF or
/// * about the optional shapeReferencePoint. The range is swept between a horizontal start and a
/// * horizontal end angle in the X-Y plane of a cartesian coordinate system specified outside of the
/// * context of this DF, in a right-hand positive angular direction w.r.t. the x-axis.
/// * A vertical opening angle in the X-Z plane may optionally be provided in a right-hand positive
/// * angular direction w.r.t. the x-axis.
/// *
/// * It shall include the following components:
/// *
/// * @field shapeReferencePoint: the optional reference point used for the definition of the shape,
/// * relative to an externally specified reference position. If this component is absent, the  
/// * externally specified reference position represents the shape's reference point.
/// *
/// * @field range: the radial range of the shape from the shape's reference point.
/// *
/// * @field horizontalOpeningAngleStart: the start of the shape's horizontal opening angle.
/// *
/// * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle.
/// *
/// * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle.
/// *
/// * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle.
/// *
/// * @category GeoReference information
/// * @revision: created in V2.1.1, names and types of the horizontal opening angles changed, constraint added and description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RadialShape {
    pub shape_reference_point: Option<CartesianPosition3d>,
    pub range: StandardLength12b,
    pub horizontal_opening_angle_start: CartesianAngleValue,
    pub horizontal_opening_angle_end: CartesianAngleValue,
    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
}

impl RadialShape {
    pub fn new(
        shape_reference_point: Option<CartesianPosition3d>,
        range: StandardLength12b,
        horizontal_opening_angle_start: CartesianAngleValue,
        horizontal_opening_angle_end: CartesianAngleValue,
        vertical_opening_angle_start: Option<CartesianAngleValue>,
        vertical_opening_angle_end: Option<CartesianAngleValue>,
    ) -> Self {
        Self {
            shape_reference_point,
            range,
            horizontal_opening_angle_start,
            horizontal_opening_angle_end,
            vertical_opening_angle_start,
            vertical_opening_angle_end,
        }
    }
}

///*
/// * This DF describes radial shape details. The circular sector or cone is
/// * constructed by sweeping the provided range about the position specified outside of the  
/// * context of this DF. The range is swept between a horizontal start and a horizontal end angle in
/// * the X-Y plane of a right-hand cartesian coordinate system specified outside of the context of
/// * this DF, in positive angular direction w.r.t. the x-axis. A vertical opening angle in the X-Z
/// * plane may optionally be provided in positive angular direction w.r.t. the x-axis.
/// *
/// * It shall include the following components:
/// *
/// * @field range: the radial range of the sensor from the reference point or sensor point offset.
/// *
/// * @field horizontalOpeningAngleStart:  the start of the shape's horizontal opening angle.
/// *
/// * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle.
/// *
/// * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle.
/// *
/// * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle.
/// *
/// * @category: Georeference information
/// * @revision: created in V2.1.1, description revised and constraint added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RadialShapeDetails {
    pub range: StandardLength12b,
    pub horizontal_opening_angle_start: CartesianAngleValue,
    pub horizontal_opening_angle_end: CartesianAngleValue,
    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
}

impl RadialShapeDetails {
    pub fn new(
        range: StandardLength12b,
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

///*
/// * This DF describes a list of radial shapes positioned w.r.t. to an offset position defined  
/// * relative to a reference position specified outside of the context of this DF and oriented w.r.t.  
/// * a cartesian coordinate system specified outside of the context of this DF.
/// *
/// * It shall include the following components:
/// *
/// * @field refPointId: the identification of the reference point in case of a sensor mounted to trailer. Defaults to ITS ReferencePoint (0).
/// *
/// * @field xCoordinate: the x-coordinate of the offset position.
/// *
/// * @field yCoordinate: the y-coordinate of the offset position.
/// *
/// * @field zCoordinate: the optional z-coordinate of the offset position.
/// *
/// * @field radialShapesList: the list of radial shape details.
/// *
/// * @category: Georeference information
/// * @revision: created in V2.1.1, description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RadialShapes {
    pub ref_point_id: Identifier1B,
    pub x_coordinate: CartesianCoordinateSmall,
    pub y_coordinate: CartesianCoordinateSmall,
    pub z_coordinate: Option<CartesianCoordinateSmall>,
    pub radial_shapes_list: RadialShapesList,
}

impl RadialShapes {
    pub fn new(
        ref_point_id: Identifier1B,
        x_coordinate: CartesianCoordinateSmall,
        y_coordinate: CartesianCoordinateSmall,
        z_coordinate: Option<CartesianCoordinateSmall>,
        radial_shapes_list: RadialShapesList,
    ) -> Self {
        Self {
            ref_point_id,
            x_coordinate,
            y_coordinate,
            z_coordinate,
            radial_shapes_list,
        }
    }
}

///*
/// * The DF contains a list of @ref RadialShapeDetails.
/// *
/// * @category: Georeference information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct RadialShapesList(pub SequenceOf<RadialShapeDetails>);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `railwayLevelCrossing` .
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`                   - in case no further detailed information on the railway level crossing status is available,
/// * - 1 `doNotCrossAbnormalSituation`   - in case when something wrong is detected by equation or sensors of the railway level crossing,
///                                         including level crossing is closed for too long (e.g. more than 10 minutes long ; default value),
/// * - 2 `closed`                        - in case the crossing is closed (barriers down),
/// * - 3 `unguarded`                     - in case the level crossing is unguarded (i.e a Saint Andrew cross level crossing without detection of train),
/// * - 4 `nominal`                       - in case the barriers are up and lights are off.
/// * - 5-255: reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RailwayLevelCrossingSubCauseCode(pub u8);

///*
/// * This DF represents the shape of a rectangular area or a right rectangular prism that is centred
/// * on a reference position defined outside of the context of this DF and oriented w.r.t. a cartesian    
/// * coordinate system defined outside of the context of this DF.
/// *
/// * It shall include the following components:
/// *
/// * @field shapeReferencePoint: represents an optional offset point which the rectangle is centred on with
/// * respect to the reference position. If this component is absent, the externally specified  
/// * reference position represents the shape's reference point.
/// *
/// * @field semiLength: represents half the length of the rectangle located in the X-Y Plane.
/// *
/// * @field semiBreadth: represents half the breadth of the rectangle located in the X-Y Plane.
/// *
/// * @field orientation: represents the optional orientation of the length of the rectangle,
/// * measured with positive values turning around the Z-axis using the right-hand rule, starting from
/// * the X-axis.
/// *
/// * @field height: represents the optional height, present if the shape is a right rectangular prism
/// * with height extending in the positive Z-axis.
/// *
/// * @category GeoReference information
/// * @revision: created in V2.1.1, centerPoint renamed to shapeReferencePoint, the type of the field orientation changed and description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RectangularShape {
    pub shape_reference_point: Option<CartesianPosition3d>,
    pub semi_length: StandardLength12b,
    pub semi_breadth: StandardLength12b,
    pub orientation: Option<CartesianAngleValue>,
    pub height: Option<StandardLength12b>,
}

impl RectangularShape {
    pub fn new(
        shape_reference_point: Option<CartesianPosition3d>,
        semi_length: StandardLength12b,
        semi_breadth: StandardLength12b,
        orientation: Option<CartesianAngleValue>,
        height: Option<StandardLength12b>,
    ) -> Self {
        Self {
            shape_reference_point,
            semi_length,
            semi_breadth,
            orientation,
            height,
        }
    }
}

///*
/// * A position within a geographic coordinate system together with a confidence ellipse.
/// *
/// * It shall include the following components:
/// *
/// * @field latitude: the latitude of the geographical point.
/// *
/// * @field longitude: the longitude of the geographical point.
/// *
/// * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position.
/// *
/// * @field altitude: the altitude and an altitude accuracy of the geographical point.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref ReferencePositionWithConfidence instead.
/// * @category: GeoReference information
/// * @revision: description updated in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ReferencePosition {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub position_confidence_ellipse: PosConfidenceEllipse,
    pub altitude: Altitude,
}

impl ReferencePosition {
    pub fn new(
        latitude: Latitude,
        longitude: Longitude,
        position_confidence_ellipse: PosConfidenceEllipse,
        altitude: Altitude,
    ) -> Self {
        Self {
            latitude,
            longitude,
            position_confidence_ellipse,
            altitude,
        }
    }
}

///*
/// * A position within a geographic coordinate system together with a confidence ellipse.
/// *
/// * It shall include the following components:
/// *
/// * @field latitude: the latitude of the geographical point.
/// *
/// * @field longitude: the longitude of the geographical point.
/// *
/// * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position.
/// *
/// * @field altitude: the altitude and an altitude accuracy of the geographical point.
/// *
/// * @category: GeoReference information
/// * @revision: created in V2.1.1 based on @ref ReferencePosition but using @ref PositionConfidenceEllipse.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ReferencePositionWithConfidence {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub position_confidence_ellipse: PositionConfidenceEllipse,
    pub altitude: Altitude,
}

impl ReferencePositionWithConfidence {
    pub fn new(
        latitude: Latitude,
        longitude: Longitude,
        position_confidence_ellipse: PositionConfidenceEllipse,
        altitude: Altitude,
    ) -> Self {
        Self {
            latitude,
            longitude,
            position_confidence_ellipse,
            altitude,
        }
    }
}

///*
/// * This DE describes a distance of relevance for information indicated in a message.
/// *
/// * The value shall be set to:
/// * - 0 `lessThan50m`   - for distances below 50 m,
/// * - 1 `lessThan100m`  - for distances below 100 m,
/// * - 2 `lessThan200m`  - for distances below 200 m,
/// * - 3 `lessThan500m`  - for distances below 300 m,
/// * - 4 `lessThan1000m` - for distances below 1 000 m,
/// * - 5 `lessThan5km`   - for distances below 5 000 m,
/// * - 6 `lessThan10km`  - for distances below 10 000 m,
/// * - 7 `over10km`      - for distances over 10 000 m.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StandardLength3b instead.
/// *
/// * @category: GeoReference information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RelevanceDistance {
    LessThan50m = 0,
    LessThan100m = 1,
    LessThan200m = 2,
    LessThan500m = 3,
    LessThan1000m = 4,
    LessThan5km = 5,
    LessThan10km = 6,
    Over10km = 7,
}

///*
/// * This DE indicates a traffic direction that is relevant to information indicated in a message.
/// *
/// * The value shall be set to:
/// * - 0 `allTrafficDirections` - for all traffic directions,
/// * - 1 `upstreamTraffic`      - for upstream traffic,
/// * - 2 `downstreamTraffic`    - for downstream traffic,
/// * - 3 `oppositeTraffic`      - for traffic in the opposite direction.
/// *
/// * The terms `upstream`, `downstream` and `oppositeTraffic` are relative to the event position.
/// *
/// * @note: Upstream traffic corresponds to the incoming traffic towards the event position,
/// * and downstream traffic to the departing traffic away from the event position.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrafficDirection instead.
/// *
/// * @category: GeoReference information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RelevanceTrafficDirection {
    AllTrafficDirections = 0,
    UpstreamTraffic = 1,
    DownstreamTraffic = 2,
    OppositeTraffic = 3,
}

///*
/// * This DE indicates whether an ITS message is transmitted as request from ITS-S or a response transmitted from
/// * ITS-S after receiving request from other ITS-Ss.
/// *
/// * The value shall be set to:
/// * - 0 `request`  - for a request message,
/// * - 1 `response` - for a response message.  
/// *
/// * @category Communication information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RequestResponseIndication {
    Request = 0,
    Response = 1,
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `rescueAndRecoveryWorkInProgress`
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`              - in case further detailed information on rescue and recovery work is unavailable,
/// * - 1 `emergencyVehicles`        - in case rescue and/or safeguarding work is ongoing by emergency vehicles, i.e. by vehicles that have the absolute right of way,
/// * - 2 `rescueHelicopterLanding`  - in case rescue helicopter is landing,
/// * - 3 `policeActivityOngoing`    - in case police activity is ongoing (only to be used if a more specific sub cause than (1) is needed),
/// * - 4 `medicalEmergencyOngoing`  - in case medical emergency recovery is ongoing (only to be used if a more specific sub cause than (1) is needed),
/// * - 5 `childAbductionInProgress` - in case a child kidnapping alarm is activated and rescue work is ongoing (only to be used if a more specific sub cause than (1) is needed),
/// * - 6 `prioritizedVehicle`       - in case rescue and/or safeguarding work is ongoing by prioritized vehicles, i.e. by vehicles that have priority but not the absolute right of way,
/// * - 7 `rescueAndRecoveryVehicle` - in case technical rescue work is ongoing by rescue and recovery vehicles.
/// * - 8-255: reserved for future usage.
///
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1, named values 6 and 7 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);

///*
/// * This DF shall contain a list of @ref StationType. to which a certain traffic restriction, e.g. the speed limit, applies.
/// *
/// * @category: Infrastructure information, Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct RestrictedTypes(pub SequenceOf<StationType>);

///*
/// * This DF provides configuration information about a road section.
/// *
/// * It shall include the following components:
/// *
/// * @field roadSectionDefinition: the topological definition of the road section for which the information in the other components applies throughout its entire length.
/// *
/// * @field roadType: the optional type of road on which the section is located.
/// *
/// * @field laneConfiguration: the optional configuration of the road section in terms of basic information per lane.
/// *
/// * @field mapemConfiguration: the optional configuration of the road section in terms of MAPEM lanes or connections.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RoadConfigurationSection {
    pub road_section_definition: RoadSectionDefinition,
    pub road_type: Option<RoadType>,
    pub lane_configuration: Option<BasicLaneConfiguration>,
    pub mapem_configuration: Option<MapemConfiguration>,
}

impl RoadConfigurationSection {
    pub fn new(
        road_section_definition: RoadSectionDefinition,
        road_type: Option<RoadType>,
        lane_configuration: Option<BasicLaneConfiguration>,
        mapem_configuration: Option<MapemConfiguration>,
    ) -> Self {
        Self {
            road_section_definition,
            road_type,
            lane_configuration,
            mapem_configuration,
        }
    }
}

///*
/// * This DF shall contain a list of @ref RoadConfigurationSection.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct RoadConfigurationSectionList(pub SequenceOf<RoadConfigurationSection>);

///*
/// * This DF provides the basic topological definition of a road section.
/// *
/// * It shall include the following components:
/// *
/// * @field startingPointSection: the position of the starting point of the section.
/// *
/// * @field lengthOfSection: the optional length of the section along the road profile (i.e. including curves).
/// *
/// * @field endingPointSection: the optional position of the ending point of the section.
/// * If this component is absent, the ending position is implicitly defined by other means, e.g. the starting point of the next RoadConfigurationSection, or the section�s length.
/// *
/// * @field connectedPaths: the identifier(s) of the path(s) having one or an ordered subset of waypoints located upstream of the RoadConfigurationSection� starting point.
/// *
/// * @field includedPaths: the identifier(s) of the path(s) that covers (either with all its length or with a part of it) a RoadConfigurationSection.
/// *
/// * @field isEventZoneIncluded: indicates, if set to TRUE, that the @ref EventZone incl. its reference position covers a RoadConfigurationSection (either with all its length or with a part of it).
/// *
/// * @field isEventZoneConnected: indicates, if set to TRUE, that the @ref EventZone incl. its reference position has one or an ordered subset of waypoints located upstream of the RoadConfigurationSection� starting point.
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RoadSectionDefinition {
    pub starting_point_section: GeoPosition,
    pub length_of_section: Option<StandardLength2B>,
    pub ending_point_section: Option<GeoPosition>,
    pub connected_paths: PathReferences,
    pub included_paths: PathReferences,
    pub is_event_zone_included: bool,
    pub is_event_zone_connected: bool,
}

impl RoadSectionDefinition {
    pub fn new(
        starting_point_section: GeoPosition,
        length_of_section: Option<StandardLength2B>,
        ending_point_section: Option<GeoPosition>,
        connected_paths: PathReferences,
        included_paths: PathReferences,
        is_event_zone_included: bool,
        is_event_zone_connected: bool,
    ) -> Self {
        Self {
            starting_point_section,
            length_of_section,
            ending_point_section,
            connected_paths,
            included_paths,
            is_event_zone_included,
            is_event_zone_connected,
        }
    }
}

///*
/// * This DE indicates an ordinal number that represents the position of a component in the list @ref RoadConfigurationSectionList.
/// *
/// * The value shall be set to:
/// * - `0`     - if no road section is identified
/// * - `1..8`  - for instances 1..8 of @ref RoadConfigurationSectionList
/// *
/// * @category: Road topology information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=8", extensible))]
pub struct RoadSectionId(pub Integer);

///*
/// * This DF represents a unique id for a road segment
/// *
/// * It shall include the following components:
/// *
/// * @field region: the optional identifier of the entity that is responsible for the region in which the road segment is placed.
/// * It is the duty of that entity to guarantee that the @ref Id is unique within the region.
/// *
/// * @field id: the identifier of the road segment.
/// *
/// * @note: when the component region is present, the RoadSegmentReferenceId is guaranteed to be globally unique.
/// * @category: GeoReference information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadSegmentReferenceId {
    pub region: Option<Identifier2B>,
    pub id: Identifier2B,
}

impl RoadSegmentReferenceId {
    pub fn new(region: Option<Identifier2B>, id: Identifier2B) -> Self {
        Self { region, id }
    }
}

///*
/// * This DE indicates the type of a road segment.
/// *
/// * The value shall be set to:
/// * - 0 `urban-NoStructuralSeparationToOppositeLanes`       - for an urban road with no structural separation between lanes carrying traffic in opposite directions,
/// * - 1 `urban-WithStructuralSeparationToOppositeLanes`     - for an urban road with structural separation between lanes carrying traffic in opposite directions,
/// * - 2 `nonUrban-NoStructuralSeparationToOppositeLanes`    - for an non urban road with no structural separation between lanes carrying traffic in opposite directions,
/// * - 3 `nonUrban-WithStructuralSeparationToOppositeLanes`  - for an non urban road with structural separation between lanes carrying traffic in opposite directions.
/// *
/// * @category: Road Topology Information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RoadType {
    UrbanNoStructuralSeparationToOppositeLanes = 0,
    UrbanWithStructuralSeparationToOppositeLanes = 1,
    NonUrbanNoStructuralSeparationToOppositeLanes = 2,
    NonUrbanWithStructuralSeparationToOppositeLanes = 3,
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `roadworks`.
/// *
///The value shall be set to:
/// * - 0 `unavailable`                 - in case further detailed information on roadworks is unavailable,
/// * - 1 `majorRoadworks`              - in case a major roadworks is ongoing,
/// * - 2 `roadMarkingWork`             - in case a road marking work is ongoing,
/// * - 3 `slowMovingRoadMaintenance`   - in case slow moving road maintenance work is ongoing,
/// * - 4 `shortTermStationaryRoadworks`- in case a short term stationary roadwork is ongoing,
/// * - 5 `streetCleaning`              - in case a vehicle street cleaning work is ongoing,
/// * - 6 `winterService`               - in case winter service work is ongoing.
/// * - 7-255                           - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RoadworksSubCauseCode(pub u8);

///*
/// * This DF provides the safe distance indication of a traffic participant with other traffic participant(s).
/// *
/// * It shall include the following components:
/// *
/// * @field subjectStation: optionally indicates one "other" traffic participant identified by its ITS-S.
/// *  
/// * @field safeDistanceIndicator: indicates whether the distance between the ego ITS-S and the traffic participant(s) is safe.
/// * If subjectStation is present then it indicates whether the distance between the ego ITS-S and the traffic participant indicated in the component subjectStation is safe.
/// *
/// * @field timeToCollision: optionally indicated the time-to-collision calculated as sqrt(LaDi^2 + LoDi^2 + VDi^2/relative speed
/// * and represented in  the  nearest 100  ms. This component may be present only if subjectStation is present.
/// *
/// * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD)
/// * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD).
/// *
/// * @category: Traffic information, Kinematic information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SafeDistanceIndication {
    pub subject_station: Option<StationId>,
    pub safe_distance_indicator: SafeDistanceIndicator,
    pub time_to_collision: Option<DeltaTimeTenthOfSecond>,
}

impl SafeDistanceIndication {
    pub fn new(
        subject_station: Option<StationId>,
        safe_distance_indicator: SafeDistanceIndicator,
        time_to_collision: Option<DeltaTimeTenthOfSecond>,
    ) -> Self {
        Self {
            subject_station,
            safe_distance_indicator,
            time_to_collision,
        }
    }
}

///*
/// * This DE indicates if a distance is safe.
/// *
/// * The value shall be set to:
/// * -  `FALSE`  if  the triple  {LaD,  LoD, VD} < {MSLaD, MSLoD, MSVD} is simultaneously  satisfied with confidence level of  90 % or  more,
/// * -  `TRUE` otherwise.
/// *
/// * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD)
/// * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD).
/// *
/// * @category: Traffic information, Kinematic information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SafeDistanceIndicator(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4095"))]
pub struct SemiAxisLength(pub u16);

///*
/// * This DE indicates the type of sensor.
/// *
/// * The value shall be set to:
/// * - 0  `undefined`         - in case the sensor type is undefined.
/// * - 1  `radar`             - in case the sensor is a radar,
/// * - 2  `lidar`             - in case the sensor is a lidar,
/// * - 3  `monovideo`         - in case the sensor is mono video,
/// * - 4  `stereovision`      - in case the sensor is stereo vision,
/// * - 5  `nightvision`       - in case the sensor is night vision,
/// * - 6  `ultrasonic`        - in case the sensor is ultrasonic,
/// * - 7  `pmd`               - in case the sensor is photonic mixing device,
/// * - 8  `inductionLoop`     - in case the sensor is an induction loop,
/// * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,
/// * - 10 `uwb`               - in case the sensor is ultra wide band,
/// * - 11 `acoustic`          - in case the sensor is acoustic,
/// * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,
/// * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages.
/// * - 14-31                  - are reserved for future usage.
/// *
/// * @category: Sensing Information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct SensorType(pub u8);

///*
/// * This DE indicates the type of sensor(s).
/// * The corresponding bit shall be set to 1 under the following conditions:
/// *
/// * - 0  `undefined`         - in case the sensor type is undefined.
/// * - 1  `radar`             - in case the sensor is a radar,
/// * - 2  `lidar`             - in case the sensor is a lidar,
/// * - 3  `monovideo`         - in case the sensor is mono video,
/// * - 4  `stereovision`      - in case the sensor is stereo vision,
/// * - 5  `nightvision`       - in case the sensor is night vision,
/// * - 6  `ultrasonic`        - in case the sensor is ultrasonic,
/// * - 7  `pmd`               - in case the sensor is photonic mixing device,
/// * - 8  `inductionLoop`     - in case the sensor is an induction loop,
/// * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,
/// * - 10 `uwb`               - in case the sensor is ultra wide band,
/// * - 11 `acoustic`          - in case the sensor is acoustic,
/// * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,
/// * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages.
/// * - 14-15                  - are reserved for future usage.
/// *
/// * @note: If all bits are set to 0, then no sensor type is used
/// *
/// * @category: Sensing Information
/// * @revision: created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16", extensible))]
pub struct SensorTypes(pub BitString);

///*
/// * This DE represents a sequence number.
/// *
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct SequenceNumber(pub u16);

///*
/// * This DF shall contain a list of DF @ref CartesianPosition3d.
/// *
/// * @category: GeoReference information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct SequenceOfCartesianPosition3d(pub SequenceOf<CartesianPosition3d>);

///*
/// * The DF contains a list of DE @ref Identifier1B.
/// *
/// * @category: Basic information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=128", extensible))]
pub struct SequenceOfIdentifier1B(pub SequenceOf<Identifier1B>);

///*
/// * The DF contains a list of DF @ref SafeDistanceIndication.
/// *
/// * @category: Traffic information, Kinematic information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct SequenceOfSafeDistanceIndication(pub SequenceOf<SafeDistanceIndication>);

///*
/// * The DF shall contain a list of DF @ref TrajectoryInterceptionIndication.
/// *
/// * @category: Traffic information, Kinematic information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct SequenceOfTrajectoryInterceptionIndication(
    pub SequenceOf<TrajectoryInterceptionIndication>,
);

///*
/// * This DF provides the definition of a geographical area or volume, based on different options.
/// *
/// * It is a choice of the following components:
/// *
/// * @field rectangular: definition of an rectangular area or a right rectangular prism (with a rectangular base) also called a cuboid, or informally a rectangular box.
/// *
/// * @field circular: definition of an area of circular shape or a right circular cylinder.
/// *
/// * @field polygonal: definition of an area of polygonal shape or a right prism.
/// *
/// * @field elliptical: definition of an area of elliptical shape or a right elliptical cylinder.
/// *
/// * @field radial: definition of a radial shape.
/// *
/// * @field radialList: definition of list of radial shapes.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Shape {
    Rectangular(RectangularShape),
    Circular(CircularShape),
    Polygonal(PolygonalShape),
    Elliptical(EllipticalShape),
    Radial(RadialShape),
    RadialShapes(RadialShapes),
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `signalViolation`.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`               - in case further detailed information on signal violation event is unavailable,
/// * - 1 `stopSignViolation`         - in case a stop sign violation is detected,
/// * - 2 `trafficLightViolation`     - in case a traffic light violation is detected,
/// * - 3 `turningRegulationViolation`- in case a turning regulation violation is detected.
/// * - 4-255                         - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SignalViolationSubCauseCode(pub u8);

///*
/// * This DE represents the sub cause codes of the @ref CauseCode "slowVehicle".
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`                    - in case further detailed information on slow vehicle driving event is
/// *                                        unavailable,
/// * - 1 `maintenanceVehicle`             - in case of a slow driving maintenance vehicle on the road,
/// * - 2 `vehiclesSlowingToLookAtAccident`- in case vehicle is temporally slowing down to look at accident, spot, etc.,
/// * - 3 `abnormalLoad`                   - in case an abnormal loaded vehicle is driving slowly on the road,
/// * - 4 `abnormalWideLoad`               - in case an abnormal wide load vehicle is driving slowly on the road,
/// * - 5 `convoy`                         - in case of slow driving convoy on the road,
/// * - 6 `snowplough`                     - in case of slow driving snow plough on the road,
/// * - 7 `deicing`                        - in case of slow driving de-icing vehicle on the road,
/// * - 8 `saltingVehicles`                - in case of slow driving salting vehicle on the road.
/// * - 9-255                              - are reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SlowVehicleSubCauseCode(pub u8);

///*
/// * The DE indicates if a vehicle is carrying goods in the special transport conditions.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 `heavyLoad`        - the vehicle is carrying goods with heavy load,
/// * - 1 `excessWidth`      - the vehicle is carrying goods in excess of width,
/// * - 2 `excessLength`     - the vehicle is carrying goods in excess of length,
/// * - 3 `excessHeight`     - the vehicle is carrying goods in excess of height.
/// *
/// * Otherwise, the corresponding bit shall be set to 0.
/// * @category Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct SpecialTransportType(pub BitString);

///*
/// * This DF represents the speed and associated confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field speedValue: the speed value.
/// *
/// * @field speedConfidence: the confidence value of the speed value.
/// *
/// * @category: Kinematic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Speed {
    pub speed_value: SpeedValue,
    pub speed_confidence: SpeedConfidence,
}

impl Speed {
    pub fn new(speed_value: SpeedValue, speed_confidence: SpeedConfidence) -> Self {
        Self {
            speed_value,
            speed_confidence,
        }
    }
}

///*
/// * This DE indicates the speed confidence value which represents the estimated absolute accuracy of a speed value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n * 0,01 m/s.
/// * - `126` if the confidence value is out of range, i.e. greater than 1,25 m/s,
/// * - `127` if the confidence value information is not available.
/// *  
/// * @note: The fact that a speed value is received with confidence value set to `unavailable(127)` can be caused by several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the speed value may be valid and used by the application.
/// *
/// * @note: If a speed value is received and its confidence value is set to `outOfRange(126)`, it means that the speed value is not valid
/// * and therefore cannot be trusted. Such is not useful for the application.
/// *
/// * @unit: 0,01 m/s
/// * @category: Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SpeedConfidence(pub u8);

///*
/// * This DE represents a speed limitation applied to a geographical position, a road section or a geographical region.
/// *
/// * @unit: km/h
/// * @category: Infrastructure information, Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct SpeedLimit(pub u8);

///*
/// * This DE represents a speed value, i.e. the magnitude of the velocity-vector.
/// *
/// * The value shall be set to:
/// * - `0` in a standstill situation.
/// * - `n` (`n > 0` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,
/// * - `16 382` for speed values greater than 163,81 m/s,
/// * - `16 383` if the speed accuracy information is not available.
/// *
/// * @note: the definition of standstill is out of scope of the present document.
/// *
/// * @unit: 0,01 m/s
/// * @category: Kinematic information
/// * @revision: Description revised in V2.1.1 (the meaning of 16382 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct SpeedValue(pub u16);

///*
/// * This DF  provides the  indication of  change in stability.
/// *
/// * It shall include the following components:
/// *
/// * @field lossProbability: the probability of stability loss.
/// *
/// * @field actionDeltaTime: the period over which the the probability of stability loss is estimated.
/// *
/// * @category: Kinematic information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct StabilityChangeIndication {
    pub loss_probability: StabilityLossProbability,
    pub action_delta_time: DeltaTimeTenthOfSecond,
}

impl StabilityChangeIndication {
    pub fn new(
        loss_probability: StabilityLossProbability,
        action_delta_time: DeltaTimeTenthOfSecond,
    ) -> Self {
        Self {
            loss_probability,
            action_delta_time,
        }
    }
}

///*
/// * This DE indicates the estimated probability of a stability level and conversely also the probability of a stability loss.
/// *
/// * The value shall be set to:
/// * - `0` to indicate an estimated probability of a loss of stability of 0 %, i.e. "stable",
/// * - `n` (`n > 0` and `n < 50`) to indicate the actual stability level,
/// * - `50` to indicate a estimated probability of a loss of stability of 100 %, i.e. "total loss of stability",
/// * - the values between 51 and 62 are reserved for future use,
/// * - `63`: this value indicates that the information is unavailable.
/// *
/// * @unit: 2 %
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=63"))]
pub struct StabilityLossProbability(pub u8);

///*
/// * The DE represents length as a measure of distance between points or as a dimension of an object or shape.
/// *
/// * @unit: 0,1 metre
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4095"))]
pub struct StandardLength12b(pub u16);

///*
/// * The DE represents length as a measure of distance between points or as a dimension of an object.
/// *
/// * @unit: 0,1 metre
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StandardLength1B(pub u8);

///*
/// * The DE represents length as a measure of distance between points or as a dimension of an object.  
/// *
/// * @unit: 0,1 metre
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct StandardLength2B(pub u16);

///*
/// * The DE represents length as a measure of distance between points.
/// *
/// * The value shall be set to:
/// * - 0 `lessThan50m`   - for distances below 50 m,
/// * - 1 `lessThan100m`  - for distances below 100 m,
/// * - 2 `lessThan200m`  - for distances below 200 m,
/// * - 3 `lessThan500m`  - for distances below 300 m,
/// * - 4 `lessThan1000m` - for distances below 1 000 m,
/// * - 5 `lessThan5km`   - for distances below 5 000 m,
/// * - 6 `lessThan10km`  - for distances below 10 000 m,
/// * - 7 `over10km`      - for distances over 10 000 m.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1 from RelevanceDistance
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum StandardLength3b {
    LessThan50m = 0,
    LessThan100m = 1,
    LessThan200m = 2,
    LessThan500m = 3,
    LessThan1000m = 4,
    LessThan5km = 5,
    LessThan10km = 6,
    Over10km = 7,
}

///*
/// * The DE represents length as a measure of distance between points or as a dimension of an object.
/// *
/// * @unit: 0,1 metre
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=511"))]
pub struct StandardLength9b(pub u16);

///*
/// * This DE represents the identifier of an ITS-S.
/// * The ITS-S ID may be a pseudonym. It may change over space and/or over time.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StationId instead.
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct StationID(pub u32);

///*
/// * This DE represents the identifier of an ITS-S.
/// * The ITS-S ID may be a pseudonym. It may change over space and/or over time.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1 based on @ref StationID
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct StationId(pub u32);

///*
/// * This DE represents the type of technical context the ITS-S is integrated in.
/// * The station type depends on the integration environment of ITS-S into vehicle, mobile devices or at infrastructure.
/// *
/// * The value shall be set to:
/// * - 0 `unknown`          - information about the ITS-S context is not provided,
/// * - 1 `pedestrian`       - ITS-S carried by human being not using a mechanical device for their trip (VRU profile 1),
/// * - 2 `cyclist`          - ITS-S mounted on non-motorized unicycles, bicycles , tricycles, quadracycles (VRU profile 2),
/// * - 3 `moped`            - ITS-S mounted on light motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16]
///                            class L1, L2 (VRU Profile 3),
/// * - 4 `motorcycles`      - ITS-S mounted on motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16]
///                            class L3, L4, L5, L6, L7 (VRU Profile 3),
/// * - 5 `passengerCar`     - ITS-S mounted on small passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M1,
/// * - 6 `bus`              - ITS-S mounted on large passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M2, M3,
/// * - 7 `lightTruck`       - ITS-S mounted on light Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N1,
/// * - 8 `heavyTruck`       - ITS-S mounted on Heavy Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N2 and N3,
/// * - 9 `trailer`          - ITS-S mounted on an unpowered vehicle that is intended to be towed by a powered vehicle as defined in
///                            UNECE/TRANS/WP.29/78/Rev.4 [16] class O,
/// * - 10 `specialVehicles` - ITS-S mounted on vehicles which have special purposes other than the above (e.g. moving road works vehicle),
/// * - 11 `tram`            - ITS-S mounted on a vehicle which runs on tracks along public streets,
/// * - 12 `lightVruVehicle` - ITS-S carried by a human being traveling on light vehicle , incl. possible use of roller skates or skateboards (VRU profile 2),
/// * - 13 `animal`          - ITS-S carried by an animal presenting a safety risk to other road users e.g. domesticated dog in a city or horse (VRU Profile 4),
/// * - 14                   - reserved for future usage,
/// * - 15 `roadSideUnit`    - ITS-S mounted on an infrastructure typically positioned outside of the drivable roadway (e.g. on a gantry, on a pole,
///                            on a stationary road works trailer); the infrastructure is static during the entire operation period of the ITS-S (e.g. no stop and go activity),
/// * - 16-255               - are reserved for future usage.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrafficParticipantType instead.
/// * @category: Communication information.
/// * @revision: revised in V2.1.1 (named values 12 and 13 added and note to value 9 deleted)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StationType(pub u8);

///*
/// * This DE indicates the duration in minutes since which something is stationary.
/// *
/// * The value shall be set to:
/// * - 0 `lessThan1Minute`         - for being stationary since less than 1 minute,
/// * - 1 `lessThan2Minutes`        - for being stationary since less than 2 minute and for equal to or more than 1 minute,
/// * - 2 `lessThan15Minutes`       - for being stationary since less than 15 minutes and for equal to or more than 1 minute,
/// * - 3 `equalOrGreater15Minutes` - for being stationary since equal to or more than 15 minutes.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum StationarySince {
    LessThan1Minute = 0,
    LessThan2Minutes = 1,
    LessThan15Minutes = 2,
    EqualOrGreater15Minutes = 3,
}

///*
/// * This DE provides the value of the sub cause codes of the @ref CauseCode "stationaryVehicle".
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`           - in case further detailed information on stationary vehicle is unavailable,
/// * - 1 `humanProblem`          - in case stationary vehicle is due to health problem of driver or passenger,
/// * - 2 `vehicleBreakdown`      - in case stationary vehicle is due to vehicle break down,
/// * - 3 `postCrash`             - in case stationary vehicle is caused by collision,
/// * - 4 `publicTransportStop`   - in case public transport vehicle is stationary at bus stop,
/// * - 5 `carryingDangerousGoods`- in case the stationary vehicle is carrying dangerous goods,
/// * - 6 `vehicleOnFire`         - in case of vehicle on fire.
/// * - 7-255 reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StationaryVehicleSubCauseCode(pub u8);

///*
/// * This DF represents the steering wheel angle of the vehicle at certain point in time.
/// *
/// * It shall include the following components:
/// *
/// * @field steeringWheelAngleValue: steering wheel angle value.
/// *
/// * @field steeringWheelAngleConfidence: confidence value of the steering wheel angle value.
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SteeringWheelAngle {
    pub steering_wheel_angle_value: SteeringWheelAngleValue,
    pub steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
}

impl SteeringWheelAngle {
    pub fn new(
        steering_wheel_angle_value: SteeringWheelAngleValue,
        steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
    ) -> Self {
        Self {
            steering_wheel_angle_value,
            steering_wheel_angle_confidence,
        }
    }
}

///*
/// * This DE indicates the steering wheel angle confidence value which represents the estimated absolute accuracy for a  steering wheel angle value with a confidence level of 95 %.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 1,5 degrees,
/// * - `126` if the confidence value is out of range, i.e. greater than 187,5 degrees,
/// * - `127` if the confidence value is not available.
/// *
/// * @note: The fact that a steering wheel angle value is received with confidence value set to `unavailable(127)`
/// * can be caused by several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the steering wheel angle value may be valid and used by the application.
/// *
/// * If a steering wheel angle value is received and its confidence value is set to `outOfRange(126)`,
/// * it means that the steering wheel angle value is not valid and therefore cannot be trusted.
/// * Such value is not useful for the application.
/// *
/// * @unit: 1,5 degree
/// * @category: Vehicle Information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SteeringWheelAngleConfidence(pub u8);

///*
/// * This DE represents the steering wheel angle of the vehicle at certain point in time.
/// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
/// *
/// * The value shall be set to:
/// * - `-511` if the steering wheel angle is equal to or greater than 511 x 1,5 degrees = 766,5 degrees to the right,
/// * - `n` (`n > -511` and `n <= 0`) if  the steering wheel angle is equal to or less than n x 1,5 degrees, and greater than (n-1) x 1,5 degrees,
///      turning clockwise (i.e. to the right),
/// * - `n` (`n >= 1` and `n < 511`) if the steering wheel angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees,
///      turning counter-clockwise (i.e. to the left),
/// * - `511` if the steering wheel angle is greater than 510 x 1,5 degrees = 765 degrees to the left,
/// * - `512` if information is not available.
/// *
/// * @unit: 1,5 degree
/// * @revision: Description revised in V2.1.1 (meaning of value 511 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-511..=512"))]
pub struct SteeringWheelAngleValue(pub i16);

///*
/// * This DE indicates the type of stored information.
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// *
/// * - `0` undefined        - in case the stored information type is undefined.
/// * - `1` staticDb         - in case the stored information type is a static database.
/// * - `2` dynamicDb        - in case the stored information type is a dynamic database
/// * - `3` realTimeDb       - in case the stored information type is a real time updated database.
/// * - `4` map              - in case the stored information type is a road topology map.
/// * - Bits 5 to 7          - are reserved for future use.
/// *
/// * @note: If all bits are set to 0, then no stored information type is used
/// *
/// * @category: Basic Information
/// * @revision: created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8", extensible))]
pub struct StoredInformationType(pub BitString);

///*
/// * This DE indicates the generic sub cause of a detected event.
/// *
/// * @note: The sub cause code value assignment varies based on value of @ref CauseCode.
/// *
/// * @category: Traffic information
/// * @revision: Description revised in V2.1.1 (this is  the generic sub cause type)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SubCauseCodeType(pub u8);

///*
/// * This DE indicates a temperature value.
///
/// * The value shall be set to:
/// * - `-60` for temperature equal to or less than -60 degrees C,
/// * - `n` (`n > -60` and `n < 67`) for the actual temperature n in degrees C,
/// * - `67` for temperature equal to or greater than 67 degrees C.
/// *
/// * @unit: degrees Celsius
/// * @category: Basic information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-60..=67"))]
pub struct Temperature(pub i8);

///*
/// * This DE represents the number of elapsed (TAI) milliseconds since the ITS Epoch.
/// * The ITS epoch is `00:00:00.000 UTC, 1 January 2004`.
/// * "Elapsed" means that the true number of milliseconds is continuously counted without interruption,
/// *  i.e. it is not altered by leap seconds, which occur in UTC.
/// *
/// * @note: International Atomic Time (TAI) is the time reference coordinate on the basis of the readings of atomic clocks,
/// * operated in accordance with the definition of the second, the unit of time of the International System of Units.
/// * TAI is a continuous time scale. UTC has discontinuities, as it is occasionally adjusted by leap seconds.
/// * As of 1 January, 2022, TimestampIts is 5 seconds ahead of UTC, because since the ITS epoch on 1 January 2004 00:00:00.000 UTC,
/// * further 5 leap seconds have been inserted in UTC.
/// *
/// * EXAMPLE: The value for TimestampIts for 1 January 2007 00:00:00.000 UTC is `94 694 401 000` milliseconds,
/// * which includes one leap second insertion since the ITS epoch.
/// * @unit: 0,001 s
/// * @category: Basic information
/// * @revision: Description revised in in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4398046511103"))]
pub struct TimestampIts(pub u64);

///*
/// * This DF represents one or more paths using @ref Path.
/// *
/// * @category: GeoReference information
/// * @revision: Description revised in V2.1.1. Is is now based on Path and not on PathHistory
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=7"))]
pub struct Traces(pub SequenceOf<Path>);

///*
/// * This DF represents one or more paths using @ref PathExtended.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=7"))]
pub struct TracesExtended(pub SequenceOf<PathExtended>);

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `trafficCondition`.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`                  - in case further detailed information on the traffic condition is unavailable,
/// * - 1 `increasedVolumeOfTraffic`     - in case the type of traffic condition is increased traffic volume,
/// * - 2 `trafficJamSlowlyIncreasing`   - in case the type of traffic condition is a traffic jam which volume is increasing slowly,
/// * - 3 `trafficJamIncreasing`         - in case the type of traffic condition is a traffic jam which volume is increasing,
/// * - 4 `trafficJamStronglyIncreasing` - in case the type of traffic condition is a traffic jam which volume is strongly increasing,
/// * - 5 `trafficJam`         `         - in case the type of traffic condition is a traffic jam and no further detailed information about its volume is available,
/// * - 6 `trafficJamSlightlyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing slowly,
/// * - 7 `trafficJamDecreasing`         - in case the type of traffic condition is a traffic jam which volume is decreasing,
/// * - 8 `trafficJamStronglyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing rapidly,
/// * - 9 `trafficJamStable`             - in case the traffic condition is a traffic jam with stable volume,
/// * - 10-255: reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1, definition of value 0 and 1 changed in V2.2.1, name and definition of value 5 changed in V2.2.1, value 9 added in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct TrafficConditionSubCauseCode(pub u8);

///*
/// * This DE indicates a direction of traffic with respect to a reference direction, and a portion of that traffic with respect to a reference position.
/// *
/// * The value shall be set to:
/// * - 0 `allTrafficDirections`                                    - for all directions of traffic,
/// * - 1 `sameAsReferenceDirection-upstreamOfReferencePosition`    - for the direction of traffic according to the reference direction, and the portion of traffic upstream of the reference position,
/// * - 2 `sameAsReferenceDirection-downstreamOfReferencePosition`  - for the direction of traffic according to the reference direction, and the portion of traffic downstream of the reference position,
/// * - 3 `oppositeToReferenceDirection`                            - for the direction of traffic opposite to the reference direction.
/// *
/// * @note: Upstream traffic corresponds to the incoming traffic towards the event position, and downstream traffic to the departing traffic away from the event position.
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1 from RelevanceTrafficDirection, description and naming of values changed in V2.2.1
/// *
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TrafficDirection {
    AllTrafficDirections = 0,
    SameAsReferenceDirectionUpstreamOfReferencePosition = 1,
    SameAsReferenceDirectionDownstreamOfReferencePosition = 2,
    OppositeToReferenceDirection = 3,
}

///*
/// * Ths DF represents the a position on a traffic island between two lanes.
/// *
/// * It shall include the following components:
/// *
/// * @field oneSide: represents one lane.
/// *
/// * @field otherSide: represents the other lane.
/// *
/// * @category: Road Topology information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TrafficIslandPosition {
    pub one_side: LanePositionAndType,
    pub other_side: LanePositionAndType,
}

impl TrafficIslandPosition {
    pub fn new(one_side: LanePositionAndType, other_side: LanePositionAndType) -> Self {
        Self {
            one_side,
            other_side,
        }
    }
}

///*
/// * This DE represents the type of a traffic participant.
/// *
/// * The value shall be set to:
/// * - 0 `unknown`          - information about traffic participant is not provided,
/// * - 1 `pedestrian`       - human being not using a mechanical device for their trip (VRU profile 1),
/// * - 2 `cyclist`          - non-motorized unicycles, bicycles , tricycles, quadracycles (VRU profile 2),
/// * - 3 `moped`            - light motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L1, L2 (VRU Profile 3),
/// * - 4 `motorcycles`      - motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L3, L4, L5, L6, L7 (VRU Profile 3),
/// * - 5 `passengerCar`     - small passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M1,
/// * - 6 `bus`              - large passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M2, M3,
/// * - 7 `lightTruck`       - light Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N1,
/// * - 8 `heavyTruck`       - Heavy Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N2 and N3,
/// * - 9 `trailer`          - unpowered vehicle that is intended to be towed by a powered vehicle as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class O,
/// * - 10 `specialVehicles` - vehicles which have special purposes other than the above (e.g. moving road works vehicle),
/// * - 11 `tram`            - vehicle which runs on tracks along public streets,
/// * - 12 `lightVruVehicle` - human being traveling on light vehicle, incl. possible use of roller skates or skateboards (VRU profile 2),
/// * - 13 `animal`          - animal presenting a safety risk to other road users e.g. domesticated dog in a city or horse (VRU Profile 4),
/// * - 14 `agricultural`    - agricultural and forestry vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class T,
/// * - 15 `roadSideUnit`    - infrastructure typically positioned outside of the drivable roadway (e.g. on a gantry, on a pole,
///                            on a stationary road works trailer); the infrastructure is static during the entire operation period of the ITS-S (e.g. no stop and go activity),
/// * - 16-255               - are reserved for future usage.
/// *
/// * @category: Communication information.
/// * @revision: Created in V2.1.1 based on StationType
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct TrafficParticipantType(pub u8);

///*
/// * This DE indicates traffic rules that apply to vehicles at a certain position.
/// *
/// * The value shall be set to:
/// * - `0` - if overtaking is prohibited for all vehicles,
/// * - `1` - if overtaking is prohibited for trucks,
/// * - `2` - if vehicles should pass to the right lane,
/// * - `3` - if vehicles should pass to the left lane.
/// * - `4` - if vehicles should pass to the left or right lane.
/// *
/// * @category: Infrastructure information, Traffic information
/// * @revision: Editorial update in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum TrafficRule {
    NoPassing = 0,
    NoPassingForTrucks = 1,
    PassToRight = 2,
    PassToLeft = 3,
    #[rasn(extension_addition)]
    PassToLeftOrRight = 4,
}

///*
/// * This DF provides detailed information about an attached trailer.
/// *
/// * It shall include the following components:
/// *
/// * @field refPointId: identifier of the reference point of the trailer.
/// *
/// * @field hitchPointOffset: optional position of the hitch point in negative x-direction (according to ISO 8855) from the
/// * vehicle Reference Point.
/// *
/// * @field frontOverhang: optional length of the trailer overhang in the positive x direction (according to ISO 8855) from the
/// * trailer Reference Point indicated by the refPointID. The value defaults to 0 in case the trailer
/// * is not overhanging to the front with respect to the trailer reference point.
/// *
/// * @field rearOverhang: optional length of the trailer overhang in the negative x direction (according to ISO 8855) from the
/// * trailer Reference Point indicated by the refPointID.
/// *
/// * @field trailerWidth: optional width of the trailer.
/// *
/// * @field hitchAngle: optional Value and confidence value of the angle between the trailer orientation (corresponding to the x
/// * direction of the ISO 8855 [21] coordinate system centered on the trailer) and the direction of
/// * the segment having as end points the reference point of the trailer and the reference point of
/// * the pulling vehicle, which can be another trailer or a vehicle looking on the horizontal plane
/// * xy, described in the local Cartesian coordinate system of the trailer. The
/// * angle is measured with negative values considering the trailer orientation turning clockwise
/// * starting from the segment direction. The angle value accuracy is provided with the
/// * confidence level of 95 %.
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TrailerData {
    pub ref_point_id: Identifier1B,
    pub hitch_point_offset: StandardLength1B,
    pub front_overhang: Option<StandardLength1B>,
    pub rear_overhang: Option<StandardLength1B>,
    pub trailer_width: Option<VehicleWidth>,
    pub hitch_angle: CartesianAngle,
}

impl TrailerData {
    pub fn new(
        ref_point_id: Identifier1B,
        hitch_point_offset: StandardLength1B,
        front_overhang: Option<StandardLength1B>,
        rear_overhang: Option<StandardLength1B>,
        trailer_width: Option<VehicleWidth>,
        hitch_angle: CartesianAngle,
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

///*
/// * This DE provides information about the presence of a trailer.
/// *
/// * The value shall be set to:
/// * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present.
/// * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is included in the vehicle length value.
/// * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value.
/// * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible.
/// * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer
/// *                                         nor the detection of trailer presence/absence is possible.
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1 based on VehicleLengthConfidenceIndication
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TrailerPresenceInformation {
    NoTrailerPresent = 0,
    TrailerPresentWithKnownLength = 1,
    TrailerPresentWithUnknownLength = 2,
    TrailerPresenceIsUnknown = 3,
    Unavailable = 4,
}

///*
/// * This DE  defines  the  confidence level of the trajectoryInterceptionProbability.
/// *
/// * The value shall be set to:
/// * - `0` - to indicate confidence level less than 50 %,
/// * - `1` - to indicate confidence level greater than or equal to 50 % and less than 70 %,
/// * - `2` - to indicate confidence level greater than or equal to 70 % and less than 90 %,
/// * - `3` - to indicate confidence level greater than or equal to 90%.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct TrajectoryInterceptionConfidence(pub u8);

///*
/// * This DF  provides the trajectory  interception  indication  of  ego-VRU  ITS-S  with another ITS-Ss.
/// *
/// * It shall include the following components:
/// *
/// * @field subjectStation: indicates the subject station.
/// *
/// * @field trajectoryInterceptionProbability: indicates the propbability of the interception of the subject station trajectory
/// * with the trajectory of the station indicated in the component subjectStation.
/// *
/// * @field trajectoryInterceptionConfidence: indicates the confidence of interception of the subject station trajectory
/// * with the trajectory of the station indicated in the component subjectStation.
/// *
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TrajectoryInterceptionIndication {
    pub subject_station: Option<StationId>,
    pub trajectory_interception_probability: TrajectoryInterceptionProbability,
    pub trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
}

impl TrajectoryInterceptionIndication {
    pub fn new(
        subject_station: Option<StationId>,
        trajectory_interception_probability: TrajectoryInterceptionProbability,
        trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
    ) -> Self {
        Self {
            subject_station,
            trajectory_interception_probability,
            trajectory_interception_confidence,
        }
    }
}

///*
/// * This  DE  defines  the  probability  that the ego trajectory  intercepts  with any  other object's  trajectory  on the  road.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 0` and `n <= 50`) to indicate the actual probability,
/// * - the values between 51 and 62 are reserved,
/// * - `63`: to indicate that the information is unavailable.
/// *
/// * @unit: 2 %
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=63"))]
pub struct TrajectoryInterceptionProbability(pub u8);

///*
/// * This DE represents the time interval between two consecutive message transmissions.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeMilliSecondPos instead.
/// * @unit: 0,001 s
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=10000"))]
pub struct TransmissionInterval(pub u16);

///*
/// * This DE provides the turning direction.
/// *
/// * The value shall be set to:
/// * - `left`  for turning to te left.
/// * - `right` for turing to the right.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TurningDirection {
    Left = 0,
    Right = 1,
}

///*
/// * This DE represents the smallest circular turn (i.e. U-turn) that the vehicle is capable of making.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 254`) to indicate the applicable value is equal to or less than n x 0,4 metre, and greater than (n-1) x 0,4 metre,
/// * - `254` to indicate that the turning radius is  greater than 253 x 0,4 metre = 101.2 metres,
/// * - `255` to indicate that the information is unavailable.
/// *
/// * For vehicle with tracker, the turning radius applies to the vehicle only.
/// *
/// * @category: Vehicle information
/// * @unit 0,4 metre
/// * @revision: Description revised V2.1.1 (the meaning of 254 has changed slightly)
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct TurningRadius(pub u8);

///*
/// * This DE represents indication of how a certain path or area will be used.
/// *
/// * The value shall be set to:
/// * - 0  - ` noIndication `     - in case it will remain free to be used,
/// * - 1  - ` specialUse `       - in case it will be physically blocked by special use,
/// * - 2  - ` rescueOperation`   - in case it is intended to be used for rescue operations,
/// *
/// * @category: Basic information
/// * @revision: Created in V2.2.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum UsageIndication {
    NoIndication = 0,
    SpecialUse = 1,
    RescueOperation = 2,
}

///*
/// * This DE represents the Vehicle Descriptor Section (VDS). The values are assigned according to ISO 3779 [6].
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("6"))]
pub struct VDS(pub Ia5String);

///*
/// * This DE represents the duration of a traffic event validity.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeSecond instead.
/// * @unit: 1 s
/// * @category: Basic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=86400"))]
pub struct ValidityDuration(pub u32);

///*
/// * This DF together with its sub DFs Ext1, Ext2 and the DE Ext3 provides the custom (i.e. not ASN.1 standard) definition of an integer with variable lenght, that can be used for example to encode the ITS-AID.
/// *
/// * @category: Basic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum VarLengthNumber {
    #[rasn(value("0..=127"), tag(context, 0))]
    Content(u8),
    #[rasn(tag(context, 1))]
    Extension(Ext1),
}

///*
/// * This DE represents the value of the sub cause codes of the @ref CauseCode `vehicleBreakdown`.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`         - in case further detailed information on cause of vehicle break down is unavailable,
/// * - 1 `lackOfFuel`          - in case vehicle break down is due to lack of fuel,
/// * - 2 `lackOfBatteryPower`  - in case vehicle break down is caused by lack of battery power,
/// * - 3 `engineProblem`       - in case vehicle break down is caused by an engine problem,
/// * - 4 `transmissionProblem` - in case vehicle break down is caused by transmission problem,
/// * - 5 `engineCoolingProblem`- in case vehicle break down is caused by an engine cooling problem,
/// * - 6 `brakingSystemProblem`- in case vehicle break down is caused by a braking system problem,
/// * - 7 `steeringProblem`     - in case vehicle break down is caused by a steering problem,
/// * - 8 `tyrePuncture`        - in case vehicle break down is caused by tyre puncture,
/// * - 9 `tyrePressureProblem` - in case low tyre pressure in detected,
/// * - 10 `vehicleOnFire`      - in case the vehicle is on fire.
/// * - 11-255                  - are reserved for future usage.
/// *
/// * @category: Traffic information
///
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct VehicleBreakdownSubCauseCode(pub u8);

///*
/// * This DE represents the height of the vehicle, measured from the ground to the highest point, excluding any antennas.
/// * In case vehicles are equipped with adjustable ride heights, camper shells, and any other
/// * equipment which may result in varying height, the largest possible height shall be used.
/// *
/// * The value shall be set to:
/// * - `n` (`n >0` and `n < 127`) indicates the applicable value is equal to or less than n x 0,05 metre, and greater than (n-1) x 0,05 metre,
/// * - `127` indicates that the vehicle width is greater than 6,3 metres,
/// * - `128` indicates that the information in unavailable.
/// *
/// * @unit: 0,05 metre
/// * @category: Vehicle information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=128"))]
pub struct VehicleHeight(pub u8);

///*
/// * This DF provides information related to the identification of a vehicle.
/// *
/// * It shall include the following components:
/// *
/// * @field wMInumber: World Manufacturer Identifier (WMI) code.
/// *
/// * @field vDS: Vehicle Descriptor Section (VDS).
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VehicleIdentification {
    pub w_m_inumber: Option<WMInumber>,
    pub v_d_s: Option<VDS>,
}

impl VehicleIdentification {
    pub fn new(w_m_inumber: Option<WMInumber>, v_d_s: Option<VDS>) -> Self {
        Self { w_m_inumber, v_d_s }
    }
}

///*
/// * This DF represents the length of vehicle and accuracy indication information.
/// *
/// * It shall include the following components:
/// *
/// * @field vehicleLengthValue: length of vehicle.
/// *
/// * @field vehicleLengthConfidenceIndication: indication of the length value confidence.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref VehicleLengthV2 instead.
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleLength {
    pub vehicle_length_value: VehicleLengthValue,
    pub vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
}

impl VehicleLength {
    pub fn new(
        vehicle_length_value: VehicleLengthValue,
        vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
    ) -> Self {
        Self {
            vehicle_length_value,
            vehicle_length_confidence_indication,
        }
    }
}

///*
/// * This DE provides information about the presence of a trailer.
/// *
/// * The value shall be set to:
/// * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present,
/// * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is  included in the vehicle length value,
/// * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value,
/// * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible,
/// * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer,
/// *                                        nor the detection of trailer presence/absence is possible.
/// *
/// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrailerPresenceInformation instead.
/// * @category: Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VehicleLengthConfidenceIndication {
    NoTrailerPresent = 0,
    TrailerPresentWithKnownLength = 1,
    TrailerPresentWithUnknownLength = 2,
    TrailerPresenceIsUnknown = 3,
    Unavailable = 4,
}

///*
/// * This DF represents the length of vehicle and accuracy indication information.
/// *
/// * It shall include the following components:
/// *
/// * @field vehicleLengthValue: length of vehicle.
/// *
/// * @field trailerPresenceInformation: information about the trailer presence.
/// *
/// * @category: Vehicle information
/// * @revision: created in V2.1.1 based on @ref VehicleLength but using @ref TrailerPresenceInformation.
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleLengthV2 {
    pub vehicle_length_value: VehicleLengthValue,
    pub trailer_presence_information: TrailerPresenceInformation,
}

impl VehicleLengthV2 {
    pub fn new(
        vehicle_length_value: VehicleLengthValue,
        trailer_presence_information: TrailerPresenceInformation,
    ) -> Self {
        Self {
            vehicle_length_value,
            trailer_presence_information,
        }
    }
}

///*
/// * This DE represents the length of a vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 1022`) to indicate the applicable value n is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,
/// * - `1 022` to indicate that the vehicle length is greater than 102.1 metres,
/// * - `1 023` to indicate that the information in unavailable.
/// *
/// *
/// * @unit: 0,1 metre
/// * @category: Vehicle information
/// * @revision: Description updated in V2.1.1 (the meaning of 1 022 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=1023"))]
pub struct VehicleLengthValue(pub u16);

///*
/// * This DE represents the mass of an empty loaded vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 1023`) to indicate that the applicable value is equal to or less than n x 10^5 gramm, and greater than (n-1) x 10^5 gramm,
/// * - `1 023` indicates that the vehicle mass is greater than 102 200 000 g,
/// * - `1 024` indicates  the vehicle mass information is unavailable.
/// *
/// * @note:	The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// *
/// * @unit: 10^5 gramm
/// * @category: Vehicle information
/// * @revision: Description updated in V2.1.1 (the meaning of 1 023 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=1024"))]
pub struct VehicleMass(pub u16);

///*
/// * This DE indicates the role played by a vehicle at a point in time.
/// *
/// * The value shall be set to:
/// * - 0 `default`          - to indicate the default vehicle role as indicated by the vehicle type,
/// * - 1 `publicTransport`  - to indicate that the vehicle is used to operate public transport service,
/// * - 2 `specialTransport` - to indicate that the vehicle is used for special transport purpose, e.g. oversized trucks,
/// * - 3 `dangerousGoods`   - to indicate that the vehicle is used for dangerous goods transportation,
/// * - 4 `roadWork`         - to indicate that the vehicle is used to realize roadwork or road maintenance mission,
/// * - 5 `rescue`           - to indicate that the vehicle is used for rescue purpose in case of an accident, e.g. as a towing service,
/// * - 6 `emergency`        - to indicate that the vehicle is used for emergency mission, e.g. ambulance, fire brigade,
/// * - 7 `safetyCar`        - to indicate that the vehicle is used for public safety, e.g. patrol,
/// * - 8 `agriculture`      - to indicate that the vehicle is used for agriculture, e.g. farm tractor,
/// * - 9 `commercial`       - to indicate that the vehicle is used for transportation of commercial goods,
/// * - 10 `military`        - to indicate that the vehicle is used for military purpose,
/// * - 11 `roadOperator`    - to indicate that the vehicle is used in road operator missions,
/// * - 12 `taxi`            - to indicate that the vehicle is used to provide an authorized taxi service,
/// * - 13 `uvar`            - to indicate that the vehicle is authorized to enter a zone according to the applicable Urban Vehicle Access Restrictions.
/// * - 14 `rfu1`            - is reserved for future usage.
/// * - 15 `rfu2`            - is reserved for future usage.
/// *
/// * @category: Vehicle Information
/// * @revision: Description updated in V2.1.1 (removed reference to CEN/TS 16157-3), value 13 assigned in V2.2.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VehicleRole {
    Default = 0,
    PublicTransport = 1,
    SpecialTransport = 2,
    DangerousGoods = 3,
    RoadWork = 4,
    Rescue = 5,
    Emergency = 6,
    SafetyCar = 7,
    Agriculture = 8,
    Commercial = 9,
    Military = 10,
    RoadOperator = 11,
    Taxi = 12,
    Uvar = 13,
    Rfu1 = 14,
    Rfu2 = 15,
}

///*
/// * This DE represents the width of a vehicle, excluding side mirrors and possible similar extensions.
///
/// * The value shall be set to:
/// * - `n` (`n > 0` and `n < 61`) indicates the applicable value is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,
/// * - `61` indicates that the vehicle width is greater than 6,0 metres,
/// * - `62` indicates that the information in unavailable.
/// *
/// * @unit: 0,1 metre
/// * @category: Vehicle information
/// * @revision: Description updated in V2.1.1 (the meaning of 61 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=62"))]
pub struct VehicleWidth(pub u8);

///*
/// * This DF represents a velocity vector with associated confidence value.
/// *
/// * The following options are available:
/// *
/// * @field polarVelocity: the representation of the velocity vector in a polar or cylindrical coordinate system.
/// *
/// * @field cartesianVelocity: the representation of the velocity vector in a cartesian coordinate system.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Velocity3dWithConfidence {
    PolarVelocity(VelocityPolarWithZ),
    CartesianVelocity(VelocityCartesian),
}

///*
/// * This DF represents a velocity vector in a cartesian coordinate system.
///
/// * It shall include the following components:
/// *
/// * @field xVelocity: the x component of the velocity vector with the associated confidence value.
/// *
/// * @field yVelocity: the y component of the velocity vector with the associated confidence value.
/// *
/// * @field zVelocity: the optional z component of the velocity vector with the associated confidence value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VelocityCartesian {
    pub x_velocity: VelocityComponent,
    pub y_velocity: VelocityComponent,
    pub z_velocity: Option<VelocityComponent>,
}

impl VelocityCartesian {
    pub fn new(
        x_velocity: VelocityComponent,
        y_velocity: VelocityComponent,
        z_velocity: Option<VelocityComponent>,
    ) -> Self {
        Self {
            x_velocity,
            y_velocity,
            z_velocity,
        }
    }
}

///*
/// * This DF represents a component of the velocity vector and the associated confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field value: the value of the component.
/// *
/// * @field confidence: the confidence value of the value.
/// *
/// * @category: Kinematic information
/// * @revision: V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VelocityComponent {
    pub value: VelocityComponentValue,
    pub confidence: SpeedConfidence,
}

impl VelocityComponent {
    pub fn new(value: VelocityComponentValue, confidence: SpeedConfidence) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DE represents the value of a velocity component in a defined coordinate system.
/// *
/// * The value shall be set to:
/// * - `-16 383` if the velocity is equal to or smaller than -163,83 m/s,
/// * - `n` (`n > -16 383` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,
/// * - `16 382` for velocity values equal to or greater than 163,81 m/s,
/// * - `16 383` if the velocity information is not available.
/// *
/// * @unit: 0,01 m/s
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-16383..=16383"))]
pub struct VelocityComponentValue(pub i16);

///*
/// * This DF represents a velocity vector in a polar or cylindrical coordinate system.
/// *
/// * It shall include the following components:
/// *
/// * @field velocityMagnitude: magnitude of the velocity vector on the reference plane, with the associated confidence value.
/// *
/// * @field velocityDirection: polar angle of the velocity vector on the reference plane, with the associated confidence value.
/// *
/// * @field zVelocity: the optional z component of the velocity vector along the reference axis of the cylindrical coordinate system, with the associated confidence value.
/// *
/// * @category: Kinematic information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VelocityPolarWithZ {
    pub velocity_magnitude: Speed,
    pub velocity_direction: CartesianAngle,
    pub z_velocity: Option<VelocityComponent>,
}

impl VelocityPolarWithZ {
    pub fn new(
        velocity_magnitude: Speed,
        velocity_direction: CartesianAngle,
        z_velocity: Option<VelocityComponent>,
    ) -> Self {
        Self {
            velocity_magnitude,
            velocity_direction,
            z_velocity,
        }
    }
}

/// four and more octets length
///*
/// * This DF indicates the vehicle acceleration at vertical direction and the associated confidence value.
/// *
/// * It shall include the following components:
/// *
/// * @field verticalAccelerationValue: vertical acceleration value at a point in time.
/// *
/// * @field verticalAccelerationConfidence: confidence value of the vertical acceleration value with a predefined confidence level.
/// *
/// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead.
/// * @category Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VerticalAcceleration {
    pub vertical_acceleration_value: VerticalAccelerationValue,
    pub vertical_acceleration_confidence: AccelerationConfidence,
}

impl VerticalAcceleration {
    pub fn new(
        vertical_acceleration_value: VerticalAccelerationValue,
        vertical_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            vertical_acceleration_value,
            vertical_acceleration_confidence,
        }
    }
}

///*
/// * This DE represents the vehicle acceleration at vertical direction in the centre of the mass of the empty vehicle.
/// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
/// *
/// * The value shall be set to:
/// * - `-160` for acceleration values equal to or less than -16 m/s^2,
/// * - `n` (`n > -160` and `n <= 0`) to indicate downwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `n` (`n > 0` and `n < 160`) to indicate upwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
/// * - `160` for acceleration values greater than 15,9 m/s^2,
/// * - `161` when the data is unavailable.
/// *
/// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// *
/// * @category: Vehicle information
/// * @unit: 0,1 m/s^2
/// * @revision: Desciption updated in V2.1.1 (the meaning of 160 has changed slightly).
/// *  
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct VerticalAccelerationValue(pub i16);

///*
/// * This DF provides information about a VRU cluster.
/// *
/// * It shall include the following components:
/// *
/// * @field clusterId: optional identifier of a VRU cluster.
/// *
/// * @field clusterBoundingBoxShape: optionally indicates the shape of the cluster bounding box, per default inside an East-North-Up coordinate system
/// * centered around a reference point defined outside of the context of this DF.
/// *
/// * @field clusterCardinalitySize: indicates an estimation of the number of VRUs in the group, e.g. the known members in the cluster + 1 (for the cluster leader) .
/// *
/// * @field clusterProfiles: optionally identifies all the VRU profile types that are estimated to be within the cluster.
/// * if this component is absent it means that the information is unavailable.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, description revised in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruClusterInformation {
    pub cluster_id: Option<Identifier1B>,
    #[rasn(value("0.."))]
    pub cluster_bounding_box_shape: Option<Shape>,
    pub cluster_cardinality_size: CardinalNumber1B,
    pub cluster_profiles: Option<VruClusterProfiles>,
}

impl VruClusterInformation {
    pub fn new(
        cluster_id: Option<Identifier1B>,
        cluster_bounding_box_shape: Option<Shape>,
        cluster_cardinality_size: CardinalNumber1B,
        cluster_profiles: Option<VruClusterProfiles>,
    ) -> Self {
        Self {
            cluster_id,
            cluster_bounding_box_shape,
            cluster_cardinality_size,
            cluster_profiles,
        }
    }
}

///*
/// * This DE Identifies all the VRU profile types within a cluster.
/// * It consist of a Bitmap encoding VRU profiles, to allow multiple profiles to be indicated in a single cluster (heterogeneous cluster if more than one profile).
/// *
/// * The corresponding bit shall be set to 1 under the following conditions:
/// * - 0 `pedestrian`  - indicates that the VRU cluster contains at least one pedestrian VRU,
/// * - 1 `bicycle`     - indicates that the VRU cluster contains at least one bicycle VRU member,
/// * - 2 `motorcyclist`- indicates that the VRU cluster contains at least one motorcycle VRU member,
/// * - 3 `animal`      - indicates that the VRU cluster contains at least one animal VRU member.
/// *
/// * Otherwise, the corresponding bit shall be set to 0.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct VruClusterProfiles(pub BitString);

///*
/// * This DE represents the possible usage conditions of the VRU device.
///
/// * - The value shall be set to:
/// * - 0 `unavailable`      - to indicate that the usage conditions are unavailable,
/// * - 1 `other`            - to indicate that the VRU device is in a state not defined below,
/// * - 2 `idle`             - to indicate that the human is currently not interacting with the device,
/// * - 3 `listeningToAudio` - to indicate that any audio source other than calling is in use,
/// * - 4 `typing`           - to indicate that the human is texting or performaing any other manual input activity,
/// * - 5 `calling`          - to indicate that the VRU device is currently receiving a call,
/// * - 6 `playingGames`     - to indicate that the human is playing games,
/// * - 7 `reading`          - to indicate that the human is reading on the VRU device,
/// * - 8 `viewing`          - to indicate that the human is watching dynamic content, including following navigation prompts, viewing videos or other visual contents that are not static.
/// * - value 9 to 15        - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruDeviceUsage(pub u8);

///*
/// * This DE represents the possible VRU environment conditions.
/// *
/// * - The value shall be set to:
/// * - 0 `unavailable`            - to indicate that the information on the type of environment is unavailable,
/// * - 1 `intersectionCrossing`   - to indicate that the VRU is on an intersection or crossing,
/// * - 2 `zebraCrossing`          - to indicate that the VRU is on a  zebra crossing (crosswalk),
/// * - 3 `sidewalk`               - to indicate that the VRU is on a sidewalk,
/// * - 4 `onVehicleRoad`          - to indicate that the VRU is on a traffic lane,
/// * - 5 `protectedGeographicArea`- to indicate that the VRU is in a protected area.
/// * - value 6 to 15              - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruEnvironment(pub u8);

///*
/// * This DF represents the status of the exterior light switches of a VRU.
/// * This DF is an extension of the vehicular DE @ref ExteriorLights.
/// *
/// * It shall include the following components:
/// *
/// * @field vehicular:  represents the status of the exterior light switches of a road vehicle.
/// *
/// * @field vruSpecific: represents the status of the exterior light switches of a VRU.
/// *
/// * @category: VRU information
/// * @revision: created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruExteriorLights {
    pub vehicular: ExteriorLights,
    pub vru_specific: VruSpecificExteriorLights,
}

impl VruExteriorLights {
    pub fn new(vehicular: ExteriorLights, vru_specific: VruSpecificExteriorLights) -> Self {
        Self {
            vehicular,
            vru_specific,
        }
    }
}

///*
/// *  This DE indicates the status of the possible human control over a VRU vehicle.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`                 - to indicate that the information is unavailable,
/// * - 1 `braking`                     - to indicate that the VRU is braking,
/// * - 2 `hardBraking`                 - to indicate that the VRU is braking hard,
/// * - 3 `stopPedaling`                - to indicate that the VRU stopped pedaling,
/// * - 4 `brakingAndStopPedaling`      - to indicate that the VRU stopped pedaling an is braking,
/// * - 5 `hardBrakingAndStopPedaling`  - to indicate that the VRU stopped pedaling an is braking hard,
/// * - 6 `noReaction`                  - to indicate that the VRU is not changing its behavior.
/// * - 7 to 15                         - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruMovementControl(pub u8);

///*
/// * This DF indicates the profile of a VRU including sub-profile information
/// * It identifies four options corresponding to the four types of VRU profiles specified in ETSI TS 103 300-2 [18]:
/// *
/// * @field pedestrian: VRU Profile 1 - Pedestrian.
/// *
/// * @field bicyclistAndLightVruVehicle: VRU Profile  2 - Bicyclist.
/// *
/// * @field motorcyclist: VRU Profile 3  - Motorcyclist.
/// *
/// * @field animal: VRU Profile  4 -  Animal.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VruProfileAndSubprofile {
    Pedestrian(VruSubProfilePedestrian),
    BicyclistAndLightVruVehicle(VruSubProfileBicyclist),
    Motorcyclist(VruSubProfileMotorcyclist),
    Animal(VruSubProfileAnimal),
}

///*
/// * This DE indicates the approximate size of a VRU including the VRU vehicle used.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`    - to indicate that there is no matched size class or due to privacy reasons in profile 1,
/// * - 1 `low`            - to indicate that the VRU size class is low depending on the VRU profile,
/// * - 2 `medium`         - to indicate that the VRU size class is medium depending on the VRU profile,
/// * - 3 `high`           - to indicate that the VRU size class is high depending on the VRU profile.
/// * - 4 to 15            - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruSizeClass(pub u8);

///*
/// * This DE describes the status of the exterior light switches of a VRU.
/// *
/// * The value of each bit indicates the state of the switch, which commands the corresponding light.
/// * The bit corresponding to a specific light shall be set to 1, when the corresponding switch is turned on, either manually by the driver or VRU
/// * or automatically by a vehicle or VRU system:
/// * - 0 `unavailable`     - indicates no information available,
/// * - 1 `backFlashLight ` - indicates the status of the back flash light,
/// * - 2 `helmetLight`     - indicates the status of the helmet light,
/// * - 3 `armLight`        - indicates the status of the arm light,
/// * - 4 `legLight`        - indicates the status of the leg light,
/// * - 5 `wheelLight`      - indicates the status of the wheel light.
/// * - Bits 6 to 8         - are reserved for future use.
/// * The bit values do not indicate if the corresponding lamps are alight or not.
/// * If  VRU is not equipped with a certain light or if the light switch status information is not available, the corresponding bit shall be set to 0.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct VruSpecificExteriorLights(pub BitString);

///*
/// * This DE indicates the profile of an animal
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`     - to indicate that the information  is unavailable,
/// * - 1 `wild-animal`     - to indicate a animal living in the wildness,
/// * - 2 `farm-animal`     - to indicate an animal beloning to a farm,
/// * - 3 `service-animal`  - to indicate an animal that supports a human being.
/// * - 4 to 15             - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruSubProfileAnimal(pub u8);

///*
/// * This DE indicates the profile of a VRU and its light VRU vehicle / mounted animal.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`           - to indicate that the information  is unavailable,
/// * - 1 `bicyclist `            - to indicate a cycle and bicyclist to which no more-specific profile applies,
/// * - 2 `wheelchair-user`       - to indicate a wheelchair and its user,
/// * - 3 `horse-and-rider`       - to indicate a horse and rider,
/// * - 4 `rollerskater`          - to indicate a roller-skater and skater,
/// * - 5 `e-scooter`             - to indicate an e-scooter and rider,
/// * - 6 `personal-transporter`  - to indicate a personal-transporter and rider,
/// * - 7 `pedelec`               - to indicate a pedelec and rider to which no more-specific profile applies,
/// * - 8 `speed-pedelec`         - to indicate a speed-pedelec and rider.
/// * - 9 `roadbike`              - to indicate a road bicycle (or road pedelec) and rider,
/// * - 10 `childrensbike`        - to indicate a children�s bicycle (or children�s pedelec) and rider,
/// * - 11 to 15                  - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, values 9 and 10 assigned in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruSubProfileBicyclist(pub u8);

///*
/// * This DE indicates the profile of a motorcyclist and corresponding vehicle.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable `                  - to indicate that the information  is unavailable,
/// * - 1 `moped`                         - to indicate a moped and rider,
/// * - 2 `motorcycle`                    - to indicate a motorcycle and rider,
/// * - 3 `motorcycle-and-sidecar-right`  - to indicate a motorcycle with sidecar on the right and rider,
/// * - 4 `motorcycle-and-sidecar-left`   - to indicate  a motorcycle with sidecar on the left and rider.
/// * - 5 to 15                           - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruSubProfileMotorcyclist(pub u8);

///*
/// * This DE indicates the profile of a pedestrian.
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`             - to indicate that the information on is unavailable,
/// * - 1 `ordinary-pedestrian`     - to indicate a pedestrian to which no more-specific profile applies,
/// * - 2 `road-worker`             - to indicate a pedestrian with the role of a road worker,
/// * - 3 `first-responder`         - to indicate a pedestrian with the role of a first responder.
/// * - value 4 to 15               - are reserved for future usage.
/// *
/// * @category: VRU information
/// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct VruSubProfilePedestrian(pub u8);

///*
/// * This DE represents the World Manufacturer Identifier (WMI). The values are assigned according to ISO 3779 [6].
/// *
/// *
/// * @category: Vehicle information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3"))]
pub struct WMInumber(pub Ia5String);

///*
/// * This DF represents an angular component along with a confidence value in the WGS84 coordinate system.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// *
/// * It shall include the following components:
/// *
/// * @field value: the angle value, which can be estimated as the mean of the current distribution.
/// *
/// * @field confidence: the confidence value associated to the angle value.
/// *
/// * @category: GeoReference information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Wgs84Angle {
    pub value: Wgs84AngleValue,
    pub confidence: Wgs84AngleConfidence,
}

impl Wgs84Angle {
    pub fn new(value: Wgs84AngleValue, confidence: Wgs84AngleConfidence) -> Self {
        Self { value, confidence }
    }
}

///*
/// * This DE indicates the angle confidence value which represents the estimated accuracy of an angle value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 1` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degrees and more than (n-1) x 0,1 degrees,
/// * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,
/// * - `127` if the confidence value is not available.
/// *
/// *
/// * @unit 0,1 degrees
/// * @category: GeoReference Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct Wgs84AngleConfidence(pub u8);

///*
/// * This DE represents an angle value in degrees described in the WGS84 reference system with respect to the WGS84 north.
/// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
/// * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used.
/// *
/// * @unit 0,1 degrees
/// * @category: GeoReference Information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct Wgs84AngleValue(pub u16);

///*
/// * This DE indicates the perpendicular distance between front and rear axle of the wheel base of vehicle.
/// *
/// * The value shall be set to:
/// * - `n` (`n >= 1` and `n < 126`) if the value is equal to or less than n x 0,1 metre  and more than (n-1) x 0,1 metre,
/// * - `126` indicates that the wheel base distance is equal to or greater than 12,5 metres,
/// * - `127` indicates that the information is unavailable.
/// *
/// * @unit 0,1 metre
/// * @category: Vehicle information
/// * @revision: Created in V2.1.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct WheelBaseVehicle(pub u8);

///*
/// * This DE represents the sub cause codes of the @ref CauseCode `wrongWayDriving` .
/// *
/// * The value shall be set to:
/// * - 0 `unavailable`    - in case further detailed information on wrong way driving event is unavailable,
/// * - 1 `wrongLane`      - in case vehicle is driving on a lane for which it has no authorization to use,
/// * - 2 `wrongDirection` - in case vehicle is driving in a direction that it is not allowed,
/// * - 3-255              - reserved for future usage.
/// *
/// * @category: Traffic information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WrongWayDrivingSubCauseCode(pub u8);

///*
/// * This DF represents a yaw rate of vehicle at a point in time.
/// *
/// * It shall include the following components:
/// *
/// * @field yawRateValue: yaw rate value at a point in time.
/// *
/// * @field yawRateConfidence: confidence value associated to the yaw rate value.
/// *
/// * @category: Vehicle Information
/// * @revision: V1.3.1
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct YawRate {
    pub yaw_rate_value: YawRateValue,
    pub yaw_rate_confidence: YawRateConfidence,
}

impl YawRate {
    pub fn new(yaw_rate_value: YawRateValue, yaw_rate_confidence: YawRateConfidence) -> Self {
        Self {
            yaw_rate_value,
            yaw_rate_confidence,
        }
    }
}

///*
/// * This DE indicates the yaw rate confidence value which represents the estimated accuracy for a yaw rate value with a default confidence level of 95 %.
/// * If required, the confidence level can be defined by the corresponding standards applying this DE.
/// *
/// * The value shall be set to:
/// * - `0` if the confidence value is equal to or less than 0,01 degree/second,
/// * - `1` if the confidence value is equal to or less than 0,05 degrees/second or greater than 0,01 degree/second,
/// * - `2` if the confidence value is equal to or less than 0,1 degree/second or greater than 0,05 degree/second,
/// * - `3` if the confidence value is equal to or less than 1 degree/second or greater than 0,1 degree/second,
/// * - `4` if the confidence value is equal to or less than 5 degrees/second or greater than 1 degrees/second,
/// * - `5` if the confidence value is equal to or less than 10 degrees/second or greater than 5 degrees/second,
/// * - `6` if the confidence value is equal to or less than 100 degrees/second or greater than 10 degrees/second,
/// * - `7` if the confidence value is out of range, i.e. greater than 100 degrees/second,
/// * - `8` if the confidence value is unavailable.
/// *
/// * NOTE: The fact that a yaw rate value is received with confidence value set to `unavailable(8)` can be caused by
/// * several reasons, such as:
/// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
/// * - the sensor cannot calculate the accuracy due to lack of variables, or
/// * - there has been a vehicle bus (e.g. CAN bus) error.
/// * In all 3 cases above, the yaw rate value may be valid and used by the application.
/// *
/// * If a yaw rate value is received and its confidence value is set to `outOfRange(7)`, it means that the
/// * yaw rate value is not valid and therefore cannot be trusted. Such value is not useful the application.
/// *
/// * @category: Vehicle information
/// * @revision: Description revised in V2.1.1
///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum YawRateConfidence {
    DegSec00001 = 0,
    DegSec00005 = 1,
    DegSec00010 = 2,
    DegSec00100 = 3,
    DegSec00500 = 4,
    DegSec01000 = 5,
    DegSec10000 = 6,
    OutOfRange = 7,
    Unavailable = 8,
}

///*
/// * This DE represents the vehicle rotation around z-axis of the coordinate system centred on the centre of mass of the empty-loaded
/// * vehicle. The leading sign denotes the direction of rotation.
/// *
/// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
/// *
/// * The value shall be set to:
/// * - `-32 766` to indicate that the yaw rate is equal to or greater than 327,66 degrees/second to the right,
/// * - `n` (`n > -32 766` and `n <= 0`) to indicate that the rotation is clockwise (i.e. to the right) and is equal to or less than n x 0,01 degrees/s,
///      and greater than (n-1) x 0,01 degrees/s,
/// * - `n` (`n > 0` and `n < 32 766`) to indicate that the rotation is anti-clockwise (i.e. to the left) and is equal to or less than n x 0,01 degrees/s,
///      and greater than (n-1) x 0,01 degrees/s,
/// * - `32 766` to indicate that the yaw rate is greater than 327.65 degrees/second to the left,
/// * - `32 767` to indicate that the information is not available.
/// *
/// * The yaw rate value shall be a raw data value, i.e. not filtered, smoothed or otherwise modified.
/// * The reading instant should be the same as for the vehicle acceleration.
/// *
/// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
/// *
/// * @unit: 0,01 degree per second.
/// * @category: Vehicle Information
/// * @revision: Desription revised in V2.1.1 (the meaning of 32766 has changed slightly).
///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-32766..=32767"))]
pub struct YawRateValue(pub i16);
