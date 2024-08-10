fn main() {
    let message: &str = "Hello World";
    let message_2: &str = print_welcome(message);
    println!("{}", message_2);

    let custom_num: i32 = 90_000;
    let hex_num: i32 = 0xfa; //0x for hex decimal (250)
    let bin_num: i32 = 0b0010_1011; //0b for binary (43)
    let byte_num: u8 = b'A'; //b for byte (utf-8) ( hexadecimal 0x41  to decimal 65)
    let bool_num: bool = true;
    let eight_bit: u8 = 255;

    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
    println!("{}", bool_num);
    println!("{}", eight_bit);
}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    "Hi There"
}
