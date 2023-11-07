#[derive(Debug, PartialEq, Clone)]
pub struct Animal {
    order: i64,
}

const ANIMAL_ALIAS: [&str; 12] = [
    "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪",
];

impl Animal {
    pub fn new(order: i64) -> Option<Animal> {
        if !Self::is_supported(order) {
            return None;
        }
        Some(Animal { order })
    }

    pub fn alias(&self) -> &str {
        ANIMAL_ALIAS[((self.order - 1) % 12) as usize]
    }

    fn is_supported(order: i64) -> bool {
        (1..=12).contains(&order)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_animal() {
        assert!(Animal::new(1).is_some());
        assert!(Animal::new(12).is_some());
        assert!(Animal::new(13).is_none());
        assert!(Animal::new(0).is_none());
    }

    #[test]
    fn test_animal_alias() {
        let animal_1 = Animal::new(1).unwrap();
        assert_eq!(animal_1.alias(), "鼠");

        let animal_12 = Animal::new(12).unwrap();
        assert_eq!(animal_12.alias(), "猪");

        let animal_13 = Animal::new(13);
        assert!(animal_13.is_none());
    }
}
