fn parse_input(input: String) -> Vec<usize> {
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

pub fn solve_part_one(input: String) -> usize {
    let masses = parse_input(input);

    let mut fuel = 0;
    for mass in masses {
        fuel += (mass / 3) - 2;
    }

    fuel
}
