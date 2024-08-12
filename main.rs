fn main() {
    let x: u32 = 10;
    let y: u32 = x; // stack

    let sum = sum(x, y);

    println!("{}", sum);

    let message: &str = "Hello";

    let message_2: String = String::from("Hello_2"); // heap

    let message_3: String = message_2; // copy pointer

    println!("{}", message_3);
}

fn sum(x: u32, y: u32) -> u32 {
    let sum: u32 = x + y;
    sum
}

// xxd -g 1 -c 8 main  | display binary files
