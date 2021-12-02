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

impl Instruction {
    fn calculate_offset(self: &Self) -> Point {
        match self.direction {
            Direction::Forward => (1 * self.distance, 0),
            Direction::Up => (0, -1 * self.distance),
            Direction::Down => (0, 1 * self.distance),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(instruction_parser).collect()
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<Instruction>) -> i32 {
    let mut current_pos: Point = (0, 0);
    for instruction in input {
        let offset = instruction.calculate_offset();
        current_pos = (current_pos.0 + offset.0, current_pos.1 + offset.1)
    }
    return current_pos.0 * current_pos.1;
}
