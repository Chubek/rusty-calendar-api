use chrono::NaiveDateTime;

fn convert_timestamp_to_date(timestamp: i64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(timestamp, 0)
}


fn convert_date_to_timestamp(date_string: String) ->  i64 {
    let default = NaiveDateTime::parse_from_str("2000-01-01 12:20", "%Y-%m-%d %H:%M").unwrap();
    let date_time = NaiveDateTime::parse_from_str(date_string.as_str(), "%Y-%m-%d %H:%M").unwrap_or(default);

    date_time.timestamp()

}