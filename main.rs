fn main() {
    let mut message: String = String::from("hello");
    let message_2: &mut String = &mut message; // mutable borrow

    message_2.push_str(" world");

    println!("{}", message);
    // println!("{}", message_2);
}
