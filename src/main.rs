mod day_01;
mod day_02;
mod day_03;

mod utils;

fn main() {
    println!("Day 1 (part 1): {answer}", answer = day_01::answer_part_1());
    println!("Day 1 (part 2): {answer}", answer = day_01::answer_part_2());
    println!("Day 2 (part 1): {answer}", answer = day_02::answer_part_1());
    println!("Day 2 (part 2): {answer}", answer = day_02::answer_part_2());
    println!("Day 3 (part 1): {answer}", answer = day_03::answer_part_1());
    println!("Day 3 (part 2): {answer}", answer = day_03::answer_part_2());
}
