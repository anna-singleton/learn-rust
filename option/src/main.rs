fn main() {
    let x = Some(5);
    let y = Some("a String");
    let z : Option<i32> = None;

    match x {
        Some(a) => println!("its {}", a),
        None => println!("its None"),
    };

    // more concise way to ignore a None value

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("configured max: {}", max);
        // this code only runs if config_max is not None
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
