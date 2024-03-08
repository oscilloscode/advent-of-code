// Improvement ideas:
// - Is the "chars" iterator double ended? If yes, don't put all nums in Vec but
// simply call next on both front and back until one digit is found each. Don't
// forget about handling one or no digits.
pub fn part1(input: &str) -> u32 {
    let mut nums: Vec<char> = Vec::with_capacity(32);
    let mut calibration_sum = 0;

    for line in input.lines() {
        println!("line: {}", line);
        nums.clear();
        for c in line.chars() {
            println!("{}", c);
            if c.is_digit(10) {
                nums.push(c);
            }
        }
        println!("nums: {:?}", nums);
        let calibration_value =
            10 * nums[0].to_digit(10).unwrap() + nums.last().unwrap().to_digit(10).unwrap();
        println!("calibration value: {}", calibration_value);
        calibration_sum += calibration_value;
    }

    calibration_sum
}

pub fn part2(input: &str) -> u32 {
    let marked_input = mark_words_with_digits(input);
    part1(&marked_input)
}

fn mark_words_with_digits(input: &str) -> String {
    let mut marked_string = input.to_owned();

    if marked_string.starts_with("one") {}

    marked_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_ones() {
        assert_eq!(mark_words_with_digits("oionejsnonee"), "oi1nejsn1nee");
    }
}
