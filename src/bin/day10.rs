fn main() {
    let input = aoc2021::get_day_input(10);
    let result = solve_part_1(&input);
    println!("Total error score for file: {}", result);
    let result_2 = solve_part_2(&input);
    println!("Autocomplete winner score for file: {}", result_2);
}

fn parse(line: &str) -> (Option<char>, Vec<char>) {
    let mut chars = line.chars().into_iter();
    let mut char_stack: Vec<char> = vec![chars.next().unwrap()];
    for c in line.chars().skip(1) {
        match c {
            c if c == '<' || c == '(' || c == '{' || c == '[' => char_stack.push(c),
            c => match c {
                c if c == '>' => {
                    if char_stack.pop().unwrap() != '<' {
                        return (Some(c), char_stack);
                    }
                }
                c if c == ')' => {
                    if char_stack.pop().unwrap() != '(' {
                        return (Some(c), char_stack);
                    }
                }
                c if c == '}' => {
                    if char_stack.pop().unwrap() != '{' {
                        return (Some(c), char_stack);
                    }
                }
                c if c == ']' => {
                    if char_stack.pop().unwrap() != '[' {
                        return (Some(c), char_stack);
                    }
                }
                _ => panic!("Illegal illegal char :P"),
            },
        }
    }
    (None, char_stack)
}

fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .fold(0, |acc, (ichar, _)| match ichar {
            Some(c) if c == '>' => acc + 25137,
            Some(c) if c == '}' => acc + 1197,
            Some(c) if c == ']' => acc + 57,
            Some(c) if c == ')' => acc + 3,
            None => acc,
            _ => panic!(),
        })
}

fn solve_part_2(input: &str) -> usize {
    let mut result: Vec<usize> = input
        .lines()
        .map(parse)
        .filter_map(|(i_char, stack)| {
            if i_char.is_some() {
                return None;
            } else {
                stack.into_iter().rev().fold(Some(0), |acc, x| match x {
                    x if x == '<' => Some(5 * acc.unwrap() + 4),
                    x if x == '{' => Some(5 * acc.unwrap() + 3),
                    x if x == '[' => Some(5 * acc.unwrap() + 2),
                    x if x == '(' => Some(5 * acc.unwrap() + 1),
                    _ => panic!(),
                })
            }
        })
        .collect();
    result.sort();
    result[(result.len() - 1) / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_for_illegal_char_test() {
        let line_1 = "{([(<{}[<>[]}>{[]{[(<()>";
        let line_2 = "[[<[([]))<([[{}[[()]]]";
        let line_3 = "[{[{({}]{}}([{[{{{}}([]";
        let line_4 = "[({(<(())[]>[[{[]{<()<>>";

        assert_eq!(parse(line_1).0, Some('}'));
        assert_eq!(parse(line_2).0, Some(')'));
        assert_eq!(parse(line_3).0, Some(']'));
        assert_eq!(parse(line_4).0, None);
    }
    #[test]
    fn solve_part_1_test() {
        let input = aoc2021::get_day_sample_input(10);
        assert_eq!(solve_part_1(&input), 26397);
    }
    #[test]
    fn solve_part_2_test() {
        let input = aoc2021::get_day_sample_input(10);
        assert_eq!(solve_part_2(&input), 288957);
    }
}
