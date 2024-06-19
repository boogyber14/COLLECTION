

use no_panic_crate::safe_divide;

fn main() {
    match safe_divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
