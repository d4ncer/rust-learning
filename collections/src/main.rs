use std::collections::HashMap;

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];

    println!("third is {}", third);

    match v.get(2) {
        Some(third) => println!("third is now {}", third),
        None => println!("none!"),
    }

    let mut v1 = vec![1, 2, 3];

    let _first = &v1[0];

    v1.push(12);

    // This won't work due to mutable/immutable borrows
    // println!("first is {}", first);

    let mut v2 = vec![100, 32, 57];

    for i in &mut v2 {
        *i += 50;
    }

    println!("{:?}", v2);

    let v3 = vec![
        Cell::Float(25.0),
        Cell::Int(1),
        Cell::Text(String::from("abc")),
    ];

    println!("{:?}", v3);

    let s1 = String::from("abc");
    let _f = s1.chars().nth(0);

    let s2 = String::from("def");

    let _s3 = format!("{} {}", s1, s2);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 5);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 5];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores2 {
        println!("{} {}", key, value);
    }

    let f_name = String::from("favourite color");
    let f_val = 12;

    let mut map = HashMap::new();
    map.insert(f_name, f_val);

    println!("{}", f_val);

    let c = map.get(&String::from("favourite color"));

    println!("{:?}", c);

    let mut scores3 = HashMap::new();

    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Blue")).or_insert(50);
    scores3.entry(String::from("Yellow")).or_insert(12);

    println!("{:?}", scores3);

    let text = "hello world wonderful world hello wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let counter = map.entry(word).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", map);
}
