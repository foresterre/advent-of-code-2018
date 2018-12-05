#[derive(Debug)]
enum Errors {}

const INPUT: &str = include_str!("../../input/01");

fn solution(input: &str) -> i32 {
    let mut seen = std::collections::BTreeSet::new();
    let mut current = 0;

    seen.insert(current);

    input
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .cycle() // "the device might need to repeat its list of frequencies many times ..."
        .take_while(|c| {
            current += *c;
            seen.insert(current)
        })
        .last(); // Only called because the iterator is lazy, otherwise it won't do anything...

    current
}

fn main() -> Result<(), Errors> {
    println!("Solution: {}", solution(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chronal_calibration_part_2() {
        assert_eq!(452, solution(INPUT));
    }

    #[test]
    fn example_1() {
        assert_eq!(0, solution("+1\n-1"));
    }

    #[test]
    fn example_2() {
        assert_eq!(10, solution("+3\n+3\n+4\n-2\n-4"));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, solution("-6\n+3\n+8\n+5\n-6"));
    }

    #[test]
    fn example_4() {
        assert_eq!(14, solution("+7\n+7\n-2\n-7\n-4"));
    }
}
