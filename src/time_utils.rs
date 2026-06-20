use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};

pub const UTC_OFFSET_HOURS: i32 = 8;

pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn start_of_day_utc(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(0, 0, 0)
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or_else(|| Utc::now())
}

pub fn end_of_day_utc(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_opt(23, 59, 59)
        .map(|dt| Utc.from_utc_datetime(&dt))
        .unwrap_or_else(|| Utc::now())
}

pub fn end_of_nth_day_utc(days: i64) -> DateTime<Utc> {
    let target_date = Utc::now().date_naive() + Duration::days(days);
    end_of_day_utc(target_date)
}

pub fn days_until_expiry(expire_at: DateTime<Utc>, now: DateTime<Utc>) -> i64 {
    let expire_date = expire_at.date_naive();
    let now_date = now.date_naive();
    (expire_date - now_date).num_days()
}

pub fn is_expiring_on_date(
    expire_at: DateTime<Utc>,
    target_date: NaiveDate,
) -> bool {
    let expire_date = expire_at.date_naive();
    expire_date == target_date
}

pub fn is_within_days(
    expire_at: DateTime<Utc>,
    days: i64,
    now: DateTime<Utc>,
) -> bool {
    let cutoff = now + Duration::days(days);
    expire_at <= cutoff
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_start_of_day_utc() {
        let date = NaiveDate::from_ymd_opt(2026, 6, 21).unwrap();
        let start = start_of_day_utc(date);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
    }

    #[test]
    fn test_end_of_day_utc() {
        let date = NaiveDate::from_ymd_opt(2026, 6, 21).unwrap();
        let end = end_of_day_utc(date);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }

    #[test]
    fn test_days_until_expiry_same_day() {
        let now = Utc::now();
        let expire = now + Duration::hours(2);
        let days = days_until_expiry(expire, now);
        assert_eq!(days, 0);
    }

    #[test]
    fn test_days_until_expiry_tomorrow() {
        let now = Utc::now();
        let expire = now + Duration::days(1);
        let days = days_until_expiry(expire, now);
        assert!(days >= 0 && days <= 1);
    }
}
