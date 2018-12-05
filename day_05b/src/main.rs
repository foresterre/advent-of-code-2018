fn do_polymers_react(first: char, second: char) -> bool {
    first != second // e.g. 'a' != 'A'
        && first.eq_ignore_ascii_case(&second) // e.g. lowercase('A') == lowercase('a')
}

const INPUT: &str = include_str!("../../input/05");

// for the second exercise:
// for any a, both is_lower(a) and is_lower(a) _and_ is_upper(a) and is_upper(a) still react
// additionally, if all units of a certain type of a will be removed, before processing
// the standard reactions the resulting polymer can be shorter
// res <- shortest polymer
//
//
// example:
//
// dabAcCaCBAcCcaDA
// > pre_process:
// dbcCCBcCcD
// > process:
// dbCBcD
fn solution(input: &str) -> usize {
    // find all types of polymer where a type 'a' is actually a type 'a'/'A'
    let mut polymer_types: Vec<char> = input.trim().to_ascii_lowercase().chars().collect();
    polymer_types.sort();
    polymer_types.dedup();

    polymer_types
        .iter()
        .map(|polymer_type| {
            let s = input
                .chars()
                .filter(|c| c.to_ascii_lowercase() != *polymer_type)
                .collect::<String>();
            process_polymer(&s)
        })
        .min()
        .unwrap()
}

fn process_polymer(input: &str) -> usize {
    let polymers: Vec<char> = input
        .trim() // the end of the input has a newline; my example didn't. Let's ensure the input hasn't anymore too.
        .chars()
        .collect();

    let zero: Vec<char> = vec![];

    let result = polymers
        .iter()
        .rfold(zero, |mut acc: Vec<char>, &x| match acc.last() {
            Some(c) if do_polymers_react(x, *c) => {
                acc.pop();
                acc
            }
            _ => {
                acc.push(x);
                acc
            }
        });

    result.iter().count()
}

fn main() {
    println!("with input 05: {}", solution(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alchemical_reduction_part_2() {
        assert_eq!(4992, solution(INPUT));
    }

    #[test]
    fn example_case() {
        let example_input = "dabAcCaCBAcCcaDA";
        assert_eq!(4, solution(example_input));
    }

    #[test]
    fn test_reactions_upper_lower() {
        assert!(do_polymers_react('A', 'a'));
    }

    #[test]
    fn test_reactions_lower_upper() {
        assert!(do_polymers_react('a', 'A'));
    }

    #[test]
    fn test_reactions_lower_lower() {
        assert!(!do_polymers_react('a', 'a'));
    }

    #[test]
    fn test_reactions_upper_upper() {
        assert!(!do_polymers_react('A', 'A'));
    }

    #[test]
    fn test_reactions_upper_lower_other() {
        assert!(!do_polymers_react('A', 'x'));
    }

    #[test]
    fn test_reactions_lower_upper_other() {
        assert!(!do_polymers_react('a', 'X'));
    }

    #[test]
    fn test_reactions_lower_lower_other() {
        assert!(!do_polymers_react('a', 'x'));
    }

    #[test]
    fn test_reactions_upper_upper_other() {
        assert!(!do_polymers_react('A', 'X'));
    }
}
