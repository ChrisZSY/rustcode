fn main() {
    let x = true;
    let y:bool = !x;//取反运算

    println!("x = {} y = {}", x, y);

    let z = x && y; 
    println!("x && y = {}", z);

    let z = x || y;
    println!("x || y = {}", z);

    let z = x & y;
    println!("x & y = {}", z);

    let z = x | y;
    println!("x | y = {}", z);

    let z = x ^ y;
    println!("x ^ y = {}", z);

    let z = x < y;
    println!("z = x < y ={}", z);
    println!("Hello, world!");
    
    let x = 3;
    let y = !x;
    println!("!{} = {}",x, y);
    let y = 6;
    println!("x = {} y = {}", x, y);

    //let z = x && y; 
    println!("x && y = {} must be bool op", z);

    //let z = x || y;
    println!("x || y = {} must be bool op", z);

    let z = x & y;
    println!("x & y = {}", z);

    let z = x | y;
    println!("x | y = {}", z);

    let z = x ^ y;
    println!("x ^ y = {}", z);

    let z = x < y;
    println!("z = x < y ={}", z);
}
