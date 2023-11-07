use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

use super::{
    animal::Animal,
    gan::Gan,
    solarterm::{self, Solarterm, SOLARTERM_FROM_YEAR},
    utils,
    zhi::Zhi,
};
#[derive(Debug, PartialEq)]
pub struct Ganzhi {
    t: DateTime<Utc>,
    pub year_gan: Gan,
    pub year_zhi: Zhi,
    pub month_gan: Gan,
    pub month_zhi: Zhi,
    pub day_gan: Gan,
    pub day_zhi: Zhi,
    pub hour_gan: Gan,
    pub hour_zhi: Zhi,
    pub prev_solarterm: Solarterm,
    pub next_solarterm: Solarterm,
}

impl Ganzhi {
    pub fn new(t: DateTime<Utc>) -> Option<Self> {
        let mut year = t.year() as i64;
        if !Self::is_supported(year) {
            return None;
        }
        if t.timestamp() < Solarterm::spring_timestamp(year)? {
            year -= 1;
        }
        let year_gan = Gan::new(utils::order_mod(year - 3, 10))?;
        let year_zhi = Zhi::new(utils::order_mod(year - 3, 12))?;

        let (p, n) = Solarterm::calc_solarterm(&t);
        let p = p?;
        let n = n?;
        let mut i = p.index();
        if n.index() - p.index() == 2 {
            i += 1;
        }
        i = utils::order_mod((i % 24) / 2, 12);

        let month_zhi = Zhi::new(utils::order_mod(i + 2, 12))?;
        let month_gan = Gan::new(utils::order_mod(i + year_gan.order() * 2, 10))?;

        let begin = Utc.ymd(SOLARTERM_FROM_YEAR as i32, 1, 1).and_hms(0, 0, 0);
        let seconds = t.signed_duration_since(begin).num_seconds();
        let day_order = utils::order_mod(seconds / 86400 + 31, 60);

        let day_gan = Gan::new(utils::order_mod(day_order, 10))?;
        let day_zhi = Zhi::new(utils::order_mod(day_order, 12))?;

        let hour_zhi = Zhi::new(utils::order_mod(((t.hour() + 1) / 2) as i64 + 1, 12))?;
        let hour_gan = Gan::new(utils::order_mod(
            hour_zhi.order() - 2 + day_gan.order() * 2,
            10,
        ))?;

        Some(Ganzhi {
            t,
            year_gan,
            year_zhi,
            month_gan,
            month_zhi,
            day_gan,
            day_zhi,
            hour_gan,
            hour_zhi,
            prev_solarterm: p,
            next_solarterm: n,
        })
    }

    // Animal returns the animal of the year
    pub fn animal(&self) -> Animal {
        Animal::new(self.year_zhi.order()).unwrap()
    }

    // YearGanzhiAlias returns the alias of the year (Jiazi, Yichou...)
    pub fn year_ganzhi_alias(&self) -> String {
        format!("{}{}", self.year_gan.alias(), self.year_zhi.alias())
    }

    // MonthGanzhiAlias returns the alias of the month (Jiazi, Yichou...)
    pub fn month_ganzhi_alias(&self) -> String {
        format!("{}{}", self.month_gan.alias(), self.month_zhi.alias())
    }

    // DayGanzhiAlias returns the alias of the day (Jiazi, Yichou...)
    pub fn day_ganzhi_alias(&self) -> String {
        format!("{}{}", self.day_gan.alias(), self.day_zhi.alias())
    }

    // HourGanzhiAlias returns the alias of the hour (Jiazi, Yichou...)
    pub fn hour_ganzhi_alias(&self) -> String {
        format!("{}{}", self.hour_gan.alias(), self.hour_zhi.alias())
    }

    // YearGanzhiOrder returns the order of the year (1,2...)
    pub fn year_ganzhi_order(&self) -> i64 {
        Self::ganzhi_order(self.year_gan.order(), self.year_zhi.order())
    }

    // MonthGanzhiOrder returns the order of the month (1,2...)
    pub fn month_ganzhi_order(&self) -> i64 {
        Self::ganzhi_order(self.month_gan.order(), self.month_zhi.order())
    }

    // DayGanzhiOrder returns the order of the day (1,2...)
    pub fn day_ganzhi_order(&self) -> i64 {
        Self::ganzhi_order(self.day_gan.order(), self.day_zhi.order())
    }

    // HourGanzhiOrder returns the order of the hour (1,2...)
    pub fn hour_ganzhi_order(&self) -> i64 {
        Self::ganzhi_order(self.hour_gan.order(), self.hour_zhi.order())
    }

    pub fn is_supported(year: i64) -> bool {
        solarterm::SOLARTERM_FROM_YEAR <= year && year < solarterm::SOLARTERM_TO_YEAR
    }

