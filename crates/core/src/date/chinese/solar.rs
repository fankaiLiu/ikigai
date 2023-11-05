use chrono::{Datelike, Timelike};

use super::{animal::Animal, constellation::Constellation, solarterm::Solarterm, utils::order_mod};

#[derive(Debug, Clone, PartialEq)]
pub struct Solar {
    t: chrono::DateTime<chrono::Utc>,
    current_solarterm: Option<Solarterm>,
    prev_solarterm: Solarterm,
    next_solarterm: Solarterm,
}

impl Solar {
    pub fn new(t: chrono::DateTime<chrono::Utc>) -> Option<Self> {
        let (mut p, mut n) = Solarterm::calc_solarterm(&t);
        let mut c = None;
        match (&mut p, &mut n) {
            (Some(p), Some(n)) => {
                if n.index() - p.index() == 1 {
                    if p.is_in_day(&t) {
                        c = Some(p.clone());
                        *p = p.prev()?;
                    }
                    if n.is_in_day(&t) {
                        c = Some(n.clone());
                        *p = c.as_ref()?.prev()?;
                        *n = c.as_ref()?.next()?;
                    }
                }
                Some(Self {
                    t,
                    current_solarterm: c,
                    prev_solarterm: p.clone(),
                    next_solarterm: n.clone(),
                })
            }
            _ => None,
        }
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

    pub fn animal(&self) -> Option<Animal> {
        Animal::new(order_mod((self.t.year() - 3) as i64, 12))
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_new_solar() {
        let t1 = Utc.with_ymd_and_hms(2018, 3, 21, 0, 0, 26).unwrap();
        let t2 = Utc.with_ymd_and_hms(2018, 3, 21, 0, 15, 26).unwrap();
        let t3 = Utc.with_ymd_and_hms(2018, 3, 21, 0, 15, 27).unwrap();
        let t4 = Utc.with_ymd_and_hms(2018, 3, 21, 0, 16, 26).unwrap();
        let t5: chrono::DateTime<Utc> = Utc.with_ymd_and_hms(2018, 4, 1, 0, 0, 0).unwrap();

        let tests = vec![
            (
                t1,
                Solarterm::new(2741),
                Solarterm::new(2740),
                Solarterm::new(2742),
            ),
            (
                t2,
                Solarterm::new(2741),
                Solarterm::new(2740),
                Solarterm::new(2742),
            ),
            (
                t3,
                Solarterm::new(2741),
                Solarterm::new(2740),
                Solarterm::new(2742),
            ),
            (
                t4,
                Solarterm::new(2741),
                Solarterm::new(2740),
                Solarterm::new(2742),
            ),
            (t5, None, Solarterm::new(2741), Solarterm::new(2742)),
        ];

        for (t, cs, ps, ns) in tests {
            let solar = Solar::new(t).unwrap();
            assert_eq!(solar.t, t);
            assert_eq!(solar.current_solarterm, cs);
            assert_eq!(solar.prev_solarterm, ps.unwrap());
            assert_eq!(solar.next_solarterm, ns.unwrap());
        }
    }
    #[test]
    fn test_solar_is_leap() {
        let t1 = Utc.ymd(2018, 3, 21).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2020, 3, 21).and_hms(0, 1, 0);

        let tests = vec![
            ("test_2018", Solar::new(t1).unwrap(), false),
            ("test_2020", Solar::new(t2).unwrap(), true),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.is_leap();
            assert_eq!(got, want, "Solar.is_leap() = {}, want {}", got, want);
        }
    }
    #[test]
    fn test_solar_week_number() {
        let t1 = Utc.ymd(2018, 3, 21).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 3, 25).and_hms(0, 1, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 3),
            ("test_2", Solar::new(t2).unwrap(), 0),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.week_number();
            assert_eq!(got, want, "Solar.week_number() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_week_alias() {
        let t1 = Utc.ymd(2018, 3, 21).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 3, 25).and_hms(0, 1, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), "三"),
            ("test_2", Solar::new(t2).unwrap(), "日"),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.week_alias();
            assert_eq!(got, want, "Solar.week_alias() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_animal() {
        let t1 = Utc.ymd(2018, 3, 21).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2019, 3, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), Animal::new(11)),
            ("test_2", Solar::new(t2).unwrap(), Animal::new(12)),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.animal();
            assert_eq!(got, want, "Solar.animal() = {:?}, want {:?}", got, want);
        }
    }

    #[test]
    fn test_solar_constellation() {
        let t1 = Utc.ymd(2018, 3, 21).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), Constellation::new(t1)),
            ("test_2", Solar::new(t2).unwrap(), Constellation::new(t2)),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.constellation();
            assert_eq!(
                got, want,
                "Solar.constellation() = {:?}, want {:?}",
                got, want
            );
        }
    }

    #[test]
    fn test_solar_get_year() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 2015),
            ("test_2", Solar::new(t2).unwrap(), 2018),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_year();
            assert_eq!(got, want, "Solar.get_year() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_get_month() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 1),
            ("test_2", Solar::new(t2).unwrap(), 11),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_month();
            assert_eq!(got, want, "Solar.get_month() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_get_day() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 20),
            ("test_2", Solar::new(t2).unwrap(), 21),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_day();
            assert_eq!(got, want, "Solar.get_day() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_get_hour() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 0),
            ("test_2", Solar::new(t2).unwrap(), 0),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_hour();
            assert_eq!(got, want, "Solar.get_hour() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_get_minute() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 0),
            ("test_2", Solar::new(t2).unwrap(), 0),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_minute();
            assert_eq!(got, want, "Solar.get_minute() = {}, want {}", got, want);
        }
    }
    #[test]
    fn test_solar_get_second() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 0),
            ("test_2", Solar::new(t2).unwrap(), 0),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_second();
            assert_eq!(got, want, "Solar.get_second() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_get_nanosecond() {
        let t1 = Utc.ymd(2015, 1, 20).and_hms(0, 0, 0);
        let t2 = Utc.ymd(2018, 11, 21).and_hms(0, 0, 0);

        let tests = vec![
            ("test_1", Solar::new(t1).unwrap(), 0),
            ("test_2", Solar::new(t2).unwrap(), 0),
        ];

        for (name, solar, want) in tests {
            println!("Running test '{}'", name);
            let got = solar.get_nanosecond();
            assert_eq!(got, want, "Solar.get_nanosecond() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_solar_equals() {
        let t1 = Utc::now();
        let t2 = t1 + chrono::Duration::days(1);

        let tests = vec![
            (
                "test_1",
                Solar::new(t1).unwrap(),
                Solar::new(t1).unwrap(),
                true,
            ),
            (
                "test_2",
                Solar::new(t1).unwrap(),
                Solar::new(t2).unwrap(),
                false,
            ),
        ];

        for (name, solar1, solar2, want) in tests {
            println!("Running test '{}'", name);
            assert_eq!(solar1 == solar2, want);
        }
    }
}
