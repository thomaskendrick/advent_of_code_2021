enum Direction {
    Up,
    Down,
    Forward,
}

type Point = (i32, i32);

pub struct Instruction {
    direction: Direction,
    distance: i32,
}

fn instruction_parser(line: &str) -> Instruction {
    let split_line: Vec<&str> = line.split(" ").collect();
    Instruction {
        direction: match split_line[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Bad instruction"),
        },
        distance: split_line[1].parse().unwrap(),
    }
}

pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(instruction_parser).collect()
}

fn solve_part_1(input: &Vec<Instruction>) -> i32 {
    let mut current_pos: Point = (0, 0);
    for instruction in input {
        let offset = match instruction.direction {
            Direction::Forward => (1 * instruction.distance, 0),
            Direction::Up => (0, -1 * instruction.distance),
            Direction::Down => (0, 1 * instruction.distance),
        };

        current_pos = (current_pos.0 + offset.0, current_pos.1 + offset.1)
    }
    current_pos.0 * current_pos.1
}
fn solve_part_2(input: &Vec<Instruction>) -> i32 {
    let mut aim: i32 = 0;
    let mut current_pos: Point = (0, 0);
    for instruction in input {
        match instruction.direction {
            Direction::Forward => {
                current_pos = (
                    current_pos.0 + instruction.distance,
                    current_pos.1 + (aim * instruction.distance),
                )
            }
            Direction::Up => aim += -1 * instruction.distance,
            Direction::Down => aim += instruction.distance,
        }
    }
    current_pos.0 * current_pos.1
}

fn main() {
    let input = aoc2021::get_day_input(2);
    let parsed_input = input
        .lines()
        .map(instruction_parser)
        .collect();

    let part1 = solve_part_1(&parsed_input);
    println!("Solution to part one: {}", part1);
    let part2 = solve_part_2(&parsed_input);
    println!("Solution to part two: {}", part2);
}
