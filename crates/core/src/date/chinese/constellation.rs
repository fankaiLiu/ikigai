use chrono::{DateTime, Datelike, Utc};

pub struct Constellation {
    t: DateTime<Utc>,
}

impl Constellation {
    pub fn new(t: DateTime<Utc>) -> Constellation {
        Constellation { t }
    }

    pub fn alias(&self) -> Option<&'static str> {
        match (self.t.month(), self.t.day()) {
            (1, day) if day >= 20 => Some("水瓶"),
            (2, day) if day <= 18 => Some("水瓶"),
            (2, day) if day >= 19 => Some("双鱼"),
            (3, day) if day <= 20 => Some("双鱼"),
            (3, day) if day >= 21 => Some("白羊"),
            (4, day) if day <= 19 => Some("白羊"),
            (4, day) if day >= 20 => Some("金牛"),
            (5, day) if day <= 20 => Some("金牛"),
            (5, day) if day >= 21 => Some("双子"),
            (6, day) if day <= 21 => Some("双子"),
            (6, day) if day >= 22 => Some("巨蟹"),
            (7, day) if day <= 22 => Some("巨蟹"),
            (7, day) if day >= 23 => Some("狮子"),
            (8, day) if day <= 22 => Some("狮子"),
            (8, day) if day >= 23 => Some("处女"),
            (9, day) if day <= 22 => Some("处女"),
            (9, day) if day >= 23 => Some("天秤"),
            (10, day) if day <= 23 => Some("天秤"),
            (10, day) if day >= 24 => Some("天蝎"),
            (11, day) if day <= 22 => Some("天蝎"),
            (11, day) if day >= 23 => Some("射手"),
            (12, day) if day <= 21 => Some("射手"),
            (12, day) if day >= 22 => Some("摩羯"),
            (1, day) if day <= 19 => Some("摩羯"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn test_time(month: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2018, month, 1, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_new_constellation() {
        let now = now();
        let constellation = Constellation::new(now);
        assert_eq!(
            constellation.alias().unwrap(),
            Constellation::new(now).alias().unwrap()
        );
    }

    #[test]
    fn test_constellation_alias() {
        let tests = vec![
            ("摩羯", test_time(1), "摩羯"),
            ("水瓶", test_time(2), "水瓶"),
            ("双鱼", test_time(3), "双鱼"),
            ("白羊", test_time(4), "白羊"),
            ("金牛", test_time(5), "金牛"),
            ("双子", test_time(6), "双子"),
            ("巨蟹", test_time(7), "巨蟹"),
            ("狮子", test_time(8), "狮子"),
            ("处女", test_time(9), "处女"),
            ("天秤", test_time(10), "天秤"),
            ("天蝎", test_time(11), "天蝎"),
            ("射手", test_time(12), "射手"),
        ];

        for (name, time, expected) in tests {
            let constellation = Constellation::new(time);
            assert_eq!(
                constellation.alias().unwrap(),
                expected,
                "{} test failed",
                name
            );
        }
    }
}
