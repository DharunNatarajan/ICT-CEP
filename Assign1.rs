fn main() {
    println!("Multiplication Table for 9:");
    println!("-------------------------");

    for i in 1..=10 {
        let result = 9 * i;
        println!("9 * {} = {}", i, result);
    }
}
