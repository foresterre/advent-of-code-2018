use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/04");

type GuardID = usize;
// At first I stored ranges instead, but this is much simpler when the guards sleepiest moment has to be found.
type SleepRecord = [usize; 60];
// GuardID + Moment of sleeping (minute)
type SleepyMoment = Option<(GuardID, usize)>;
// GuardID + Minute + Frequency
type SleepFrequency = Option<(GuardID, usize, usize)>;

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
    let mut most_frequent: SleepFrequency = None;
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
                        if most_frequent != None && entry[i] + 1 > most_frequent.unwrap().2 {
                            most_frequent = Some((current_guard.unwrap().0, i, entry[i] + 1));
                        }
                        if most_frequent == None {
                            most_frequent = Some((current_guard.unwrap().0, i, entry[i] + 1));
                        }

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

    let (id, min, freq) = most_frequent.unwrap();
    let solution = id * min;

    println!(
        "Guard #{}, sleeps the most on a single minute: {}x (this was on minute: {}).",
        id, freq, min
    );
    println!("This gives us: {} as answer to this exercise.", solution);

    solution
}

fn main() {
    solution(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repose_record_part_2() {
        assert_eq!(32070, solution(INPUT));
    }

    #[test]
    fn example_given() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(4455, solution(input));
    }
}
