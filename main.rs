fn main() {
    let mut message: String = String::from("hello");
    let message_2: &mut String = &mut message;

    (*message_2).push_str(" world"); // Dereferenced from: &mut String | To type: String | Coerced to: &mut String

    message_2.push_str(" world"); // Type: &mut String | Coerced to: &mut String

    println!("{}", message_2);
}
