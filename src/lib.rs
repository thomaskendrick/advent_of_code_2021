pub fn get_day_input(day: i8) -> String {
    std::fs::read_to_string(format!("./input/day{}.txt", day))
        .unwrap()
}
pub fn get_day_sample_input(day: i8) -> String {
    std::fs::read_to_string(format!("./input/day{}_sample.txt", day))
        .unwrap()
}
