use std::{cmp::max, fmt::Display};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Debug, Clone, Copy)]
enum Insn {
    Addx(i32),
    Noop,
}

impl Insn {
    fn duration(&self) -> u32 {
        match self {
            Insn::Addx(_) => 2,
            _ => 1,
        }
    }
}

fn compile(program: &str) -> crate::Result<Vec<Insn>> {
    let mut compiled = Vec::new();
    for line in program.lines() {
        compiled.push(match &line[..4] {
            "addx" => Insn::Addx(line[5..].parse()?),
            "noop" => Insn::Noop,
            _ => return Err(crate::Error::boxed(Error::InvalidInput)),
        });
    }
    Ok(compiled)
}

#[derive(Debug)]
struct Cpu {
    program: Vec<Insn>,
    ip: usize,
    x: i32,
    pipeline: (Insn, u32),
}

impl Cpu {
    fn new(program: Vec<Insn>) -> Self {
        Cpu {
            program,
            ip: 0,
            x: 1,
            pipeline: (Insn::Noop, 0),
        }
    }

    fn tick(&mut self, cycles: u32) -> bool {
        for _ in 0..cycles {
            if self.pipeline.1 == 0 {
                if self.ip >= self.program.len() {
                    return false;
                }

                let insn = self.program[self.ip];
                self.pipeline = (insn, insn.duration());
                self.ip += 1;
            }

            self.pipeline.1 -= 1;
            if self.pipeline.1 == 0 {
                match self.pipeline.0 {
                    Insn::Addx(imm) => self.x += imm,
                    Insn::Noop => (),
                }
            }
        }
        true
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let program = compile(input)?;
    let mut cpu = Cpu::new(program);

    cpu.tick(19);
    let mut cycle = 20; // "during"

    let mut sum = cycle * cpu.x;
    while cpu.tick(40) {
        cycle += 40;
        sum += cycle * cpu.x;
    }
    Ok(sum)
}

struct Crt {
    pixels: Vec<Vec<char>>,
}

impl Crt {
    fn new() -> Self {
        Crt {
            pixels: vec![vec!['#'; 40]; 6],
        }
    }

    fn tick(&mut self, cycle: usize, sprite_pos: i32) -> bool {
        let row = (cycle - 1) / 40;
        let col = (cycle - 1) % 40;
        if row > 5 {
            return false;
        }

        let from = max(0, sprite_pos - 1) as usize;
        let to = max(0, sprite_pos + 1) as usize;
        self.pixels[row][col] = if col >= from && col <= to { '#' } else { '.' };
        true
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part2(input: &str) -> crate::Result<String> {
    let program = compile(input)?;
    let mut cpu = Cpu::new(program);
    let mut crt = Crt::new();

    let mut cycle = 1; // "during"
    while crt.tick(cycle, cpu.x) {
        cycle += 1;
        cpu.tick(1);
    }
    Ok(format!("{}", crt))
}
