use math;

fn main() {
    let sum = math::sum(1, 2);
    println!("Sum number {}", sum);
    println!("Greetings, {}", welcome());
}

fn welcome() -> String {
    return String::from("Hello, Sambo")
}