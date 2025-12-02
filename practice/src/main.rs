//use practice::aoc_2025_01::read_rotations_from_file;
use practice::aoc_2025_02;

fn main() {
    
    // Day 01
    /*let answer = read_rotations_from_file("input/2025_01a.txt");
    println!("Answer: {:?}", answer.unwrap());
    
    let answer = read_rotations_from_file("input/2025_01b.txt");
    println!("Answer: {:?}", answer.unwrap());*/
    
    // Day 02
    //let answer = part1("input/2025_02b.txt");
    //println!("Answer: {:?}", answer.unwrap());
    
    let answer = aoc_2025_02::part2("input/2025_02b.txt");
    println!("Answer: {:?}", answer.unwrap());

    //let i:u64 = 11;
    //let answer = aoc_2025_02::has_repetition(&i.to_string(), &2);
    //println!("Answer: {:?}", answer);
}
