fn parse_input(input: &String) -> Vec<usize> {
    let mut lines = input.lines();

    let mut output: Vec<usize> = Vec::new();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let mass = str::parse::<usize>(line.unwrap()).unwrap();
        output.push(mass);
    }

    output
}

pub fn solve_part_one(input: &String) -> usize {
    let masses = parse_input(input);

    let mut fuel = 0;
    for mass in masses {
        fuel += (mass / 3) - 2;
    }

    fuel
}

pub fn solve_part_two(input: &String) -> usize {
    let masses = parse_input(input);

    let mut total_fuel = 0;
    for mass in masses {
        let mut fuel: usize = (mass / 3) - 2;
        total_fuel += fuel;
        while fuel / 3 > 2 {
            fuel = (fuel / 3) - 2;
            total_fuel += fuel;
        }
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use crate::days::day_one::{solve_part_one, solve_part_two};
    use crate::utilities::load_input::load_input;

    #[test]
    fn part_one_test() {
        let test_input = load_input("day_one", true);
        let test_result = solve_part_one(&test_input);
        assert_eq!(test_result, 34241);
    }

    #[test]
    fn part_one() {
        let input = load_input("day_one", false);
        let result = solve_part_one(&input);
        assert_eq!(result, 3256599);
    }

    #[test]
    fn part_two_test() {
        let test_input = load_input("day_one", true);
        let test_result = solve_part_two(&test_input);
        assert_eq!(test_result, 51316);
    }

    #[test]
    fn part_two() {
        let input = load_input("day_one", false);
        let result = solve_part_two(&input);
        assert_eq!(result, 4882038);
    }
}
