use std::fs;

fn main() {
    let file_path = "puzzle_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    cpu.load_from_str(contents).expect("err");

    for line in crt.request_animation_frame() {
        println!("{}", line);
    }

    let mut answer1 = 0;
    loop {
        crt.advance(cpu.register.X);
        match cpu.advance() {
            Some(_) => {
                println!(
                    "clk: {:4?}, instr: {:?}, step: {:?}, regX: {:?}",
                    cpu.clock, cpu.current_instruction, cpu.step, cpu.register.X
                );
                match cpu.clock {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        answer1 += cpu.clock as isize * cpu.register.X;
                    }
                    _ => {}
                }
            }
            None => {
                println!("done: regX: {:?}, answer1 : {}", cpu.register.X, answer1);
                break;
            }
        };
    }

    for line in crt.request_animation_frame() {
        println!("{}", line);
    }
}

struct Registers {
    X: isize,
}

impl Registers {
    fn new() -> Self {
        Self { X: 1 }
    }
}

#[derive(Clone, Debug)]
enum Instructions {
    NOOP,
    ADDX(isize),
}

struct CPU {
    register: Registers,
    clock: usize,
    step: usize,
    program: Vec<Instructions>,
    new_program: Vec<Instructions>,
    program_counter: Option<usize>,
    current_instruction: Option<Instructions>,
}

impl CPU {
    fn new() -> Self {
        Self {
            clock: 0,
            step: 0,
            register: Registers::new(),
            program: Vec::new(),
            new_program: Vec::new(),
            program_counter: None,
            current_instruction: None,
        }
    }

    fn advance(&mut self) -> Option<()> {
        self.clock += 1;

        if self.program.len().eq(&0) {
            return None;
        }

        match &self.current_instruction {
            None => return self.next_instruction(),
            Some(instr) => match instr {
                Instructions::NOOP => {
                    return self.next_instruction();
                }
                Instructions::ADDX(a) => {
                    if self.step.ge(&1) {
                        self.register.X += a;
                        return self.next_instruction();
                    } else {
                        self.step += 1;
                    }
                }
            },
        }

        Some(())
    }

    fn next_instruction(&mut self) -> Option<()> {
        match &mut self.program_counter {
            None => self.program_counter = Some(0),
            Some(a) => {
                *a += 1;
            }
        }
        self.step = 0;
        match self.program.get(self.program_counter.unwrap()).cloned() {
            Some(val) => self.current_instruction = Some(val),
            None => return None,
        }

        Some(())
    }

    fn load_from_str(&mut self, prog: String) -> Result<&mut Self, String> {
        self.new_program = Vec::new();

        for line in prog.lines() {
            let split: Vec<&str> = line.split(" ").collect();
            let instruction_name = split[0];
            let mut instruction_argument: Option<isize> = None;
            match split.get(1) {
                Some(v) => instruction_argument = Some(v.parse().unwrap()),
                None => {}
            }

            match instruction_name.to_lowercase().as_str() {
                "noop" => self.new_program.push(Instructions::NOOP),
                "addx" => self
                    .new_program
                    .push(Instructions::ADDX(instruction_argument.unwrap())),
                _ => return Err(format!("unknown instruction: {}", line)),
            }
        }
        self.program.clear();
        self.program.append(&mut self.new_program);

        Ok(self)
    }
}

#[derive(Clone, Debug)]
struct Pixel<T> {
    value: T,
}

impl<T> Pixel<T> {
    fn new(v: T) -> Self {
        Self { value: v }
    }
}

struct CRT {
    width: usize,
    height: usize,
    frame_buffer: Vec<Pixel<bool>>,
    current_pixel_idx: isize,
}

impl CRT {
    fn new() -> Self {
        const WIDTH: usize = 40;
        const HEIGHT: usize = 6;
        Self {
            width: WIDTH,
            height: HEIGHT,
            frame_buffer: {
                let mut v = Vec::new();
                for _ in 1..=(WIDTH * HEIGHT) {
                    v.push(Pixel::new(false));
                }
                v
            },
            current_pixel_idx: 0,
        }
    }

    fn request_animation_frame(&mut self) -> Vec<String> {
        let mut animation_frame = Vec::new();

        for i in 1..=self.height {
            animation_frame.push(String::new());
        }

        let mut current_line = String::new();
        let mut idx: usize = 0;

        for (i, pixel) in self.frame_buffer.iter().enumerate() {
            let char = match pixel.value {
                true => '#',
                false => '.',
            };
            animation_frame[(i as f32 / self.width as f32).floor() as usize].push(char);
        }

        animation_frame
    }

    fn reset(&mut self) {
        self.current_pixel_idx = 0;
    }

    fn advance(&mut self, sprite_position: isize) {
        match self.frame_buffer.get_mut(self.current_pixel_idx as usize) {
            Some(pixel) => {
                pixel.value = (self.current_pixel_idx % self.width as isize) >= sprite_position
                    && (self.current_pixel_idx % self.width as isize) <= sprite_position + 2
            }
            None => {}
        }
        self.current_pixel_idx += 1;
    }
}
