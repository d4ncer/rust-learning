fn main() {
    println!("Hello, world!");
    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces len is: {}", spaces);

    let _guess: i32 = "42".parse().expect("Not a number");

    // Ints default to i32
    // _a :: i32
    let _a = 42;

    // Floats default to f64
    // Roughly same speed as f32, but lots more precision
    // _b :: f64
    let _b = 1.234;

    // Math
    let _sum = 5 + 10;

    let _sub = 10 - 5;

    let _mult = 5 * 10;

    let _div = 10 / 5;

    let _mod = 12 % 5;

    // Bool
    let _t = true;
    let _f = false;

    // Char
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuples
    let _tuple_one = (1, 'a', 2.5);

    let (_x, _y, _z) = _tuple_one;

    let _x1 = _tuple_one.0;
    let _x2 = _tuple_one.1;
    let _x3 = _tuple_one.2;
}
