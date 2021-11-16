pub fn learn_data_type() {
    scalar_types();
    compound_types();
}

fn scalar_types() {
    println!("Scalar types:");
    println!("\tinteger");
    println!("\tfloat point number");
    println!("\tboolean");
    println!("\tcharacter\n");
    integer();
    float();
    numeric_operations();
    boolean();
    character();
}

fn compound_types() {
    tuples();
    array();
}

fn integer() {
    println!("integer:");
    println!("\ti(u)(8 ~ 128)");
    println!("\ti(u)size: based on the computer archtecture (e.g. 32 bits or 64 bits)");
    let decimal: i32 = 123_456;
    let hex: i32 = 0x123;
    let octal:i32 = 0o123;
    let binary:i32 = 0b1111_0000;
    let byte:u8 = b'A';
    println!("123_456       : {}", decimal);
    println!("0x123         : {}", hex);
    println!("0o123         : {}", octal);
    println!("0b1111_0000   : {}", binary);
    println!("b'A' (u8 only): {}", byte);
    println!();
}

fn float() {
    println!("float:");
    let double = 1.0; // default f64
    let float: f32 = 2.0;
    println!("f64:\n\tlet x = 1.0    : {}", double);
    println!("f32:\n\tlet x:f32 = 2.0: {}", float);
    println!();
}

fn numeric_operations() {
    let int1:i32 = 10;
    let int2:i32 = 7;
    let small_int:i8 = 2;
    let float1:f32 = 10.1;  // must have .0 to present as float
    let float2:f32 = 7.1;
    println!("numeric operations:");
    println!("different type cannot do numeric opeations\ne.g.\n");
    println!("\ti32 + i8");
    println!("\ti32 + f32");
    println!("integer operations:");
    println!("\t10 + 7 = {}", int1 + int2);
    println!("\t10 - 7 = {}", int1 - int2);
    println!("\t10 * 7 = {}", int1 * int2);
    println!("\t10 / 7 = {}", int1 / int2);
    println!("float operations:");
    println!("\t10.1 + 7.1 = {}", float1 + float2);
    println!("\t10.1 - 7.1 = {}", float1 - float2);
    println!("\t10.1 * 7.1 = {}", float1 * float2);
    println!("\t10.1 / 7.1 = {}", float1 / float2);
    println!();
}

fn boolean() {
    let b: bool = true;
    println!("boolean:");
    println!("\tb: {}", b);
    println!();
}

fn character() {
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    let chinese_char: char = '五';
    println!("character:");
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart eyed cat: {}", heart_eyed_cat);
    println!("chinese char: {}",  chinese_char);
    println!();
}

fn tuples() {
    let tup: (i32, f32, i8) = (100, 50.0, 5);
    let (x, y, _) = tup;
    let empty_tup = ();
    println!("tuples:");
    println!("\tlet tup: (i32, f32, i8) = {:?}", tup);
    println!("\tlet (x, y, _) = tup");
    println!("\ttup.0: {}", tup.0);
    println!("unit type:");
    println!("\t empty tuple (), cannot display");
    println!();
}

fn array() {
    let a: [i32; 5] = [1; 5];
    println!("array:");
    println!("\tlet a: [i32; 5]] = [1; 5]: {:?}", a);
    println!();
}