fn main() {
    let first = String::from("Ferris"); // L1
    let full = add_suffix(first); // L4
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    // L2
    name.push_str(" Jr."); // L3
    name
}
