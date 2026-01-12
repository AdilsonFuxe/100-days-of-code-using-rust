fn main() {
    println!("result: {:?} ", shuffle(vec![1,2,3,4,4,3,2,1],4))
}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let z: usize = n as usize;
    for i in 0..z  {
        result.push(nums[i as usize]);
        result.push(nums[i+n as usize]);
    }

    result
}