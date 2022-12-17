use std::{collections::HashMap, fs};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let header = lines.clone().take(10);

    for (i, line) in header.clone().into_iter().enumerate() {
        println!("{i}: {line}");
    }

    let moves = lines.skip(10);

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 1..=9 {
        stacks.insert(i, Vec::new());
    }

    for i in (0..8).rev() {
        let line = header.clone().nth(i).unwrap().to_string();

        for j in 0..=8 {
            let char = line.as_bytes()[1 + j * 4] as char;

            if char != ' ' {
                let index = j + 1;
                stacks.get_mut(&index).unwrap().push(char);
            }

            print!("{char}");
        }
        println!();
    }

    for j in 0..=8 {
        let index = j + 1;
        for char in stacks.get(&index).unwrap().iter() {
            print!("{char}");
        }
        println!();
    }

    for m in moves {
        apply_movement(&mut stacks, m);
    }
    //apply_movement(&mut stacks, "move 3 from 2 to 5");

    println!();
    for j in 0..=8 {
        let index = j + 1;
        for char in stacks.clone().get(&index).unwrap().iter() {
            print!("{char}");
        }
        println!();
    }
}

fn apply_movement(stacks: &mut HashMap<usize, Vec<char>>, move_description: &str) {
    let split: Vec<&str> = move_description.split(' ').collect();
    let amount = split[1].parse::<usize>().unwrap();
    let from = split[3].parse::<usize>().unwrap();
    let to = split[5].parse::<usize>().unwrap();

    // part 1:
    // for i in 1..=amount {
    // let payload = stacks.get_mut(&from).unwrap().pop().unwrap();
    // stacks.get_mut(&to).unwrap().push(payload);
    // }

    // part 2:
    let len = stacks.clone().get(&from).unwrap().len() - amount;

    let mut payload =
        stacks
        .get_mut(&from)
        .unwrap()
        .split_off(len);

    stacks.get_mut(&to).unwrap().append(&mut payload);
}
