fn main() {
    println!("{}", find_max_consecutive_ones(vec![1,1,0,1,1,1]));
}

fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    
    let mut temp = 0;
    for i in 0..nums.len() {
        if nums[i as usize] == 1 {
            temp +=1;
        } else {
            if temp > max {
                max = temp;
            }

            temp = 0;
        }
    }

    if temp > max {
        return temp
    } 


    max
}