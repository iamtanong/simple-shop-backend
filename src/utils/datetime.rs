use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn get_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn get_date() -> NaiveDate {
    Local::now().date_naive()
}

pub fn get_time() -> NaiveTime {
    Local::now().time()
}
