fn main() {
    let f = 150.0;
    println!(" 32 fahrenheit is {} celsius", fahrenheit_to_celsius(f));
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}