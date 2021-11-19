struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {                               // only f32 type hase this function
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct ComplexPoint<T, U> {
    x: T,
    y: U
}

impl<T, U> ComplexPoint<T, U> {
    fn mixup<V, W>(self, other: ComplexPoint<V, W>) -> ComplexPoint<T, W> {
        ComplexPoint {
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String;   // without defalut implementation
    fn summarize(&self) -> String {     // default implementation
        format!("read more, {}", self.summarize_author())
    }                                   // use function from same trait
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}         // apply default implementation

struct Tweet {
    username: String,
    content: String,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest: {}\n", result);

    generics_struct();

    struct_trait();
    println!();
}

// for any type who has partialord and copy trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

// another way to specify traits
//fn largest<T>(list: &[T]) -> T
//    where T: PartialOrd + Copy {
//    ...
//}

fn generics_struct() {
    let point = Point {
        x: 5.0,
        y: 10.0
    };
    println!("p: {{{}, {}}}", point.x(), point.y());       // escape { by {{, } by }}
    println!("distance: {}", point.dist_from_origin());

    let point1 = ComplexPoint {
        x: 10,
        y: 15.0
    };
    let point2 = ComplexPoint {
        x: "hello",
        y: 'a'
    };
    let point3 = point1.mixup(point2);
    println!("{}, {}", point3.x, point3.y);
    //println!("{}", point2.x);             both p1 and p2 has moved to p3
    println!();
}

fn struct_trait() {
    let tweet = Tweet {
        username: String::from("ABC"),
        content: String::from("first tweet"),
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    let news = NewsArticle {
        headline: String::from("News"),
        author: String::from("ABC"),
        content: String::from("new news")
    };
    println!("news: {}", news.summarize());
}

// any type has implement Summary is an acceptable return
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        retweet: false,
    }
}

// any type has implement Summary is an acceptable parameter
fn notify(item1: &impl Summary) {

}