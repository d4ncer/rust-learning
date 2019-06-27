use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
// use std::io::Read;

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // Long version
    // let f = File::open(path);

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // Shorter version
    // let mut s = String::new();
    // File::open(path)?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string(path)
}

// Custom validation
struct Guess {
    value: Option<i32>,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            return Guess { value: None };
        }

        Guess { value: Some(value) }
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }
}

fn main() {
    let _f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried creating file but failed {:?}", e),
            },
            other_error => panic!("problem opening the file {:?}", other_error),
        },
    };

    let g = Guess::new(150);

    println!("guess value of g is {:?}", g.value());

    let username = read_username_from_file("hello.txt");

    match username {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("ooooooh {}", e),
    };
}
