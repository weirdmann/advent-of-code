use std::{fs, collections::HashMap};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let header = lines.clone().take(10);


    for (i, line) in header.clone().into_iter().enumerate() {
        println!("{i}: {line}");
    }

    let moves = lines.skip(10);

    let mut stacks : HashMap<usize, Vec<char>> = HashMap::new();
    for i in 1..=9 {
        stacks.insert(i, Vec::new());
    }

    for i in (0..8).rev() {

        let mut line = header.clone().nth(i).unwrap().to_string();
        
        for j in (0..=8) {
            let char = line.as_bytes()[1 + j * 4] as char;

            if char !=' ' {
                let index = j + 1;
                stacks.get_mut(&index).unwrap().push(char);
            }

            print!("{char}");
        }
        println!();
    }

    for j in (0..=8) {
        let index = j + 1;
        for char in stacks.get(&index).unwrap().iter() {
            print!("{char}");
        }
        println!();
    }
    
}


