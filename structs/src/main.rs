struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let user1 = build_user(
        String::from("raghu@movio.co"),
        String::from("raghuvir kasturi"),
    );

    let user2 = User {
        username: String::from("soumya parker"),
        email: String::from("soumyaparker@gmail.com"),
        ..user1
    };

    let point = Point(0, 0, 0);
    let color = Color(0, 0, 0);

    let r = Rectangle {
        height: 20,
        width: 20,
    };

    let r2 = Rectangle {
        height: 10,
        width: 15,
    };

    let s = Rectangle::square(12);

    println!("rect is {:?}", r);
    println!("area is {}", r.area());
    println!("can r hold r2? {}", r.can_hold(&r2));
    println!("can r hold square? {}", r.can_hold(&s));
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
