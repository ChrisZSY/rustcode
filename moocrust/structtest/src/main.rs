#[derive(Debug, PartialEq, Default)] //{:?}{:#?}
struct Point<T>{
    x:T,
    y:T,
}
impl <T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::fmt::Result {
        write!(f, "({}, {})", self.x , self.y)
    }
}

impl<T:Clone + std::cmp::PartialOrd> Point<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        }else {
            self.y.clone()
        }

    }

}
//fn show<T:std::fmt::Display>(a: T){
fn show(a: impl std::fmt::Display){
    println!("show a= {}", a);
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point = Point{
        x:18,
        y:20,
    };
    println!("{:?}", point.largest());
    //println!("{:?}", point.distance_from_origin());
    println!("{:?}", point);
    println!("{}", point);
    println!("{:#?}", point);
    show(point);
    let point = Point {
        x:3.0,
        y:4.0,
    };
    println!("{:?}", point.largest());
    println!("{:?}", point.distance_from_origin());
    let point1 = Point{
        x:18,
        y:20,
    };

    let point2 = Point{
        x:18,
        y:20,
    };
    println!("p1 = p2 {}", point1 == point2);
    println!("Hello, world!");
}
