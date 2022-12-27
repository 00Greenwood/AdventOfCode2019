use crate::days::day_one;
use crate::utilities::load_file::load_file;

mod days;
mod utilities;

fn main() {
    let input = load_file(String::from(
        "C:/Projects/AdventOfCode2019/inputs/day_one.txt",
    ));
    let part_one = day_one::solve_part_one(input);
    println!("Day One -> Part One -> {part_one}");
}
