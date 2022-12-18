extern crate queues;

use queues::*;
//use std::{rc::Rc};

fn got_bored(item : isize) -> isize {
    (item as f64 / 3f64).floor() as isize
}

fn main() {
    println!("Hello, world!");
    let mut business = MonkeyBusiness {
        monkeys: Vec::new(),
        monkey_amount: 8,
    };
    // 0
    business.monkeys.push(Monkey::new(
        Vec::from([99, 67, 92, 61, 83, 64, 98]),
        Box::new(|old| {
            // let new = ((old as f64 * 17f64) / 3f64).floor() as usize;
            let new = (old as u128 * 17) as usize;
            (
                match new % 3 {
                    0 => 4,
                    _ => 2,
                },
                new,
            )
        }),
    ));

    // 1
    business.monkeys.push(Monkey::new(
        Vec::from([78, 74, 88, 89, 50]),
        Box::new(|old| {
            // let new = ((old as f64 * 11f64) / 3f64).floor() as usize;
            let new =( old as u128 * 11 ) as usize;
            (
                match new % 5 {
                    0 => 3,
                    _ => 5,
                },
                new,
            )
        }),
    ));

    // 2
    business.monkeys.push(Monkey::new(
        Vec::from([98, 91]),
        Box::new(|old| {
            // let new = ((old as f64 + 4f64) / 3f64).floor() as usize;
            let new = old + 4;
            (
                match new % 2 {
                    0 => 6,
                    _ => 4,
                },
                new,
            )
        }),
    ));

    // 3
    business.monkeys.push(Monkey::new(
        Vec::from([59, 72, 94, 91, 79, 88, 94, 51]),
        Box::new(|old| {
            let new = (old as u128 * old as u128) as usize;
            (
                match new % 13 {
                    0 => 0,
                    _ => 5,
                },
                new,
            )
        }),
    ));

    // 4
    business.monkeys.push(Monkey::new(
        Vec::from([95, 72, 78]),
        Box::new(|old| {
            let new = old + 7;
            (
                match new % 11 {
                    0 => 7,
                    _ => 6,
                },
                new,
            )
        }),
    ));

    // 5
    business.monkeys.push(Monkey::new(
        Vec::from([76]),
        Box::new(|old| {
            let new = old + 8;
            (
                match new % 17 {
                    0 => 0,
                    _ => 2,
                },
                new,
            )
        }),
    ));

    // 6
    business.monkeys.push(Monkey::new(
        Vec::from([69, 60, 53, 89, 71, 88]),
        Box::new(|old| {
            let new = old  + 5;
            (
                match new % 19 {
                    0 => 7,
                    _ => 1,
                },
                new,
            )
        }),
    ));

    // 7
    business.monkeys.push(Monkey::new(
        Vec::from([72, 54, 63, 80]),
        Box::new(|old| {
            let new = old + 3;
            (
                match new % 7 {
                    0 => 1,
                    _ => 3,
                },
                new,
            )
        }),
    ));

    for _ in 0..10000 {
        business.round();
    }
    let mut max = 0;
    let mut last_max = 0;

    for (index, m) in business.monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times.",
            index, m.inspection_count
        );
        if m.inspection_count.ge(&max) {
            last_max = max.clone();
            max = m.inspection_count.clone();
        } else if m.inspection_count.ge(&last_max) {
            last_max = m.inspection_count.clone();
        }
    }

    println!("Answer: {} * {} = {}", max, last_max, max * last_max);
}

struct Monkey {
    items: Queue<usize>,
    operation: Box<dyn Fn(usize) -> (usize, usize)>,
    inspection_count: u64,
}

impl Monkey {
    fn new(items: Vec<usize>, op: Box<dyn Fn(usize) -> (usize, usize)>) -> Self {
        Self {
            items: {
                let mut q = Queue::new();
                for item in items {
                    q.add(item).expect("error adding to queue");
                }
                q
            },
            operation: op,
            inspection_count: 0,
        }
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    monkey_amount: usize,
}

impl MonkeyBusiness {
    fn round(&mut self) {
        let monkeys_amount: usize = self.monkeys.len();
        let mut for_monkey: Vec<Queue<usize>> = {
            let mut zero_vec: Vec<Queue<usize>> = Vec::with_capacity(monkeys_amount);
            for _ in 0..monkeys_amount {
                zero_vec.push(Queue::new());
            }
            zero_vec
        };
        {
            for index in 0..self.monkey_amount.clone() {
                let monkey = self.monkeys.get_mut(index).unwrap();
                loop {
                    match monkey.items.remove() {
                        Ok(item) => {
                            monkey.inspection_count += 1;
                            let (index, item) = (monkey.operation)(item);
                            for_monkey[index as usize]
                                .add(item)
                                .expect("error adding an item");
                            //self.monkeys.get_mut((monkey.operation)(item) as usize).unwrap().items.add(item).expect("error adding an item");
                        }
                        Err(_) => break,
                    }
                }

                for (index, queue) in for_monkey.iter_mut().enumerate() {
                    loop {
                        match queue.remove() {
                            Ok(item) => {
                                self.monkeys
                                    .get_mut(index)
                                    .unwrap()
                                    .items
                                    .add(item)
                                    .expect("error adding an item");
                            }
                            Err(_) => break,
                        }
                    }
                }
            }
        }
    }
}
