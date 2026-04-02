// Copyright (c) 2025 consider it GmbH

//! Common data conversions
//!
//! ETSI types to common (SI) units

#[cfg(feature = "_cdd_1_3_1_1")]
use crate::standards::cdd_1_3_1_1;
#[cfg(feature = "_cdd_2_2_1")]
use crate::standards::cdd_2_2_1;
#[cfg(feature = "cpm_1")]
use crate::standards::cpm_1;
#[cfg(feature = "_dsrc_2_2_1")]
use crate::standards::dsrc_2_2_1;

/// Create conversions for ETSI type `t` and some "unavailable" value
macro_rules! latlon_to_deg {
    ($t:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn as_deg(&self) -> f64 {
                f64::from(self.0) / 10_000_000.
            }

            /// convert ETSI Latitude/ Longitude to degrees or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg(&self) -> Option<f64> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg())
                }
            }

            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn from_deg(other: f64) -> Self {
                Self((other * 10_000_000.) as i32)
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::Longitude, 1_800_000_001);
#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::Latitude, 900_000_001);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Longitude, 1_800_000_001);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Latitude, 900_000_001);

#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLongitude, 131_072);
#[cfg(feature = "_cdd_1_3_1_1")]
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLatitude, 131_072);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLongitude, 131_072);
#[cfg(feature = "_cdd_2_2_1")]
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLatitude, 131_072);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv`
#[cfg(any(feature = "cpm_1", feature = "_cdd_2_2_1", feature = "_dsrc_2_2_1"))]
macro_rules! etsi_to_meters {
    ($t:ty, $tt:ty, $conv:expr) => {
        impl $t {
            /// convert ETSI data to meters
            #[must_use]
            pub fn as_meters(&self) -> f32 {
                self.0 as f32 / $conv
            }

            /// create ETSI data from meters
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_meters(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_meters()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_meters()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_meters(value)
            }
        }
    };
}

#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB09, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB10, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB11, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB12, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB13, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB14, i16, 100.);
#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_meters!(dsrc_2_2_1::etsi_its_dsrc::OffsetB16, i16, 100.);

#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::DistanceValue, i32, 100.);
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters!(cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::ObjectDimensionValue, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::Radius, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::Range, u16, 10.);
#[cfg(feature = "cpm_1")]
etsi_to_meters!(cpm_1::cpm_pdu_descriptions::SemiRangeLength, u16, 10.);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! etsi_to_meters_unavailable {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI data to meters
            #[must_use]
            pub fn as_meters(&self) -> f32 {
                self.0 as f32 / $conv
            }

            /// convert ETSI data to meters or `None` if "unavailable"
            #[must_use]
            pub fn try_as_meters(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_meters())
                }
            }

            /// create ETSI data from meters
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_meters(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_meters()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_meters()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_meters(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(cdd_1_3_1_1::its_container::VehicleWidth, u8, 10., 62);
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::VehicleWidth, u8, 10., 62);
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_meters_unavailable!(
    cdd_1_3_1_1::its_container::VehicleLengthValue,
    u16,
    10.,
    1023
);
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_meters_unavailable!(cdd_2_2_1::etsi_its_cdd::VehicleLengthValue, u16, 10., 1023);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
macro_rules! etsi_to_mps {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI speed to m/s
            #[must_use]
            pub fn as_mps(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI speed to m/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_mps(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_mps())
                }
            }

            /// create ETSI speed from m/s
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_mps(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_mps()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_mps()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_mps(value)
            }
        }
    };
}

#[cfg(feature = "cpm_1")]
etsi_to_mps!(
    cpm_1::cpm_pdu_descriptions::SpeedValueExtended,
    i16,
    100.,
    16_383
); // Unit: 0,01 m/s
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mps!(cdd_1_3_1_1::its_container::SpeedValue, u16, 100., 16_383); // Unit: 0,01 m/s
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mps!(cdd_2_2_1::etsi_its_cdd::SpeedValue, u16, 100., 16_383); // Unit: 0,01 m/s

#[cfg(feature = "_dsrc_2_2_1")]
etsi_to_mps!(dsrc_2_2_1::etsi_its_dsrc::Velocity, u16, 50., 8191); // Unit: 0.02 m/s

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! etsi_to_mpss {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI acceleration to m/s/s
            #[must_use]
            pub fn as_mpss(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI acceleration to m/s/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_mpss(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_mpss())
                }
            }

            /// create ETSI acceleration from m/s/s
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_mpss(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_mpss()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_mpss()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_mpss(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::LongitudinalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::LateralAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::LateralAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_1_3_1_1")]
etsi_to_mpss!(
    cdd_1_3_1_1::its_container::VerticalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2
#[cfg(feature = "_cdd_2_2_1")]
etsi_to_mpss!(
    cdd_2_2_1::etsi_its_cdd::VerticalAccelerationValue,
    i16,
    10.,
    161
); // Unit: 0,1 m/s^2

/// Check for unavailable data of ETSI type `t` (which has underlying data type `tt`)
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! etsi_raw_unavailable {
    ($t:ty, $tt:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI acceleration to m/s/s or `None` if "unavailable"
            #[must_use]
            pub fn try_as_raw(&self) -> Option<$tt> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.0)
                }
            }

            /// create ETSI acceleration from raw value
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_raw(value: $tt) -> Result<Self, String> {
                use rasn::AsnType;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&value) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(value))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for $tt {
            fn from(other: &$t) -> $tt {
                other.0
            }
        }
        impl From<$t> for $tt {
            fn from(other: $t) -> $tt {
                other.0
            }
        }

        impl TryFrom<$tt> for $t {
            type Error = String;

            fn try_from(value: $tt) -> Result<Self, Self::Error> {
                Self::from_raw(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_1_3_1_1")]
etsi_raw_unavailable!(cdd_1_3_1_1::its_container::CurvatureValue, i16, 1023);
#[cfg(feature = "_cdd_2_2_1")]
etsi_raw_unavailable!(cdd_2_2_1::etsi_its_cdd::CurvatureValue, i16, 1023);

/// Create conversions for ETSI type `t` with conversion factor `conv` and some "unavailable" value
#[cfg(any(
    feature = "cpm_1",
    feature = "_cdd_1_3_1_1",
    feature = "_cdd_2_2_1",
    feature = "_dsrc_2_2_1"
))]
macro_rules! angle_to_deg {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees
            #[must_use]
            pub fn as_deg(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg())
                }
            }

            /// create ETSI WGS84AngleValue/ CartesianAngleValue from degrees
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_deg()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_deg()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_deg(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::CartesianAngleValue, u16, 10., 3601); // Unit: 0,1 degrees
#[cfg(feature = "cpm_1")]
angle_to_deg!(
    cpm_1::cpm_pdu_descriptions::CartesianAngleValue,
    u16,
    10.,
    3601
); // Unit: 0,1 degrees
#[cfg(feature = "cpm_1")]
angle_to_deg!(cpm_1::cpm_pdu_descriptions::WGS84AngleValue, u16, 10., 3601); // Unit: 0,1 degrees
#[cfg(feature = "_dsrc_2_2_1")]
angle_to_deg!(dsrc_2_2_1::etsi_its_dsrc::Angle, u16, 80., 28800); // Unit: 0.0125 degrees
#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(cdd_2_2_1::etsi_its_cdd::HeadingValue, u16, 10., 3601); // Unit: 0,1 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(cdd_1_3_1_1::its_container::HeadingValue, u16, 10., 3601); // Unit: 0,1 degree

#[cfg(feature = "_cdd_2_2_1")]
angle_to_deg!(
    cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue,
    i16,
    (1. / 1.5),
    512
); // Unit: 1,5 degree
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_deg!(
    cdd_1_3_1_1::its_container::SteeringWheelAngleValue,
    i16,
    (1. / 1.5),
    512
); // Unit: 1,5 degree

/// Create conversions for ETSI type `t` with conversion factor `conv` and some "unavailable" value
#[cfg(any(feature = "_cdd_2_2_1", feature = "_cdd_1_3_1_1"))]
macro_rules! angle_to_degrate {
    ($t:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI YawRateValue to degrees
            #[must_use]
            pub fn as_deg_rate(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// convert ETSI YawRateValue to degrees or `None` if "unavailable"
            #[must_use]
            pub fn try_as_deg(&self) -> Option<f32> {
                if self.is_unavailable() {
                    None
                } else {
                    Some(self.as_deg_rate())
                }
            }

            /// create ETSI YawRateValue from degrees
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg_rate(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as i16;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI type with "unavailable" value
            pub fn unavailable() -> Self {
                Self($unavailable)
            }

            /// determines if the ETSI value is special "unavailable" value
            pub fn is_unavailable(&self) -> bool {
                self.0 == $unavailable
            }
        }

        impl From<&$t> for f32 {
            fn from(other: &$t) -> f32 {
                other.as_deg_rate()
            }
        }
        impl From<$t> for f32 {
            fn from(other: $t) -> f32 {
                other.as_deg_rate()
            }
        }

        impl TryFrom<f32> for $t {
            type Error = String;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                Self::from_deg_rate(value)
            }
        }
    };
}

#[cfg(feature = "_cdd_2_2_1")]
angle_to_degrate!(cdd_2_2_1::etsi_its_cdd::YawRateValue, 100., 3601); // Unit: 0,01 degree per second
#[cfg(feature = "_cdd_1_3_1_1")]
angle_to_degrate!(cdd_1_3_1_1::its_container::YawRateValue, 100., 32767); // Unit: 0,01 degree per second

// DeltaTime: unit 10 seconds, clamping to -121 for <-20 minutes and +120 for >+20 minutes, -122 for unavailable
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::DeltaTime {
    /// convert ETSI DeltaTime to seconds
    #[must_use]
    pub fn as_sec(&self) -> i16 {
        i16::from(self.0) * 10
    }

    /// convert ETSI DeltaTime to seconds or `None` if "unavailable"
    #[must_use]
    pub fn try_as_sec(&self) -> Option<i16> {
        if self.is_unavailable() {
            None
        } else {
            Some(self.as_sec())
        }
    }

    /// create ETSI DeltaTime from seconds, clamping at min. and max. bounds
    #[must_use]
    pub fn from_sec(value: i16) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let etsi_val = (value / 10) as i8;

        Self(etsi_val.clamp(-121, 120))
    }

    /// create ETSI type with "unavailable" value
    pub fn unavailable() -> Self {
        Self(-122)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == -122
    }
}

#[cfg(feature = "_dsrc_2_2_1")]
impl From<&dsrc_2_2_1::etsi_its_dsrc::DeltaTime> for i16 {
    fn from(other: &dsrc_2_2_1::etsi_its_dsrc::DeltaTime) -> i16 {
        other.as_sec()
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<dsrc_2_2_1::etsi_its_dsrc::DeltaTime> for i16 {
    fn from(other: dsrc_2_2_1::etsi_its_dsrc::DeltaTime) -> i16 {
        other.as_sec()
    }
}

// DSecond: unit milliseconds, 65535 for unavailable
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::DSecond {
    /// convert ETSI DeltaTime to milliseconds
    #[must_use]
    pub fn as_millis(&self) -> u16 {
        self.0
    }

    /// convert ETSI DeltaTime to milliseconds or `None` if "unavailable"
    #[must_use]
    pub fn try_as_millis(&self) -> Option<u16> {
        if self.is_unavailable() {
            None
        } else {
            Some(self.as_millis())
        }
    }

    /// create ETSI DSecond from milliseconds
    ///
    /// # Errors
    /// human-readable string when input value is out of bounds
    pub fn from_millis(value: u16) -> Result<Self, String> {
        use rasn::AsnType;

        if let Some(constraints) = Self::CONSTRAINTS.value() {
            if !constraints.constraint.in_bound(&value) {
                return Err(format!("Value out of bounds"));
            }
        }

        Ok(Self(value))
    }

    /// create ETSI type with "unavailable" value
    pub fn unavailable() -> Self {
        Self(65535)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == 65535
    }
}

#[cfg(feature = "_dsrc_2_2_1")]
impl From<&dsrc_2_2_1::etsi_its_dsrc::DSecond> for u16 {
    fn from(other: &dsrc_2_2_1::etsi_its_dsrc::DSecond) -> u16 {
        other.as_millis()
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<dsrc_2_2_1::etsi_its_dsrc::DSecond> for u16 {
    fn from(other: dsrc_2_2_1::etsi_its_dsrc::DSecond) -> u16 {
        other.as_millis()
    }
}

// MinuteOfTheYear: unit minute, 527040 for invalid
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear {
    /// create ETSI MinuteOfTheYear with "invalid" value
    pub fn invalid() -> Self {
        Self(527040)
    }

    /// determines if the ETSI value is special "invalid" value
    pub fn is_invalid(&self) -> bool {
        self.0 == 527040
    }
}

// MsgCount 0..127
#[cfg(feature = "_dsrc_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::MsgCount {
    pub fn increment(&self) -> Self {
        Self((self.0 + 1) % 128)
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<u8> for dsrc_2_2_1::etsi_its_dsrc::MsgCount {
    fn from(value: u8) -> Self {
        Self(value % 128)
    }
}

// RequestID 0..255
#[cfg(feature = "_dsrc_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::RequestID {
    pub fn increment(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<u8> for dsrc_2_2_1::etsi_its_dsrc::RequestID {
    // for convenience and interface unification only
    fn from(value: u8) -> Self {
        Self(value)
    }
}

// convenience getter
#[cfg(feature = "_dsrc_2_2_1")]
impl dsrc_2_2_1::etsi_its_dsrc::SpeedLimitList {
    /// Extracts a certain speed limit in m/s, if existing
    pub fn get_speed_limit_mps(
        &self,
        limit_type: dsrc_2_2_1::etsi_its_dsrc::SpeedLimitType,
    ) -> Option<f32> {
        self.0.iter().find_map(|item| {
            if item.r_type == limit_type {
                Some(item.speed.as_mps())
            } else {
                None
            }
        })
    }
}
