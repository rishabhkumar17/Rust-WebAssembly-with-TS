fn main() {
    let mut message: String = String::from("hello");
    let message_2: String = message.clone(); // deep clone

    message.clear();

    println!("message is {:}", message);
    println!("{}", message_2);
}
