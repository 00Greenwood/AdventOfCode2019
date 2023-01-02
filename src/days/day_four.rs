fn parse_input(input: &str) -> (usize, usize) {
    let mut numbers = input.split('-');

    // Should only be two numbers!
    let first = str::parse::<usize>(numbers.next().unwrap()).unwrap();
    let second = str::parse::<usize>(numbers.next().unwrap()).unwrap();

    (first, second)
}

fn get_digits(number: usize) -> Vec<usize> {
    let mut digits: Vec<usize> = Vec::new();
    let mut remainder = 0;
    for power in 0..=5 {
        let divisor = 10usize.pow(5 - power);
        digits.push(number / divisor - remainder / divisor);
        remainder += digits.last().unwrap() * divisor;
    }
    digits
}

fn is_valid_size_and_increasing(digits: &Vec<usize>) -> bool {
    // It is a six-digit number!
    if digits.len() != 6 {
        return false;
    }

    let mut increasing = true;
    for index in 0..5 {
        increasing &= digits[index] <= digits[index + 1]
    }

    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    if !increasing {
        return false;
    }

    true
}

pub fn solve_part_one(input: &str) -> usize {
    let numbers = parse_input(input);
    let first_number = numbers.0;
    let second_number = numbers.1;

    let mut count = 0;

    // The value is within the range given in your puzzle input!
    for number in first_number..=second_number {
        let digits = get_digits(number);

        if !is_valid_size_and_increasing(&digits) {
            continue;
        }

        // Two adjacent digits are the same (like 22 in 122345).
        let mut adjacent = false;
        for index in 0..5 {
            if digits[index] == digits[index + 1] {
                adjacent = true;
                break;
            }
        }
        if !adjacent {
            continue;
        }

        count += 1;
    }

    count
}

pub fn solve_part_two(input: &str) -> usize {
    let numbers = parse_input(input);
    let first_number = numbers.0;
    let second_number = numbers.1;

    let mut count = 0;

    // The value is within the range given in your puzzle input!
    for number in first_number..=second_number {
        let digits = get_digits(number);

        if !is_valid_size_and_increasing(&digits) {
            continue;
        }

        // Two adjacent digits are the same (like 22 in 122345).
        // But the two adjacent matching digits are not part of a larger group of matching digits.
        let mut adjacent = false;
        for index in 0..5 {
            if digits[index] == digits[index + 1] {
                if let Some(next_digit) = digits.get(index + 2) {
                    if *next_digit == digits[index] {
                        continue;
                    }
                }

                if index > 0 {
                    if let Some(previous_digit) = digits.get(index - 1) {
                        if *previous_digit == digits[index] {
                            continue;
                        }
                    }
                }

                adjacent = true;
                break;
            }
        }
        if !adjacent {
            continue;
        }

        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::load_input::load_input;

    #[test]
    fn part_one_test() {
        let test_input = load_input("day_four", true);
        let test_result = solve_part_one(&test_input);
        assert_eq!(test_result, 45);
    }

    #[test]
    fn part_one() {
        let input = load_input("day_four", false);
        let result = solve_part_one(&input);
        assert_eq!(result, 925);
    }

    #[test]
    fn part_two_test() {
        let test_input = load_input("day_four", true);
        let test_result = solve_part_two(&test_input);
        assert_eq!(test_result, 8);
    }

    #[test]
    fn part_two() {
        let input = load_input("day_four", false);
        let result = solve_part_two(&input);
        assert_eq!(result, 607);
    }
}
