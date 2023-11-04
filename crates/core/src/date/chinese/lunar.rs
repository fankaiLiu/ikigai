use chrono::{Datelike, TimeZone, Utc};

use super::animal::Animal;

#[derive(Debug, PartialEq)]
pub struct Lunar {
    t: chrono::DateTime<chrono::Utc>,
    year: i64,
    month: i64,
    day: i64,
    month_is_leap: bool,
}
const NUMBER_ALIAS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

const DATE_ALIAS: [&str; 4] = ["初", "十", "廿", "卅"];

const LUNAR_MONTH_ALIAS: [&str; 12] = [
    "正", "二", "三", "四", "五", "六", "七", "八", "九", "十", "冬", "腊",
];

const LUNARS: [i64; 201] = [
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0,
    0x055d2, // 1900-1909
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, 0x0d6a0, 0x0ada2, 0x095b0,
    0x14977, // 1910-1919
    0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, 0x02b60, 0x09570, 0x052f2,
    0x04970, // 1920-1929
    0x06566, 0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60, 0x186e3, 0x092e0, 0x1c8d7,
    0x0c950, // 1930-1939
    0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950,
    0x0b557, // 1940-1949
    0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5b0, 0x14573, 0x052b0, 0x0a9a8, 0x0e950,
    0x06aa0, // 1950-1959
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, 0x0f263, 0x0d950, 0x05b57,
    0x056a0, // 1960-1969
    0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, 0x0d558, 0x0b540, 0x0b6a0,
    0x195a6, // 1970-1979
    0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0af46, 0x0ab60,
    0x09570, // 1980-1989
    0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, 0x055c0, 0x0ab60, 0x096d5,
    0x092e0, // 1990-1999
    0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, 0x0abb7, 0x025d0, 0x092d0,
    0x0cab5, // 2000-2009
    0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, 0x0a5b0, 0x15176, 0x052b0,
    0x0a930, // 2010-2019
    0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, 0x0a4e0, 0x0d260, 0x0ea65,
    0x0d530, // 2020-2029
    0x05aa0, 0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0, 0x1d0b6, 0x0d250, 0x0d520,
    0x0dd45, // 2030-2039
    0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255, 0x06d20,
    0x0ada0, // 2040-2049
    0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06b20, 0x1a6c4,
    0x0aae0, // 2050-2059
    0x0a2e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0a6d0,
    0x055d4, // 2060-2069
    0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0,
    0x052b0, // 2070-2079
    0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55, 0x04b60, 0x0a570, 0x054e4,
    0x0d160, // 2080-2089
    0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, 0x0a9d4, 0x0a2d0, 0x0d150,
    0x0f252, // 2090-2099
    0x0d520, // 2100
];

impl Lunar {
    pub fn new(t: chrono::DateTime<chrono::Utc>) -> Lunar {
        let (year, month, day, is_leap) = Self::from_solar_timestamp(t.timestamp());
        Lunar {
            t: t,
            year: year,
            month: month,
            day: day,
            month_is_leap: is_leap,
        }
    }

    pub fn from_solar_timestamp(ts: i64) -> (i64, i64, i64, bool) {
        let lunar_year;
        let lunar_month;
        let lunar_day;
        let lunar_month_is_leap;

        // Get date from input Unix timestamp
        let dt = Utc.timestamp(ts, 0);
        // Create a new Utc datetime with the same date but time at zero
        let dt1 = Utc.ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0);
        // Create a new Utc datetime for 1900-01-31 at zero time
        let dt2 = Utc.ymd(1900, 1, 31).and_hms(0, 0, 0);
        // Calculate the difference in days
        let mut offset = (dt1.timestamp() - dt2.timestamp()) / 86400;

        // // 1900-01-31 in Unix timestamp
        // let start_timestamp: i64 = -2206425600;
        // let mut offset = (ts - start_timestamp) / 86400;

