fn main() {
    //test loop
    let mut counter = 0;
    let result = loop { 
        counter += 1;
        if counter == 10 { 
            break counter * 2;
        }
    };
    println!("loop return rusult = {}", result);
    println!("liftoff!!!!");
    //test while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("liftoff!!!!");
    //test for
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    for i in &vec {
        println!("i = {}", i);
    }
    println!("liftoff!!!!");
    //test while iter arr
    let arr = [10, 20, 30, 40,50];
    let mut i = 0;
    while i < 5 {
        println!("arr[{}] = {}", i , arr[i]);
        i = i + 1;
    }
    println!("liftoff!!!!");
    //test for iter arr
    for element in arr.iter() {
        println!("the value is : {}", element);
    }

    println!("liftoff!!!!");
    for number in (1..4).rev() {
        println!("{} !", number);
    }
    println!("liftoff!!!!");
    println!("Hello, world!");
}
