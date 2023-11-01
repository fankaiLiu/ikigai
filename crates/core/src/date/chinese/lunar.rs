use chrono::{Datelike, Timelike, Weekday};

use super::{animal::Animal, constellation::Constellation, solarterm::Solarterm};

pub struct Solar {
    t: chrono::DateTime<chrono::Utc>,
    current_solarterm: Option<Solarterm>,
    prev_solarterm: Solarterm,
    next_solarterm: Solarterm,
}

impl Solar {
    pub fn new(t: chrono::DateTime<chrono::Utc>) -> Self {
        todo!()
    }
    pub fn is_leap(&self) -> bool {
        let year = self.t.year();
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    pub fn week_number(&self) -> usize {
        self.t.weekday().num_days_from_sunday() as usize
    }

    pub fn week_alias(&self) -> &'static str {
        let week_alias = ["日", "一", "二", "三", "四", "五", "六"];
        week_alias[self.week_number()]
    }

    pub fn animal(&self) -> Animal {
        //Animal::new(order_mod((self.t.year() - 3) as i64, 12))
        todo!()
    }

    pub fn constellation(&self) -> Constellation {
        Constellation::new(self.t)
    }

    pub fn get_year(&self) -> i32 {
        self.t.year()
    }

    pub fn get_month(&self) -> u32 {
        self.t.month()
    }

    pub fn get_day(&self) -> u32 {
        self.t.day()
    }

    pub fn get_hour(&self) -> u32 {
        self.t.hour()
    }

    pub fn get_minute(&self) -> u32 {
        self.t.minute()
    }

    pub fn get_second(&self) -> u32 {
        self.t.second()
    }

    pub fn get_nanosecond(&self) -> u32 {
        self.t.nanosecond()
    }
}
