

fn main() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    // This loop prints: 0 1 2
    for x in array {
        print!("{x} ");
    }
    println!("-------------------------");
    let mut  array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    for x in &array {
        print!("{x} ");
    }
    println!("---Rust 2015 and 2018:----------------------");
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    // This creates a slice iterator, producing references to each value.
    for item in array.into_iter().enumerate() {
        let (i, x): (usize, i32) =  item; //let (i, x): (usize, &i32) =  item;  new compiler error
        println!("array[{i}] = {x}");
    }

    // The `array_into_iter` lint suggests this change for future compatibility:
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }

    // You can explicitly iterate an array by value using `IntoIterator::into_iter`
    for item in IntoIterator::into_iter(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}] = {x}");
    }

    println!("----Rust 2021---------------------");
    // Rust 2021:
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    // This iterates by reference:
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }

    // This iterates by value:
    for item in array.into_iter().enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}] = {x}");
    }

    println!("-------------------------");
    // Rust 2015 and 2018:
    
    let  mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;
    // This iterates by reference:
    for item in array.iter() {
        let x: &i32 = item;
        println!("{x}");
    }

    // This iterates by value:
    for item in IntoIterator::into_iter(array) {
        let x: i32 = item;
        println!("{x}");
    }   

    // This iterates by value:
    for item in array {
        let x: i32 = item;
        println!("{x}");
    }

    // IntoIter can also start a chain.
    // This iterates by value:
    for item in IntoIterator::into_iter(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}] = {x}");
    }
    println!("-------------------------");
    println!("-------------------------");
    println!("-------------------------");
}
