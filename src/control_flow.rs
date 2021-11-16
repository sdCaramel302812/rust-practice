pub fn condition() {
    if_else(5);
    ternary_operator(5);
    loop_label();
    return_from_loop();
    for_loop();
}

fn if_else(x: i32) {
    if x > 10 {
        println!("too large");
    } else if x == 10 {
        println!("10");
    } else {
        println!("too small");
    }
    println!();
}

fn ternary_operator(x: i32) {
    println!("{}", if x > 10 { true } else { false });
    println!();
}

fn loop_label() {
    'outer_loop: loop {
        let mut count: u32 = 10;
        loop {
            count -= 1;
            if count <= 0 {
                break 'outer_loop;
            }
        }
    }
    println!("end of loop");
    println!();
}

fn return_from_loop() {
    let mut count = 0;
    let ret = loop {
        if count > 10 {
            break count;
        }
        count += 1;
    };
    println!("ret: {}", ret);
    println!();
}

fn for_loop() {
    let arr: [String; 5] = ["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()];
    for element in arr {
        println!("arr[{}]: {}", element, element);
    }

    println!("range iterator: n in (1..5)");
    for number in (1..5) {
        println!("{}", number);
    }
    println!("reverse range: n in (1..5).rev()");
    for number in (1..5).rev() {
        println!("{}", number);
    }
    println!();
}