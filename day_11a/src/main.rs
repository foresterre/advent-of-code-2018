const INPUT: &str = include_str!("../../input/11");

fn parse_input(input: &str) -> i32 {
    input.parse().unwrap()
}

fn hundred_digit(input: i32) -> i32 {
    if input >= 100 {
        (input / 100) % 10
    } else {
        0
    }
}

fn total_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;

    hundred_digit((rack_id * y as i32 + serial) * rack_id) - 5
}

fn solution(serial: i32) -> (usize, usize) {
    const AMOUNT: usize = 300;

    let mut grid = [[0; AMOUNT]; AMOUNT];

    for y in 0..AMOUNT {
        for x in 0..AMOUNT {
            grid[y][x] = total_power(x, y, serial);
        }
    }

    let mut max_sum = 0;
    let mut top_coord = (0, 0);

    for y in 1..AMOUNT - 2 {
        for x in 1..AMOUNT - 2 {
            let mut sum = 0;

            for iy in 0..3 {
                for ix in 0..3 {
                    sum += grid[y + iy][x + ix];
                }
            }

            if sum > max_sum {
                max_sum = sum;
                top_coord = (x, y);
            }
        }
    }

    println!(
        "best fuel cell (x,y,power): {},{},{}",
        top_coord.0, top_coord.1, max_sum
    );

    top_coord
}

fn main() {
    let serial = parse_input(INPUT);
    let sol = solution(serial);
    println!("solution: {:?}", sol);
}
