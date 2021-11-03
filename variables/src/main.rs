fn main() {
    basic();
    shadowing();
    numbers();
    arithmetic();
    tuples();
    arrays();
    fns1(50);
    ifs(12);
    loopels();
    liftoff();
    iterz();
    liftoff2();
    println!("87.29123F is {}C", ftoc(87.29123));
    println!("fib 30 is {}", fib(30));
}

fn basic() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("the value of x is {}", x);

    let spaces = "        ";
    let spaces = spaces.len();

    println!("there are {} spaces in spaces", spaces);
}

fn numbers() {
    let i = 12;

    let j = 12.5;

    println!("i is {}, and j is {}", i, j);
}

fn arithmetic() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "{} {} {} {} {}",
        sum, difference, product, quotient, remainder
    );
}

fn tuples() {
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    let x = tup.0;

    println!("{:?}, {}, {}", tup, y, x);
}

fn arrays() {
    let a = [1, 2, 3, 4, 5, 6];

    let result = match a.get(100) {
        Some(&x) => x,
        None => 5,
    };

    println!("{:?}, {}", a, result);
}

fn fns1(x: i32) {
    let x = x + 5;
    let y = {
        let x = 12;
        x + 12
    };
    println!("x is {}, y is {}", x, y);
}

fn ifs(x: i32) {
    let j = if x < 12 { "yo" } else { "what" };

    println!("{}", j);
}

fn loopels() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}, counter is {}", result, counter);
}

fn liftoff() {
    let mut n = 3;

    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }

    println!("wooooooo");
}

fn iterz() {
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("{}", element);
    }
}

fn liftoff2() {
    for i in (1..4).rev() {
        println!("{}!", i);
    }

    println!("wooooooo");
}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn fib(n: i32) -> usize {
    if n < 0 {
        panic!("{} is negative", n);
    } else if n == 0 {
        panic!("{} is 0", n);
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}
