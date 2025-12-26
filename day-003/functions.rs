fn main() {
    // another_function();
    // print_integer(5);

    let y = {
        let x= 12;
        x + 5
    };
    println!("The value of y is: {}", y);

    let result = sum(5, 6);

    println!("The sum of 3 + 5 is {}", result);
}


// fn another_function() {
//     println!("Hello, world!");
// }
// fn print_integer(x: i32) {
//     println!("The value of x is: {}", x);
// }

fn sum(x: i32, y: i32) -> i32 {
    x + y
}