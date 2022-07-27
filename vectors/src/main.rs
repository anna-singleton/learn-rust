fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    v.push(5);
    v.push(6);

    match v.get(2) {
        Some(x) => println!("Third Element: {}", x),
        None => println!("There is no third element"),
    }
}
