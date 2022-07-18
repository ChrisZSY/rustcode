fn calculate_length(s:String) ->(String, usize) {
    let len = s.len();
    (s, len)
}
fn calculate_len(s:&mut String) -> usize {
    s.push_str("!!!!");
    s.len()
}
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
fn main() {
    let s1 = String::from("hello world");

    let (mut s1, len) = calculate_length(s1);

    println!("{}", s1);
    println!("{}", len);

    let len = calculate_len(&mut s1);
    println!("{}", len);

    let p = Person{
        name: String::from("John"),
        age:30,
    };
    let age =|p:&Person| p.age;
    let name:for<'a> fn(&'a Person) ->&'a String= |p:&Person| &p.name;
    
    println!("{}", age(&p));
    println!("{}", name(&p));

    println!("{:#?}", p);

    let mut num = 5;
    {
        let mut add_num = |x:i32| num +=x;
        add_num(5);
        println!("num1 = {}", num);
    }
    println!("num2 = {}", num);
    let mut num = 5;
    {
        let mut add_num = move |x:i32| {num +=x;println!("num3 = {}", num);};
        add_num(1);
        println!("num4 = {}", num);
    }
    println!("num5 = {}", num);
    let mut add_num = move |x:i32| {num +=x;println!("num6 = {}", num);};
    add_num(1);
    println!("num2 = {}", num);
    println!("Hello, world!");
}
