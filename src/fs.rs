pub fn get_input(day: usize) -> std::io::Result<String> {
    std::fs::read_to_string(format!("./input/day{day}"))
}
