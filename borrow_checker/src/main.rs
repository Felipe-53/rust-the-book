fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("Felipe André Soares Barbosa");
    let my_first_word = first_word(&my_string);
    println!("{my_first_word}");
}
