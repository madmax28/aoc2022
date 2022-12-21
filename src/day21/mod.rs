use std::{
    collections::HashMap,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug)]
enum Action<'a> {
    Num(i64),
    Op(&'a str, char, &'a str),
}

fn parse(s: &str) -> HashMap<&str, Action> {
    s.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0];

            let action = if let Ok(num) = parts[1].parse::<i64>() {
                Action::Num(num)
            } else {
                let parts: Vec<&str> = parts[1].split(' ').collect();
                let c = parts[1].chars().next().unwrap();
                Action::Op(parts[0], c, parts[2])
            };

            (name, action)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Polynomial(Vec<f64>);

impl Add for Polynomial {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (idx, value) in rhs.0.iter().enumerate() {
            self[idx] += value;
        }
        self
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (idx, value) in rhs.0.iter().enumerate() {
            self[idx] -= value;
        }
        self
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut poly = Polynomial(Vec::new());
        for (idx1, coeff1) in self.0.iter().enumerate() {
            for (idx2, coeff2) in rhs.0.iter().enumerate() {
                poly[idx1 + idx2] += coeff1 * coeff2;
            }
        }
        poly
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        if rhs.0.len() > 2 && (rhs[0] == 0.0 || rhs[1] == 0.0) {
            panic!("not going to do this");
        }

        let idx = if rhs[0] != 0.0 { 0 } else { 1 };
        self.0.iter_mut().for_each(|v| *v /= rhs[idx]);
        if idx == 1 {
            self.0.remove(0);
        }
        self
    }
}

impl Index<usize> for Polynomial {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.0.len() {
            &0.0
        } else {
            &self.0[index]
        }
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.0.len() {
            self.0.resize(index + 1, 0.0);
        }
        &mut self.0[index]
    }
}

fn eval(monkeys: &HashMap<&str, Action>, name: &str) -> i64 {
    match monkeys[name] {
        Action::Num(num) => num,
        Action::Op(name1, op, name2) => {
            let val1 = eval(monkeys, name1);
            let val2 = eval(monkeys, name2);
            match op {
                '+' => val1 + val2,
                '-' => val1 - val2,
                '*' => val1 * val2,
                '/' => val1 / val2,
                _ => panic!("invalid op"),
            }
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let monkeys = parse(input);
    Ok(eval(&monkeys, "root"))
}

fn eval2(monkeys: &HashMap<&str, Action>, name: &str) -> Polynomial {
    if name == "humn" {
        return Polynomial(vec![0.0, 1.0]);
    }

    match monkeys[name] {
        Action::Num(num) => Polynomial(vec![num as f64]),
        Action::Op(name1, op, name2) => {
            let poly1 = eval2(monkeys, name1);
            let poly2 = eval2(monkeys, name2);
            match op {
                '+' => poly1 + poly2,
                '-' => poly1 - poly2,
                '*' => poly1 * poly2,
                '/' => poly1 / poly2,
                _ => panic!("invalid op"),
            }
        }
    }
}

pub fn part2(input: &str) -> crate::Result<i64> {
    let monkeys = parse(input);
    if let Action::Op(name1, _, name2) = monkeys["root"] {
        let (poly1, poly2) = (eval2(&monkeys, name1), eval2(&monkeys, name2));
        let humn = (poly2[0] - poly1[0]) / (poly1[1] - poly2[1]);
        Ok(humn.round() as i64)
    } else {
        panic!("invalid input");
    }
}
