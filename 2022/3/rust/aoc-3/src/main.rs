use std::fs;

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let trimmed = contents.trim();

    let mut priority_sum_answer1: i32 = 0;
    let mut priority_sum_answer2: i32 = 0;

    let mut elf_group_lines: Vec<String> = Vec::new();

    for line in trimmed.lines() {
        let trimmed_line = line.clone();

        let line_length = trimmed_line.len();
        assert!(line_length % 2 == 0);

        let halves = trimmed_line.split_at(line_length / 2);
        assert_eq!(halves.0.len(), halves.1.len());

        for char1 in halves.0.chars() {
            let mut char_found = false;
            for char2 in halves.1.chars() {
                if char1 == char2 {
                    let priority = item_priority(char1); // - 0x48;
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

        elf_group_lines.push(String::from(line));

        if elf_group_lines.len() >= 3 {
            let common_letter = get_common_letter(elf_group_lines);
            priority_sum_answer2 = priority_sum_answer2 + item_priority(common_letter) as i32;

            elf_group_lines = Vec::new();
        }
    }
    println!("Puzzle answer part 1:\n{priority_sum_answer1}\n");
    println!("Puzzle answer part 2:\n{priority_sum_answer2}\n");
}

fn item_priority(item: char) -> u8 {
    let b = item as u8;
    if b >= 97 {
        b - 96
    } else {
        b - 64 + 26
    }
}

fn get_common_letter(group: Vec<String>) -> char {
    for letter in group[0].chars() {
        if group[1].find(letter).is_some() {
            if group[2].find(letter).is_some() {
                return letter;
            }
        }
    }
    ' '
}
