fn main() {
    let s = String::from("hello Korea");

    println!("s: {}", first_word(&s));
    let len = s.len();

    let slice = &s[3..len];
    println!("slice: {}", slice);


    let slice = &s[3..];
    println!("slice: {}", slice);

}



fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

