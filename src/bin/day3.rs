use std::collections::BTreeMap;

fn generate_bit_occurance_map(binary_code_list: &Vec<&str>) -> BTreeMap<usize, (i32, i32)> {
    let mut occurance_map: BTreeMap<usize, (i32, i32)> = BTreeMap::new();
    for binary_code in binary_code_list {
        for (i, bit) in binary_code.chars().enumerate() {
            match bit {
                '0' => {
                    let (zero_count, _) = occurance_map.entry(i).or_insert((0, 0));
                    *zero_count += 1
                }
                '1' => {
                    let (_, one_count) = occurance_map.entry(i).or_insert((0, 0));
                    *one_count += 1
                }
                _ => panic!("Quantum computing not implemented"),
            }
        }
    }
    occurance_map
}

fn solve_part_1(occurance_map: BTreeMap<usize, (i32, i32)>) -> isize {
    let mut gamma_rate_string = String::new();
    let mut epsilon_rate_string = String::new();
    for (zero_count, one_count) in occurance_map.values() {
        if one_count > zero_count {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        } else {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        }
    }
    let gamma_rate = isize::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_string, 2).unwrap();
    gamma_rate * epsilon_rate
}

fn main() {
    let input = aoc2021::get_day_input(3);
    let occurance_map = generate_bit_occurance_map(&input.lines().collect());
    let part1 = solve_part_1(occurance_map);
    println!("Solution to part one: {}", part1);
    // let part2 = solve_part_2(&parsed_input);
    // println!("Solution to part two: {}", part2);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        let test_data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(solve_part_1(generate_bit_occurance_map(&test_data)), 198);
    }
}
