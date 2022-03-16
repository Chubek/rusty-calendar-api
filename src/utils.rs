use chrono::NaiveDateTime;
use std::hash::{Hash, Hasher};
use fasthash::{metro, MetroHasher};

pub fn fmt_date(date_string: String) -> NaiveDateTime {
    let default = NaiveDateTime::parse_from_str("2000-01-01 12:20", "%Y-%m-%d %H:%M").unwrap();
    let date_time = NaiveDateTime::parse_from_str(date_string.as_str(), "%Y-%m-%d %H:%M").unwrap_or(default);

    date_time

}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: MetroHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}


pub fn hash_str(tbh: String) -> u64 {
    return hash(&tbh)
}