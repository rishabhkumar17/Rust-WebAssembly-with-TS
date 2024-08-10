fn main() {
    let x: u32 = 10;
    let y: u32 = x;

    let sum = sum(x, y);

    println!("{}", sum);
}

fn sum(x: u32, y: u32) -> u32 {
    let sum: u32 = x + y;
    sum
}
