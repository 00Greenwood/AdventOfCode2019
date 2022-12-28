use crate::days::{day_one, day_two};
use crate::utilities::load_input::load_input;

mod days;
mod utilities;

fn main() {
    let mut threads = Vec::new();

    threads.push(std::thread::spawn(run_day_one));
    threads.push(std::thread::spawn(run_day_two));

    for thread in threads {
        thread.join().unwrap();
    }
}

fn run_day_one() {
    let input = load_input("day_one", false);
    let part_one = day_one::solve_part_one(&input);
    let part_two = day_one::solve_part_two(&input);
    println!("Day 1:\n\tPart 1: {:?}\n\tPart 2: {:?}", part_one, part_two);
}

fn run_day_two() {
    let input = load_input("day_two", false);
    let part_one = day_two::solve_part_one(&input);
    let part_two = day_two::solve_part_two(&input);
    println!("Day 2:\n\tPart 1: {:?}\n\tPart 2: {:?}", part_one, part_two);
}
