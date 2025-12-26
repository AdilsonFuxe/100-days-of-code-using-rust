fn main() {
    print!("Fibonacci sequence: ");
    for n in 0..10 {
        print!("{} ", fibonacci(n));
    }
    println!("\n----------------");
    print!("Fibonacci sequence: ");
    print_fibonacci_sequence(10);
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0  => 0,
        1  => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn print_fibonacci_sequence(n: u32) {
    let mut prev = 0;
    let mut curr = 1;

    let mut i = 0;

    while i < n {

        let sum = prev + curr;

        print!("{} ", prev);

        prev  = curr;
        curr  = sum;

        i += 1;
    }

}