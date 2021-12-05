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

fn is_vert_or_horizontal(LineSegment {start, end}: LineSegment) -> bool {
    (start.x == end.x) || (start.y == end.y)
}

fn main() {
    let input = aoc2021::get_day_input(5);
    let line_segements: Vec<LineSegment> = input.lines().map(parse_line).collect();
    println!("Parsed line line_segments {:?}", line_segements);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_lines_test() {
        let test_line = "932,123 -> 1233,21";
        assert_eq!(
            parse_line(test_line),
            LineSegment {
                start: Point { x: 932, y: 123 },
                end: Point { x: 1233, y: 21 }
            }
        );
    }
    fn is_vert_or_horizontal_test(){
            let vertical_line = LineSegment {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            };
            let horizontal_line = LineSegment {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            };
            let angled_line = LineSegment {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 12 }
            };
            assert_eq!(is_vert_or_horizontal(vertical_line), true);
            assert_eq!(is_vert_or_horizontal(horizontal_line), true);
            assert_eq!(is_vert_or_horizontal(angled_line), false);
    }
}
