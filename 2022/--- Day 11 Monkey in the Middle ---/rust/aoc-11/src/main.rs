extern crate queues;
extern crate  num_bigint;
use queues::*;
use num_bigint::BigUint;
//use num_traits::{Zero};
//use std::{rc::Rc};


fn main() {
    println!("Hello, world!");
    let mut business = MonkeyBusiness {
        monkeys: Vec::new(),
        monkey_amount: 8,
    };
    // 0
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(99u32),
                       BigUint::from(67u32),
                       BigUint::from(92u32),
                       BigUint::from(61u32),
                       BigUint::from(83u32),
                       BigUint::from(64u32),
                       BigUint::from(98u32)
                       ]),
        Box::new(|old| {
            // let new = ((old as f64 * 17f64) / 3f64).floor() as u128;
            let new = old * 17u32;
            (
                    match (&new % BigUint::from(3u32)).to_string().eq("0") {
                    true => 4,
                    _ => 2,
                },
                new,
            )
        }),
    ));

    // 1
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(78u32),
                       BigUint::from(74u32),
                       BigUint::from(88u32),
                       BigUint::from(89u32),
                       BigUint::from(50u32)]),
        Box::new(|old| {
            // let new = ((old as f64 * 11f64) / 3f64).floor() as u128;
            let new = old * 11u32;
            (
                    match (&new % BigUint::from(5u32)).to_string().eq("0") {
                        true => 3,
                    _ => 5,
                },
                new,
            )
        }),
    ));

    // 2
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(98u32), BigUint::from(91u32)]),
        Box::new(|old| {
            // let new = ((old as f64 + 4f64) / 3f64).floor() as u128;
            let new = old + 4u32;
            (
                    match (&new % BigUint::from(2u32)).to_string().eq("0") {
                        true => 6,
                    _ => 4,
                },
                new,
            )
        }),
    ));

    // 3
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(59u32),
                       BigUint::from(72u32),
                       BigUint::from(94u32),
                       BigUint::from(91u32),
                       BigUint::from(79u32),
                       BigUint::from(88u32),
                       BigUint::from(94u32),
                       BigUint::from(51u32)]),
        Box::new(|old| {
            let new = &old * &old;
            (
                    match (&new % BigUint::from(13u32)).to_string().eq("0") {
                        true => 0,
                    _ => 5,
                },
                new,
            )
        }),
    ));

    // 4
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(95u32), BigUint::from(72u32), BigUint::from(78u32)]),
        Box::new(|old| {
            let new = old + 7u32;
            (
                    match (&new % BigUint::from(11u32)).to_string().eq("0") {
                        true => 7,
                    _ => 6,
                },
                new,
            )
        }),
    ));

    // 5
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(76u32)]),
        Box::new(|old| {
            let new = old + 8u32;
            (
                    match (&new % BigUint::from(17u32)).to_string().eq("0") {
                        true => 0,
                    _ => 2,
                },
                new,
            )
        }),
    ));

    // 6
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(69u32),
                       BigUint::from(60u32),
                       BigUint::from(53u32),
                       BigUint::from(89u32),
                       BigUint::from(71u32),
                       BigUint::from(88u32)]),
        Box::new(|old| {
            let new = old + 5u32;
            (
                    match (&new % BigUint::from(19u32)).to_string().eq("0") {
                    true => 7,
                    _ => 1,
                },
                new,
            )
        }),
    ));

    // 7
    business.monkeys.push(Monkey::new(
            Vec::from([BigUint::from(72u32),
                       BigUint::from(54u32),
                       BigUint::from(63u32),
                       BigUint::from(80u32)]),
        Box::new(|old| {
            let new = old + 3u32;
            (
                    match (&new % BigUint::from(7u32)).to_string().eq("0") {
                        true => 1,
                        _ => 3,
                },
                new,
            )
        }),
    ));

    for _ in 0..200 {
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
    items: Queue<BigUint>,
    operation: Box<dyn Fn(BigUint) -> (u128, BigUint)>,
    inspection_count: u64,
}

impl Monkey {
    fn new(items: Vec<BigUint>, op: Box<dyn Fn(BigUint) -> (u128, BigUint)>) -> Self {
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
        let mut for_monkey: Vec<Queue<BigUint>> = {
            let mut zero_vec: Vec<Queue<BigUint>> = Vec::with_capacity(monkeys_amount);
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
                            //self.monkeys.get_mut((monkey.operation)(item) as u128).unwrap().items.add(item).expect("error adding an item");
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
