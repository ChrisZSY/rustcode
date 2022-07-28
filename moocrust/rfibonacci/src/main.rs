
fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        n 
    }else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}


fn main() {
    println!("fibonacci = {}", fibonacci(10));
    println!("Hello, world!");
}
