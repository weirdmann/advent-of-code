use std::{collections::HashMap, fs};

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    // let mut head_position = (0, 0);
    // let mut tail_position = (0, 0);
    // let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    // visited.insert(tail_position.clone(), 0);

    // for line in lines.clone() {
    //     let m = parse_move(line);
    //     for _ in 1..=m.2 {
    //         let new_head_position = (&head_position.0 + &m.0, &head_position.1 + &m.1);
    //         let head_tail_difference = (
    //             &new_head_position.0 - &tail_position.0,
    //             &new_head_position.1 - &tail_position.1,
    //         );
    //         if head_tail_difference.0.abs().gt(&1) || head_tail_difference.1.abs().gt(&1) {
    //             tail_position = head_position.clone();
    //             match visited.insert(tail_position.clone(), 0) {
    //                 Some(_) => {}
    //                 None => {}
    //             }
    //         }
    //         head_position = new_head_position;
    //     }
    // }
    // let answer1 = &visited.len();
    // println!("answer1: {}", answer1);

    let mut head = Knot::new();
    for _ in 1..=9 {
        head.add_knot();
    }
    for line in lines.clone() {
        //println!("{}", line);
        head.apply_move(line);
    }
    println!("{}", head.get_visited());
}

fn parse_move(mv: &str) -> (i32, i32, i32) {
    let moves: HashMap<&str, (i32, i32)> =
        HashMap::from([("U", (0, 1)), ("D", (0, -1)), ("L", (-1, 0)), ("R", (1, 0))]);

    let split = mv.split(' ').collect::<Vec<&str>>();
    let direction = split[0];
    let amount: i32 = split[1].parse().unwrap();

    match moves.get(direction) {
        Some(m) => (m.0, m.1, amount),
        None => {
            panic!("Unknown movement: {}", direction)
        }
    }
}

struct Knot {
    position: Position,
    visited: HashMap<(i32, i32), bool>,
    next: Option<Box<Knot>>,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            position: Position { x: 0, y: 0 },
            visited: HashMap::from([((0, 0), true)]),
            next: None,
        }
    }

    fn add_knot(&mut self) {
        match self.next.as_mut() {
            None => {
                self.next = Some(Box::new(Knot::new()));
            }
            Some(next) => {
                next.add_knot();
            }
        }
    }

    fn get_visited(&self) -> String {
        match &self.next {
            None => format!("{}", self.visited.len()),
            Some(next) => format!("{},\n{}", self.visited.len(), next.get_visited()),
        }
    }

    fn apply_move(&mut self, line: &str) {
        let m = parse_move(line);

        for _ in 1..=m.2 {
            let new_head_position = Position {
                x: &self.position.x + &m.0,
                y: &self.position.y + &m.1,
            };

            match self.next.as_mut() {
                Some(next) => next.follow(&new_head_position.clone(), &self.position.clone()),
                None => {}
            }
            self.position.mv(&new_head_position);
            self.visited.insert(self.position.as_tuple(), false);

            // match self.next.as_mut() {
            //     None => self.position.mv(&new_head_position),
            //     Some(next) => {
            //         let head_tail_difference = Position {
            //             x: &new_head_position.x - &next.position.x,
            //             y: &new_head_position.y - &next.position.y,
            //         };
            //         if head_tail_difference.x.abs().gt(&1) || head_tail_difference.y.abs().gt(&1) {
            //             next.position.mv(&self.position);
            //             match self.visited.insert(self.position.as_tuple(), false) {
            //                 Some(_) => {}
            //                 None => {}
            //             }
            //         }
            //         self.position.mv(&new_head_position);
            //     }
            // }
        }
    }

    fn follow(&mut self, new_head_position: &Position, old_head_position: & Position) {
        let head_tail_difference = Position {
            x: &new_head_position.x - &self.position.x,
            y: &new_head_position.y - &self.position.y,
        };

        //println!("x:{} y:{}", &head_tail_difference.x, &head_tail_difference.y);

        if head_tail_difference.x.abs().gt(&1) || head_tail_difference.y.abs().gt(&1) {
            match self.next.as_mut() {
                Some(next) => {
                    next.follow(old_head_position, &self.position.clone());
                }
                None => {}
            }

            self.position.mv(&old_head_position);
            self.visited.insert(self.position.as_tuple(), false);
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn mv(&mut self, pos: &Position) {
        self.x = pos.x;
        self.y = pos.y;
    }
}
