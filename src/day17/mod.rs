use std::collections::HashMap;

fn shapes() -> Vec<Vec<Vec<char>>> {
    "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
    .split("\n\n")
    .map(|shape| {
        shape
            .lines()
            .rev()
            .map(|line| line.chars().collect())
            .collect()
    })
    .collect()
}

struct Chamber {
    map: Vec<Vec<char>>,
}

impl Chamber {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn fits(&self, shape: &[Vec<char>], x: usize, y: usize) -> bool {
        if x + shape[0].len() > 7 {
            return false;
        }

        for (dy, line) in shape.iter().enumerate() {
            if y + dy + 1 > self.map.len() {
                break;
            }

            if self.map[y + dy]
                .iter()
                .skip(x)
                .zip(line.iter())
                .any(|(c1, c2)| *c1 == '#' && *c2 == '#')
            {
                return false;
            }
        }
        true
    }

    fn add(&mut self, shape: &[Vec<char>], x: usize, y: usize) {
        if y + shape.len() > self.map.len() {
            self.map.resize(y + shape.len(), vec!['.'; 7]);
        }

        for (dy, line) in shape.iter().enumerate() {
            for (dx, c) in line.iter().enumerate().filter(|(_, c)| *c == &'#') {
                self.map[y + dy][x + dx] = *c;
            }
        }
    }

    fn fall(
        &mut self,
        shape: &[Vec<char>],
        dirs: &[char],
        dir_idxs: &mut dyn Iterator<Item = usize>,
    ) {
        let mut x = 2;
        let mut y = self.map.len() + 3;
        loop {
            match dirs[dir_idxs.next().unwrap()] {
                '<' => {
                    if x > 0 && self.fits(shape, x - 1, y) {
                        x -= 1;
                    }
                }
                '>' => {
                    if self.fits(shape, x + 1, y) {
                        x += 1;
                    }
                }
                _ => panic!("invalid input"),
            }

            if y == 0 || !self.fits(shape, x, y - 1) {
                self.add(shape, x, y);
                return;
            }
            y -= 1;
        }
    }

    fn depth(&self, x: usize) -> usize {
        self.map
            .iter()
            .rev()
            .take_while(|line| line[x] == '.')
            .count()
    }

    #[allow(dead_code)]
    fn draw(&self) {
        println!();
        for line in self.map.iter().rev() {
            print!("+");
            for c in line {
                print!("{}", c);
            }
            println!("+");
        }
        println!("+++++++++");
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let shapes = shapes();
    let mut shape_idxs = (0..shapes.len()).cycle();
    let dirs: Vec<char> = input.chars().collect();
    let mut dir_idxs = (0..dirs.len()).cycle();
    let mut chamber = Chamber::new();
    for _ in 0..2022 {
        chamber.fall(&shapes[shape_idxs.next().unwrap()], &dirs, &mut dir_idxs);
    }
    Ok(chamber.map.len())
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let shapes = shapes();
    let mut shape_idxs = (0..shapes.len()).cycle().peekable();
    let dirs: Vec<char> = input.chars().collect();
    let mut dir_idxs = (0..dirs.len()).cycle().peekable();
    let mut chamber = Chamber::new();

    const LIMIT: usize = 1000000000000;
    let mut seen: HashMap<(Vec<usize>, usize, usize), (usize, usize)> = HashMap::new();
    let mut i = 0;
    while i < LIMIT {
        chamber.fall(&shapes[shape_idxs.next().unwrap()], &dirs, &mut dir_idxs);
        i += 1;

        let key = (
            (0..7).map(|x| chamber.depth(x)).collect(),
            *shape_idxs.peek().unwrap(),
            *dir_idxs.peek().unwrap(),
        );
        if let Some((j, height)) = seen.get(&key) {
            let loop_len = i - j;
            let height_diff = chamber.map.len() - height;
            let skip_cnt = (LIMIT - i) / loop_len;
            let skip_height = skip_cnt * height_diff;
            i += skip_cnt * loop_len;
            while i < LIMIT {
                chamber.fall(&shapes[shape_idxs.next().unwrap()], &dirs, &mut dir_idxs);
                i += 1;
            }
            return Ok(skip_height + chamber.map.len());
        }
        seen.insert(key, (i, chamber.map.len()));
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(part1(input).unwrap(), 3068);
        assert_eq!(part2(input).unwrap(), 1514285714288);
    }
}
