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
    println!("part2");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(2 + 3, 5, "works");
    }
}
