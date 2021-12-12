use ndarray::prelude::*;

fn parser(raw_input: &str) -> Array2<u32> {
    let lines: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).expect("not a number"))
                .collect()
        })
        .collect();
    let y_size = &lines.iter().next().expect("no data").len();
    let x_size = &lines.len();
    Array::from_shape_vec((*x_size, *y_size), lines.into_iter().flatten().collect())
        .expect("unable to create 2d array from input")
}
fn is_low_point(((cx, cy), value): ((usize, usize), &u32), map: &Array2<u32>) -> bool {
    let mut checks: Vec<(usize, usize)> = vec![(cx + 1, cy), (cx, cy + 1)];
    if cx > 0 {
        checks.push((cx - 1, cy));
    }
    if cy > 0 {
        checks.push((cx, cy - 1));
    }
    checks.into_iter().all(|c| match map.get(c) {
        Some(x) if x < &value => false,
        _ => true,
    })
}

fn read_basin<'a>(
    (cx, cy): (usize, usize),
    map: &Array2<u32>,
    history: &'a mut Vec<(usize, usize)>,
) -> &'a mut Vec<(usize, usize)> {
    if history.contains(&(cx, cy)) {
        return history;
    };
    history.push((cx, cy));
    let mut checks: Vec<(usize, usize)> = vec![(cx + 1, cy), (cx, cy + 1)];
    let lp = map.get((cx, cy)).unwrap();
    println!("Currently checking point {:?} with value {}", (cx, cy), lp);
    if cx > 0 {
        checks.push((cx - 1, cy));
    }
    if cy > 0 {
        checks.push((cx, cy - 1));
    }
    for check in checks {
        match map.get(check) {
            Some(x) if x > lp && x < &9 => {
                read_basin(check, map, history);
            }
            _ => (),
        }
    }
    history
}

fn solve_part_1(map: &Array2<u32>) -> u32 {
    return map.indexed_iter().fold(0, |acc, v| {
        if is_low_point(v, map) {
            acc + *v.1 + 1
        } else {
            acc
        }
    });
}

fn solve_part_2(map: &Array2<u32>) -> usize {
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for v in map.indexed_iter() {
        if is_low_point(v, map) {
            low_points.push(v.0);
        }
    }
    let mut basin_areas: Vec<usize> = Vec::new();
    for low_point in low_points {
        let mut visit_history: Vec<(usize, usize)> = Vec::new();
        basin_areas.push(read_basin(low_point, map, &mut visit_history).len());
    }
    basin_areas.sort();
    basin_areas.pop().unwrap() * basin_areas.pop().unwrap() * basin_areas.pop().unwrap()
}

fn main() {
    let input = aoc2021::get_day_input(9);
    let result = solve_part_1(&parser(&input));
    println!("Sum of all of the risk levels {}", result);
    let result_pt2 = solve_part_2(&parser(&input));
    println!("Size of all  {}", result_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_data_test_pt1() {
        assert_eq!(solve_part_1(&parser(&aoc2021::get_day_sample_input(9))), 15);
    }
    #[test]
    fn is_low_point_test() {
        let map = parser(&aoc2021::get_day_sample_input(9));
        assert!(!is_low_point(((0, 0), &2), &map));
        assert!(is_low_point(((0, 1), &1), &map));
        assert!(is_low_point(((0, 9), &0), &map));
    }
    #[test]
    fn basin_test() {
        let map = parser(&aoc2021::get_day_sample_input(9));
        let mut visit_history_1: Vec<(usize, usize)> = Vec::new();
        assert_eq!(read_basin((0, 1), &map, &mut visit_history_1).len(), 3);
        let mut visit_history_2: Vec<(usize, usize)> = Vec::new();
        assert_eq!(read_basin((0, 9), &map, &mut visit_history_2).len(), 9);
        let mut visit_history_3: Vec<(usize, usize)> = Vec::new();
        assert_eq!(read_basin((2, 2), &map, &mut visit_history_3).len(), 14);
    }
}
