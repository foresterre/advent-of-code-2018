use std::collections::HashMap;

use ring_queue::Ring;

const INPUT: &str = include_str!("../../input/09");

fn parse_input(input: &str) -> (usize, usize) {
    let inputs = input.split(" ").collect::<Vec<&str>>();

    let mut out = inputs.iter().filter_map(|v| v.parse().ok());

    (out.next().unwrap(), out.next().unwrap())
}

fn solution(players: usize, points: usize) -> usize {
    // keep the head (end) of the queue as the current
    // rotate around the head
    let mut ring: Ring<usize> = Ring::with_capacity(points);

    // player -> score
    let mut scores: HashMap<usize, usize> = HashMap::with_capacity(players);

    // assuming always at least two turns
    ring.push(0);
    ring.push(1);

    for marble in 2..points + 1 {
        if marble % 23 == 0 {
            ring.rotate(7);
            let marble7 = ring.pop();
            ring.rotate(-1);

            // up score
            if let Some(m7) = marble7 {
                *scores.entry(marble % players).or_insert(0) += marble + m7;
            } else {
                panic!("Unable to rot 7.");
            }
        } else {
            ring.rotate(-1);
            ring.push(marble);
        }

        //        println!("ring: {:?}", ring);
    }

    let mut maximum = 0;
    for (_k, v) in scores {
        if v > maximum {
            maximum = v;
        }
    }

    // why does *v with the max() below not work?
    //    let (_k, v) = scores.iter().max().unwrap();
    maximum
}

fn main() {
    let (players, points) = parse_input(INPUT);
    println!("{}, {}", players, points);
    let sol = solution(players, points);
    println!("solution: {}", sol);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (ex_players, ex_points) = (9, 25);
        let expected: usize = 32;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

    #[test]
    fn test_example_1() {
        let (ex_players, ex_points) = (10, 1618);
        let expected: usize = 8317;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

    #[test]
    fn test_example_2() {
        let (ex_players, ex_points) = (13, 7999);
        let expected: usize = 146373;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

    #[test]
    fn test_example_3() {
        let (ex_players, ex_points) = (17, 1104);
        let expected: usize = 2764;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

    #[test]
    fn test_example_4() {
        let (ex_players, ex_points) = (21, 6111);
        let expected: usize = 54718;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

    #[test]
    fn test_example_5() {
        let (ex_players, ex_points) = (30, 5807);
        let expected: usize = 37305;

        assert_eq!(expected, solution(ex_players, ex_points));
    }

}
