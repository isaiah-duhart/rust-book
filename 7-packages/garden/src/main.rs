use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let veggie = Asparagus{};
    println!("I'm planting {veggie:?}");
}