    pub fn ganzhi_order(gan_order: i64, zhi_order: i64) -> i64 {
        utils::order_mod(((gan_order + 10 - zhi_order) % 10) / 2 * 12 + zhi_order, 60)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn maker(
        t: DateTime<Utc>,
        yg: i64,
        yz: i64,
        mg: i64,
        mz: i64,
        dg: i64,
        dz: i64,
        hg: i64,
        hz: i64,
        p: i64,
        n: i64,
    ) -> Ganzhi {
        Ganzhi {
            t: t,
            year_gan: Gan::new(yg).unwrap(),
            year_zhi: Zhi::new(yz).unwrap(),
            month_gan: Gan::new(mg).unwrap(),
            month_zhi: Zhi::new(mz).unwrap(),
            day_gan: Gan::new(dg).unwrap(),
            day_zhi: Zhi::new(dz).unwrap(),
            hour_gan: Gan::new(hg).unwrap(),
            hour_zhi: Zhi::new(hz).unwrap(),
            prev_solarterm: Solarterm::new(p).unwrap(),
            next_solarterm: Solarterm::new(n).unwrap(),
        }
    }

    #[test]
    fn test_new_ganzhi() {
        let t1 = Utc
            .ymd((solarterm::SOLARTERM_FROM_YEAR - 1) as i32, 6, 1)
            .and_hms(0, 0, 0);
        let t2 = Utc
            .ymd((solarterm::SOLARTERM_TO_YEAR + 1) as i32, 6, 1)
            .and_hms(0, 0, 0);
        let t3 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t4 = Utc.ymd(2018, 2, 4).and_hms(5, 28, 28);
        let t5 = Utc.ymd(2018, 2, 4).and_hms(5, 28, 29);
        let t6 = Utc.ymd(2018, 2, 4).and_hms(5, 28, 30);

        let test_cases = vec![
            (t1, None),
            (t2, None),
            (t3, Some(maker(t3, 4, 10, 9, 1, 10, 6, 9, 1, 23, 0))),
            (t4, Some(maker(t4, 4, 10, 10, 2, 4, 4, 10, 4, 1, 2))),
            (t5, Some(maker(t5, 5, 11, 1, 3, 4, 4, 10, 4, 1, 3))),
            (t6, Some(maker(t6, 5, 11, 1, 3, 4, 4, 10, 4, 2, 3))),
        ];

        for (t, expected) in test_cases {
            let got = Ganzhi::new(t);
            match got {
                Some(got) => {
                    let expected = expected.unwrap();
                    assert_eq!(got.year_gan, expected.year_gan, "For yg: {}", t);
                    assert_eq!(got.year_zhi, expected.year_zhi, "For yz: {}", t);
                    assert_eq!(got.month_gan, expected.month_gan, "For mg: {}", t);
                    assert_eq!(got.month_zhi, expected.month_zhi, "For mz: {}", t);
                    assert_eq!(got.day_gan, expected.day_gan, "For dg: {}", t);
                    assert_eq!(got.day_zhi, expected.day_zhi, "For dz: {}", t);
                    assert_eq!(got.hour_gan, expected.hour_gan, "For hg: {}", t);
                    assert_eq!(got.hour_zhi, expected.hour_zhi, "For hz: {}", t);
                    assert_eq!(
                        got.prev_solarterm.order(),
                        expected.prev_solarterm.order(),
                        "For p: {}",
                        t
                    );
                    assert_eq!(
                        got.next_solarterm.order(),
                        expected.next_solarterm.order(),
                        "For n: {}",
                        t
                    );
                }
                None => assert_eq!(got, expected, "For date: {}", t),
            }
        }
    }

    #[test]
    fn test_ganzhi_animal() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), Animal::new(10)),
            ("test_2", Ganzhi::new(t2), Animal::new(11)),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().animal();
            assert_eq!(
                got,
                want.clone().unwrap(),
                "Ganzhi::animal() = {:?}, want {:?}",
                got,
                want.unwrap()
            );
        }
    }

    #[test]
    fn test_ganzhi_year_ganzhi_alias() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), "丁酉"),
            ("test_2", Ganzhi::new(t2), "戊戌"),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().year_ganzhi_alias();
            assert_eq!(
                got, want,
                "Ganzhi::year_ganzhi_alias() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_month_ganzhi_alias() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), "壬子"),
            ("test_2", Ganzhi::new(t2), "甲寅"),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().month_ganzhi_alias();
            assert_eq!(
                got, want,
                "Ganzhi::month_ganzhi_alias() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_day_ganzhi_alias() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), "癸巳"),
            ("test_2", Ganzhi::new(t2), "戊辰"),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().day_ganzhi_alias();
            assert_eq!(
                got, want,
                "Ganzhi::day_ganzhi_alias() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_hour_ganzhi_alias() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), "壬子"),
            ("test_2", Ganzhi::new(t2), "壬子"),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().hour_ganzhi_alias();
            assert_eq!(
                got, want,
                "Ganzhi::hour_ganzhi_alias() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_year_ganzhi_order() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), 34),
            ("test_2", Ganzhi::new(t2), 35),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().year_ganzhi_order();
            assert_eq!(
                got, want,
                "Ganzhi::year_ganzhi_order() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_month_ganzhi_order() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), 49),
            ("test_2", Ganzhi::new(t2), 51),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().month_ganzhi_order();
            assert_eq!(
                got, want,
                "Ganzhi::month_ganzhi_order() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_day_ganzhi_order() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), 30),
            ("test_2", Ganzhi::new(t2), 5),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().day_ganzhi_order();
            assert_eq!(
                got, want,
                "Ganzhi::day_ganzhi_order() = {}, want {}",
                got, want
            );
        }
    }
    #[test]
    fn test_ganzhi_hour_ganzhi_order() {
        let t1 = Utc.ymd(2018, 1, 1).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 2, 5).and_hms(0, 0, 0);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), 49),
            ("test_2", Ganzhi::new(t2), 49),
        ];

        for (name, gz, want) in tests {
            println!("{}", name);
            let got = gz.unwrap().hour_ganzhi_order();
            assert_eq!(
                got, want,
                "Ganzhi::hour_ganzhi_order() = {}, want {}",
                got, want
            );
        }
    }

    #[test]
    fn test_ganzhi_equals() {
        let t1 = Utc::now();
        let t2 = t1 + Duration::days(1);
        let tests = vec![
            ("test_1", Ganzhi::new(t1), Ganzhi::new(t1), true),
            ("test_2", Ganzhi::new(t1), Ganzhi::new(t2), false),
        ];

        for (name, gz, gz2, want) in tests {
            println!("{}", name);
            assert_eq!(gz == gz2, want, "Ganzhi::equals() failed");
        }
    }
}
