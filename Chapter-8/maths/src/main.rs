use std::collections::HashMap;

fn main() {
    let nums = vec![1,2,7,7,4,2,5,9];
    let avg = mean(&nums);
    println!("average is {}", avg);

    let med = median(&nums);
    println!("median is {}", med);

    let mode = mode(&nums);
    println!("modes are {:#?}", mode);
}

fn mean(nums: &Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, &b| a + b) / (nums.len() as i32)
}

fn median(nums: &Vec<i32>) -> f32 {
    let mut sorted = nums.to_vec();
    sorted.sort();
    let mid: f32 = if nums.len() % 2 != 0 {
        (sorted[nums.len()/2] as f32)
    } else {
        ((((sorted[nums.len()/2] + sorted[(nums.len()/2)-1]) as f32) / 2.0) as f32)
    };
    mid
}

fn mode(nums: &Vec<i32>) -> Vec<i32> {

    // Simple frequency counter to count the occurance of elements
    let mut counts = HashMap::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    let max = counts.values().max().unwrap_or(&0);

    // Let's filter through selecting only values that match our max value
    // and then send that tuple along to a map to return us only the keys and
    // collect this up into a vector to be returned
    counts.iter()
        .filter(|&(_k, v)| v == max)
        .map(|(&k, _v)| k)
        .collect()
}
