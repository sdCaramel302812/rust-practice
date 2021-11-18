use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn collect() {
    vector();
    string();
    hashmap();
}

fn vector() {
    let imut_v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    println!("{:?}", v);
    let third: &i32 = &v[2];    //  immutable reference
    println!("third: {}", third);
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i *= 10;
    }
    println!("{}, {}, {}, {}, {}", v[0], v[1], v[2], v[3], v[4]);
    // println!("third: {}", third);    error because mutable borrow in line 16

    // two ways to get reference
    //let does_not_exist = &v[100];                 // type is i32, because of out of range, following code won't execute
    //println!("does not exist {}", does_not_exist);
    let does_not_exist = v.get(100);                // type is Option<&i32>
    match does_not_exist {
        Some(number) => println!("number: {}", number),
        None => println!("None")
    };

    // store different type in vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!();
}

fn string() {
    let s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    s.push_str(" ");
    s += &hello;
    s = s + " ";
    s.push_str(&hello);
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}, {}, {}, {}", s, s1, s2, s3);

    println!("cannot index string because string is utf8 encoded, each character may have different bytes");
    // let c = s1[0];
    
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
    // let s = &hello[0..1];        panic becuase each character is 2 bytes

    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    for b in "नमस्ते".bytes() {
        print!("{} ", b);
    }
    println!("\n");
}

fn hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let name = String::from("Yellow");
    scores.insert(name, 50);                // name has been moved to scores
    println!("{:?}", scores);
    // get hashmap value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let score = scores.get(&String::from("Blue"));
    if let Some(n) = score {
        println!("blue: {}", n);
    }
    // update hashmap
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    // udpate hashmap if key not exist
    scores.entry(String::from("Blue")).or_insert(50);
    let count = scores.entry(String::from("Blue")).or_insert(50);   // retrun mutable reference
    *count += 1;
    println!("{:?}", scores);

    // initiate from two vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}