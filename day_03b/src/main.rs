use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("../../input/03");

type ID = u32;
type Position = (u32, u32);

fn solution(input: &str) -> u32 {
    // would it be cleaner to use regex?
    const SPLIT_ON: fn(char) -> bool = |c| {
        c == '#'
            || c == ' '
            || c == '@' // -> leaves two times  empty str "" (because split only works on char, not str)
            || c == ','
            || c == ':' // -> leaves one time empty str ""
            || c == 'x'
    };

    fn inputs(input: &str) -> Vec<u32> {
        // example input:
        // #870 @ 923,246: 28x23
        input
            .split(SPLIT_ON)
            .filter(|&s| s != "")
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }

    let lines = input
        .lines()
        .map(|line| inputs(line))
        .collect::<Vec<Vec<u32>>>();

    // fabric is at least 1000*1000; we might not allocate that many units though...
    // how much capacity should we sensibly initialize TODO
    let capacity = lines.len();
    let mut fabric: HashMap<Position, ID> = std::collections::HashMap::with_capacity(capacity);
    let mut overlap: HashSet<ID> = std::collections::HashSet::with_capacity(capacity);

    // there might be a faster way (this works but is rather naive)?
    // a start would be allocating less?
    for line in lines {
        let id = line[0];

        let x_from = line[1];
        let x_to = line[1] + line[3];

        let y_from = line[2];
        let y_to = line[2] + line[4];

        // for each tile (each square inch), find if there is an overlap
        // if for a tile t, t > 1, there is an overlap
        for x in x_from..x_to {
            for y in y_from..y_to {
                let get = fabric.get(&(x, y));

                match get {
                    Some(_) => {
                        overlap.insert(id);
                        overlap.insert(*get.unwrap());
                    }
                    None => {
                        fabric.insert((x, y), id);
                    }
                };
            }
        }
    }

    let values = HashSet::from_iter(fabric.values().into_iter().map(|v| *v));
    *values.difference(&overlap).last().unwrap()
}

fn main() {
    let sol = solution(INPUT);

    println!("{:?}", sol);
}
