fn main() {
    let s = String::from("hello world");

    // if the 1st index of the range is 0, its implicit with the dots
    let _hello = &s[..5];
    let _world = &s[6..];
    // if the 2nd index of the range is the length, its implicit with the dots

    let s = first_word(&s);

    println!("{}", s);
}

//&str means string slice, and when its a parameter, you can pass it a whole
//string, and have it be type coerced into a string slice
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
