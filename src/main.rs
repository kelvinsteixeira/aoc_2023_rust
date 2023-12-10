mod day3;

use std::time::Instant;

pub fn main() {

    //DAY 3

    // PART 1
    let start = Instant::now();
    let res_pt1 = day3::part1::run();
    println!("Part 1: {:?} ({:?})", res_pt1, start.elapsed());

    // PART 2
    let start2 = Instant::now();
    let res_pt2 = day3::part2::run();
    println!("Part 2: {:?} ({:?})", res_pt2, start2.elapsed());
}
