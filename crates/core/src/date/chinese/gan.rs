#[derive(Debug, PartialEq)]
pub struct Gan {
    order: i64,
}

const GAN_ALIAS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

impl Gan {
    pub fn new(order: i64) -> Option<Gan> {
        if !Self::is_supported(order) {
            return None;
        }
        Some(Gan { order })
    }

    pub fn alias(&self) -> &str {
        GAN_ALIAS[(self.order as usize - 1) % 10]
    }

    pub fn order(&self) -> i64 {
        self.order
    }

    fn is_supported(order: i64) -> bool {
        (1..=10).contains(&order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_gan() {
        assert!(Gan::new(1).is_some());
        assert!(Gan::new(10).is_some());
        assert!(Gan::new(0).is_none());
        assert!(Gan::new(11).is_none());
    }

    #[test]
    fn test_gan_alias() {
        let gan = Gan::new(1).unwrap();
        assert_eq!(gan.alias(), "甲");

        let gan = Gan::new(10).unwrap();
        assert_eq!(gan.alias(), "癸");
    }

    #[test]
    fn test_gan_order() {
        let gan = Gan::new(1).unwrap();
        assert_eq!(gan.order(), 1);

        let gan = Gan::new(10).unwrap();
        assert_eq!(gan.order(), 10);
    }
}
