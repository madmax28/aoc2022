use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn mv(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Facing::Right => (pos.0 + 1, pos.1),
            Facing::Down => (pos.0, pos.1 + 1),
            Facing::Left => (pos.0 - 1, pos.1),
            Facing::Up => (pos.0, pos.1 - 1),
        }
    }

    fn turn(self, c: char) -> Self {
        match c {
            'R' => match self {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            },
            'L' => match self {
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Up => Facing::Left,
            },
            _ => panic!("invalid turn"),
        }
    }

    fn val(self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    pos: (i32, i32),
    facing: Facing,
}

impl Pos {
    fn mv(mut self) -> Self {
        self.pos = self.facing.mv(self.pos);
        self
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    height: i32,
    width: i32,
    pos: Pos,
}

impl Map {
    fn get(&self, pos: &(i32, i32)) -> Option<char> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.1 < self.height && pos.0 < self.width {
            Some(self.map[pos.1 as usize][pos.0 as usize])
        } else {
            None
        }
    }

    fn mv(&mut self, n: i32) {
        'outer: for _ in 0..n {
            let mut cand = self.pos.mv();
            loop {
                match self.get(&cand.pos) {
                    Some('.') => break,
                    Some('#') => break 'outer,
                    Some(' ') => cand = cand.mv(),
                    Some(_) => panic!("invalid input"),
                    None => match cand.facing {
                        Facing::Right => cand.pos.0 = 0,
                        Facing::Down => cand.pos.1 = 0,
                        Facing::Left => cand.pos.0 = self.width - 1,
                        Facing::Up => cand.pos.1 = self.height - 1,
                    },
                }
            }
            self.pos = cand;
        }
    }

    fn mv_p2(&mut self, n: i32) {
        'outer: for _ in 0..n {
            let mut cand = self.pos.mv();
            loop {
                match self.get(&cand.pos) {
                    Some('.') => break,
                    Some('#') => break 'outer,
                    _ => match self.pos {
                        Pos { pos: (x, y), facing: Facing::Left } if x < 50 && y < 150 => {
                            cand = Pos {
                                pos: (50, 49 - (y - 100)),
                                facing: Facing::Right,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Up } if x < 50 && y < 150 => {
                            cand = Pos {
                                pos: (50, 50 + x),
                                facing: Facing::Right,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Right } if x < 50 => {
                            cand = Pos {
                                pos: (50 + y - 150, 149),
                                facing: Facing::Up,
                            }
                        }
                        Pos { pos: (x, _y), facing: Facing::Down } if x < 50 => {
                            cand = Pos {
                                pos: (100 + x, 0),
                                facing: Facing::Down,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Left } if x < 50 => {
                            cand = Pos {
                                pos: (50 + y - 150, 0),
                                facing: Facing::Down,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Left } if x < 100 && y < 50 => {
                            cand = Pos {
                                pos: (0, 149 - y),
                                facing: Facing::Right,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Up } if x < 100 && y < 50 => {
                            cand = Pos {
                                pos: (0, 150 + x - 50),
                                facing: Facing::Right,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Right } if x < 100 && y < 100 => {
                            cand = Pos {
                                pos: (100 + y - 50, 49),
                                facing: Facing::Up,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Left } if x < 100 && y < 100 => {
                            cand = Pos {
                                pos: (y - 50, 100),
                                facing: Facing::Down,
                            }
                        }
                        Pos { pos: (x, y), facing: Facing::Right } if x < 100 => {
                            cand = Pos {
                                pos: (149, 49 - (y - 100)),
                                facing: Facing::Left,
                            }
                        }
                        Pos { pos: (x, _y), facing: Facing::Down } if x < 100 => {
                            cand = Pos {
                                pos: (49, 150 + x - 50),
                                facing: Facing::Left,
                            }
                        }
                        Pos { pos: (_x, y), facing: Facing::Right } => {
                            cand = Pos {
                                pos: (99, 149 - y),
                                facing: Facing::Left,
                            }
                        }
                        Pos { pos: (x, _y), facing: Facing::Down } => {
                            cand = Pos {
                                pos: (99, 50 + x - 100),
                                facing: Facing::Left,
                            }
                        }
                        Pos { pos: (x, _y), facing: Facing::Up } => {
                            cand = Pos {
                                pos: (x - 100, 199),
                                facing: Facing::Up,
                            }
                        }
                        _ => panic!("invalid pos"),
                    },
                }
            }
            self.pos = cand;
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(&(x, y)).unwrap_or(' '));
            }
            println!();
        }
    }
}

impl FromStr for Map {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut map: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = map.len() as i32;
        let width = map[0].len() as i32;
        map.iter_mut().for_each(|row| row.resize(width as usize, ' '));

        let x = (0..).find(|x| map[0][*x] == '.').unwrap();

        Ok(Map {
            map,
            height,
            width,
            pos: Pos {
                pos: (x as i32, 0),
                facing: Facing::Right,
            },
        })
    }
}

#[derive(Debug)]
enum Instruction {
    Go(i32),
    Turn(char),
}

fn parse(s: &str) -> (Map, Vec<Instruction>) {
    let parts: Vec<&str> = s.split("\n\n").collect();

    let mut insn = Vec::new();
    let mut cnt = String::new();
    for c in parts[1].trim().chars() {
        if c.is_ascii_digit() {
            cnt.push(c);
        } else {
            if !cnt.is_empty() {
                insn.push(Instruction::Go(cnt.parse().unwrap()));
                cnt.clear();
            }
            insn.push(Instruction::Turn(c));
        }
    }
    if !cnt.is_empty() {
        insn.push(Instruction::Go(cnt.parse().unwrap()));
    }

    (parts[0].parse().unwrap(), insn)
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let (mut map, insns) = parse(input);
    for insn in insns {
        match insn {
            Instruction::Go(n) => map.mv(n),
            Instruction::Turn(c) => map.pos.facing = map.pos.facing.turn(c),
        }
    }
    let pass = (1 + map.pos.pos.1) * 1000 + (1 + map.pos.pos.0) * 4 + map.pos.facing.val();
    Ok(pass)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let (mut map, insns) = parse(input);
    for insn in insns {
        match insn {
            Instruction::Go(n) => map.mv_p2(n),
            Instruction::Turn(c) => map.pos.facing = map.pos.facing.turn(c),
        }
    }
    let pass = (1 + map.pos.pos.1) * 1000 + (1 + map.pos.pos.0) * 4 + map.pos.facing.val();
    Ok(pass)
}
