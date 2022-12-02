fn main() {
    println!("Hello, world!");
    vars();
    shadowing();
    let (a, b) = blocks(12);
    println!("blocks returns ({}, {})", a, b);
    let temp = ftoc(425.0);
    println!("c of temp of 425 is {}", temp);
    println!("fibz of 50 is {}", fib(50));
}

fn vars() {
    println!("---vars---");
    let mut x = 5;
    println!("x is {}", x);
    x = with_input(x);
    println!("x is now {}", x);
}

fn shadowing() {
    println!("something");
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

fn with_input(x: i32) -> i32 {
    x + 5
}

fn blocks(x: i32) -> (i32, i32) {
    return (x, y);

    let y = {
        let x = 1;
        x + 1
    };
}

// Exercises for control flow
const F_TO_C_RATIO: f64 = 5.0 / 9.0;

fn ftoc(f: f64) -> f64 {
    (f - 32.0) * F_TO_C_RATIO
}

fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("can't be negative bro");
    } else if n == 0 {
        panic!("cant be 0")
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut current = 1;

    for _i in 1..n {
        sum = last + current;
        last = current;
        current = sum;
    }

    sum
}
