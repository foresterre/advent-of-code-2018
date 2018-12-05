fn do_polymers_react(first: &char, second: &char) -> bool {
    // reflection: I used to compare a bit more times, first.is_lowercase && !second.is_lowercase
    // || second.is_lowercase && !first.is_lowercase etc.
    // It turned out, so many comparisons weren't necessary.

    first != second // e.g. 'a' != 'A'
    && first.eq_ignore_ascii_case(second) // e.g. lowercase('A') == lowercase('a')
}

const INPUT: &str = include_str!("../../input/05");

fn solution(input: &str) -> usize {
    let polymers: Vec<char> = input
        .trim() // the end of the input has a newline; my example didn't. Let's ensure the input hasn't anymore too.
        .chars()
        .collect();

    let zero: Vec<char> = vec![];

    let result = polymers
        .iter()
        .rfold(zero, |mut acc: Vec<char>, &x| match acc.last() {
            Some(c) if do_polymers_react(&x, c) => {
                acc.pop();
                acc
            }
            _ => {
                acc.push(x);
                acc
            }
        });

    let res = String::from(result.iter().collect::<String>());
    println!("~> {}.", res);
    res.len()
}

fn main() {
    println!("with input 05: {}", solution(INPUT));
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_reactions_a_A() {
        assert!(do_polymers_react(&'a', &'A'));
    }

    #[test]
    fn test_reactions_A_a() {
        assert!(do_polymers_react(&'A', &'a'));
    }

    #[test]
    fn test_reactions_z_Z() {
        assert!(do_polymers_react(&'Z', &'z'));
    }

    #[test]
    fn test_reactions_x_Z() {
        assert!(!do_polymers_react(&'x', &'Z'));
    }

    #[test]
    fn test_reactions_a_a() {
        assert!(!do_polymers_react(&'a', &'a'));
    }

    #[test]
    fn test_reactions_A_A() {
        assert!(!do_polymers_react(&'A', &'A'));
    }

    #[test]
    fn example_case() {
        let example_input = "dabAcCaCBAcCcaDA";
        assert_eq!(10, solution(example_input));
    }
}
