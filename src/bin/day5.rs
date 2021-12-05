use std::cmp::max;
use std::cmp::min;
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
        if self.start.x != self.end.x {
            let range = min(self.start.x, self.end.x)..=max(self.start.x, self.end.x);
            for i in range {
                walk_points.push(Point {
                    x: i,
                    y: self.start.y,
                })
            }
        } else if self.start.y != self.end.y {
            let range = min(self.start.y, self.end.y)..=max(self.start.y, self.end.y);
            for i in range {
                walk_points.push(Point {
                    x: self.start.x,
                    y: i,
                })
            }
        } else {
            panic!("Diagonals not implemented")
        }
        walk_points
    }
    fn is_vert_or_horizontal(&self) -> bool {
        (self.start.x == self.end.x) || (self.start.y == self.end.y)
    }
}

fn parse_line(point_str_line: &str) -> LineSegment {
    let point_strs: Vec<&str> = point_str_line.split(" -> ").collect();
    let start_point_str: Vec<&str> = point_strs[0].split(",").collect();
    let end_point_str: Vec<&str> = point_strs[1].split(",").collect();
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

fn solve_part_1(lines: &Vec<LineSegment>) -> i32 {
    let orth_lines: Vec<&LineSegment> = lines
        .into_iter()
        .filter(|x| x.is_vert_or_horizontal())
        .collect();

    let mut visit_map: HashMap<Point, i32> = HashMap::new();

    for line in orth_lines {
       for point in line.walk() {
           let val = visit_map.entry(point).or_insert(0);
           *val += 1;
       }
    }
    let mut intersections = 0;
    for (_, val) in visit_map {
        if val > 1 {intersections += 1}
    }
    intersections
}

fn main() {
    let input = aoc2021::get_day_input(5);
    let line_segments: Vec<LineSegment> = input.lines().map(parse_line).collect();
    let part_1_result = solve_part_1(&line_segments);
    println!("Solution to part 1: {}", part_1_result);
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
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };
        let angled_line = LineSegment {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 12 },
        };
        assert_eq!(vertical_line.is_vert_or_horizontal(), true);
        assert_eq!(horizontal_line.is_vert_or_horizontal(), true);
        assert_eq!(angled_line.is_vert_or_horizontal(), false);
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
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 },
                Point { x: 3, y: 9 },
                Point { x: 4, y: 9 },
            ]
        );
    }
}
