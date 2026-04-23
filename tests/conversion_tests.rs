// Copyright (c) 2026 consider it GmbH

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn lat_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to degrees
        let test = cdd_2_2_1::etsi_its_cdd::Latitude(10_000_000);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::Latitude(-100_000_000);
        assert_float_eq::assert_float_absolute_eq!(-10., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::Latitude(420_000);
        assert_float_eq::assert_float_absolute_eq!(0.042, test.as_deg());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::Latitude::unavailable();
        assert_eq!(900_000_001, test.0);
        assert!(test.is_unavailable());

        // from degrees to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::Latitude(10_000_000),
            cdd_2_2_1::etsi_its_cdd::Latitude::from_deg(1.)
        );
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::Latitude(100_000_000),
            cdd_2_2_1::etsi_its_cdd::Latitude::from_deg(10.)
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn long_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to degrees
        let test = cdd_2_2_1::etsi_its_cdd::Longitude(10_000_000);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::Longitude(-100_000_000);
        assert_float_eq::assert_float_absolute_eq!(-10., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::Longitude(420_000);
        assert_float_eq::assert_float_absolute_eq!(0.042, test.as_deg());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::Longitude::unavailable();
        assert_eq!(1_800_000_001, test.0);
        assert!(test.is_unavailable());

        // from degrees to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::Longitude(10_000_000),
            cdd_2_2_1::etsi_its_cdd::Longitude::from_deg(1.)
        );
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::Longitude(100_000_000),
            cdd_2_2_1::etsi_its_cdd::Longitude::from_deg(10.)
        );
    }

    #[test]
    #[cfg(feature = "_dsrc_2_2_1")]
    fn offset_b09_test() {
        use etsi_web::standards::dsrc_2_2_1;

        // from ETSI to meters
        let test = dsrc_2_2_1::etsi_its_dsrc::OffsetB09(100);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_meters());
        assert!(!test.is_unavailable());

        let test = dsrc_2_2_1::etsi_its_dsrc::OffsetB09(255);
        assert_float_eq::assert_float_absolute_eq!(2.55, test.as_meters());
        assert!(!test.is_unavailable());
        let test = dsrc_2_2_1::etsi_its_dsrc::OffsetB09(-255);
        assert_float_eq::assert_float_absolute_eq!(-2.55, test.as_meters());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = dsrc_2_2_1::etsi_its_dsrc::OffsetB09::unavailable();
        assert_eq!(-256, test.0);
        assert!(test.is_unavailable());

        // from meters to ETSI
        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09(100),
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(1.).unwrap()
        );

        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09(255),
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(2.55).unwrap()
        );
        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09(-255),
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(-2.55).unwrap()
        );
        assert!(dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(2.56).is_err());
        assert!(dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(-2.55).is_ok());
        assert!(dsrc_2_2_1::etsi_its_dsrc::OffsetB09::from_meters(-2.56).is_err());

        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::OffsetB09(100),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn object_dimension_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to meters
        let test = cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue(10);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_meters());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue(25);
        assert_float_eq::assert_float_absolute_eq!(2.5, test.as_meters());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue::unavailable();
        assert_eq!(256, test.0);
        assert!(test.is_unavailable());

        // from meters to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue(10),
            cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue::from_meters(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue(25),
            cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue::from_meters(2.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue::from_meters(25.5).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue::from_meters(25.6).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::ObjectDimensionValue(10),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn vehicle_width_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to meters
        let test = cdd_2_2_1::etsi_its_cdd::VehicleWidth(10);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_meters());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::VehicleWidth(25);
        assert_float_eq::assert_float_absolute_eq!(2.5, test.as_meters());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::VehicleWidth::unavailable();
        assert_eq!(62, test.0);
        assert!(test.is_unavailable());

        // from meters to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleWidth(10),
            cdd_2_2_1::etsi_its_cdd::VehicleWidth::from_meters(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleWidth(25),
            cdd_2_2_1::etsi_its_cdd::VehicleWidth::from_meters(2.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::VehicleWidth::from_meters(6.1).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::VehicleWidth::from_meters(6.2).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleWidth(10),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn vehicle_length_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to meters
        let test = cdd_2_2_1::etsi_its_cdd::VehicleLengthValue(10);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_meters());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::VehicleLengthValue(25);
        assert_float_eq::assert_float_absolute_eq!(2.5, test.as_meters());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::VehicleLengthValue::unavailable();
        assert_eq!(1023, test.0);
        assert!(test.is_unavailable());

        // from meters to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleLengthValue(10),
            cdd_2_2_1::etsi_its_cdd::VehicleLengthValue::from_meters(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleLengthValue(25),
            cdd_2_2_1::etsi_its_cdd::VehicleLengthValue::from_meters(2.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::VehicleLengthValue::from_meters(102.2).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::VehicleLengthValue::from_meters(102.3).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::VehicleLengthValue(10),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn speed_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to m/s
        let test = cdd_2_2_1::etsi_its_cdd::SpeedValue(100);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_mps());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::SpeedValue(1);
        assert_float_eq::assert_float_absolute_eq!(0.01, test.as_mps());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::SpeedValue::unavailable();
        assert_eq!(16_383, test.0);
        assert!(test.is_unavailable());

        // from m/s to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SpeedValue(100),
            cdd_2_2_1::etsi_its_cdd::SpeedValue::from_mps(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SpeedValue(1),
            cdd_2_2_1::etsi_its_cdd::SpeedValue::from_mps(0.01).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::SpeedValue::from_mps(163.82).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::SpeedValue::from_mps(163.83).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SpeedValue(100),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn longitudinal_acceleration_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to m/s/s
        let test = cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue(10);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_mpss());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue(25);
        assert_float_eq::assert_float_absolute_eq!(2.5, test.as_mpss());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue::unavailable();
        assert_eq!(161, test.0);
        assert!(test.is_unavailable());

        // from m/s/s to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue(10),
            cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue::from_mpss(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue(25),
            cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue::from_mpss(2.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue::from_mpss(16.0).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue::from_mpss(16.1).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::LongitudinalAccelerationValue(10),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn cartesian_angle_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to degrees
        let test = cdd_2_2_1::etsi_its_cdd::CartesianAngleValue(10);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::CartesianAngleValue(25);
        assert_float_eq::assert_float_absolute_eq!(2.5, test.as_deg());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::CartesianAngleValue::unavailable();
        assert_eq!(3601, test.0);
        assert!(test.is_unavailable());

        // from degrees to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::CartesianAngleValue(10),
            cdd_2_2_1::etsi_its_cdd::CartesianAngleValue::from_deg(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::CartesianAngleValue(25),
            cdd_2_2_1::etsi_its_cdd::CartesianAngleValue::from_deg(2.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::CartesianAngleValue::from_deg(360.0).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::CartesianAngleValue::from_deg(360.1).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::CartesianAngleValue(10),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_dsrc_2_2_1")]
    fn dsrc_angle_test() {
        use etsi_web::standards::dsrc_2_2_1;

        // from ETSI to degrees
        let test = dsrc_2_2_1::etsi_its_dsrc::Angle(80);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_deg());
        assert!(!test.is_unavailable());

        let test = dsrc_2_2_1::etsi_its_dsrc::Angle(1);
        assert_float_eq::assert_float_absolute_eq!(0.0125, test.as_deg());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = dsrc_2_2_1::etsi_its_dsrc::Angle::unavailable();
        assert_eq!(28800, test.0);
        assert!(test.is_unavailable());

        // from degrees to ETSI
        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::Angle(80),
            dsrc_2_2_1::etsi_its_dsrc::Angle::from_deg(1.).unwrap()
        );

        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::Angle(1),
            dsrc_2_2_1::etsi_its_dsrc::Angle::from_deg(0.0125).unwrap()
        );

        assert!(dsrc_2_2_1::etsi_its_dsrc::Angle::from_deg(359.9875).is_ok());
        assert!(dsrc_2_2_1::etsi_its_dsrc::Angle::from_deg(28801. / 80.).is_err());

        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::Angle(80),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn steering_wheel_angle_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to degrees
        let test = cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue(510);
        assert_float_eq::assert_float_absolute_eq!(765., test.as_deg());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue(1);
        assert_float_eq::assert_float_absolute_eq!(1.5, test.as_deg());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue::unavailable();
        assert_eq!(512, test.0);
        assert!(test.is_unavailable());

        // from degrees to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue(510),
            cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue::from_deg(765.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue(1),
            cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue::from_deg(1.5).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue::from_deg(511. * 1.5).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue::from_deg(512. * 1.5).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::SteeringWheelAngleValue(510),
            (765.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_cdd_2_2_1")]
    fn yaw_rate_test() {
        use etsi_web::standards::cdd_2_2_1;

        // from ETSI to deg/s
        let test = cdd_2_2_1::etsi_its_cdd::YawRateValue(100);
        assert_float_eq::assert_float_absolute_eq!(1., test.as_deg_rate());
        assert!(!test.is_unavailable());

        let test = cdd_2_2_1::etsi_its_cdd::YawRateValue(1);
        assert_float_eq::assert_float_absolute_eq!(0.01, test.as_deg_rate());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = cdd_2_2_1::etsi_its_cdd::YawRateValue::unavailable();
        assert_eq!(32767, test.0);
        assert!(test.is_unavailable());

        // from deg/s to ETSI
        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::YawRateValue(100),
            cdd_2_2_1::etsi_its_cdd::YawRateValue::from_deg_rate(1.).unwrap()
        );

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::YawRateValue(1),
            cdd_2_2_1::etsi_its_cdd::YawRateValue::from_deg_rate(0.01).unwrap()
        );

        assert!(cdd_2_2_1::etsi_its_cdd::YawRateValue::from_deg_rate(327.66).is_ok());
        assert!(cdd_2_2_1::etsi_its_cdd::YawRateValue::from_deg_rate(327.67).is_err());

        assert_eq!(
            cdd_2_2_1::etsi_its_cdd::YawRateValue(100),
            (1.).try_into().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "_dsrc_2_2_1")]
    fn dsecond_test() {
        use etsi_web::standards::dsrc_2_2_1;

        // from ETSI to seconds
        let test = dsrc_2_2_1::etsi_its_dsrc::DSecond(1000);
        assert_eq!(1000, test.as_millis());
        assert!(!test.is_unavailable());

        let test = dsrc_2_2_1::etsi_its_dsrc::DSecond(1);
        assert_eq!(1, test.as_millis());
        assert!(!test.is_unavailable());

        // unavailable value
        let test = dsrc_2_2_1::etsi_its_dsrc::DSecond::unavailable();
        assert_eq!(65535, test.0);
        assert!(test.is_unavailable());

        // from seconds to ETSI
        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::DSecond(1000),
            dsrc_2_2_1::etsi_its_dsrc::DSecond::from_millis(1000).unwrap()
        );

        assert_eq!(
            dsrc_2_2_1::etsi_its_dsrc::DSecond(1),
            dsrc_2_2_1::etsi_its_dsrc::DSecond::from_millis(1).unwrap()
        );

        assert!(dsrc_2_2_1::etsi_its_dsrc::DSecond::from_millis(60999).is_ok());
        assert!(dsrc_2_2_1::etsi_its_dsrc::DSecond::from_millis(61000).is_err());
    }
}
