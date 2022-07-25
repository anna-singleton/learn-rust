fn main() {
    f(5);
    let x = g(5);
    println!("value of g: {x}");
}

fn f(x : i32){
    println!("The value passed to f is {x}");
}

fn g(x : i32) -> i32{
    x * 5
}
