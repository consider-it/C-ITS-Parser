//! Conversions from ETSI to geo-types
//!
//! - Positions are converted to [`geo_types::Point`]
//! - Paths ([`PathHistory`](`crate::standards::cdd_1_3_1_1::its_container::PathHistory`)/ [`Path`](`crate::standards::cdd_2_2_1::etsi_its_cdd::Path`))
//!   and MAPEM lanes [`NodeSetXY`](`crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeSetXY`) are converted to [`geo_types::LineString`]
//!
//! Take a look at the individual data types in [`crate::standards`] to discover available conversion methods and initialization functions.
//!
//!  Note: These conversions are only available with the optional `geo` feature flag.

const EARTH_CIRCUMFERENCE: f32 = 39_940_653.; // Earth's circumference in meters

// generic conversion helpers

/// Convert ETSI XY deltas to absolute lon/lat position (in degrees)
///
/// X points east (longitude), Y points north (latitude)!
#[must_use]
pub fn point_from_dxy(dx: f32, dy: f32, ref_pos: &geo_types::Point) -> geo_types::Point {
    dxy_to_geo(dx, dy, ref_pos).into()
}

// used by DSRC 2.2.1
#[cfg(feature = "_dsrc_2_2_1")]
impl From<crate::standards::dsrc_2_2_1::etsi_its_dsrc::Position3D> for geo_types::Point {
    fn from(other: crate::standards::dsrc_2_2_1::etsi_its_dsrc::Position3D) -> Self {
        geo_types::Point::new(other.long.as_deg(), other.lat.as_deg())
    }
}
#[cfg(feature = "_dsrc_2_2_1")]
impl From<geo_types::Point> for crate::standards::dsrc_2_2_1::etsi_its_dsrc::Position3D {
    fn from(other: geo_types::Point) -> Self {
        use crate::standards::cdd_2_2_1::etsi_its_cdd::Latitude;
        use crate::standards::cdd_2_2_1::etsi_its_cdd::Longitude;

        Self {
            lat: Latitude::from_deg(other.y()),
            long: Longitude::from_deg(other.x()),
            elevation: None,
            regional: None,
        }
    }
}

/// convert ETSI ReferencePosition to [`geo_types::Point`]
#[cfg(any(feature = "_cdd_1_3_1_1", feature = "_cdd_2_2_1"))]
macro_rules! refpos_to_point {
    ($t:ty) => {
        impl From<$t> for geo_types::Point {
            fn from(other: $t) -> Self {
                geo_types::Point::new(other.longitude.as_deg(), other.latitude.as_deg())
            }
        }
    };
}

// used by CDD 1.3.1 (DENM 1.3.1, CAM 1.4.1, CPM 1)
#[cfg(feature = "_cdd_1_3_1_1")]
refpos_to_point!(crate::standards::cdd_1_3_1_1::its_container::ReferencePosition);

// used by CDD 2.2.1 (DENM 2.2.1, CPM 2.1.1)
#[cfg(feature = "_cdd_2_2_1")]
refpos_to_point!(crate::standards::cdd_2_2_1::etsi_its_cdd::ReferencePosition);

// ETSI PathHistory

#[cfg(any(feature = "_cdd_1_3_1_1", feature = "denm_2_2_1"))]
macro_rules! ph_to_line_string {
    ($t:ty) => {
        impl $t {
            /// Resolve delta positions to absolute geo positions
            ///
            /// Output geo positions are in degrees lon/lat starting with the reference position.
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
#[cfg(feature = "_cdd_1_3_1_1")]
ph_to_line_string!(crate::standards::cdd_1_3_1_1::its_container::PathHistory);

// used by CDD 2.2.1 (DENM 2.1.1)
#[cfg(feature = "denm_2_2_1")]
ph_to_line_string!(crate::standards::cdd_2_2_1::etsi_its_cdd::Path);

// Used by DSRC 2.2.1 (MAPEM lanes)
#[cfg(feature = "mapem_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeSetXY {
    /// Resolve delta positions to absolute geo positions
    ///
    /// Output geo positions are in degrees lon/lat excluding the reference position.
    ///
    /// Input data is assumed to be in ETSI format. This means that the first point is
    /// relative to the `ref_pos` and subsequent points are relative to the point before.
    #[allow(
        clippy::missing_panics_doc,
        reason = "panic on last().unwrap() is impossible here"
    )]
    #[must_use]
    pub fn to_line_string(&self, ref_pos: geo_types::Point) -> geo_types::LineString {
        let mut path = self.0.iter().fold(vec![ref_pos], |mut acc, pt| {
            // vector is primed with one element, so should never return None
            let origin = acc.last().unwrap();
            let pos = pt.delta.to_geo(origin);

            acc.push(pos);
            acc
        });
        path.remove(0); // remove ref_pos again

        path.into()
    }
}

