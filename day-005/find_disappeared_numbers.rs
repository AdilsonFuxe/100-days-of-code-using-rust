fn main() {
    println!("{:?}", find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]));
}

fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    let mut map = std::collections::HashMap::new();

    for i in 0..nums.len() {
        map.insert(nums[i], nums[i]);
    }

    for i in 1..nums.len()+1 {
        if !map.contains_key(&(i as i32)) {
            result.push(i as i32);
        }
    }

    result
}