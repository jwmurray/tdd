use factorial::iterative_factorial;
use factorial::recursive_factorial;

fn main() {
    println!("Iterative Factorial(9) = {}", iterative_factorial(9));
    println!(
        "Recursive Factorial(9) = {}",
        recursive_factorial(9)
    );
}
