use std::{fs::File, io::{ErrorKind, self, Read}};

fn main() {
    // panic!("halt and catch fire");
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("unable to open file: {:?}", error),
    // };

    // let f = File::open("hello.txt").expect("failed to open");
    // quick way to panic on error, chosing the message

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other => {
                panic!("Problem opening file: {:?}", other)
            }
        }
    };
}

fn read_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // propagate the error up
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // will implicitly return either the error or the OK
}

fn read_name_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // would be a more concise way of writing this.

    Ok(s)
}
// the ? means propagate the error up, otherwise continue on. monad???
