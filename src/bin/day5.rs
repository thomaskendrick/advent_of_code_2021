#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct LineSegment {
    start: Point,
    end: Point,
}

fn parse_line(point_str_line: &str) -> LineSegment {
    let point_strs : Vec<&str> = point_str_line.split(" -> ").collect();
    let start_point_str: Vec<&str> = point_strs[0].split(",").collect();
    let end_point_str: Vec<&str> = point_strs[1].split(",").collect();
    LineSegment {
       start: Point {
           x: start_point_str[0].parse().unwrap(),
           y: start_point_str[1].parse().unwrap()
        },
       end: Point {
           x: end_point_str[0].parse().unwrap(),
           y: end_point_str[1].parse().unwrap()
        },
    }
}

pub fn input_parser(input: &str) {
    // let point_strings: Vec<LineSegment> = input.lines().map(
    // ).collect();
}
fn main() {
    let input = aoc2021::get_day_input(5);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_lines_test() {
        let test_line = "0,9 -> 5,9";
        assert_eq!(
            parse_line(test_line),
            LineSegment {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            }
        )
    }
}
