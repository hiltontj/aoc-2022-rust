mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

mod utils;

fn main() {
    println!("Day 1 (Part 1): {answer}", answer = day_01::answer_part_1());
    println!("Day 1 (Part 2): {answer}", answer = day_01::answer_part_2());
    println!("Day 2 (Part 1): {answer}", answer = day_02::answer_part_1());
    println!("Day 2 (Part 2): {answer}", answer = day_02::answer_part_2());
    println!("Day 3 (Part 1): {answer}", answer = day_03::answer_part_1());
    println!("Day 3 (Part 2): {answer}", answer = day_03::answer_part_2());
    println!("Day 4 (Part 1): {answer}", answer = day_04::answer_part_1());
    println!("Day 4 (Part 2): {answer}", answer = day_04::answer_part_2());
    println!("Day 5 (Part 1): {answer}", answer = day_05::answer_part_1());
    // println!("Day 5 (Part 2): {answer}", answer = day_05::answer_part_2());
}
