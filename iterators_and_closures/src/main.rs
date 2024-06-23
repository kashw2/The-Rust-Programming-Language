use std::fmt::Debug;

use lib::{Item, Type};

use crate::lib::{Color, Inventory};

pub mod lib;

fn main() {
    let inventory: Inventory = Inventory {
        manager: "Keanu Ashwell",
        items: vec![
            Item { number_in_stock: 15, name: "ACDC", r#type: Type::Shirt },
            Item { number_in_stock: 10, name: "Pinkfloyd", r#type: Type::Shirt },
        ],
    };

    // No customer preference
    let result1 = inventory.get_most_stocked_item()
        .map(|item| item.giveaway(None));

    // Customer preference is for the Red color
    let result2 = inventory.get_most_stocked_item()
        .map(|item| item.giveaway(Some(Color::Red)));

    dbg!(result1, result2);
}
