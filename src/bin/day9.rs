// NO FOR LOOPS ALLOWED

use ndarray::prelude::*;

fn parser(raw_input: &str) -> Array2<u32> {
    let lines: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).expect("not a number"))
                .collect()
        })
        .collect();
    let n = &lines.iter().next().expect("no data").len();
    Array::from_shape_vec((*n, *n), lines.into_iter().flatten().collect())
        .expect("Unable to create 2d array from input")
}

fn solve_part_1(input: &Array2<u32>) {}

fn main() {
    let input = aoc2021::get_day_input(9);
    let parsed_input = parser(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_data_test() {}
    // #[test]
    // fn sample_data_test_pt2() {
    // }
}
