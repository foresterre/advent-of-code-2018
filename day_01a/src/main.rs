const INPUT: &str = include_str!("../../input/01");

fn solution(input: &str) -> i32 {
    input.lines().filter_map(|x| x.parse::<i32>().ok()).sum()
}

fn main() {
    println!("Solution: {}", solution(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chronal_calibration_part_1() {
        assert_eq!(574, solution(INPUT));
    }
}
