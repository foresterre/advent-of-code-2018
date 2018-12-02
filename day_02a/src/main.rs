#[derive(Debug)]
enum Errors {}

const INPUT: &str = include_str!("../../input/02");

// TODO?: refactor this monstrosity
fn solution(input: &str) -> usize {
    let p: Vec<(bool, bool)> = input
        .lines()
        .map(|line| {
            let chars = line.chars();
            let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

            for c in chars {
                let get = map.get(&c);

                match get {
                    Some(i) => map.insert(c, i + 1),
                    None => map.insert(c, 1),
                };
            }

            map
        })
        .map(|hm| {
            (
                hm.values().filter(|v| **v == 2).count() >= 1,
                hm.values().filter(|v| **v == 3).count() >= 1,
            )
        })
        .collect();

    let twos = p.iter().filter(|(l, _r)| *l).count();

    let threes = p.iter().filter(|(_l, r)| *r).count();

    twos * threes
}

fn main() -> Result<(), Errors> {
    println!("Solution: {}", solution(INPUT));

    Ok(())
}
