fn main() {
    let message: String = String::from("Hello");
    let message_2 = extend_message(message);
    println!("{}", message_2);
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World!");
    a
}
