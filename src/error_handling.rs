use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn error_handling() {
    usage_of_result();
    let s1 = propagate_errors();
    match s1 {
        Ok(s) => println!("{}", s),
        Err(_) => println!("not Ok")
    };
    let s1 = propagate_errors_short_hand();
    match s1 {
        Ok(s) => println!("{}", s),
        Err(_) => println!("not Ok")
    };
}

fn usage_of_result() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // if result is ok, unwrap return value in ok. else, it call panic!
    let f = File::open("hello.txt").unwrap();
    // expect can customize error messeage
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagate_errors() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagate_errors_short_hand() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;      // ? operator return if the result is Error
    Ok(s)
}