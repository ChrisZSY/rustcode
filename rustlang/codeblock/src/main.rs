fn main() {
    let x:i32  = 5;
    let y:i32 = {
        let x:i32 = 3;
        x + 1
        //x+1;means return ()
    };
    println!("y = {}, ", y);
    println!("y = {},x = {}" , y, x);
    println!("Hello, world!");
}
