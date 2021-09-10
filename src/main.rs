
mod lib;

fn main() {

    println!("{:?} {:?} {:?} {:?}", lib::CardId::NullCard, lib::CardId::Reload, lib::CardId::Steady, lib::CardId::Fire);

    // Null Card
    println!("{:?}", lib::CARDS[0]);
    (lib::CARDS[0].effect)();           // need to do (variable where function is stored)() with the extra parentheses to call a function stored in a field.

    // Reload
    println!("\n{:?}", lib::CARDS[1]);
    (lib::CARDS[1].effect)();

    // Steady
    println!("\n{:?}", lib::CARDS[2]);
    (lib::CARDS[2].effect)();

    // Fire
    println!("\n{:?}", lib::CARDS[3]);
    (lib::CARDS[3].effect)();
}
