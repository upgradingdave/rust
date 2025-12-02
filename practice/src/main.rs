use practice::aoc_2025_01::read_rotations_from_file;

fn main() {
    let answer = read_rotations_from_file("input/2025_01a.txt");
    println!("Answer: {:?}", answer.unwrap());
    
    let answer = read_rotations_from_file("input/2025_01b.txt");
    println!("Answer: {:?}", answer.unwrap());
}
