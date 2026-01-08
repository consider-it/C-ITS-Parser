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
                f32::from(self.0) / $conv
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

/// Create conversions for ETSI type `t` with conversion factor `conv` and some "unavailable" value
macro_rules! angle_to_deg {
    ($t:ty, $conv:expr, $unavailable:expr) => {
        impl $t {
            /// convert ETSI WGS84AngleValue/ CartesianAngleValue to degrees
            #[must_use]
            pub fn as_deg(&self) -> f32 {
                f32::from(self.0) / $conv
            }

            /// create ETSI WGS84AngleValue/ CartesianAngleValue from degrees
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_deg(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * $conv) as u16;

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

angle_to_deg!(is_1_3_1::etsi_schema::CartesianAngleValue, 10., 3601); // Unit: 0,1 degrees
angle_to_deg!(is_1_3_1::etsi_schema::WGS84AngleValue, 10., 3601); // Unit: 0,1 degrees
angle_to_deg!(is_1_3_1::etsi_schema::Angle, 80., 28800); // Unit: 0.0125 degrees

// DeltaTime: unit 10 seconds, clamping to -121 for <-20 minutes and +120 for >+20 minutes, -122 for unavailable
impl is_1_3_1::etsi_schema::DeltaTime {
    /// convert ETSI DeltaTime to seconds
    #[must_use]
    pub fn as_sec(&self) -> i16 {
        i16::from(self.0) * 10
    }

    /// create ETSI DeltaTime from seconds, clamping at min. and max. bounds
    pub fn from_sec(value: i16) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let etsi_val = (value / 10) as i8;

        Self(etsi_val.clamp(-121, 120))
    }

    /// create ETSI DeltaTime with "unavailable" value
    pub fn unavailable() -> Self {
        Self(-122)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == -122
    }
}

impl From<&is_1_3_1::etsi_schema::DeltaTime> for i16 {
    fn from(other: &is_1_3_1::etsi_schema::DeltaTime) -> i16 {
        other.as_sec()
    }
}
impl From<is_1_3_1::etsi_schema::DeltaTime> for i16 {
    fn from(other: is_1_3_1::etsi_schema::DeltaTime) -> i16 {
        other.as_sec()
    }
}

// DSecond: unit milliseconds, 65535 for unavailable
impl is_1_3_1::etsi_schema::DSecond {
    /// convert ETSI DeltaTime to milliseconds
    #[must_use]
    pub fn as_millis(&self) -> u16 {
        self.0
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

    /// create ETSI DSecond with "unavailable" value
    pub fn unavailable() -> Self {
        Self(65535)
    }

    /// determines if the ETSI value is special "unavailable" value
    pub fn is_unavailable(&self) -> bool {
        self.0 == 65535
    }
}

impl From<&is_1_3_1::etsi_schema::DSecond> for u16 {
    fn from(other: &is_1_3_1::etsi_schema::DSecond) -> u16 {
        other.as_millis()
    }
}
impl From<is_1_3_1::etsi_schema::DSecond> for u16 {
    fn from(other: is_1_3_1::etsi_schema::DSecond) -> u16 {
        other.as_millis()
    }
}

// MinuteOfTheYear: unit minute, 527040 for invalid
impl is_1_3_1::etsi_schema::MinuteOfTheYear {
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
impl crate::standards::is_1_3_1::etsi_schema::MsgCount {
    pub fn increment(&self) -> Self {
        Self((self.0 + 1) % 128)
    }
}
impl From<u8> for is_1_3_1::etsi_schema::MsgCount {
    fn from(value: u8) -> Self {
        Self(value % 128)
    }
}

// RequestID 0..255
impl crate::standards::is_1_3_1::etsi_schema::RequestID {
    pub fn increment(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
impl From<u8> for is_1_3_1::etsi_schema::RequestID {
    // for convenience and interface unification only
    fn from(value: u8) -> Self {
        Self(value)
    }
}

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
