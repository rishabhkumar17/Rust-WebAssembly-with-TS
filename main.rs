fn main() {
    let message: String = String::from("Hello");
    let message_2 = extend_message(message);
    println!("{}", message_2);

    let age: u32 = 27;
    let age_2: u32 = extend_age(age);
    println!("{}", age);
    println!("{}", age_2);
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World!");
    a
}

fn extend_age(mut a: u32) -> u32 {
    a += 1;
    a
}
