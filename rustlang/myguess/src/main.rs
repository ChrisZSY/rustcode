use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("let us begin play guess game");
    let sercetnum = rand::thread_rng().gen_range(1..101);
    loop{
        let mut guess = String::new();
        println!("please input you guess:"); 
        io::stdin().read_line(&mut guess).expect("faild to read line");
        let guess:u32 = match guess.trim().parse(){
            Ok(result) => result,
            Err(_)   =>continue,
        };
        println!("you guessed:{}", guess);
        match guess.cmp(&sercetnum){
            Ordering::Less =>println!("less"),
            Ordering::Greater =>println!("greater"),
            Ordering::Equal =>{
                println!("you win");
                break;
            }
        }
    }

    println!("Hello, world!");
}
