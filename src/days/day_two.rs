use crate::utilities::code_computer;

fn parse_input(input: &str) -> Vec<usize> {
    let mut values = input.split(',');

    let mut output: Vec<usize> = Vec::new();
    loop {
        let value = values.next();
        if value.is_none() {
            break;
        }

        output.push(str::parse(value.unwrap()).unwrap());
    }

    output
}

pub fn solve_part_one(input: &str) -> usize {
    let mut codes = parse_input(input);

    // Change initial values to 12 and 2!
    codes[1] = 12;
    codes[2] = 2;

    code_computer::run_codes(&mut codes);

    codes[0]
}

pub fn solve_part_two(input: &str) -> usize {
    let codes = parse_input(input);

    let mut noun = 0;
    let mut verb = 0;

    'outer_loop: for i in 0..=99 {
        for j in 0..=99 {
            let mut copy_codes = codes.clone();

            // Change initial values to i and j!
            copy_codes[1] = i;
            copy_codes[2] = j;

            code_computer::run_codes(&mut copy_codes);

            if copy_codes[0] == 19690720 {
                noun = i;
                verb = j;
                break 'outer_loop;
            }
        }
    }

    100 * noun + verb
}

#[cfg(test)]
mod tests {
    use crate::days::day_two::{solve_part_one, solve_part_two};
    use crate::utilities::load_input::load_input;

    #[test]
    fn part_one_test() {
        let test_input = load_input("day_two", true);
        let test_result = solve_part_one(&test_input);
        assert_eq!(test_result, 100);
    }

    #[test]
    fn part_one() {
        let input = load_input("day_two", false);
        let result = solve_part_one(&input);
        assert_eq!(result, 3224742);
    }

    #[test]
    fn part_two_test() {
        let test_input = load_input("day_two", true);
        let test_result = solve_part_two(&test_input);
        assert_eq!(test_result, 0);
    }

    #[test]
    fn part_two() {
        let input = load_input("day_two", false);
        let result = solve_part_two(&input);
        assert_eq!(result, 7960);
    }
}
