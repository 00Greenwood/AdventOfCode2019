use crate::utilities::{
    line::Line,
    point::{Point, ORIGIN},
};

type Wire = Vec<Line>;

fn create_wire(input: &str) -> Wire {
    let mut wire: Wire = Wire::new();

    for instruction in input.split(',') {
        let direction = instruction.chars().next().unwrap();
        let distance = str::parse::<isize>(&instruction[1..]).unwrap();
        let last_end = wire.last().map(|line| &line.end).unwrap_or(&ORIGIN);

        match direction {
            'R' => wire.push(Line {
                start: last_end.clone(),
                end: Point {
                    x: last_end.x + distance,
                    y: last_end.y,
                },
            }),
            'D' => wire.push(Line {
                start: last_end.clone(),
                end: Point {
                    x: last_end.x,
                    y: last_end.y - distance,
                },
            }),
            'L' => wire.push(Line {
                start: last_end.clone(),
                end: Point {
                    x: last_end.x - distance,
                    y: last_end.y,
                },
            }),
            'U' => wire.push(Line {
                start: last_end.clone(),
                end: Point {
                    x: last_end.x,
                    y: last_end.y + distance,
                },
            }),
            _ => panic!("Unexpected Direction!"),
        }
    }

    wire
}

fn find_intersections(first_wire: &Wire, second_wire: &Wire) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();

    for first_line in first_wire {
        for second_line in second_wire {
            if first_line.intersect(second_line) {
                if first_line.is_vertical() && second_line.is_horizontal() {
                    intersections.push(Point {
                        x: first_line.start.x,
                        y: second_line.start.y,
                    });
                } else if first_line.is_horizontal() && second_line.is_vertical() {
                    intersections.push(Point {
                        x: second_line.start.x,
                        y: first_line.start.y,
                    });
                } else {
                    // Only happens at the origin and should only happen once!
                    intersections.push(ORIGIN);
                }
            }
        }
    }

    intersections
}

fn count_steps_to_intersection(wire: &Wire, intersection: &Point) -> usize {
    let mut steps = 0;
    for line in wire {
        if line.on_line(&intersection) {
            steps += line.start.manhattan(&intersection);
            break;
        }
        steps += line.length();
    }
    steps
}

fn parse_input(input: &str) -> (Wire, Wire) {
    let mut lines = input.lines();

    // Should only be two wires!
    let first = create_wire(lines.next().unwrap());
    let second = create_wire(lines.next().unwrap());

    (first, second)
}

pub fn solve_part_one(input: &str) -> usize {
    let wires = parse_input(input);
    let first_wire = wires.0;
    let second_wire = wires.1;
    let intersections = find_intersections(&first_wire, &second_wire);

    let mut distances: Vec<usize> = Vec::new();

    for intersection in &intersections {
        distances.push(ORIGIN.manhattan(intersection));
    }

    distances.sort();

    // The first index will always be 0, i.e. the origin!
    distances[1]
}

pub fn solve_part_two(input: &str) -> usize {
    let wires = parse_input(input);
    let first_wire = wires.0;
    let second_wire = wires.1;
    let intersections = find_intersections(&first_wire, &second_wire);

    let mut steps: Vec<usize> = Vec::new();

    for intersection in &intersections {
        steps.push(
            count_steps_to_intersection(&first_wire, intersection)
                + count_steps_to_intersection(&second_wire, intersection),
        );
    }

    steps.sort();

    // The first index will always be 0, i.e. the origin!
    steps[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::load_input::load_input;

    #[test]
    fn part_one_test() {
        let test_input = load_input("day_three", true);
        let test_result = solve_part_one(&test_input);
        assert_eq!(test_result, 135);
    }

    #[test]
    fn part_one() {
        let input = load_input("day_three", false);
        let result = solve_part_one(&input);
        assert_eq!(result, 303);
    }

    #[test]
    fn part_two_test() {
        let test_input = load_input("day_three", true);
        let test_result = solve_part_two(&test_input);
        assert_eq!(test_result, 410);
    }

    #[test]
    fn part_two() {
        let input = load_input("day_three", false);
        let result = solve_part_two(&input);
        assert_eq!(result, 11222);
    }
}
