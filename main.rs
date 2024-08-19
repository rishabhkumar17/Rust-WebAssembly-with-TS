fn main() {
    let message: String = String::from("hello");
    let slice: &str = &message[2..4]; //slice (2 -> 3) 2 is inclusive and 4 is exclusive

    println!("{}", slice);
}
