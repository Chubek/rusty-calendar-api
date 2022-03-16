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

pub fn get_queries(url: &str) -> Vec<Vec<String>> {
    let url_split: Vec<&str> = url.split("?").collect();
    let params = url_split[1];

    let params_split= params.split("&");

    let mut ret: Vec<Vec<String>> = Vec::new();

    for p in params_split {
        let p_split: Vec<&str> = p.split("=").collect();
        ret.push(vec![p_split[0].to_string(), p_split[1].to_string()]);
    }

    return ret
}

pub fn select_query(queries: Vec<Vec<String>>, sel: &str) -> String {
    let mut got = String::new();

    for q in queries {
        if q[0] == sel.to_string() {
            got = q[1].clone()
        }
    }

    return got
}

pub fn get_url_path(url: &str) -> String {
    let url_split: Vec<&str> = url.split("?").collect();
    return url_split[0].to_string();
}