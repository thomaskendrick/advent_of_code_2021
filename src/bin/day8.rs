use itertools::Itertools;
use std::collections::HashMap;
use std::error;

#[derive(Debug)]
struct SegmentEntry<'a> {
    signals: Vec<&'a str>,
    codes: Vec<&'a str>,
}
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn parse_input(input: &str) -> Result<Vec<SegmentEntry>> {
    let mut segment_vec: Vec<SegmentEntry> = Vec::new();
    for line in input.lines() {
        let signal_and_code: Vec<&str> = line.split('|').collect();
        segment_vec.push(SegmentEntry {
            signals: signal_and_code[0].trim().split(' ').collect(),
            codes: signal_and_code[1].trim().split(' ').collect(),
        })
    }
    Ok(segment_vec)
}

fn solve_part_1(input: &Vec<SegmentEntry>) -> i32 {
    let mut total_occurances = 0;
    for segment_entry in input {
        let mut simple_chars: Vec<&str> = Vec::new();
        for signal in &segment_entry.signals {
            match signal.len() {
                l if l == 2 => simple_chars.push(signal),
                l if l == 3 => simple_chars.push(signal),
                l if l == 4 => simple_chars.push(signal),
                l if l == 7 => simple_chars.push(signal),
                _ => (),
            };
        }
        let mut occurances = 0;
        for code in &segment_entry.codes {
            for valid_code in &simple_chars {
                let valid_chars: Vec<char> = valid_code.chars().sorted().collect();
                let candidate_chars: Vec<char> = code.chars().sorted().collect();
                if valid_chars == candidate_chars {
                    occurances += 1
                }
            }
        }
        total_occurances += occurances;
    }
    total_occurances
}
fn solve_part_2(input: &Vec<SegmentEntry>) -> i32 {
    let mut grand_total = 0;
    for segment_entry in input {
        let mut decoder_map: HashMap<i32, Vec<char>> = HashMap::new();
        for signal in &segment_entry.signals {
            let sorted_signal : Vec<char> = signal.chars().sorted().collect();
            match signal.len() {
                l if l == 2 => {
                    decoder_map.insert(1, sorted_signal);
                }
                l if l == 3 => {
                    decoder_map.insert(7, sorted_signal);
                }
                l if l == 4 => {
                    decoder_map.insert(4, sorted_signal);
                }
                l if l == 7 => {
                    decoder_map.insert(8, sorted_signal);
                }
                _ => (),
            };
        }
        let mut tr_and_br: Vec<char> = Vec::new();
        for c in decoder_map.get(&7).unwrap() {
            if decoder_map.get(&1).unwrap().contains(c) {
                tr_and_br.push(*c);
            }
        }
        let mut tl_and_middle: Vec<char> = Vec::new();
        for c in decoder_map.get(&4).unwrap() {
            if !decoder_map.get(&1).unwrap().contains(c) {
                tl_and_middle.push(*c)
            }
        }
        for signal in &segment_entry.signals {

            let sorted_signal : Vec<char> = signal.chars().sorted().collect();
            match signal.len() {
                l if l == 5
                    && signal.contains(tl_and_middle[0])
                    && signal.contains(tl_and_middle[1]) =>
                {
                    decoder_map.insert(5, sorted_signal);
                }
                l if l == 5 && signal.contains(tr_and_br[0]) && signal.contains(tr_and_br[1]) => {
                    decoder_map.insert(3, sorted_signal);
                }
                l if l == 5 => {
                    decoder_map.insert(2, sorted_signal);
                }
                l if l == 6
                    && signal.contains(tr_and_br[0])
                    && signal.contains(tr_and_br[1])
                    && signal.contains(tl_and_middle[0])
                    && signal.contains(tl_and_middle[1]) =>
                {
                    decoder_map.insert(9, sorted_signal);
                }
                l if l == 6
                    && signal.contains(tl_and_middle[0])
                    && signal.contains(tl_and_middle[1]) =>
                {
                    decoder_map.insert(6, sorted_signal);
                }
                l if l == 6 => {
                    decoder_map.insert(0, sorted_signal);
                }
                _ => {}
            };
        }
        let mut decoded_digits: Vec<i32>  = Vec::new();
        for code in &segment_entry.codes {
            let sorted_code: Vec<char> = code.chars().sorted().collect();
            for (digit, valid_code) in decoder_map.iter() {
                if valid_code == &sorted_code {
                    decoded_digits.push(*digit);
                }
            }
        }
        // Concat the digits
        let mut value = 0;
        for n in decoded_digits {
            value *= 10;
            value += n;
        }
        grand_total += value;
    }
    grand_total
}

fn main() {
    let result = solve_part_1(&parse_input(&aoc2021::get_day_input(8)).unwrap());
    println!("{} total simple number occurances", result);
    let result = solve_part_2(&parse_input(&aoc2021::get_day_input(8)).unwrap());
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_data_test() {
        let sample_input = aoc2021::get_day_sample_input(8);
        let parse_input = parse_input(&sample_input);
        assert_eq!(solve_part_1(&parse_input.unwrap()), 26);
    }
    #[test]
    fn sample_data_test_pt2() {
        let sample_input = aoc2021::get_day_sample_input(8);
        let parse_input = parse_input(&sample_input);
        assert_eq!(solve_part_2(&parse_input.unwrap()), 61229);
    }
}
