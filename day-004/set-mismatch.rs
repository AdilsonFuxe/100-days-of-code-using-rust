fn main() {
    println!("{:?}", find_error_nums(vec![1,5,3,2,2,7,6,4,8,9]));
}

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut map = std::collections::HashMap::new();
    let mut wrong_number = -1;
    let mut sorted = nums.clone();
    let mut existed_values = Vec::new();
    let mut last_value = -1;
    sorted.sort();

    //println!("{:?}", sorted);


    for i in 0..sorted.len() {

        if (i as i32) + 1 != sorted[i as usize] && wrong_number == -1 {
            wrong_number = (i as i32) + 1;
        }

        if map.contains_key(&sorted[i as usize]) {
            result.push(sorted[i as usize]);
        } else {
            existed_values.push(sorted[i as usize]);
            map.insert(sorted[i as usize], sorted[i as usize]);
        }
    }

   
    for i in 0..existed_values.len() {
        last_value = existed_values[i as usize];
        if existed_values[i as usize] != (i as i32) + 1 {
            wrong_number = (i as i32) + 1;
            result.push(wrong_number);
            return result;
        }
    }

    if last_value != (nums.len() as i32) {
        result.push(last_value + 1);
    }

    result
}