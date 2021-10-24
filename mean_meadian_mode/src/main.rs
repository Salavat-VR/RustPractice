use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 3];
    let mut nums_2 = vec![1, 5, 3, 5];
    let mut nums_3 = vec![1, 1, 1, 2, 3, 2, 5, 2, 0];

    println!("1) the mean of out vector is {}", define_mean(&nums));
    println!("2) the median of out vector is {:?}", define_median(&mut nums_2));
    println!("3) the mode of out vector is {:?}", define_mode(&mut nums_3));
}

fn define_mean(nums: &Vec<i32>) -> f32 {
    let sum: i32 = nums.iter().sum();
    let general_amount = nums.len();

    sum as f32 / general_amount as f32
}

fn define_median(nums: &mut Vec<i32>) -> i32 {
    nums.sort();

    let mid = nums.len() / 2;
    if nums.len() % 2 == 0 {
        define_mean(&vec![nums[mid - 1], nums[mid]]) as i32
    } else {
        nums[mid]
    }
}

fn define_mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    /*
    learned from doc.rust-lang.ru how to count a unique
              elements in the Vector or string literals

    it proved to be very useful and vital in some particular situations
    */
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    // return the vector of mode elements in case there are more than 1 elements
    // occurs in the vector the same times
    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}





