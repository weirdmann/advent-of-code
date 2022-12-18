extern crate queues;
use std::{borrow::Borrow, collections::btree_map::IterMut, vec};

use queues::*;

fn main() {
    println!("Hello, world!");
    let mut business = MonkeyBusiness {
        monkeys: Vec::new(),
    };
    // 0
    business.monkeys.push(Monkey::new(
        Vec::from([99, 67, 92, 61, 83, 64, 98]),
        Box::new(|old| match (old * 17) % 3 {
            0 => 4,
            _ => 2,
        }),
    ));

    // 1
    business.monkeys.push(Monkey::new(
        Vec::from([78, 74, 88, 89, 50]),
        Box::new(|old| match (old * 11) % 5 {
            0 => 3,
            _ => 5,
        }),
    ));

    // 2
    business.monkeys.push(Monkey::new(
        Vec::from([98, 91]),
        Box::new(|old| match (old + 4) % 2 {
            0 => 6,
            _ => 4,
        }),
    ));

    // 3
    business.monkeys.push(Monkey::new(
        Vec::from([59, 72, 94, 91, 79, 88, 94, 51]),
        Box::new(|old| match (old * old) % 13 {
            0 => 0,
            _ => 5,
        }),
    ));

    // 4
    business.monkeys.push(Monkey::new(
        Vec::from([95, 72, 78]),
        Box::new(|old| match (old + 7) % 11 {
            0 => 7,
            _ => 6,
        }),
    ));

    // 5
    business.monkeys.push(Monkey::new(
        Vec::from([76]),
        Box::new(|old| match (old + 8) % 17 {
            0 => 0,
            _ => 2,
        }),
    ));

    // 6
    business.monkeys.push(Monkey::new(
        Vec::from([69, 60, 53, 89, 71, 88]),
        Box::new(|old| match (old + 5) % 19 {
            0 => 7,
            _ => 1,
        }),
    ));

    // 7
    business.monkeys.push(Monkey::new(
        Vec::from([72, 54, 63, 80]),
        Box::new(|old| match (old + 3) % 7 {
            0 => 1,
            _ => 3,
        }),
    ));
}


struct Monkey {
    items: Queue<isize>,
    operation: Box<dyn Fn(isize) -> isize>,
}

impl Monkey {
    fn new(items: Vec<isize>, op: Box<dyn Fn(isize) -> isize>) -> Self {
        Self {
            items: {
                let mut q = Queue::new();
                for item in items {
                    q.add(item).expect("error adding to queue");
                }
                q
            },
            operation: op,
        }
    }
}


struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn round(&mut self) {
        self.monkeys.iter_mut().for_each(|monkey| {
            loop {
                match monkey.items.remove() {
                    Ok(item) => {
                        self.monkeys[(monkey.operation)(item) as usize]
                            .items
                            .add(item);
                    }
                    Err(_) => break,
                }
            }
        });
        
    }
}
