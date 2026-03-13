use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let mut v:Vec<i32> = vec![1,2,3,4,5];
    v.push(6);
    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(20);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Error due to the fact you cant have a mutable and immutable reference in the same scope
    // let second: &i32 = &v[1];
    // v.push(6);
    // println!("{:?}", second);

    for i in &v {
        println!("vec val: {i}",)
    };

    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // println!("{row}")

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("s1, s2, s3 is {}{}{}", s1, s2, s3); # s1 has been borrowed due too '+' which uses add
    println!("s2, s3 is {} {}", s2, s3); // s2 is fine because a reference was used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // better for concat multiple strings
    let s = format!("{s1}-{s2}-{s3}");
    println!("concat s is {s}");
    println!("s1, s2, s3 still owned {s1}-{s2}-{s3}");

    // rust doesn't support string indexing like other languages because under the hood
    // a string is implemented via Vec<u8>. Rust strings can be represented in 3 ways:
    // Bytes, Scalar Values, and Grapheme Clusters (closest thing to letters)

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);

    println!("{scores2:?}");

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


}
