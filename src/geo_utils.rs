//! Conversions from ETSI to geo-types

const EARTH_CIRCUMFERENCE: f32 = 39_940_653.; // Earth's circumference in meters

// used by IS 1.3.1
impl From<crate::standards::is_1_3_1::etsi_schema::Position3D> for geo_types::Point {
    fn from(other: crate::standards::is_1_3_1::etsi_schema::Position3D) -> Self {
        geo_types::Point::new(other.long.as_deg(), other.lat.as_deg())
    }
}

/// convert ETSI ReferencePosition to [`geo_types::Point`]
macro_rules! refpos_to_point {
    ($t:ty) => {
        impl From<$t> for geo_types::Point {
            fn from(other: $t) -> Self {
                geo_types::Point::new(other.longitude.as_deg(), other.latitude.as_deg())
            }
        }
    };
}

// used by DENM 1.3.1 and CAM
refpos_to_point!(crate::standards::cdd_1_3_1_1::its_container::ReferencePosition);

// used by DENM 2.2.1
refpos_to_point!(crate::standards::cdd_2_2_1::etsi_its_cdd::ReferencePosition);

// used by CPM from IS 1.3.1
refpos_to_point!(crate::standards::is_1_3_1::etsi_schema::ReferencePosition);

// ETSI PathHistory

macro_rules! ph_to_line_string {
    ($t:ty) => {
        impl $t {
            /// Resolve delta positions to absolute geo positions
            ///
            /// Output geo positions are in degrees lon/lat.
            ///
            /// Input data is assumed to be in ETSI format. This means that the first point is
            /// relative to the `ref_pos` and subsequent points are relative to the point before.
            #[allow(
                clippy::missing_panics_doc,
                reason = "panic on last().unwrap() is impossible here"
            )]
            #[must_use]
            pub fn to_line_string(&self, ref_pos: geo_types::Point) -> geo_types::LineString {
                self.0
                    .iter()
                    .fold(vec![ref_pos], |mut acc, pt| {
                        // vector is primed with one element, so should never return None
                        let origin = acc.last().unwrap();
                        let delta = geo_types::Point::new(
                            pt.path_position.delta_longitude.as_deg(),
                            pt.path_position.delta_latitude.as_deg(),
                        );

                        acc.push(*origin + delta);
                        acc
                    })
                    .into()
            }
        }
    };
}

// used by CDD 1.3.1 (CAM 1.4.1 and DENM 1.3.1)
ph_to_line_string!(crate::standards::cdd_1_3_1_1::its_container::PathHistory);

// used by CDD 2.2.1 (DENM 2.1.1)
ph_to_line_string!(crate::standards::cdd_2_2_1::etsi_its_cdd::Path);

// Used by IS 1.3.1 (MAPEM lanes)
impl crate::standards::is_1_3_1::etsi_schema::NodeSetXY {
    /// Resolve delta positions to absolute geo positions
    ///
    /// Output geo positions are in degrees lon/lat.
    ///
    /// Input data is assumed to be in ETSI format. This means that the first point is
    /// relative to the `ref_pos` and subsequent points are relative to the point before.
    #[allow(
        clippy::missing_panics_doc,
        reason = "panic on last().unwrap() is impossible here"
    )]
    #[must_use]
    pub fn to_line_string(&self, ref_pos: geo_types::Point) -> geo_types::LineString {
        self.0
            .iter()
            .fold(vec![ref_pos], |mut acc, pt| {
                // vector is primed with one element, so should never return None
                let origin = acc.last().unwrap();
                let delta = pt.delta.to_dlat(&ref_pos);

                acc.push(*origin + delta);
                acc
            })
            .into()
    }
}

