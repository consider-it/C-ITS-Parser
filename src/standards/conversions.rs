// Copyright (c) 2025 consider it GmbH

//! Common data conversions
//!
//! ETSI types to common (SI) units

macro_rules! latlon_to_deg {
    ($t:ty) => {
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
        }
    };
}

latlon_to_deg!(crate::standards::is_1_3_1::etsi_schema::Longitude);
latlon_to_deg!(crate::standards::is_1_3_1::etsi_schema::Latitude);
latlon_to_deg!(crate::standards::cdd_1_3_1_1::its_container::Longitude);
latlon_to_deg!(crate::standards::cdd_1_3_1_1::its_container::Latitude);
latlon_to_deg!(crate::standards::cdd_2_2_1::etsi_its_cdd::Longitude);
latlon_to_deg!(crate::standards::cdd_2_2_1::etsi_its_cdd::Latitude);

// latlon_to_deg!(crate::standards::is_1_3_1::etsi_schema::DeltaLongitude);
// latlon_to_deg!(crate::standards::is_1_3_1::etsi_schema::DeltaLatitude);
latlon_to_deg!(crate::standards::cdd_1_3_1_1::its_container::DeltaLongitude);
latlon_to_deg!(crate::standards::cdd_1_3_1_1::its_container::DeltaLatitude);
latlon_to_deg!(crate::standards::cdd_2_2_1::etsi_its_cdd::DeltaLongitude);
latlon_to_deg!(crate::standards::cdd_2_2_1::etsi_its_cdd::DeltaLatitude);

macro_rules! offset_to_meters {
    ($t:ty) => {
        impl $t {
            /// convert ETSI OffsetB* to meters
            #[must_use]
            pub fn as_meters(&self) -> f32 {
                f32::from(self.0) / 100.
            }

            /// create ETSI OffsetB* from meters
            ///
            /// # Errors
            /// human-readable string when input value is out of bounds
            pub fn from_meters(value: f32) -> Result<Self, String> {
                use rasn::AsnType;

                #[allow(clippy::cast_possible_truncation)]
                let etsi_val = (value * 100.) as i16;

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

offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB09);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB10);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB11);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB12);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB13);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB14);
offset_to_meters!(crate::standards::is_1_3_1::etsi_schema::OffsetB16);
