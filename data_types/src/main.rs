fn main() {
    let _guess:u32 = "42".parse().expect("Not a number!");
    // the compiler can infer types in many places, but in places like this
    // we must explicitly tell the compiler we want to parse to a u32

    let _x : f32 = 2.0;

    let tup : (i32, f64, u8) = (500, 6.4, 1);

    let (a, _, _) = tup;

    let b = tup.1;

    println!("The value of a is: {a}");
    println!("The value of b is {b}");

    let c = [5, 3, 6, 2, 1];

    // this makes an array like [3, 3, 3, 3, 3]
    let d = [3;5]; 
}
