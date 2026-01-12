fn main() {
    println!("result: {:?}", concatenation_of_array(vec![1, 2, 3]));
}

fn concatenation_of_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();
    result.extend(nums);
    result
}