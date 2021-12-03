use std::collections::BTreeMap;

fn solve_part_1(input: &Vec<&str>) -> isize {
    let mut occurance_map: BTreeMap<usize, i32> = BTreeMap::new();
    for binary_code in input {
        for (i, bit) in binary_code.chars().enumerate() {
            match bit {
                '0' => {*occurance_map.entry(i).or_insert(-1) += -1},
                '1' => {*occurance_map.entry(i).or_insert(1) += 1},
                _ => panic!("Quantum computing not implemented")
            }
        }
    }
    let mut gamma_rate_string = String::new();
    let mut epsilon_rate_string = String::new();
    for (_key, value) in occurance_map {
        if value > 0 {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        } else if value < 0 {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        }    
    }
    let gamma_rate: isize = isize::from_str_radix(&gamma_rate_string, 2).unwrap() ;
    let epsilon_rate: isize = isize::from_str_radix(&epsilon_rate_string, 2).unwrap() ;
    gamma_rate * epsilon_rate
}

fn solve_part_2(input: &Vec<&str>) -> i32 {
    0
}

fn main() {
    let input = aoc2021::get_day_input(3);
    let parsed_input = input
        .lines()
        .collect();

    let part1 = solve_part_1(&parsed_input);
    println!("Solution to part one: {}", part1);
    let part2 = solve_part_2(&parsed_input);
    println!("Solution to part two: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let test_data = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ];
        assert_eq!(solve_part_1(&test_data), 198);
    }
}
