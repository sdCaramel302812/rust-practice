pub mod lib_tutorial {
    pub fn tutorial() {
        println!("hello world");

        let breakfast = Breakfast {
            toast: String::from("wheat"),
            seasonal_fruit: String::from("mongo")
        };

        println!("{}, {}", breakfast.toast, breakfast.seasonal_fruit);

        super::eat();       //  relative path
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn create() -> Breakfast {
            Breakfast {
                toast: String::from("wheat"),
                seasonal_fruit: String::from("mongo")
            }
        }
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;    //  use alias to import two same name items

//use std::cmp::Ordering;
//use std::io;
use std::{cmp::Ordering, io};       // combine the path

//use std::io;
//use std::io::Write;
//use std::io::{self, Write};

use std::collections::*;            // bring all into the scope

fn eat() {
    //let breakfast = lib_tutorial::Breakfast {
    //    toast: String::from("wheat"),
    //    seasonal_fruit: String::from("mongo")     //  error because seasonal_fruit is private
    //};
    let breakfast = lib_tutorial::Breakfast::create();
    println!("{}", breakfast.toast);
    // println!("{}", breakfast.seasonal_fruit);    //  error because seasonal_fruit is private
}

// idiomatic way to use crate
//use lib_tutorial;             lib_tutorial::tutorial();
//use lib_tutorial::Breakfast;  let breakfast = Breakfast::create();