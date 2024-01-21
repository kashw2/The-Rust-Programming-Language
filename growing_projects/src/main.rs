use garden::fruit::Tomato;
use garden::vegetable::Cucumber;

pub mod garden;

fn main() {
    let cucumber1: Cucumber = Cucumber {
        water_content: 1.572
    };

    let tomato1: Tomato = Tomato {
        water_content: 7.913
    };

    dbg!(cucumber1);
    dbg!(tomato1);
}
