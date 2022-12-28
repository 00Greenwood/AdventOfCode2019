use crate::days::day_one;
use crate::utilities::load_input::load_input;

mod days;
mod utilities;

fn main() {
    std::thread::spawn(run_day_one);
}

fn run_day_one() {
    let input = load_input("day_one", false);
    let part_one = day_one::solve_part_one(&input);
    let part_two = day_one::solve_part_two(&input);
    println!("Day One -> Part One -> {part_one}\nDay One -> Part Two -> {part_two}");
}
