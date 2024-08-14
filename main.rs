fn main() {
    let a: i32 = 27;
    let b: &i32 = &a;
    let mut c: &&i32 = &b;
    let d: &i32 = b;

    let e: &&i32 = &&17;
    c = e;

    // println!("{:p}", &a);
    // println!("{:p}", b);
    println!("{:p}", c);
    println!("{:p}", *c);
    // println!("{:p}", d);

    println!("{:p}", e);
    println!("{:p}", &(**e));
}
