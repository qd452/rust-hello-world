#[derive(Debug, PartialEq, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory { shirts: Vec::new() }
    }

    pub fn new_with_shirt(shirts: Vec<ShirtColor>) -> Inventory {
        Inventory { shirts: shirts }
    }

    pub fn add(&mut self, color: ShirtColor) {
        self.shirts.push(color);
    }

    pub fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        match user_preferences {
            Some(color) => color,
            None => self.most_stocked(),
        }
    }

    pub fn giveaway_closure(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        // The closure captures an immutable reference to the self Inventory instance
        user_preferences.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inventory() {
        let inventory = Inventory::new();
        assert_eq!(inventory.shirts.len(), 0);
    }

    #[test]
    fn test_new_inventory_with_shirts() {
        let shirts = vec![ShirtColor::Red, ShirtColor::Blue];
        let inventory = Inventory::new_with_shirt(shirts.clone());
        assert_eq!(inventory.shirts, shirts);
    }

    #[test]
    fn test_add_shirt() {
        let mut inventory = Inventory::new();
        inventory.add(ShirtColor::Red);
        inventory.add(ShirtColor::Blue);
        assert_eq!(inventory.shirts.len(), 2);
        assert_eq!(inventory.shirts[0], ShirtColor::Red);
        assert_eq!(inventory.shirts[1], ShirtColor::Blue);
    }

    #[test]
    fn test_giveaway_with_preferences() {
        let inventory = Inventory::new_with_shirt(vec![ShirtColor::Red, ShirtColor::Blue]);
        let preference = Some(ShirtColor::Red);
        assert_eq!(inventory.giveaway(preference), ShirtColor::Red);
    }

    #[test]
    fn test_giveaway_without_preferences() {
        let inventory =
            Inventory::new_with_shirt(vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]);
        assert_eq!(inventory.giveaway(None), ShirtColor::Red);
    }

    #[test]
    fn test_most_stocked() {
        let inventory =
            Inventory::new_with_shirt(vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]);
        assert_eq!(inventory.most_stocked(), ShirtColor::Red);
    }
}
