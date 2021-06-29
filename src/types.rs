use chrono::Month;
use num_traits::FromPrimitive;

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
        _ => panic!("h1")
    }
}
