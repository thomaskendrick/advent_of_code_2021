#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x|x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part_1(input: &Vec<i32>) -> i32 {
    let mut previous: i32 = input[0];
    let mut count:i32 = 0;

    for x in input {
        if x > &previous {count+=1}
        previous = *x;
    }
    return count;
}
#[aoc(day1, part2)]
fn solve_part_2(input: &Vec<i32>) -> i32 {
    let mut input2 = input.clone();
    input2.remove(0);

    let window_1_iterator = input.windows(3);
    let window_2_iterator = input2.windows(3);

    let combined_iterator = window_1_iterator.zip(window_2_iterator);

    let mut increase_count = 0;

    for vals in combined_iterator {
        let window_1_sum: i32 = vals.0.iter().sum();
        let window_2_sum: i32 = vals.1.iter().sum();

        if window_2_sum > window_1_sum {increase_count += 1}
    }
    return increase_count;
}
