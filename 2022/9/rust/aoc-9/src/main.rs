use std::{collections::HashMap, fs, hash::Hash};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut pos = (0,0);
    let mut visited : HashMap<(i32,i32), i32> = HashMap::new();

    for line in lines {
        let m = applyMove(line);
    }
}

fn applyMove(mv : &str) -> (i32,i32) {
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
        Some(m) => { (m.0 * amount, m.1 * amount) },
        None => { panic!("Unknown movement: {}", direction) },
    }

}
