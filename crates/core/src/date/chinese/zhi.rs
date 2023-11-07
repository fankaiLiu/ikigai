#[derive(Debug, PartialEq)]
pub struct Zhi {
    order: i64,
}

const ZHI_ALIAS: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

impl Zhi {
    // Creates a `Zhi` object
    pub fn new(order: i64) -> Option<Zhi> {
        if Zhi::is_supported(order) {
            Some(Zhi { order })
        } else {
            None
        }
    }

    // Returns the name of the 'Zhi' (子丑寅卯...)
    pub fn alias(&self) -> &str {
        ZHI_ALIAS[(self.order - 1) as usize % 12]
    }

    // Returns the order of the 'Zhi' (1234...)
    pub fn order(&self) -> i64 {
        self.order
    }

    fn is_supported(order: i64) -> bool {
        (1..=12).contains(&order)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn min_order() -> i64 {
        1
    }

    fn max_order() -> i64 {
        12
    }

    #[test]
    fn test_new_zhi() {
        let tests = vec![
            ("nil_min", min_order() - 1, None),
            ("nil_max", max_order() + 1, None),
            ("test_min", min_order(), Some(Zhi { order: min_order() })),
            ("test_max", max_order(), Some(Zhi { order: max_order() })),
            ("test", 8, Some(Zhi { order: 8 })),
        ];

        for (name, order, want) in tests {
            println!("Running test '{}'", name);
            let got = Zhi::new(order);
            assert_eq!(
                got, want,
                "Zhi::new({}) = {:?}, want {:?}",
                order, got, want
            );
        }
    }

    #[test]
    fn test_zhi_alias() {
        let tests = vec![
            ("test_1", Zhi::new(min_order()).unwrap(), "子"),
            ("test_2", Zhi::new(max_order()).unwrap(), "亥"),
        ];

        for (name, zhi, want) in tests {
            println!("Running test '{}'", name);
            let got = zhi.alias();
            assert_eq!(got, want, "Zhi.alias() = {}, want {}", got, want);
        }
    }

    #[test]
    fn test_zhi_order() {
        let tests = vec![
            ("test_1", Zhi::new(min_order()).unwrap(), 1),
            ("test_2", Zhi::new(max_order()).unwrap(), 12),
        ];

        for (name, zhi, want) in tests {
            println!("Running test '{}'", name);
            let got = zhi.order();
            assert_eq!(got, want, "Zhi.order() = {}, want {}", got, want);
        }
    }
}
