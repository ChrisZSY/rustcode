fn another_function(x:i32, y:i32)->i32{
    println!("x = {}, y = {}", x, y);
    x+y
}
fn cp_str(str:String)->String{
    str
}
fn cp_ref(str:&String)->&String{
    str
}
fn new_str()->String{
    String::from("new str")
}
fn main() {
    let a:i32 = 3;
    let b:i32 = 5;

    let sum = another_function(a, b);
    println!("a = {}, b = {}", a, b);
    println!("sum = {}", sum);

    let strhello = String::from("new hello");
    let rethello = cp_str(strhello);

    //println!("strhello = {}", strhello); borrow control error
    println!("cpstr & rethello = {}", rethello);

    let strhello = String::from("new hello");
    let rethello = cp_ref(&strhello);

    println!("cpref & strhello = {}", strhello);
    println!("cpref & rethello = {}", rethello);
    
    let newstr = new_str();
    println!("newstr = {}", newstr);
    println!("Hello, world!");
}
