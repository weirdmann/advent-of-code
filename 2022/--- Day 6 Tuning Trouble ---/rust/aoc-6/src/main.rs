use std::{collections::HashMap, fs, process::exit};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let marker_length = 4;


    assert!(contents.chars().count() > marker_length);
    println!("Counting...");

    for i in (marker_length)..contents.chars().count() {
        let window = contents.get((i-marker_length + 1)..=i).unwrap();



        if is_a_marker(window) {
            let pos = i+1;
            println!("{pos}");
            exit(0);
        }
    }
}


fn is_a_marker(window : &str) -> bool {

    println!("{window}");
    for (i, ch) in window.chars().enumerate() {
        for j in 0..window.len() {
            let byte = window.as_bytes()[j] as u8;
            let char = ch as u8;
            if (byte == char ) & i.ne(&j) {
                return false;
            }
        }
    }
    println!("{window}");
    true
}
