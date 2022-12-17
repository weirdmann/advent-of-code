use std::fs;

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let trimmed = contents.trim();

    let mut answer1 = 0;
    let mut answer2 = 0;

    for line in trimmed.lines() {
        let halves: Vec<&str> = line.split(',').collect();
        let range1: Vec<&str> = halves[0].split('-').collect();
        let range2: Vec<&str> = halves[1].split('-').collect();

        let range1start = range1[0].parse::<i32>().unwrap();
        let range1end = range1[1].parse::<i32>().unwrap();
        let range2start = range2[0].parse::<i32>().unwrap();
        let range2end = range2[1].parse::<i32>().unwrap();

        if range_contains(range1start, range1end, range2start, range2end) {
            answer1 = answer1 + 1;
        }

        if range_overlaps(range1start, range1end, range2start, range2end) {
            answer2 = answer2 + 1;
        }
    }

    println!("{answer1}");
    println!("{answer2}");
}

fn range_contains(r1s: i32, r1e: i32, r2s: i32, r2e: i32) -> bool {
    (r1s <= r2s && r1e >= r2e) || (r2s <= r1s && r2e >= r1e)
}

fn range_overlaps(r1s: i32, r1e: i32, r2s: i32, r2e: i32) -> bool {
    (r2s <= r1e && r2e >= r1s) || (r1s <= r2e && r1e >= r2s)
}
