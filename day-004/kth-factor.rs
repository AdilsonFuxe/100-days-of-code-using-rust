fn main() {
    println!("result: {}", kth_factor(4, 4));
    println!("result: {}", kth_factor(12, 3));
    println!("result: {}", kth_factor(7, 2));
}

fn kth_factor(n: i32, k: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n  {
      if n % i == 0 { 
        sum += 1;
      }

      if sum == k { 
        return i; 
       }
    }
    -1
}
