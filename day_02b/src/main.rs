#[derive(Debug)]
enum Errors {}

const INPUT: &str = include_str!("../../input/02");

// Observation:
// Strings are the same size so solution to this problem is
// the hamming distance == 1 between the strings.
//
// If the strings were not evenly size we would have to use
// the Levenshtein distance instead.
fn solution(input: &str, input_cpy: &str) -> Option<String> {
    for x in input.lines() {
        for y in input_cpy.lines() {
            let dist = hamming_distance(x, y);

            if dist == 1 {
                println!("x: {}, y: {}, len: {}", x, y, dist);
                return Some(common_letters(x, y));
            }
        }
    }

    None
}

fn hamming_distance(this: &str, that: &str) -> usize {
    this.chars()
        .zip(that.chars())
        .fold(0, |acc, (l, r)| if l != r { acc + 1 } else { acc })
}

// Similar to the hamming distance above, but filters out the character which is different.
fn common_letters(this: &str, that: &str) -> String {
    this.chars()
        .zip(that.chars())
        .filter(|(l, r)| *l == *r)
        .map(|(l, _r)| l)
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

fn main() -> Result<(), Errors> {
    println!("Solution: {}", solution(INPUT, INPUT).unwrap());

    Ok(())
}

#[test]
fn hamming_test_1() {
    assert_eq!(1, hamming_distance("lalala", "lalbla"))
}

#[test]
fn hamming_test_2() {
    assert_eq!(6, hamming_distance("alotofchars", "alotfosrahc"))
}

#[test]
fn solution_test() {
    let test_input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    assert_eq!("fgij", &solution(test_input))
}
