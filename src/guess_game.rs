use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess() {
    let random_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        let mut input: String = String::new();

        println!("guess the number!");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line\n");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guess: {}", input);

        match input.cmp(&random_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => { 
                println!("correct");
                break;
            },
        }
    } 
}