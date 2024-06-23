#[derive(Debug, Clone)]
pub struct Inventory<'a> {
    pub manager: &'a str,
    pub items: Vec<Item<'a>>,
}

impl<'a> Inventory<'a> {
    pub fn get_most_stocked_item(&self) -> Option<Item<'a>> {
        self.items
            .iter()
            .cloned()
            .reduce(|a, b| if a.has_greater_stock(&b) { a } else { b })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item<'a> {
    pub name: &'a str,
    pub r#type: Type,
    pub number_in_stock: i16,
}

impl<'a> Item<'a> {
    pub fn giveaway(&self, preference: Option<Color>) -> Color {
        preference.unwrap_or_else(|| Color::Blue)
    }

    pub fn has_greater_stock(&self, other: &Item) -> bool {
        self.number_in_stock > other.number_in_stock
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Shirt,
    Shorts,
    Pants,
    Hat,
    Socks,
    Underwear,
    Shoes,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_greater_stock_is_correct() {
        let items = vec![
            Item { number_in_stock: 1, r#type: Type::Shirt, name: "One" },
            Item { number_in_stock: 2, r#type: Type::Shirt, name: "One" },
        ];
        let result: Option<Item> = items.iter().reduce(|a, b| if a.has_greater_stock(&b) { a } else { b })
            .cloned();

        assert!(result.eq(&items.get(1).cloned()))
    }

    #[test]
    fn giveaway_should_default() {
        let item = Item { number_in_stock: 1, r#type: Type::Shirt, name: "One" };
        let result = item.giveaway(None);

        assert_eq!(result, Color::Blue)
    }

    #[test]
    fn giveaway_should_not_default() {
        let item = Item { number_in_stock: 1, r#type: Type::Shirt, name: "One" };
        let result = item.giveaway(Some(Color::Green));

        assert_eq!(result, Color::Green)
    }
}