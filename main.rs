fn main() {
    let a: i32 = 27;
    let b: &i32 = &a;
    let c: &&i32 = &b;

    println!("{}", a == *b); // dereferencing b using *
    println!("{}", a == **c);
}