        let mut i = 1900;
        let mut days_of_year = 0;
        while i < 2101 && offset > 0 {
            days_of_year = Self::days_of_lunar_year(i);
            offset -= days_of_year;
            i += 1;
        }
        if offset < 0 {
            offset += days_of_year;
            i -= 1;
        }
        lunar_year = i;

        let leap = Self::leap_month(i);
        let mut is_leap = false;
        let mut days_of_month = 0;

        let mut month_counter = 1;
        while month_counter < 13 {
            //leap month
            if leap > 0 && month_counter == leap + 1 && !is_leap {
                month_counter -= 1;
                is_leap = true;
                days_of_month = Self::leap_days(lunar_year);
            } else {
                days_of_month = Self::lunar_days(lunar_year, month_counter);
            }
            if is_leap && month_counter == leap + 1 {
                is_leap = false;
            }
            offset -= days_of_month;
            if offset <= 0 {
                break;
            }
            month_counter += 1;
        }
        // If offset is 0, and the month just calculated is a leap month, correct
        if offset == 0 && leap > 0 && month_counter == leap + 1 {
            if is_leap {
                is_leap = false;
            } else {
                is_leap = true;
                month_counter -= 1;
            }
        }
        if offset < 0 {
            offset += days_of_month;
            month_counter -= 1;
        }
        lunar_month = month_counter + 1;
        lunar_day = offset + 1;
        lunar_month_is_leap = is_leap;
        (lunar_year, lunar_month, lunar_day, lunar_month_is_leap)
    }

    fn to_solar_timestamp(
        year: i64,
        month: i64,
        day: i64,
        hour: i64,
        minute: i64,
        second: i64,
        is_leap_month: bool,
    ) -> i64 {
        let mut offset = 0;

        // validity check
        if year < 1900 || year > 2100 {
            return 0;
        }

        let m = Self::leap_month(year);
        let mut is_leap_month = is_leap_month && (m == month);

        // beyond the max boundary
        if (year == 2100 && month == 12 && day > 1) || (year == 1900 && month == 1 && day < 31) {
            return 0;
        }

        let days = Self::lunar_days(year, month);
        let mut max_days = days;

        // if month is leap, use leap_days method
        if is_leap_month {
            max_days = Self::leap_days(year);
        }

        // validity check
        if day > max_days {
            return 0;
        }

        // calculate lunar offset
        for i in 1900..year {
            offset += Self::days_of_lunar_year(i);
        }

        let mut is_add = false;
        for i in 1..month {
            let leap = Self::leap_month(year);
            if !is_add {
                // handle leap month
                if leap <= i && leap > 0 {
                    offset += Self::leap_days(year);
                    is_add = true;
                }
            }
            offset += Self::lunar_days(year, i);
        }

        // if it's a leap month, add the days of this month
        if is_leap_month {
            offset += days;
        }

        // 1900-01-30 in Unix timestamp
        let start_timestamp: i64 = -2206512000;

        (offset + day) * 86400 + start_timestamp + hour * 3600 + minute * 60 + second
    }

    pub fn is_leap(&self) -> bool {
        Self::leap_month(self.year) != 0
    }

    pub fn is_leap_month(&self) -> bool {
        self.month_is_leap
    }

    pub fn get_year(&self) -> i64 {
        self.year
    }

    pub fn get_month(&self) -> i64 {
        self.month
    }

    pub fn get_day(&self) -> i64 {
        self.day
    }

    pub fn days_of_lunar_year(year: i64) -> i64 {
        let mut sum = 29 * 12;
        let mut mask = 0x8000;
        while mask > 0x8 {
            if LUNARS[(year - 1900) as usize] & mask != 0 {
                sum += 1;
            }
            mask >>= 1;
        }
        sum + Self::leap_days(year)
    }

    pub fn leap_month(year: i64) -> i64 {
        LUNARS[(year - 1900) as usize] & 0xf
    }

    pub fn leap_days(year: i64) -> i64 {
        let leap = Self::leap_month(year);
        if leap == 0 {
            0
        } else if LUNARS[(year - 1900) as usize] & 0x10000 != 0 {
            30
        } else {
            29
        }
    }

    pub fn lunar_days(year: i64, month: i64) -> i64 {
        if month > 12 || month < 1 {
            0
        } else if LUNARS[(year - 1900) as usize] & (0x10000 >> month) != 0 {
            30
        } else {
            29
        }
    }
    // 获取生肖
    pub fn animal(&self) -> Animal {
        let index = (self.year - 3) % 12;
        Animal::new(index).unwrap()
    }

    // 汉字表示年份
    pub fn year_alias(&self) -> String {
        let mut result = self.year.to_string();
        for (i, alias) in NUMBER_ALIAS.iter().enumerate() {
            result = result.replace(&i.to_string(), *alias);
        }
        result
    }

    // 汉字表示月份
    pub fn month_alias(&self) -> String {
        let mut alias = String::new();
        if self.month_is_leap {
            alias += "闰";
        }
        alias += LUNAR_MONTH_ALIAS[self.month as usize - 1];
        alias += "月";
        alias
    }

    // 汉字表示日期
    pub fn day_alias(&self) -> String {
        let mut alias = String::new();
        match self.day {
            10 => alias += "初十",
            20 => alias += "二十",
            30 => alias += "三十",
            _ => {
                let n = (self.day / 10) as usize;
                alias += DATE_ALIAS[n];
                alias += NUMBER_ALIAS[(self.day % 10) as usize];
            }
        }
        alias
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn test_from_solar_timestamp() {
        let tests = vec![
            ("test_2", 1522422690, 2018, 2, 14, false),
            ("test_1", 1502769600, 2017, 6, 24, true),
            ("test_3", 1699025232, 2023, 9, 20, false),
        ];

        for (
            name,
            ts,
            want_lunar_year,
            want_lunar_month,
            want_lunar_day,
            want_lunar_month_is_leap,
        ) in tests
        {
            println!("Running {}", name);
            let (got_lunar_year, got_lunar_month, got_lunar_day, got_lunar_month_is_leap) =
                Lunar::from_solar_timestamp(ts);

            assert_eq!(
                got_lunar_year, want_lunar_year,
                "Mismatch in lunar year for {}",
                name
            );
            assert_eq!(
                got_lunar_month, want_lunar_month,
                "Mismatch in lunar month for {}",
                name
            );
            assert_eq!(
                got_lunar_day, want_lunar_day,
                "Mismatch in lunar day for {}",
                name
            );
            assert_eq!(
                got_lunar_month_is_leap, want_lunar_month_is_leap,
                "Mismatch in lunar month leap status for {}",
                name
            );
        }
    }

    #[test]
    fn test_to_solar_timestamp() {
        let test_cases = vec![
            ("test_1", 2017, 6, 24, 12, 0, 0, true, 1502769600),
            ("test_2", 2018, 2, 14, 23, 11, 30, true, 1522422690),
            ("test_3", 2018, 2, 14, 23, 11, 30, false, 1522422690),
            ("test_4", 1900, 1, 14, 23, 11, 30, false, 0),
            ("test_5", 2100, 12, 14, 23, 11, 30, false, 0),
            ("test_6", 1900 - 1, 1, 14, 23, 11, 30, false, 0),
            ("test_7", 2100 + 1, 12, 14, 23, 11, 30, false, 0),
            ("test_8", 1900, 2, 100000, 23, 11, 30, false, 0),
        ];

        for case in test_cases {
            let name = case.0;
            let want = case.8;

            let got =
                Lunar::to_solar_timestamp(case.1, case.2, case.3, case.4, case.5, case.6, case.7);

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_new_lunar() {
        let t1 = Utc.ymd(2017, 8, 15).and_hms(12, 0, 0);
        let t2 = Utc.ymd(2018, 3, 30).and_hms(23, 11, 30);

        let test_cases = vec![
            (
                "test_1",
                &t1,
                Lunar {
                    t: t1,
                    year: 2017,
                    month: 6,
                    day: 24,
                    month_is_leap: true,
                },
            ),
            (
                "test_2",
                &t2,
                Lunar {
                    t: t2,
                    year: 2018,
                    month: 2,
                    day: 14,
                    month_is_leap: false,
                },
            ),
        ];

        for case in test_cases {
            let name = case.0;
            let arg = case.1;
            let want = case.2;

            let got = Lunar::new(arg.clone());

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_leap_month() {
        let t1 = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 1).and_hms(0, 0, 0);

        let test_cases = vec![("test_1", Lunar::new(t1), 0), ("test_2", Lunar::new(t2), 6)];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;
            assert_eq!(Lunar::leap_month(lunar.year), want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_is_leap() {
        let t1 = Utc.ymd(2018, 6, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 1).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), false),
            ("test_2", Lunar::new(t2), true),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.is_leap();

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_is_leap_month() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), false),
            ("test_2", Lunar::new(t2), false),
            ("test_3", Lunar::new(t3), true),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.is_leap_month();

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_animal() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), Animal::new(11)),
            ("test_2", Lunar::new(t2), Animal::new(10)),
            ("test_3", Lunar::new(t3), Animal::new(10)),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2.unwrap();

            let got = lunar.animal();

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_year_alias() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), "二零一八"),
            ("test_2", Lunar::new(t2), "二零一七"),
            ("test_3", Lunar::new(t3), "二零一七"),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.year_alias();

            assert_eq!(got, want, "{} failed", name);
        }
    }
    #[test]
    fn test_lunar_get_year() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), 2018),
            ("test_2", Lunar::new(t2), 2017),
            ("test_3", Lunar::new(t3), 2017),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.year;

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_month_alias() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), "三月"),
            ("test_2", Lunar::new(t2), "五月"),
            ("test_3", Lunar::new(t3), "闰六月"),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.month_alias();

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_get_month() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 15).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 8, 15).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), 3),
            ("test_2", Lunar::new(t2), 5),
            ("test_3", Lunar::new(t3), 6),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.month;

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_day_alias() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 4).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 6, 14).and_hms(0, 0, 0);
        let t4 = Utc.ymd(2017, 8, 21).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), "十六"),
            ("test_2", Lunar::new(t2), "初十"),
            ("test_3", Lunar::new(t3), "二十"),
            ("test_33", Lunar::new(t4), "三十"),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.day_alias();

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_get_day() {
        let t1 = Utc.ymd(2018, 5, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2017, 6, 4).and_hms(0, 0, 0);
        let t3 = Utc.ymd(2017, 6, 14).and_hms(0, 0, 0);
        let t4 = Utc.ymd(2017, 8, 21).and_hms(0, 0, 0);

        let test_cases = vec![
            ("test_1", Lunar::new(t1), 16),
            ("test_2", Lunar::new(t2), 10),
            ("test_3", Lunar::new(t3), 20),
            ("test_33", Lunar::new(t4), 30),
        ];

        for case in test_cases {
            let name = case.0;
            let lunar = case.1;
            let want = case.2;

            let got = lunar.day;

            assert_eq!(got, want, "{} failed", name);
        }
    }

    #[test]
    fn test_lunar_days() {
        let test_cases = vec![
            ("test_1", 2018, 0, 0),
            ("test_2", 2018, 13, 0),
            ("test_3", 2017, 6, 29),
            ("test_4", 2017, 8, 30),
        ];

        for case in test_cases {
            let name = case.0;
            let want = case.3;

            let got = Lunar::lunar_days(case.1, case.2);

            assert_eq!(got, want, "{} failed", name);
        }
    }
}
