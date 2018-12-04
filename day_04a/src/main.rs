use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/04");

type GuardID = usize;
// At first I stored ranges instead, but this is much simpler when the guards sleepiest moment has to be found.
type SleepRecord = [usize; 60];
type SleepyMoment = Option<(GuardID, usize)>;

// In retrospect, shouldn't have used so much tuples.
fn solution(input: &str) -> usize {
    // not the most efficient way to get this sorted, but very useful, and
    // to be honest with the amount of data input here, good enough :).
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines.sort();

    lazy_static! {
        static ref MINUTES_MATCH: Regex =
            Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:(\d{2})] (.+)").unwrap();
        static ref GUARD_ID_MATCH: Regex = Regex::new(r"Guard #(\d+)").unwrap();
    }

    // Current guard and its start time of sleep
    let mut current_guard: SleepyMoment = None;
    let mut guard_sleep_records: HashMap<GuardID, SleepRecord> = HashMap::new();

    for line in lines {
        for capture in MINUTES_MATCH.captures_iter(line) {
            let now = &capture[1]
                .parse::<usize>()
                .expect("My watch broke, oh noes!");
            let remnant = &capture[2];

            match remnant {
                "falls asleep" => {
                    current_guard = current_guard.map(|(id, _)| (id, *now));
                }
                "wakes up" => {
                    let entry = guard_sleep_records
                        .entry(current_guard.unwrap().0)
                        .or_insert([0; 60]);

                    // another minute at sleep
                    for i in current_guard.unwrap().1..*now {
                        entry[i] += 1;
                    }
                }
                look_a_guard => {
                    for cap in GUARD_ID_MATCH.captures_iter(look_a_guard) {
                        let guard = &cap[1].parse::<GuardID>().expect(
                            "Is this guard an intruder? Where's the ID? panic mode enabled.",
                        );
                        current_guard = Some((*guard, 0))
                    }
                }
            };
        }
    }

    // We stored all minutes per guard (actually for finding the sleepiest moment, which is the second thing we need to find a solution)
    // Now use that to find the guard with the guard with the most minutes.
    let (guard_id, time_asleep) = guard_sleep_records
        .iter()
        .map(|(k, v)| (k, v.iter().sum::<usize>()))
        .max_by_key(|(_k, v)| *v)
        .unwrap();

    // Now use those stored records to find the guards sleepiest moment.
    // That is, the moment for which the record array has the highest count.
    let sleepiest_at = guard_sleep_records[guard_id]
        .iter()
        .enumerate()
        .max_by_key(|(_k, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    let solution = guard_id * sleepiest_at;

    println!(
        "id#{}, slept: {} minutes which gives the answer: {}",
        guard_id, time_asleep, solution
    );

    solution
}

fn main() {
    solution(INPUT);
}
