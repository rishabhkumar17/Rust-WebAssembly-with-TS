fn main() {
    let mut message: String = String::from("hello");
    let message_2: &mut String = &mut message; // mutable borrow

    unpredictable_mutate(message_2);  
    println!("{}", message);
    // println!("{}", message_2);
}

fn unpredictable_mutate(val: &mut String) {
    val.push_str("_unpredictable");
}
