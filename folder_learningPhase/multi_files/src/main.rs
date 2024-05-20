
mod fibo; // Import the fibo module

fn main() {
    let result = fibo::fibonacci(10); // Call the fibonacci function from the fibo module
    println!("Fibonacci result: {}", result);
}