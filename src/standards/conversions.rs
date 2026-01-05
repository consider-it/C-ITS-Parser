// Copyright (c) 2025 consider it GmbH

//! Common data conversions
//!
//! ETSI types to common (SI) units

use crate::standards::cdd_1_3_1_1;
use crate::standards::cdd_2_2_1;
use crate::standards::is_1_3_1;

/// Create conversions for ETSI type `t` and some "unavailable" value
macro_rules! latlon_to_deg {
    ($t:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn as_deg(&self) -> f64 {
                f64::from(self.0) / 10_000_000.
            }

            /// convert ETSI Latitude/ Longitude to degrees
            #[must_use]
            pub fn from_deg(other: f64) -> Self {
                Self((other * 10_000_000.) as i32)
            }

            /// create ETSI Latitude/ Longitude with "unavailable" value
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

latlon_to_deg!(is_1_3_1::etsi_schema::Longitude, 1_800_000_001);
latlon_to_deg!(is_1_3_1::etsi_schema::Latitude, 900_000_001);
latlon_to_deg!(cdd_1_3_1_1::its_container::Longitude, 1_800_000_001);
latlon_to_deg!(cdd_1_3_1_1::its_container::Latitude, 900_000_001);
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Longitude, 1_800_000_001);
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::Latitude, 900_000_001);

// latlon_to_deg!(is_1_3_1::etsi_schema::DeltaLongitude, 131_072);
// latlon_to_deg!(is_1_3_1::etsi_schema::DeltaLatitude, 131_072);
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLongitude, 131_072);
latlon_to_deg!(cdd_1_3_1_1::its_container::DeltaLatitude, 131_072);
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLongitude, 131_072);
latlon_to_deg!(cdd_2_2_1::etsi_its_cdd::DeltaLatitude, 131_072);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv`
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

etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB09, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB10, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB11, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB12, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB13, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB14, i16, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::OffsetB16, i16, 100.);

etsi_to_meters!(is_1_3_1::etsi_schema::DistanceValue, i32, 100.);
etsi_to_meters!(is_1_3_1::etsi_schema::ObjectDimensionValue, u16, 10.);
etsi_to_meters!(is_1_3_1::etsi_schema::Radius, u16, 10.);
etsi_to_meters!(is_1_3_1::etsi_schema::Range, u16, 10.);
etsi_to_meters!(is_1_3_1::etsi_schema::SemiRangeLength, u16, 10.);

/// Create conversions for ETSI type `t` (which has underlying data type `tt`) with conversion factor `conv` and some "unavailable" value
macro_rules! etsi_to_mps {
    ($t:ty, $tt:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI speed to m/s
            pub fn as_mps(&self) -> f32 {
                f32::from(self.0) / 100.
            }

            /// create ETSI speed from m/s
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_mps(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * 100.) as $tt;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI speed with "unavailable" value
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

etsi_to_mps!(is_1_3_1::etsi_schema::SpeedValueExtended, i16, 100., 16_383); // Unit: 0,01 m/s
etsi_to_mps!(is_1_3_1::etsi_schema::SpeedValue, u16, 100., 16_383); // Unit: 0,01 m/s

etsi_to_mps!(is_1_3_1::etsi_schema::Velocity, u16, 50., 8191); // Unit: 0.02 m/s

/// Create conversions for ETSI type `t` with some "unavailable" value
macro_rules! angle_to_deg {
    ($t:ty, $unavailable:expr) => {
        impl $t {
            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees
            #[must_use]
            pub fn as_deg(&self) -> f32 {
                f32::from(self.0) / 10.
            }

            /// create ETSI WGS84AngleValue/ CartesianAngleValue from degrees
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * 10.) as u16;

                if let Some(constraints) = Self::CONSTRAINTS.value() {
                    if !constraints.constraint.in_bound(&etsi_val) {
                        return Err(format!("Value out of bounds"));
                    }
                }

                Ok(Self(etsi_val))
            }

            /// create ETSI WGS84AngleValue/ CartesianAngleValue with "unavailable" value
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

angle_to_deg!(is_1_3_1::etsi_schema::CartesianAngleValue, 3601);
angle_to_deg!(is_1_3_1::etsi_schema::WGS84AngleValue, 3601);

// convenience getter
impl is_1_3_1::etsi_schema::SpeedLimitList {
    /// Extracts a certain speed limit in m/s, if existing
    pub fn get_speed_limit_mps(
        &self,
        limit_type: is_1_3_1::etsi_schema::SpeedLimitType,
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
