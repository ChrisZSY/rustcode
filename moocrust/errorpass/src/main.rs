fn add(a : u32, b:u32) -> u32 {
    //unimplemented!()
    unreachable!()
}
fn main() {
    //panic!("error!");
    assert!(1==1);
    assert_eq!(1,1);
    //add(1,2);
    let a: Result<u32, &'static str> = Result::Ok(1);

    let b: Result<u32, &'static str> = Result::Err("result error");

    println!("a = {:?}\nb = {:?}", a, b);
    
    let r = std::fs::read("./temp.txt");
    match r {
        Ok(data) =>println!("{:?}", std::str::from_utf8(&data).unwrap()),
        Err(err) => println!("{:?}", err),
    }

    println!("Hello, world!");
}
