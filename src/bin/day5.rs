use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn walk(&self) -> Vec<Point> {
        let mut walk_points: Vec<Point> = Vec::new();
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let mut line_remaining = max(dx.abs(), dy.abs());
        let mut pos = self.start.clone();

        while line_remaining >= 0 {
            walk_points.push(pos.clone());
            match dy {
                d if d > 0 => pos.y += 1,
                d if d < 0 => pos.y += -1,
                _ => {}
            };
            match dx {
                d if d > 0 => pos.x += 1,
                d if d < 0 => pos.x += -1,
                _ => {}
            };
            line_remaining += -1;
        }
        walk_points
    }
    fn is_vert_or_horizontal(&self) -> bool {
        (self.start.x == self.end.x) || (self.start.y == self.end.y)
    }
}

fn parse_line(point_str_line: &str) -> LineSegment {
    let point_strs: Vec<&str> = point_str_line.split(" -> ").collect();
    let start_point_str: Vec<&str> = point_strs[0].split(',').collect();
    let end_point_str: Vec<&str> = point_strs[1].split(',').collect();
    LineSegment {
        start: Point {
            x: start_point_str[0].parse().unwrap(),
            y: start_point_str[1].parse().unwrap(),
        },
        end: Point {
            x: end_point_str[0].parse().unwrap(),
            y: end_point_str[1].parse().unwrap(),
        },
    }
}

fn solve_part_1(lines: &[LineSegment]) -> usize {
    let orth_lines: Vec<&LineSegment> = lines
        .iter()
        .filter(|x| x.is_vert_or_horizontal())
        .collect();

    let mut visit_map: HashMap<Point, i32> = HashMap::new();

    for line in orth_lines {
       for point in line.walk() {
           *visit_map.entry(point).or_insert(0) += 1;
       }
    }
    visit_map.retain(|_k, v| v > &mut 1);
    visit_map.keys().len()
}
fn solve_part_2(lines: &[LineSegment]) -> usize {
    let mut visit_map: HashMap<Point, i32> = HashMap::new();

    for line in lines {
       for point in line.walk() {
           *visit_map.entry(point).or_insert(0) += 1;
       }
    }
    visit_map.retain(|_k, v| v > &mut 1);
    visit_map.keys().len()
}

fn main() {
    let input = aoc2021::get_day_input(5);
    let line_segments: Vec<LineSegment> = input.lines().map(parse_line).collect();
    let part_1_result = solve_part_1(&line_segments);
    println!("Solution to part 1: {}", part_1_result);
    let part_2_result = solve_part_2(&line_segments);
    println!("Solution to part 2: {}", part_2_result);
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
    #[test]
    fn is_vert_or_horizontal_test() {
        let vertical_line = LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };
        let horizontal_line = LineSegment {
            start: Point { x: 5, y: 3 },
            end: Point { x: 5, y: 9 },
        };
        let angled_line = LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 12 },
        };
        assert!(vertical_line.is_vert_or_horizontal());
        assert!(horizontal_line.is_vert_or_horizontal());
        assert!(!angled_line.is_vert_or_horizontal());
    }
    #[test]
    fn walk_test() {
        let vertical_line = LineSegment {
            start: Point { x: 3, y: 4 },
            end: Point { x: 3, y: 6 },
        };
        assert_eq!(
            vertical_line.walk(),
            vec![
                Point { x: 3, y: 4 },
                Point { x: 3, y: 5 },
                Point { x: 3, y: 6 }
            ]
        );
        let horizontal_line = LineSegment {
            start: Point { x: 4, y: 9 },
            end: Point { x: 1, y: 9 },
        };
        assert_eq!(
            horizontal_line.walk(),
            vec![
                Point { x: 4, y: 9 },
                Point { x: 3, y: 9 },
                Point { x: 2, y: 9 },
                Point { x: 1, y: 9 },
            ]
        );
        let up_right_line = LineSegment {
            start: Point { x: 0, y: 0 },
            end: Point { x: 3, y: 3 },
        };
        assert_eq!(
            up_right_line.walk(),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 },
            ]
        );
        let up_left_line = LineSegment {
            start: Point { x: 3, y: 0 },
            end: Point { x: 0, y: 3 },
        };
        assert_eq!(
            up_left_line.walk(),
            vec![
                Point { x: 3, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 0, y: 3 },
            ]
        );
        let down_left_line = LineSegment {
            start: Point { x: 3, y: 3 },
            end: Point { x: 0, y: 0 },
        };
        assert_eq!(
            down_left_line.walk(),
            vec![
                Point { x: 3, y: 3 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 },
            ]
        );
        let down_right_line = LineSegment {
            start: Point { x: 0, y: 3 },
            end: Point { x: 3, y: 0 },
        };
        assert_eq!(
            down_right_line.walk(),
            vec![
                Point { x: 0, y: 3 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 0 },
            ]
        );
    }
}
