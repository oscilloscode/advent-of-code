use log::{debug, info, trace};

const BIRTH_COUNT: u8 = 8;
const DAY_COUNT: usize = 256;

// Improvement ideas:
// - Is the "chars" iterator double ended? If yes, don't put all nums in Vec but
// simply call next on both front and back until one digit is found each. Don't
// forget about handling one or no digits.
pub fn part1(implementation: u8, input: &str) -> usize {
    match implementation {
        0 | 1 => part1_implementation1(input),
        _ => panic!("No implementation {} for part 1", implementation),
    }
}

// Parsing improvement:
// chars().filter(|c| c.is_ascii_numeric()).to_digit().collect()

fn part1_implementation1(input: &str) -> usize {
    info!("part1_implementation1");

    let mut fish = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    debug!("Parsed Vec: {fish:?}");

    for i in 1..=DAY_COUNT {
        let mut baby_count = 0;
        for f in &mut fish {
            if *f == 0 {
                *f = 6;
                baby_count += 1;
            } else {
                *f -= 1;
            }
        }

        fish.extend(std::iter::repeat(BIRTH_COUNT).take(baby_count));

        trace!("After {i} days: {fish:?}");
    }
    fish.len()
}

pub fn part2(implementation: u8, input: &str) -> usize {
    match implementation {
        0 | 1 => part2_implementation1(input),
        _ => panic!("No implementation {} for part 2", implementation),
    }
}

fn part2_implementation1(input: &str) -> usize {
    info!("part2_implementation1");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_ones() {
        assert_eq!(2 + 2, 4);
    }
}
