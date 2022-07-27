struct User {
    active: bool,
    uname: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
#[derive(Debug)] // allows the struct to be printed with debug
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User{
        email: String::from("someone@example.com"),
        uname: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("email2@example.com");
    // either the whole struct is mutable or none of it is, you cannot have
    // partial mutability in structs.

    // assigning to fields can get tedious, so there is an init shorthand.
    

    // updating / creating new similar structs can also be tedious, so we have
    // the update syntax.

    let user2 = User {
        email: String::from("example2@email.com"),
        ..user1
    };

    // using tuple structs

    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);
}

fn build_user(email: String, uname: String) -> User {
    User {
        email, // this works because the variable and field have the same name
        uname,
        active: true,
        sign_in_count: 1
    }
}
