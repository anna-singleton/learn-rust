fn main() {
    let mut s = String::from("hello");
    // the double colon namespaces `from` from the string type

    s.push_str(", world");

    println!("{}", s);

    // this line moves s to s2, s is no longer valid and would produce a
    // compile time error if used.
    let _s2 = s;

    // if we want to deep copy heap variables, we use `clone`
    let _s3 = _s2.clone();

    let x = 5;
    let _y = x;
    // since x is a stack variable, it is inexpensive to copy at runtime
    // and is just copied without a call to clone

    // when passing a value to a function, the function takes ownership of
    // it, and it will go out of scope at the end of the function, unless
    // it is returned and rebound, which is tedious to do every time, so
    // we can use references.

    // references & borrowing

    let mut s4 = String :: from("hello");
    let len = strlen(&s4);

    println!("The length of '{}' is {}.", s4, len);

    // &s4 creates a reference that refers to the value of s4 but doesnt own it
    // this is called borrowing, the value is not mutable when borrowed, and is
    // given back to its owner when out of scope.
    
    // however, sometimes we want to borrow a value and change it, so we can
    // create a mutable reference to it.
    //
    // if a mutable reference to a value exists, no other references can exist
    // to that value.

    change(&mut s4);
    println!("new value of s4 -> '{}'", s4);

}

fn strlen(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
