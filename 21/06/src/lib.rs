use log::{debug, info, trace};

const BIRTH_COUNT: u8 = 8;
const RESET_COUNT: u8 = 6;

pub fn part1(implementation: u8, input: &str) -> usize {
    match implementation {
        1 => part1_implementation1(input),
        0 | 2 => part1_implementation2(input),
        _ => panic!("No implementation {} for part 1", implementation),
    }
}

// Parsing improvement:
// chars().filter(|c| c.is_ascii_numeric()).to_digit().collect()

fn part1_implementation1(input: &str) -> usize {
    info!("part1_implementation1");

    let mut line_iter = input.lines();
    let days = line_iter
        .next()
        .expect("Input contains no first line for amount of days")
        .parse::<usize>()
        .expect("Can't parse first line for amount of days");

    let mut fish = line_iter
        .next()
        .expect("Input contains no first line for amount of days")
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // let mut fish = line_iter
    //     .next()
    //     .expect("Input contains no first line for amount of days")
    //     .chars()
    //     .filter(|c| c.is_ascii_digit())
    //     .flat_map(|c| c.to_digit(10))
    //     .map(|d| d as u8)
    //     .collect::<Vec<u8>>();

    debug!("Parsed days: {days:?}");
    debug!("Parsed Vec: {fish:?}");

    for i in 1..=days {
        let mut baby_count = 0;
        for f in &mut fish {
            if *f == 0 {
                *f = RESET_COUNT;
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

// Calls single_fish function which calls itself recursively whenever a new fish is born.
// Takes approx. 70 - 80 minutes on home desktop, but no memory issues
fn part1_implementation2(input: &str) -> usize {
    info!("part1_implementation2");

    let mut line_iter = input.lines();
    let days = line_iter
        .next()
        .expect("Input contains no first line for amount of days")
        .parse::<usize>()
        .expect("Can't parse first line for amount of days");

    let mut fish = line_iter
        .next()
        .expect("Input contains no first line for amount of days")
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut amount = 0;

    for f in fish {
        amount += single_fish(f, days);
    }
    amount
}

fn single_fish(mut count: u8, mut days: usize) -> usize {
    debug!("single fish start: count {count} days {days}");

    let mut children = 0;
    if count > RESET_COUNT {
        if days >= (count - RESET_COUNT) as usize {
            days -= (count - RESET_COUNT) as usize;
            count = RESET_COUNT;
        } else {
            return 1;
        }
    }

    loop {
        if days >= ((count + 1) as usize) {
            days = days - ((count + 1) as usize);
            children += single_fish(BIRTH_COUNT, days);
            count = RESET_COUNT;
        } else {
            break;
        }
    }

    // println!("children: {children}");
    children + 1
}

// Just call part1 functions as the difference is only the amount of days.
pub fn part2(implementation: u8, input: &str) -> usize {
    match implementation {
        1 => part1_implementation1(input),
        0 | 2 => part1_implementation2(input),
        _ => panic!("No implementation {} for part 2", implementation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_fish_no_children() {
        assert_eq!(single_fish(3, 3), 1);
        assert_eq!(single_fish(7, 3), 1);
        assert_eq!(single_fish(8, 3), 1);
        assert_eq!(single_fish(0, 0), 1);
    }

    #[test]
    fn single_fish_children() {
        assert_eq!(single_fish(8, 9), 2);
        assert_eq!(single_fish(3, 4), 2);
        assert_eq!(single_fish(3, 10), 2);
        assert_eq!(single_fish(3, 11), 3);
        assert_eq!(single_fish(0, 8), 3);
    }
}