impl crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointXY {
    /// Convert to delta lon/lat (in degrees)
    ///
    /// [`NodeOffsetPointXY`] may contain XY delta positions or lat/lon deltas.
    /// XY positions will be converted to delta geo-coordinates near the `ref_pos`.
    #[must_use]
    pub fn to_dlat(&self, ref_pos: &geo_types::Point) -> geo_types::Point {
        use crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointXY;

        let (lon, lat) = match self {
            NodeOffsetPointXY::node_XY1(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY2(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY3(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY4(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY5(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY6(etsi) => {
                Self::dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_LatLon(etsi) => (etsi.lon.as_deg(), etsi.lat.as_deg()),
            NodeOffsetPointXY::regional(_) => (0., 0.),
        };

        geo_types::Point::new(lon, lat)
    }

    /// Convert to delta distance (in meters)
    ///
    /// [`NodeOffsetPointXY`] may contain XY delta positions or lat/lon deltas.
    /// lat/lon delta coordinates will be converted to XY offsets near the `ref_pos`.
    ///
    /// Output will be according to ETSI coordinate system (X pointing east, X pointing north)!
    #[must_use]
    pub fn to_ddist(&self, ref_pos: &geo_types::Point) -> geo_types::Point {
        use crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointXY;

        let (x, y) = match self {
            NodeOffsetPointXY::node_XY1(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY2(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY3(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY4(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY5(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY6(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_LatLon(etsi) => Self::dlatlon_to_cart(etsi, ref_pos),
            NodeOffsetPointXY::regional(_) => (0., 0.),
        };

        geo_types::Point::new(x.into(), y.into())
    }

    /// Convert ETSI XY deltas to lon/lat delta (in degrees)
    ///
    /// X points east (longitude), Y points north (latitude)!
    fn dxy_to_geo(dx: f32, dy: f32, ref_pos: &geo_types::Point) -> (f64, f64) {
        // latitude degree per meter is independent from longitude
        let dlat = f64::from(dy) / f64::from(EARTH_CIRCUMFERENCE) * 360.;

        // longitude degree per meter has a different value depending on the latitude
        let ref_lat_rad = ref_pos.to_radians().y();
        let dlon = f64::from(dx) / (f64::from(EARTH_CIRCUMFERENCE) * ref_lat_rad.cos()) * 360.;

        (dlon, dlat)
    }

    /// Convert lon/lat deltas to ETSI XY position (in meters)
    ///
    /// X points east (longitude), Y points north (latitude)!
    fn dlatlon_to_cart(
        dpos: &crate::standards::is_1_3_1::etsi_schema::NodeLLmD64b,
        ref_pos: &geo_types::Point,
    ) -> (f32, f32) {
        // latitude degree per meter is independent from longitude
        #[allow(clippy::cast_possible_truncation)]
        let dlat_deg = dpos.lat.as_deg() as f32;
        let dy = dlat_deg / 360. * EARTH_CIRCUMFERENCE;

        // longitude degree per meter has a different value depending on the latitude
        #[allow(clippy::cast_possible_truncation)]
        let ref_lat_rad = ref_pos.to_radians().y() as f32;
        #[allow(clippy::cast_possible_truncation)]
        let dlon_deg = dpos.lon.as_deg() as f32;
        let dx = dlon_deg / 360. * (EARTH_CIRCUMFERENCE * ref_lat_rad.cos());

        (dx, dy)
    }
}

impl crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointZ {
    #[must_use]
    pub fn to_meters(&self) -> f32 {
        use crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointZ;

        match self {
            NodeOffsetPointZ::node_Z1(etsi) => etsi.into(),
            NodeOffsetPointZ::node_Z2(etsi) => etsi.into(),
            NodeOffsetPointZ::node_Z3(etsi) => etsi.into(),
            NodeOffsetPointZ::node_Z4(etsi) => etsi.into(),
            NodeOffsetPointZ::node_Z5(etsi) => etsi.into(),
            NodeOffsetPointZ::node_Z6(etsi) => etsi.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dxy_to_geo_test() {
        use crate::standards::is_1_3_1::etsi_schema::NodeOffsetPointXY;

        let ref_pos = geo_types::point! {x: 9.936521, y: 53.550728};

        // trivial test
        assert_eq!((0., 0.), NodeOffsetPointXY::dxy_to_geo(0., 0., &ref_pos));

        // 1/10th nautical mile north/ south
        let (dlon, dlat) = NodeOffsetPointXY::dxy_to_geo(0., 185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(0., dlon);
        assert_float_eq::assert_float_absolute_eq!(0.001667, dlat);

        let (dlon, dlat) = NodeOffsetPointXY::dxy_to_geo(0., -185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(0., dlon);
        assert_float_eq::assert_float_absolute_eq!(-0.001667, dlat);

        // east/west from online calculation tool
        let (dlon, dlat) = NodeOffsetPointXY::dxy_to_geo(66., 0., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(0.001001, dlon);
        assert_float_eq::assert_float_absolute_eq!(0., dlat);

        // combine both
        let (dlon, dlat) = NodeOffsetPointXY::dxy_to_geo(66., 185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(0.001001, dlon);
        assert_float_eq::assert_float_absolute_eq!(0.001667, dlat);
    }
}
