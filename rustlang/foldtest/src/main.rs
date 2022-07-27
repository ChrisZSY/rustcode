//#![feature(inclusive_range_syntax)]
fn main() {

    for i in 1..11 {
        print!("{} ", i);
    }

    let mut n: i32 = 0;
    for _ in 0..10 {
        n += 1;
    }
    println!("num = {}", n);

    println!("num = {}", (0..10).count());

    for i in (0..21).filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }

    for i in (0..21).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
        print!("{} ", i);
    }

    for i in (0..11).rev() {
        print!("{} ", i);
    }

    for i in (1..11).map(|x| x * x) {
        print!("{} ", i);
    }

    let result : i32 = (1..1000).fold(0, |acc, x| acc + x * x);
    println!("result = {}", result);

    let mut acc = 0;

    for x in 1..1000 {
        acc += x * x;
    }

    let result = acc;
    println!("result = {}", result);

    let cities = ["Toronto", "New York", "Melbourne"];

    for city in cities.iter() {
        print!("{}, ", city);
    }

    for i in (0..10).rev().filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }

    let c = (1..4).chain(6..9);

    for i in c {
        print!("{} ", i);
    }

    let r = (1..20)
    .filter(|&x| x % 5 == 0)
    .chain((6..9).rev());

    for i in r {
        print!("{} ", i);
    }
    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, ‎4_529_500];

    let matrix = cities.iter().zip(populations.iter());

    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }
    extern crate char_iter;

    for c in char_iter::new('Д', 'П') {
        print!("{} ", c);
    }

    let nums = vec![1, 2, 3, 4, 5];

    for i in nums.iter() {
        print!("{} ", i);
    }

    let nums = vec![1, 2, 3, 4, 5];
    for i in &nums {
        print!("{} ", i);
    }

    let mut nums = vec![1, 2, 3, 4, 5];
    for i in &mut nums {
        *i *= 2;
    }
    println!("{:?}", nums);

    let nums = vec![1, 2, 3, 4, 5];
    let nums = nums.iter().map(|x| x * 2);
    println!("{:?}", nums);

    //let mut vnums = vec![1, 2, 3, 4, 5];
    //for i in &mut vnums {
    //    vnums.push(*i);
    //}
    //cannot borrow nums as mutable more than once at a time
    //println!("{:?}", vnums);

    let v = (1..11).collect::<Vec<i32>>();
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }

    let v = vec![3, 5, 0, -2, 3, 4, 1];
    let max = v.iter().max();
    let min = v.iter().min();

    println!("max = {:?}, min = {:?}", max, min);


    let grades = vec![4, 5, 6, 9, 7, 4, 8];
    let sum: i32 = grades.iter().sum();
    let gpa = sum as f32 / grades.len() as f32;

    println!("sum = {}, gpa = {:.2}", sum, gpa);


    let _r = (1..100).collect::<Vec<i32>>();
    let v = (1..100)
    .map(|x| x * x)
    .filter(|x| x % 5 == 0 )
    .take(10)
    .collect::<Vec<i32>>();

    println!("{:?} ", v);

    extern crate itertools;
    use itertools::Itertools;

    for i in (0..11).step(2) {
        print!("{} ", i);
    }
    //extern crate itertools;
    //use itertools::Itertools;

    let data = vec![1, 4, 3, 1, 4, 2, 5];
    let unique = data.iter().unique();

    for d in unique {
        print!("{} ", d);
    }

    //extern crate itertools;
    //use itertools::Itertools;

    let creatures = vec!["banshee", "basilisk", "centaur"];
    let list = creatures.iter().join(", ");
    println!("In the enchanted forest, we found {}.", list);

    let happiness_index = vec![ ("Austria", 12), ("Costa Rica", 14), ("Norway", 4),
    ("Australia", 9), ("Netherlands", 7), ("New Zealand", 8), ("United States", 13),
    ("Israel", 11), ("Denmark", 1), ("Finland", 5), ("Iceland", 3),
    ("Sweden", 10), ("Canada", 6), ("Puerto Rico", 15), ("Switzerland", 2) ];

    let top_contries = happiness_index
    .into_iter()
    .sorted_by(|a, b| (&a.1).cmp(&b.1))
    .into_iter()
    .take(5);

    for (country, rating) in top_contries {
        println!("# {}: {}", rating, country);
    }

    println!("Hello, world!");
}
