
use lazy_static::lazy_static;

// The lazy_static macro allows you to set constants that require a function call to initialize. In this case, it's String::from(s)
lazy_static! {

    pub static ref CARDS : Vec<Card> = vec![Card { display_name : String::from("Null Card"), id: CardId::NullCard, effect: null_card_effect},
                                            Card { display_name : String::from("Reload"), id: CardId::Reload, effect:reload_effect},
                                            Card { display_name : String::from("Steady..!"), id: CardId::Steady, effect:steady_effect},
                                            Card { display_name : String::from("FIRE!"), id: CardId::Fire, effect:fire_effect}];

}

// CARD ID ENUM
#[derive(Debug)]        // #[derive(Debug)] allows the struct/enum to be printed via println! when using {:?} or {:#?}
pub enum CardId {

    NullCard,
    Reload,
    Steady,
    Fire,

}

// Card Struct
#[derive(Debug)]
pub struct Card {   
    pub display_name: String,
    pub id: CardId,
    pub effect: fn(),
}

// Card Effects

// Null Card Effect
fn null_card_effect(){
    println!("This is the Null Card Effect");
}

// Reload Card Effect
fn reload_effect(){
    println!("Draw 2 cards");
}

// Steady Card Effect
fn steady_effect(){
    println!("increment the cards played this turn counter");
}

// Fire Card Effect
fn fire_effect(){
    println!("For every other card you played this turn, deal 2 damage");
}