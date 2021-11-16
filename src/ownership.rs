pub fn ownership() {
    copy_simple_type();
    copy_non_fixed_type();
    clone_non_fiexed_type();

    let s1 = String::from("take ownership");
    take_ownership(s1);  //  s is no longer valid
    let s2 = give_ownership();
    println!("{}\n", s2);

    let s3 = String::from("give immutable reference");
    immutable_reference(&s3);
    println!("still valid: {}\n", s3);

    let mut s4 = String::from("give mutable reference");
    mutable_reference(&mut s4);
    println!("s4: {}\n", s4);

    read_write_lock();

    let s5 = String::from("safe");
    println!("{}\n", dangle(&s5));

    slice();
    array_slice();
}

fn copy_simple_type() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    println!();
}

fn copy_non_fixed_type() {
    let x = String::from("hello");
    let y = x;      //  x is no longer valid
    println!("y: {}", y);
    println!();
}

fn clone_non_fiexed_type() {
    let x = String::from("hello");
    let y = x.clone();
    println!("x: {}, y: {}", x, y);
    println!();
}

fn take_ownership(s: String) {
    println!("{}", s);
    println!();
}

fn give_ownership() -> String {
    let s = String::from("give ownership");
    return s;
}

fn immutable_reference(s: &String) {
    println!("{}", s);
}

fn mutable_reference(s: &mut String) {
    s.push_str(", hello");
}

fn read_write_lock() {
    let mut s = String::from("lock");
    {
        let s1 = &s;
        let s2 = &s;
        // let s3 = &mut s;     //  invalid, mutable reference can coexist with other reference
        println!("s1: {}, s2: {}", s1, s2);
        {
            let s3 = &mut s;
        }
        let s4 = &mut s;        // valid because s3 is out of scope
        println!("s4: {}", s4);
        //println!("{}, {}", s1, s2);   // still invalid because there is a mutable reference after declaration
    }
    s.push_str(", hi");     // valid because immutable reference is out of scope
    println!("{}", s);
    println!();
}

fn dangle(safe_str: &String) -> &String {
    let unsafe_str = String::from("unsafe");
    // return &unsafe_str;      // error
    return &safe_str;
}

fn slice() {
    let mut s = String::from("hello world");
    println!("{}", s);
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("&s[0..5]: {}, &s[6..11]: {}", hello, world);
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
    println!("{}, {}, {}", hello, world, hello_world);
    //let test = &mut s[0..5];      // slice cannot be mutable

    s.clear();
    // println!("{}, {}, {}", hello, world, hello_world);   // error, exist immutable borrow
    println!();
}

fn array_slice() {
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..4];
    for elem in arr_slice {
        println!("{}", elem);
    }
    println!();
}