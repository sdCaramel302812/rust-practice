struct StructIncludeRef<'a> {
    val: &'a str
}

// lifetime annotation should be added
impl<'a> StructIncludeRef<'a> {
    fn level(&self) -> i32 {
        3
    }
}

pub fn lifetime() {
    valid_lifetime();
    invalid_lifetime();
    println!();

    struct_lifetime();

    // every string literal have static lifetime, means they live for entire program
    let s: &'static str = "I have a static lifetime.";
    println!();
}

fn valid_lifetime() {
    let string1 = String::from("abcd");
    
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

fn invalid_lifetime() {
    let string1 = String::from("abcd");
    let result;

    //{         // with this scope, string2 will be invalid at line of println!
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    //}
    println!("The longest string is {}", result);
}

fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago");
    let part = novel.split('.').next().expect("could not find .");
    let the_struct = StructIncludeRef {
        val: part
    };
    println!("{}", the_struct.val);
}

// error because it does not know borrow from x or y
//fn longest(x: &str, y: &str) -> &str {
//    if x.len() > y.len() {
//        x
//    } else {
//        y
//    }
//}

//  return reference lifetime equal to the smaller one of parameter's lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
