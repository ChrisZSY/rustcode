fn main() {
    //let mut i = 0;

    for i in 0..5 {
        println!("i = {}", i);
    }
    println!("--------------");
    for i in 0..=5 {
        println!("i = {}", i);
    }
    println!("--------------");
    let myarr = ["a", "b", "c"];

    for i in myarr.iter() {
        println!("i = {}", i);
    }

    let mut myarr = [1, 2, 3];
    for i in myarr.iter_mut() {
        *i = *i *2;
    }
    for i in myarr.iter() {
        println!("i = {}" , i);
    }
    println!("--------------");
    println!("Hello, world!");
}
