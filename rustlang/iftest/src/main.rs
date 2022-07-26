fn main() {
    let number:i32 = 3;
    if number != 0 {
        println!("number bigger");
    }
    let num = 12;
    if num % 4 == 0 {
        println!("number is divisible by 4");
    }else if num % 3 == 0 {
        println!("number is divisible by 3");
    }else if num % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3 ,2");
    }
    match num {
        num if num % 4 == 0 => println!("number is divisible by 4"),
        num if num % 3 == 0 => println!("number is divisible by 3"),
        num if num % 2 == 0 => println!("number is divisible by 2"),
        _   =>println!("number is not divisible by everyone"),
    }
    println!("Hello, world!");
}
