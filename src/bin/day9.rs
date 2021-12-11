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
fn solve_part_1(map: &Array2<u32>) -> u32 {
    return map.indexed_iter().fold(0, |acc, v| {
        if is_low_point(v, map) {
            acc + *v.1 + 1
        } else {
            acc
        }
    });
}

fn main() {
    let input = aoc2021::get_day_input(9);
    let result = solve_part_1(&parser(&input));
    println!("Sum of all of the risk levels {}", result);
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
}
