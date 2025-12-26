use std::io;

fn main() {
    let data = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");

    let element = data[index];

    println!("The element at {} is {}", index, element);
}