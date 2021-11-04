fn main() {
    println!("Hello, world!");
    vars();
    shadowing();
}

fn vars() {
    println!("---vars---");
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is now {}", x);
}

fn shadowing() {
    println!("---shadowing---");
    let x = 5;
    let x = x + 1;
    println!("x in outer scope is {}", x);

    {
        let x = 12;
        println!("x in inner scope is {}", x);
    }

    println!("x in outer scope is still {}", x);
}
