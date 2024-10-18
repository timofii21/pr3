
use std::io::{self, BufRead, Write};

// Функція для обчислення суми елементів масиву
fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

#[cfg(not(test))]
fn main() {
    let stdin = std::io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = std::env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = std::fs::File::create(&output_path).expect("Unable to create file");

    let _ar_count: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().expect("Parse error");

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Parse error"))
        .collect();

    let result = simple_array_sum(&ar);

    writeln!(&mut fptr, "{}", result).expect("Unable to write to file");
}

// Тест для функції simple_array_sum
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_array_sum() {
        let array = vec![1, 2, 3, 4, 5];
        let result = simple_array_sum(&array);
        assert_eq!(result, 15);
    }
}
