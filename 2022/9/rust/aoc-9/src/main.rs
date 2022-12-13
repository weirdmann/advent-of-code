use std::{collections::HashMap, fs};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut head_position = (0,0);
    let mut tail_position = (0,0);
    let mut visited : HashMap<(i32,i32), i32> = HashMap::new();
    visited.insert(tail_position.clone(), 0);

    for line in lines {
        let m = applyMove(line);
        //println!("{:?}", &m);
        for i in 1..=m.2 {
            let new_head_position = (&head_position.0 + &m.0, &head_position.1 + &m.1);
            let head_tail_difference = (&new_head_position.0 - &tail_position.0, &new_head_position.1 - &tail_position.1);
            if head_tail_difference.0.abs().gt(&1) || head_tail_difference.1.abs().gt(&1)  {
                //println!("move tail: {:?}", &head_tail_difference);
                tail_position = head_position.clone();
                match visited.insert(tail_position.clone(), 0) {
                    Some(_) => {},//println!("new"),
                    None => {}//println!("old")
                };
            }// } else {
            //     if &new_head_position == &tail_position {
            //         println!("covers");
            //     }
            //     //println!("no change: {:?}{:?}", &new_head_position, &tail_position);
            // }
            head_position = new_head_position;
        }

    }

    println!("answer1: {}", visited.len())
}

fn applyMove(mv : &str) -> (i32,i32, i32) {
    let moves : HashMap<&str, (i32, i32)> = HashMap::from([
        ("U", (0,1)),
        ("D", (0,-1)),
        ("L", (-1,0)),
        ("R", (1,0)),
    ]);

    let split = mv.split(' ').collect::<Vec<&str>>();
    let direction = split[0];
    let amount : i32 = split[1].parse().unwrap();

    match moves.get(direction) {
        Some(m) => { (m.0, m.1, amount) },
        None => { panic!("Unknown movement: {}", direction) },
    }

}
