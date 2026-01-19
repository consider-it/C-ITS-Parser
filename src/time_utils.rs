//! Conversions between ETSI and chrono values

// used by IS 1.3.1

/// Converts a time point to `MinuteOfTheYear` and `DSecond` (milliseconds)
#[allow(
    clippy::missing_panics_doc,
    reason = "unwrap is safe b/c of preconditions"
)]
#[cfg(any(
    feature = "mapem_2_2_1",
    feature = "spatem_2_2_1",
    feature = "srem_2_2_1",
    feature = "ssem_2_2_1",
))]
#[must_use]
pub fn moy_and_dsecond(
    time: chrono::DateTime<chrono::Utc>,
) -> (
    crate::standards::dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear,
    crate::standards::dsrc_2_2_1::etsi_its_dsrc::DSecond,
) {
    use chrono::{Datelike, Timelike};

    // build start of year timestamp
    let naive_time = time.naive_utc();
    let start_of_year = chrono::NaiveDate::from_ymd_opt(naive_time.year(), 1, 1)
        .expect("year of ref time suddenly out of range")
        .and_time(chrono::NaiveTime::default());

    // determine minute of the year and millis
    let diff = time.naive_utc() - start_of_year;
    #[allow(clippy::cast_possible_truncation, reason = "max of 527040 fits in u32")]
    #[allow(clippy::cast_sign_loss, reason = "precondition assures positive value")]
    let minutes = diff.num_minutes() as u32;

    #[allow(clippy::cast_possible_truncation, reason = "max of 60000 fits in u16")]
    let millis = (naive_time.second() * 1000 + naive_time.nanosecond() / 1_000_000) as u16;

    let moy = crate::standards::dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear(minutes);
    let dsec = crate::standards::dsrc_2_2_1::etsi_its_dsrc::DSecond::from_millis(millis)
        .expect("DSecond suddenly out of range");
    (moy, dsec)
}

/// Converts `MinuteOfTheYear` and `DSecond` (milliseconds) to a date and time
#[allow(
    clippy::missing_panics_doc,
    reason = "unwrap is safe b/c of preconditions"
)]
#[cfg(any(
    feature = "mapem_2_2_1",
    feature = "spatem_2_2_1",
    feature = "srem_2_2_1",
    feature = "ssem_2_2_1",
))]
#[must_use]
pub fn time_from_moy_and_dsecond(
    moy: &crate::standards::dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear,
    second: &crate::standards::dsrc_2_2_1::etsi_its_dsrc::DSecond,
    year: i32,
) -> chrono::DateTime<chrono::Utc> {
    // build start of year timestamp
    let start_of_year = chrono::NaiveDate::from_ymd_opt(year, 1, 1)
        .expect("year of ref time suddenly out of range")
        .and_time(chrono::NaiveTime::default());

    // add minutes of the year and milliseconds
    let time = start_of_year
        .checked_add_signed(chrono::TimeDelta::minutes(i64::from(moy.0)))
        .expect("Resulting DateTime suddenly out of range")
        .checked_add_signed(chrono::TimeDelta::milliseconds(i64::from(second.0)))
        .expect("Resulting DateTime suddenly out of range");

    time.and_utc()
}

/// convert between ETSI TimestampIts and [`chrono::DateTime`]
///
/// Note: UTC is 37 seconds behind TAI as of 2017-01-01 (when the last leap seconds was
/// added to UTC).
/// But the TimestampIts epoch starts at 2004-01-01 00:00:00 UTC which was 32 seconds
/// behind TAI, so the difference is 5 seconds since the last leap second insertion on
/// 2022-01-01.
#[cfg(feature = "etsi")]
macro_rules! timestampits_conv_datetime {
    ($t:ty) => {
        impl From<$t> for chrono::DateTime<chrono::Utc> {
            fn from(other: $t) -> Self {
                const ITS_EPOCH_UNIX_MS: i64 = 1_072_915_200_000; // UNIX timestamp of ITS epoch begin

                #[allow(clippy::cast_possible_wrap, reason = "42 bits fit in i64")]
                let its_millis = other.0 as i64 + ITS_EPOCH_UNIX_MS;
                // Note: This will use the wrong leap second count around the timestamp
                //       where a leap second is introduced since we're comparing to UNIX
                //       timestamps and the "corrected" timestamp. But this is irrelevant
                //       for applications after 2022-01-01 and this was written in 2026.
                let utc_millis = its_millis - i64::from(its_offset_ms(its_millis.cast_unsigned()));

                chrono::DateTime::from_timestamp_millis(utc_millis)
                    .expect("ITS Timestamp suddenly out of range for chrono::DateTime")
            }
        }

        impl From<chrono::DateTime<chrono::Utc>> for $t {
            fn from(other: chrono::DateTime<chrono::Utc>) -> $t {
                const ITS_EPOCH_UNIX_MS: u64 = 1_072_915_200_000; // UNIX timestamp of ITS epoch begin

                #[allow(
                    clippy::cast_sign_loss,
                    reason = "expecting positive UNIX time is fine"
                )]
                let utc_millis = other.timestamp_millis() as u64;
                let its_time =
                    utc_millis - ITS_EPOCH_UNIX_MS + u64::from(its_offset_ms(utc_millis));

                Self(its_time)
            }
        }
    };
}

