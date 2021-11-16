struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u32
}

struct Color(i8, i8, i8);

struct AlwaysEqual;

#[derive(Debug)]    //  allow {:?} or {:#?} to print information
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn width(&self) -> u32 {       // could have same name as struct variable
        return self.width;
    }

    fn is_contain(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size
        };
    }
}

pub fn rust_struct() {
    instantiate();
    tuple_struct();
    unit_like_struct();
    debug_message();
    method();
}

fn instantiate() {
    let user1 = User {
        email: String::from("someone@example.com"),
        name: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };  //  immutable
    let mut user2 = User {
        email: String::from("someone@example.com"),
        name: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    user2.email = String::from("otheruser@example.com");
    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    println!("{}, {}, {}", user1.email, user2.email, user3.email);

    let user4 = User {
        email: String::from("user4.example.com"),
        ..user1
    };              
    // println!("{}", user1.name);  // user1's name is no longer valid because name was been moved to user4
    println!("{}", user1.email);    // but user1's email is still valid
    println!();
}

fn build_user(email: String, name: String) -> User {
    User {
        email,      // shorthand
        name,
        active: true,
        sign_in_count: 1
    }
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!();
}

fn unit_like_struct() {
    let subject = AlwaysEqual;  //  detail in chapter 10
}

fn debug_message() {
    let rect = Rectangle {
        width: 10,
        height: 15
    };

    println!("{:#?}", rect);
    dbg!(&rect);
    println!();
}

fn method() {
    let rect = Rectangle {
        width: 10,
        height: 15
    };
    let rect2 = Rectangle {
        width: 5,
        height: 2
    };
    let rect3 = Rectangle::square(20);

    println!("area: {}", rect.area());
    println!("width: {}", rect.width());
    println!("rect contains rect2: {}", rect.is_contain(&rect2));
    println!("rect contains rect3: {}", rect.is_contain(&rect3));
}