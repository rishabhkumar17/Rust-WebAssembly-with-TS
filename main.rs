fn main() {
    let float_num: f32 = 3.14;
    let float_num_2: f64 = 3.1234;

    let tup: (i32, f64, i32, &str) = (20, 3.12, 1, "hello");

    println!("{}", tup.1);

    let (a, b, c, d) = tup; //tuples
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    let x: [i32; 4] = [1, 5, 6, 7];

    println!("{}", x[3]);

    let y: [i32; 6] = [2; 6]; // [2,2,2,2,2,2]

    println!("{}", y[6]);

}
