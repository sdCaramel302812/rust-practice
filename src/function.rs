pub fn rust_function() {
    println!("{}", plus_one(5));
    println!("{}", plus_one_return(5));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_one_return(x: i32) -> i32 {
    return x + 1;
}
