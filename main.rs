fn main() {
    let message: String = String::from("Hello"); // message coming into the scope
    print_message(message); // message is moved into this function
    // message is not longer valid
}
// message is going out of scope, but nothing will happen because it was moved

fn print_message(a: String) { // a comes into the scope
    println!("{}", a); //prints
    let c: String = a; // c is coming into the scope and a is moved into c;
    println!("{}", c); //prints
    // a is no longer valid;
}

// a is going out of scope, but nothing will happen because it was moved

// c is going out of scope and "drop" is called which clears the memory from the heap