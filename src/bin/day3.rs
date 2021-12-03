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

fn solve_part_1(occurance_map: &BTreeMap<usize, (i32, i32)>) -> isize {
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

fn solve_part_2(binary_code_list: &Vec<&str>) -> isize {
    let mut o2_code_list = binary_code_list.clone();
    let mut co2_code_list = binary_code_list.clone();

    for i in 0..binary_code_list[0].len() {
        if o2_code_list.len() > 1 {
            let (one_count, zero_count) =
                *generate_bit_occurance_map(&o2_code_list).get(&i).unwrap();
            if one_count > zero_count {
                o2_code_list = o2_code_list
                    .into_iter()
                    .filter(|&code| code.chars().collect::<Vec<char>>()[*&i] == '1')
                    .collect::<Vec<&str>>();
            } else {
                o2_code_list = o2_code_list
                    .into_iter()
                    .filter(|&code| code.chars().collect::<Vec<char>>()[*&i] == '0')
                    .collect::<Vec<&str>>();
            }
        }
        if co2_code_list.len() > 1 {
            let (one_count, zero_count) =
                *generate_bit_occurance_map(&co2_code_list).get(&i).unwrap();
            if zero_count >= one_count {
                co2_code_list = co2_code_list
                    .into_iter()
                    .filter(|&code| code.chars().collect::<Vec<char>>()[*&i] == '1')
                    .collect::<Vec<&str>>();
            } else {
                co2_code_list = co2_code_list
                    .into_iter()
                    .filter(|&code| code.chars().collect::<Vec<char>>()[*&i] == '0')
                    .collect::<Vec<&str>>();
            }
        }
    }
    let o2_rating = isize::from_str_radix(&o2_code_list[0], 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2_code_list[0], 2).unwrap();
    o2_rating * co2_rating
}

fn main() {
    let input = aoc2021::get_day_input(3);
    let parsed_input = input.lines().collect();
    let occurance_map = generate_bit_occurance_map(&parsed_input);
    let part1 = solve_part_1(&occurance_map);
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
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(solve_part_1(&generate_bit_occurance_map(&test_data)), 198);
    }
    #[test]
    fn part_two_test() {
        let test_data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(solve_part_2(&test_data), 230);
    }
}
