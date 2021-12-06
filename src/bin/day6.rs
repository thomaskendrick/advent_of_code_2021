const PART_1_GENERATIONS: usize = 80;
const PART_2_GENERATIONS: usize = 256;
use std::collections::VecDeque;

fn solve(input: &[usize], generations: &usize) -> usize {
    let mut fish_count: VecDeque<usize> = [0; 9].iter().copied().collect();
    for i in input {
        fish_count[*i] += 1;
    }
    for _ in 0..*generations {
        let pregnant_fish = fish_count.pop_front().unwrap();
        fish_count.push_back(pregnant_fish);
        fish_count[6] += pregnant_fish
    }
    fish_count.into_iter().sum::<usize>()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn main() {
    let input = aoc2021::get_day_input(6);
    let parsed_input = parse_input(&input);
    let part_1_result = solve(&parsed_input, &PART_1_GENERATIONS);
    println!(
        "Part 1 Result: {} little fish swimming in the sea 🐟..oO",
        part_1_result
    );
    let part_2_result = solve(&parsed_input, &PART_2_GENERATIONS);
    println!(
        "Part 2 Result: {} little fish swimming in the sea 🐟..oO",
        part_2_result
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_data_test() {
        let parsed_input = parse_input(&aoc2021::get_day_sample_input(6));
        assert_eq!(solve(&parsed_input, &18), 26);
        assert_eq!(solve(&parsed_input, &80), 5934);
        assert_eq!(solve(&parsed_input, &256), 26984457539);
    }
}
