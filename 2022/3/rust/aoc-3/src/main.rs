
use std::fs;

fn main() {
    println!("Hello, world!");

    let file_path = "../../puzzle_input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        
    
    let trimmed = contents.trim();
    
    

    let mut priority_sum: i32 = 0;
    for line in trimmed.split("\n") {
        let line_length = line.len();
        assert!(line_length % 2 == 0);

        let halves = line.split_at(line_length / 2);
        assert_eq!(halves.0.len(), halves.1.len());

        for char1 in halves.0.chars() {
            let mut char_found = false;
            for char2 in halves.1.chars() {
                if char1 == char2 {
                    let priority = item_priority(char1);// - 0x48;
                    priority_sum = priority_sum + priority as i32;
                    println!("{line} : {priority}");
                    char_found = true;
                    break;
                }
            }
            if char_found {
                break;
            }
        }
    }
    println!("Puzzle answer 1: {priority_sum}");

}

fn item_priority(item: char) -> u8 {
    let b = item as u8;
    if (b >= 97) {
        b - 96        
    } else {
        b - 64 + 26
    }
}