#[cfg(feature = "etsi")]
fn its_offset_ms(unix_time_ms: u64) -> u16 {
    if unix_time_ms >= 1_483_228_800_000 {
        // leap second introduced at 2016-12-31, so +5 since 2017-01-01
        5000
    } else if unix_time_ms >= 1_435_708_800_000 {
        // leap second introduced at 2015-06-30, so +4 since 2015-07-01
        4000
    } else if unix_time_ms >= 1_341_100_800_000 {
        // leap second introduced at 2012-06-30, so +3 since 2012-07-01
        3000
    } else if unix_time_ms >= 1_199_145_600_000 {
        // leap second introduced at 2008-12-31, so +2 since 2009-01-01
        2000
    } else if unix_time_ms >= 1_136_073_600_000 {
        // leap second introduced at 2005-12-31, so +1 since 2006-01-01
        1000
    } else {
        0
    }
}

// used by DENM 1.3.1, IVIM 2.1.1
#[cfg(any(feature = "denm_1_3_1", feature = "ivim_2_1_1"))]
timestampits_conv_datetime!(crate::standards::cdd_1_3_1_1::its_container::TimestampIts);

// used by CPM 2.1.1, DENM 2.2.1 and IVIM 2.2.1
#[cfg(any(feature = "cpm_2_1_1", feature = "denm_2_2_1", feature = "ivim_2_2_1"))]
timestampits_conv_datetime!(crate::standards::cdd_2_2_1::etsi_its_cdd::TimestampIts);

#[cfg(all(test, feature = "etsi"))]
mod tests {

    #[test]
    #[cfg(any(
        feature = "mapem_2_2_1",
        feature = "spatem_2_2_1",
        feature = "srem_2_2_1",
        feature = "ssem_2_2_1",
    ))]
    fn time_to_moy_and_dsecond() {
        use crate::time_utils::moy_and_dsecond;

        // at 2026-01-01 00:00:00, moy shall be 0 and dsecond shall be 0
        let date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc();
        let (moy, dsec) = moy_and_dsecond(date);

        assert_eq!(0, moy.0);
        assert_eq!(0, dsec.0);

        // at 2026-01-01 00:42:23, moy shall be 42 and dsecond shall be 23000 ms
        let date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 42, 23).unwrap())
            .and_utc();
        let (moy, dsec) = moy_and_dsecond(date);

        assert_eq!(42, moy.0);
        assert_eq!(23_000, dsec.0);

        // at 2026-02-01 00:00:42, moy shall be (31*24*60) and dsecond shall be 42000 ms
        let date = chrono::NaiveDate::from_ymd_opt(2026, 2, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 42).unwrap())
            .and_utc();
        let (moy, dsec) = moy_and_dsecond(date);

        assert_eq!(31 * 24 * 60, moy.0);
        assert_eq!(42_000, dsec.0);
    }

    #[test]
    #[cfg(any(
        feature = "mapem_2_2_1",
        feature = "spatem_2_2_1",
        feature = "srem_2_2_1",
        feature = "ssem_2_2_1",
    ))]
    fn moy_and_dsecond_to_time() {
        use crate::standards::dsrc_2_2_1::etsi_its_dsrc::DSecond;
        use crate::standards::dsrc_2_2_1::etsi_its_dsrc::MinuteOfTheYear;
        use crate::time_utils::time_from_moy_and_dsecond;

        // year 2026, moy 0, dsecond 0 shall give 2026-01-01 00:00:00
        let ref_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc();

        let date = time_from_moy_and_dsecond(&MinuteOfTheYear(0), &DSecond(0), 2026);
        assert_eq!(ref_date, date);

        // year 2026, moy 42 and dsecond 23000 ms shall give 2026-01-01 00:42:23
        let ref_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 42, 23).unwrap())
            .and_utc();

        let date = time_from_moy_and_dsecond(&MinuteOfTheYear(42), &DSecond(23_000), 2026);
        assert_eq!(ref_date, date);

        // year 2024, moy (31*24*60) and dsecond 42000 ms shall give 2024-02-01 00:00:42,
        let ref_date = chrono::NaiveDate::from_ymd_opt(2024, 2, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 42).unwrap())
            .and_utc();

        let date =
            time_from_moy_and_dsecond(&MinuteOfTheYear(31 * 24 * 60), &DSecond(42_000), 2024);
        assert_eq!(ref_date, date);
    }

    #[test]
    #[cfg(any(feature = "cpm_2_1_1", feature = "denm_2_2_1", feature = "ivim_2_2_1"))]
    fn utc_to_its_timestamp() {
        use crate::standards::cdd_2_2_1::etsi_its_cdd::TimestampIts;

        // From ASN.1 definition: "The value for TimestampIts for 1 January 2007 00:00:00.000 UTC is `94 694 401 000` milliseconds"
        let ref_date = chrono::NaiveDate::from_ymd_opt(2007, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc();

        let its: TimestampIts = ref_date.into();
        assert_eq!(94_694_401_000, its.0);
    }

    #[test]
    #[cfg(any(feature = "cpm_2_1_1", feature = "denm_2_2_1", feature = "ivim_2_2_1"))]
    fn its_to_utc_timestamp() {
        use crate::standards::cdd_2_2_1::etsi_its_cdd::TimestampIts;

        // From ASN.1 definition: "The value for TimestampIts for 1 January 2007 00:00:00.000 UTC is `94 694 401 000` milliseconds"
        let ref_date = chrono::NaiveDate::from_ymd_opt(2007, 1, 1)
            .unwrap()
            .and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc();

        let utc: chrono::DateTime<chrono::Utc> = TimestampIts(94_694_401_000).into();
        assert_eq!(ref_date, utc);
    }
}
