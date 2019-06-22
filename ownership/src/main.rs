fn main() {
    let j = "abc";
    let mut s = String::from(j);
    println!("s is {}", s);

    s.push_str(j);

    println!("s is {}", s);

    let x = 5;
    let mut y = x;

    println!("{}, {}", x, y);

    y = 12;

    println!("{}, {}", x, y);

    let i_own = String::from("cupcake am a string");

    println!("i own before {}", i_own);

    let i_own = takes_ownership(i_own);

    println!("i own after {}", i_own);

    println!("first word in i_own is at {}", first_word(&i_own));
}

fn takes_ownership(s: String) -> String {
    println!("s is {}", s);
    return s;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
