// use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let fruits = &["apple", "banana", "cherry"];
    //
    // for fruit in fruits {
    //     println!("{}", fruit);
    // }
    //
    // let mut sum = 0;
    //
    // for i in 1..=11 {
    //     println!("{}", i);
    //     sum += i;
    // }

    // let mut  v: Vec<char> = Vec::new();

    //dbg!(v);


    // for item in 1..=9 {
    //     (&mut v).push("");
    // }
    // println!("{:?}", v);
    //println!("sum = {}", sum);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("You guessed: {}", guess);

    let number: u8 = 255;

    println!("The number is {}", number);
    //
    // // let mut  space = " ";
    // // space = space.len();;
    // // println!("{}{}", space, space.len());
    //
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    //println!("x + y = {}", x + y);

    let data = b'A';

    println!("The value of data is: {}", data);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}, {}", y, tup.2);


    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The value of months is: {:?}", months);

    let array = [3; 5];
    println!("The value of array is: {:?}", array);

    let first = array[0];
    let second = array[1];

    let array2 = [0; 10];

    println!("The value of array2 is: {:?}", array2);
}