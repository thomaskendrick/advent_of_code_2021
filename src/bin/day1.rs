fn solve_part_1(input: &[i32]) -> i32 {
    let mut previous: i32 = input[0];
    let mut count: i32 = 0;

    for x in input {
        if x > &previous {
            count += 1
        }
        previous = *x;
    }
    count
}

fn solve_part_2(input: &[i32]) -> i32 {
    let mut input2 = input.to_owned();
    input2.remove(0);

    let window_1_iterator = input.windows(3);
    let window_2_iterator = input2.windows(3);

    let combined_iterator = window_1_iterator.zip(window_2_iterator);

    let mut increase_count = 0;

    for vals in combined_iterator {
        let window_1_sum: i32 = vals.0.iter().sum();
        let window_2_sum: i32 = vals.1.iter().sum();

        if window_2_sum > window_1_sum {
            increase_count += 1
        }
    }
    increase_count
}

fn main() {
    let input = aoc2021::get_day_input(1);
    let parsed_input :Vec<i32> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let part1 = solve_part_1(&parsed_input);
    println!("Solution to part one: {}", part1);
    let part2 = solve_part_2(&parsed_input);
    println!("Solution to part two: {}", part2);
}
