use futures::Future;
use futures::executor;

//async fn hello1(){
//   println!("hello world!!!");
//}

fn hello() ->impl Future<Output=()> {
    async{
        println!("hello world!!!");
    }
}

fn main()   {
    let fhello = hello();
    executor::block_on(fhello);
    my_function();
}

fn my_function(){
    println!("my fun hello world");
}