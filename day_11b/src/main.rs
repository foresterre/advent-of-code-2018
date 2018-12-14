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

fn solution(serial: i32) -> String {
    const AMOUNT: usize = 300;

    let mut grid = [[0; AMOUNT]; AMOUNT];

    for y in 0..AMOUNT {
        for x in 0..AMOUNT {
            grid[y][x] = total_power(x, y, serial);
        }
    }

    let mut max_sum = 0;
    let mut top_coord = (0, 0);

    // measure completion in %
    // and the duration per additional loop
    let mut completion = 0;
    let duration_total = std::time::Instant::now();

    // Hoped rayon could par_iter on the outer loop, but range iter has no par iters implemented...
    // Let's just brute force it... whew. This is not pretty, but it works...
    // measure => TotalSeconds      : 59.3972272
    (1..AMOUNT + 1).for_each(|cell_size| {
        let duration_loop = std::time::Instant::now();

        (0..AMOUNT - cell_size).for_each(|y| {
            (0..AMOUNT - cell_size).for_each(|x| {
                // This cell size loop is a pain with map and fold/sum.
                // In retrospect it would've been better to use for, for all loops.
                let mut sum = 0;

                for iy in 0..cell_size {
                    for ix in 0..cell_size {
                        sum += grid[y + iy][x + ix];
                    }
                }

                if sum > max_sum {
                    max_sum = sum;
                    top_coord = (x, y);
                }
            })
        });

        // display progress:
        if cell_size % 3 == 0 {
            completion += 1;
            let elapsed = duration_loop.elapsed();
            println!(
                "{}%, last loop took roughly (+-): {}s and {}ms",
                completion,
                elapsed.as_secs(),
                elapsed.subsec_millis()
            );
        }
    });

    let elapsed = duration_total.elapsed();
    println!(
        "Total time elapsed (guess): {}s and {}ms",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    format!("{},{},{}", top_coord.0, top_coord.1, max_sum)
}

fn main() {
    let serial = parse_input(INPUT);
    let sol = solution(serial);
    println!("solution: {:?}", sol);
}
