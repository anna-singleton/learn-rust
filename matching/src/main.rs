enum Coin {
    Ten,
    Twenty,
    Fifty,
    Pound,
    TwoPound,
}

fn val_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Ten => 10,
        Coin::Twenty => 20,
        Coin::Fifty => 50,
        Coin::Pound => 100,
        Coin::TwoPound => 200,
    }
}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("its 3"),
        7 => println!("its 7"),
        _ => println!("its something else")
    };
}
