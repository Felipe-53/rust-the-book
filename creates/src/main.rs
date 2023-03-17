mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist")
        }
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();

    let s = String::from("hello");
    for c in s.chars() {
        println!("{}", c);
    }
}
