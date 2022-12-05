
use std::{fs, thread::current};

fn main() {
    
    let file_path = "../../puzzle_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
        
    
    let trimmed = contents.trim();
    
    

    let mut priority_sum_answer1: i32 = 0;
    let mut line_number: u16 = 0;
    let mut current_elf_group_line = String::from("");
    
    for line in trimmed.split("\n") {
        line_number = line_number + 1;
        let line_length = line.len();
        assert!(line_length % 2 == 0);

        let halves = line.split_at(line_length / 2);
        assert_eq!(halves.0.len(), halves.1.len());

        for char1 in halves.0.chars() {
            let mut char_found = false;
            for char2 in halves.1.chars() {
                if char1 == char2 {
                    let priority = item_priority(char1);// - 0x48;
                    priority_sum_answer1 = priority_sum_answer1 + priority as i32;
                    //println!("{line} : {priority}");
                    char_found = true;
                    break;
                }
            }
            if char_found {
                break;
            }
        }

        if line_number <= 3 {
            current_elf_group_line.push_str(line);
        }

        if line_number == 3 {
            line_number = 0;
            let collected_lines = current_elf_group_line;
            current_elf_group_line = String::from("");
            println!("{collected_lines}");
        }

    }
    println!("Puzzle answer part 1:\n{priority_sum_answer1}\n");




}

fn item_priority(item: char) -> u8 {
    let b = item as u8;
    if b >= 97 {
        b - 96        
    } else {
        b - 64 + 26
    }
}