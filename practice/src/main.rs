//use practice::aoc_2025_01::read_rotations_from_file;
use practice::aoc_2025_02;
use practice::aoc_2025_03;

fn main() {
    
    // Day 01
    /*let answer = read_rotations_from_file("input/2025_01a.txt");
    println!("Answer: {:?}", answer.unwrap());
    
    let answer = read_rotations_from_file("input/2025_01b.txt");
    println!("Answer: {:?}", answer.unwrap());*/
    
    // Day 02
    //let answer = part1("input/2025_02b.txt");
    //println!("Answer: {:?}", answer.unwrap());

    // let start_time = std::time::Instant::now();    
    // let answer = aoc_2025_02::part2("input/2025_02b.txt");
    // println!("Time: {:?}", std::time::Instant::now() - start_time);
    // println!("Answer: {:?}", answer.unwrap());

    // Day 03
    let answer = aoc_2025_03::part1("input/2025_03a.txt");
    println!("Answer: {:?}", answer.unwrap());
}
