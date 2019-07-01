use std::fmt;

pub mod pair;

// Generics in structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generics in enums
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

// TODO: refactor this to be generic
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    &largest
}

// Trait with default behaviour
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Using a trait as a parameter
fn notify(item: impl Summary) {
    println!("breaking news! {}", item.summarize());
}

// Trait bound syntax (+ for multiple traits)
fn notify2<T: Summary + fmt::Display>(item: T) {
    println!("breaking news! {}", item.summarize());
}

// Using where for cleaner trait bounds
fn some_function<T, U>(_t: T, _u: U) -> i32
where
    T: fmt::Display + Clone,
    U: Clone + fmt::Debug,
{
    0
}

fn summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("some things inside"),
        reply: false,
        retweet: false,
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let y = pair::Pair { x: 2, y: 1 };
    y.cmp_display();

    let z = pair::Pair {
        x: Point { x: 1, y: 1 },
        y: Point { x: 2, y: 2 },
    };
    // z.cmp_display(); This doesn't work because Point doesn't satisfy trait bounds
    let v = vec![1, 2, 3];
    let l = largest(&v);
    println!("largest number in v is {}", l);

    let v = vec![String::from("abc"), String::from("def")];
    println!("largest string in v is {}", largest(&v));

    let p = Point { x: 1, y: 2 };
    println!("point is {:?}", p);

    let j = Point { x: 1.0, y: 5.0 };
    println!("point is {:?}", j);

    let jx = j.x();
    println!("j's x is {}", jx);

    let o = Option::Some(12);
    println!("option is {:?}", o);

    // Have to specify type here
    let n: Option<i32> = Option::None;
    println!("option is {:?}", n);

    let art = NewsArticle {
        headline: String::from("something"),
        location: String::from("something else"),
        author: String::from("author"),
        content: String::from("here is some content"),
    };

    println!("{}", art.summarize());

    let tweet = Tweet {
        username: String::from("raghuvir"),
        content: String::from("here is a tweet"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    // Lifetimes

    // This doesn't work because r is trying to reference something that has been
    // dropped out of scope by the time we care about it
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r is {}", r);

    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("result is {}", result); // this works because result uses the shortest scope
    }

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    // println!("result is here? {}", result); // this doesn't work because string2 doesn't live long enough here

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");

    let _i = pair::ImportantExcerpt {
        part: first_sentence,
    };
}
