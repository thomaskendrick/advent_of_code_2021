use std::cmp::{max, min};

fn solve(input: &[i32], factor_in_efficiency: bool) -> i32 {
    let max_position = input.iter().max().unwrap();
    let crabs = input.to_owned();
    let mut best_fuel_cost: i32 = crabs
        .iter()
        .map(|x| calc_fuel(x, &0, factor_in_efficiency))
        .sum();
    let mut best_position: i32 = *max_position;
    for current_position in 1..*max_position {
        let total_fuel = &crabs
            .iter()
            .map(|x| calc_fuel(x, &current_position, factor_in_efficiency))
            .sum();
        if *total_fuel < best_fuel_cost {
            best_fuel_cost = *total_fuel;
            best_position = current_position;
        }
    }
    best_fuel_cost
}

fn calc_fuel(crab_position: &i32, current_position: &i32, factor_in_efficiency: bool) -> i32 {
    if !factor_in_efficiency {
        return (crab_position - current_position).abs();
    };
    let min_pos = min(crab_position, current_position);
    let max_pos = max(crab_position, current_position);
    let mut fuel_cost = 0;
    let mut ineff_cost = 1;
    for _ in *min_pos..*max_pos {
        fuel_cost += ineff_cost;
        ineff_cost += 1;
    }
    // dbg!(fuel_cost);
    fuel_cost
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn main() {
    let input = parse_input(&aoc2021::get_day_input(7));
    println!("Best fuel position: {}", solve(&input, false));
    println!(
        "Best fuel position with efficiency factor: {}",
        solve(&input, true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_data_test() {
        let parsed_input = parse_input(&aoc2021::get_day_sample_input(7));
        assert_eq!(solve(&parsed_input, false), 37);
        assert_eq!(solve(&parsed_input, true), 168);
    }
}
