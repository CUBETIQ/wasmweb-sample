fn main() {
    let a = 10;
    let b = 5;
    let sum = sum(a, b);

    println!("Sum of {} + {} = {}", a, b, sum);
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}