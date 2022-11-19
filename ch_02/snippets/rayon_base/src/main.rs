use rayon::prelude::*;

fn main() {
    println!("sum: {}", sum_of_squares(&[3, 4, 5, 9]));
}

fn sum_of_squares(input: &[u64]) -> u64 {
    input.par_iter()
        .map(|&i| i * i)
        .sum()
}