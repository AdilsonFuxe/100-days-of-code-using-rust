fn main() {
    println!("{:?}", smaller_numbers_than_current(vec![8,1,2,2,3]))
}

fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut count = 0;
    for i in 0..nums.len() {
        count = 0;
        for j in 0..nums.len() {
            count += (i != j && nums[i] > nums[j]) as i32;
        }
        result.push(count);
    }

    result
}