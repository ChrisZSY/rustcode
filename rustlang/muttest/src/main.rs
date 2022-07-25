fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v = v;
    for i in &v{
        println!("i = {}", i);
    }

    println!("---------------");
    let v = Vec::new();

    let mut v = v;
    v.push(1);
    v.push(2);
    v.push(3);
    
    for i in &v { 
        println!("i = {}", i);
    }

    println!("Hello, world!");
}
