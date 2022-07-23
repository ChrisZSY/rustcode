fn test1() {
    let  mut spaces = "  ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
    println!("test1 Hello, world!");
}
fn test2() {
    let mut spaces = "  ".to_string();
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
    println!("test2 Hello, world!");
}
fn test3() {
    let spaces = "  ".to_string();
    let spaces = spaces.len();
    println!("spaces = {}", spaces);
    println!("test3 Hello, world!");
}
/*
fn test4() {
    let mut spaces = "  ".to_string();
    spaces = spaces.len();
    println!("spaces = {}", spaces);
    println!("test4 Hello, world!");
}*/
fn main(){
    test1();
    test2();
    test3();
    println!("main Hello, world!");
}