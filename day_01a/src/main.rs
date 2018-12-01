#[derive(Debug)]
enum Errors {}

const INPUT: &str = include_str!("../../input/01");

fn main() -> Result<(), Errors> {
    let solution = INPUT
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .fold(0, |acc, x| acc + x);

    println!("Solution: {}", solution);

    Ok(())
}
