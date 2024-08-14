fn main() {
   let message: String = String::from("hello");
   let message_2: &String = &message; // pointer
    // message_2 is not owner of data
    // message_2 is "borrowing" a reference to message

   println!("{}", message);
   println!("{}", message_2); 
}

