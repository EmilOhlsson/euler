#[macro_use]
extern crate project_euler;
extern crate chrono;

use chrono::{NaiveDate, Datelike, Weekday};

fn main() {
    let mut date = NaiveDate::from_ymd(1901, 1, 1);
    let mut count = 0;

    while date.year() < 2001 {
        if date.weekday() == Weekday::Sun && date.day() == 1 {
            count += 1;
        }
        date = date.succ();
    }
    answer!("{}", count);
}
