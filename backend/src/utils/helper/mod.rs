use chrono;

pub fn is_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    if !(b.len() == 10  // length
    && b[4] == b'-' && b[7] == b'-'  // find '-'
    && b[..4].iter().all(|c| c.is_ascii_digit()) // all billow ascci digits
    && b[5..7].iter().all(|c| c.is_ascii_digit())
    && b[8..].iter().all(|c| c.is_ascii_digit()))
    {
        return false;
    }
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
}

/// Helper fn to provide data with exact saem format for every other db insertion.
/// "%Y-%m-%d %H-%M-%S"
pub fn time_now_utc_string() -> String { chrono::Utc::now().format("%Y-%m-%d %H-%M-%S").to_string() }
