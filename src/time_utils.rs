//! Conversions between ETSI and chrono values

// used by IS 1.3.1

/// Converts a time point to `MinuteOfTheYear` and `DSecond` (milliseconds)
#[allow(
    clippy::missing_panics_doc,
    reason = "unwrap is safe b/c of preconditions"
)]
#[cfg(feature = "etsi")]
#[must_use]
pub fn moy_and_dsecond(
    time: chrono::DateTime<chrono::Utc>,
) -> (
    crate::standards::is_1_3_1::etsi_schema::MinuteOfTheYear,
    crate::standards::is_1_3_1::etsi_schema::DSecond,
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

    let moy = crate::standards::is_1_3_1::etsi_schema::MinuteOfTheYear(minutes);
    let dsec = crate::standards::is_1_3_1::etsi_schema::DSecond::from_millis(millis)
        .expect("DSecond suddenly out of range");
    (moy, dsec)
}

/// Converts `MinuteOfTheYear` and `DSecond` (milliseconds) to a date and time
#[allow(
    clippy::missing_panics_doc,
    reason = "unwrap is safe b/c of preconditions"
)]
#[cfg(feature = "etsi")]
#[must_use]
pub fn time_from_moy_and_dsecond(
    moy: &crate::standards::is_1_3_1::etsi_schema::MinuteOfTheYear,
    second: &crate::standards::is_1_3_1::etsi_schema::DSecond,
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

#[cfg(all(test, feature = "etsi"))]
mod tests {

    #[test]
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
    fn moy_and_dsecond_to_time() {
        use crate::standards::is_1_3_1::etsi_schema::DSecond;
        use crate::standards::is_1_3_1::etsi_schema::MinuteOfTheYear;
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
}