#[cfg(feature = "mapem_2_2_1")]
impl crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeOffsetPointXY {
    /// Convert to absolute lon/lat (in degrees)
    ///
    /// [`NodeOffsetPointXY`](`Self`) may contain XY delta positions or absolute lat/lon positions.
    /// XY positions will be converted to delta geo-coordinates near the `ref_pos`.
    #[must_use]
    pub fn to_geo(&self, ref_pos: &geo_types::Point) -> geo_types::Point {
        use crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeOffsetPointXY;

        let (lon, lat) = match self {
            NodeOffsetPointXY::node_XY1(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY2(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY3(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY4(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY5(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_XY6(etsi) => {
                dxy_to_geo((&etsi.x).into(), (&etsi.y).into(), ref_pos)
            }
            NodeOffsetPointXY::node_LatLon(etsi) => (etsi.lon.as_deg(), etsi.lat.as_deg()),
            NodeOffsetPointXY::regional(_) => (0., 0.),
        };

        geo_types::Point::new(lon, lat)
    }

    /// Convert to delta distance (in meters)
    ///
    /// [`NodeOffsetPointXY`](`Self`) may contain XY delta positions or absolute lat/lon positions.
    /// lat/lon delta coordinates will be converted to XY offsets near the `ref_pos`.
    ///
    /// Output will be according to ETSI coordinate system (X pointing east, X pointing north)!
    #[must_use]
    pub fn to_ddist(&self, ref_pos: &geo_types::Point) -> geo_types::Point {
        use crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeOffsetPointXY;

        let (x, y) = match self {
            NodeOffsetPointXY::node_XY1(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY2(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY3(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY4(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY5(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_XY6(etsi) => ((&etsi.x).into(), (&etsi.y).into()),
            NodeOffsetPointXY::node_LatLon(etsi) => latlon_to_dcart(etsi, ref_pos),
            NodeOffsetPointXY::regional(_) => (0., 0.),
        };

        geo_types::Point::new(x.into(), y.into())
    }
}

/// Convert ETSI XY deltas to absolute lon/lat position (in degrees)
///
/// X points east (longitude), Y points north (latitude)!
fn dxy_to_geo(dx: f32, dy: f32, ref_pos: &geo_types::Point) -> (f64, f64) {
    // latitude degree per meter is independent from longitude
    let dlat = f64::from(dy) / f64::from(EARTH_CIRCUMFERENCE) * 360.;

    // longitude degree per meter has a different value depending on the latitude
    let ref_lat_rad = ref_pos.to_radians().y();
    let dlon = f64::from(dx) / (f64::from(EARTH_CIRCUMFERENCE) * ref_lat_rad.cos()) * 360.;

    (ref_pos.x() + dlon, ref_pos.y() + dlat)
}

/// Convert absolute lon/lat position to ETSI XY position (in meters)
///
/// X points east (longitude), Y points north (latitude)!
#[cfg(feature = "mapem_2_2_1")]
fn latlon_to_dcart(
    dpos: &crate::standards::dsrc_2_2_1::etsi_its_dsrc::NodeLLmD64b,
    ref_pos: &geo_types::Point,
) -> (f32, f32) {
    // latitude degree per meter is independent from longitude
    #[allow(clippy::cast_possible_truncation)]
    let dlat_deg = dpos.lat.as_deg() as f32 - ref_pos.y() as f32;
    let dy = dlat_deg / 360. * EARTH_CIRCUMFERENCE;

    // longitude degree per meter has a different value depending on the latitude
    #[allow(clippy::cast_possible_truncation)]
    let ref_lat_rad = ref_pos.to_radians().y() as f32;
    #[allow(clippy::cast_possible_truncation)]
    let dlon_deg = dpos.lon.as_deg() as f32 - ref_pos.x() as f32;
    let dx = dlon_deg / 360. * (EARTH_CIRCUMFERENCE * ref_lat_rad.cos());

    (dx, dy)
}

#[cfg(feature = "cpm_1")]
impl crate::standards::cpm_1::cpm_pdu_descriptions::NodeOffsetPointZ {
    #[must_use]
    pub fn to_meters(&self) -> f32 {
        use crate::standards::cpm_1::cpm_pdu_descriptions::NodeOffsetPointZ;

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
        use crate::geo_utils::dxy_to_geo;

        let ref_pos = geo_types::point! {x: 9.936_521, y: 53.550_728};

        // trivial test
        assert_eq!((ref_pos.x(), ref_pos.y()), dxy_to_geo(0., 0., &ref_pos));

        // 1/10th nautical mile north/ south
        let (dlon, dlat) = dxy_to_geo(0., 185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.x() + 0., dlon);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.y() + 0.001_667, dlat);

        let (dlon, dlat) = dxy_to_geo(0., -185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.x() + 0., dlon);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.y() + -0.001_667, dlat);

        // east/west from online calculation tool
        let (dlon, dlat) = dxy_to_geo(66., 0., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.x() + 0.001_001, dlon);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.y() + 0., dlat);

        // combine both
        let (dlon, dlat) = dxy_to_geo(66., 185., &ref_pos);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.x() + 0.001_001, dlon);
        assert_float_eq::assert_float_absolute_eq!(ref_pos.y() + 0.001_667, dlat);
    }

    #[test]
    #[cfg(feature = "mapem_2_2_1")]
    fn nodeset_to_line_string() {
        use crate::standards::cdd_2_2_1::etsi_its_cdd::{Latitude, Longitude};
        use crate::standards::dsrc_2_2_1::etsi_its_dsrc::{
            NodeLLmD64b, NodeOffsetPointXY, NodeSetXY, NodeXY, NodeXY32b, OffsetB16,
        };

        let ref_pos = geo_types::point! {x: 9.936_521, y: 53.550_728};

        // latlon test
        {
            let point1 = NodeLLmD64b::new(
                Longitude::from_deg(ref_pos.x() + 0.001),
                Latitude::from_deg(ref_pos.y() + 0.005),
            );
            let point2 = NodeLLmD64b::new(
                Longitude::from_deg(ref_pos.x() + -0.042),
                Latitude::from_deg(ref_pos.y() + 0.),
            );
            let nodes = vec![
                NodeXY::new(NodeOffsetPointXY::node_LatLon(point1), None),
                NodeXY::new(NodeOffsetPointXY::node_LatLon(point2), None),
            ];

            let geo_nodes = NodeSetXY(nodes).to_line_string(ref_pos).into_points();
            let geo_point1 = geo_types::Point::new(9.936_521 + 0.001, 53.550_728 + 0.005);
            assert_float_eq::assert_float_absolute_eq!(geo_point1.x(), geo_nodes[0].x());
            assert_float_eq::assert_float_absolute_eq!(geo_point1.y(), geo_nodes[0].y());
            let geo_point2 = geo_types::Point::new(9.936_521 - 0.042, 53.550_728 + 0.);
            assert_float_eq::assert_float_absolute_eq!(geo_point2.x(), geo_nodes[1].x());
            assert_float_eq::assert_float_absolute_eq!(geo_point2.y(), geo_nodes[1].y());
        }

        // delta X/Y test
        {
            let point1 = NodeXY32b::new(
                OffsetB16::from_meters(0.).unwrap(),
                OffsetB16::from_meters(185.).unwrap(),
            );
            let point2 = NodeXY32b::new(
                OffsetB16::from_meters(66.).unwrap(),
                OffsetB16::from_meters(0.).unwrap(),
            );
            let nodes = vec![
                NodeXY::new(NodeOffsetPointXY::node_XY6(point1), None),
                NodeXY::new(NodeOffsetPointXY::node_XY6(point2), None),
            ];

            let geo_nodes = NodeSetXY(nodes).to_line_string(ref_pos).into_points();
            let geo_point1 = geo_types::Point::new(9.936_521, 53.550_728 + 0.001_667);
            assert_float_eq::assert_float_absolute_eq!(geo_point1.x(), geo_nodes[0].x());
            assert_float_eq::assert_float_absolute_eq!(geo_point1.y(), geo_nodes[0].y());

            let geo_point2 = geo_types::Point::new(9.936_521 + 0.001_001, 53.550_728 + 0.001_667);
            assert_float_eq::assert_float_absolute_eq!(geo_point2.x(), geo_nodes[1].x());
            assert_float_eq::assert_float_absolute_eq!(geo_point2.y(), geo_nodes[1].y());
        }
    }
}
