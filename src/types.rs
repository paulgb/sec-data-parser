use chrono::{Month, NaiveDate};
use num_traits::FromPrimitive;

const DATE_FORMAT: &str = "%Y%m%d";

#[derive(Debug, PartialEq, Clone)]
pub struct MonthDayPair(chrono::Month, u32);

impl MonthDayPair {
    pub fn parse(st: &str) -> MonthDayPair {
        let month_n: u32 = st[..2].parse().unwrap();
        let day: u32 = st[2..].parse().unwrap();

        let month = Month::from_u32(month_n).unwrap();
        MonthDayPair(month, day)
    }
}

pub fn parse_bool(v: &str) -> bool {
    match v {
        "N" => false,
        "Y" => true,
        _ => panic!("h1"),
    }
}

pub fn parse_date(value: &str) -> NaiveDate {
    NaiveDate::parse_from_str(value, DATE_FORMAT).unwrap()
}