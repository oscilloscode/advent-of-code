use log::{debug, info, trace};

// Improvement ideas:
// - Is the "chars" iterator double ended? If yes, don't put all nums in Vec but
// simply call next on both front and back until one digit is found each. Don't
// forget about handling one or no digits.
pub fn part1(implementation: u8, input: &str) -> u32 {
    match implementation {
        0 | 1 => part1_implementation1(input),
        _ => panic!("No implementation {} for part 1", implementation),
    }
}

fn part1_implementation1(input: &str) -> u32 {
    info!("part1_implementation1");

    let fish = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    println!("fish: {fish:?}");
    0
}

pub fn part2(implementation: u8, input: &str) -> u32 {
    match implementation {
        0 | 1 => part2_implementation1(input),
        _ => panic!("No implementation {} for part 2", implementation),
    }
}

fn part2_implementation1(input: &str) -> u32 {
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
