use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    // Calculate the total sum of the array elements
    let total_sum: i32 = arr.iter().sum();

    // Find the minimum and maximum value in the array
    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    // Calculate the minimum and maximum sums
    let min_sum = total_sum - max_value; // Exclude maximum
    let max_sum = total_sum - min_value; // Exclude minimum

    // Print results
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
