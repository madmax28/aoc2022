use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Entry {
    List(List),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Value(i32);

impl Value {
    fn parse(s: &str, idx: &mut usize) -> Self {
        let s: String = s[*idx..].chars().take_while(char::is_ascii_digit).collect();
        *idx += s.len();
        Value(s.parse().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct List(Vec<Entry>);

impl List {
    fn parse(s: &str, idx: &mut usize) -> Self {
        *idx += 1; // skip [
        let mut list = Vec::new();
        loop {
            match s[*idx..].chars().next().unwrap() {
                '[' => list.push(Entry::List(List::parse(s, idx))),
                c if c.is_ascii_digit() => list.push(Entry::Value(Value::parse(s, idx))),
                ']' => {
                    *idx += 1;
                    return List(list);
                }
                _ => *idx += 1,
            }
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &List) -> Ordering {
        for entries in self.0.iter().zip(other.0.iter()) {
            match entries {
                (Entry::Value(lhs), Entry::Value(rhs)) => {
                    let cmp = lhs.0.cmp(&rhs.0);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                (Entry::List(lhs), Entry::List(rhs)) => {
                    let cmp = lhs.cmp(rhs);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                (lhs, Entry::List(rhs)) => {
                    let lhs = List(vec![lhs.clone()]);
                    let cmp = lhs.cmp(rhs);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                (Entry::List(lhs), rhs) => {
                    let rhs = List(vec![rhs.clone()]);
                    let cmp = lhs.cmp(&rhs);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }
        }
        self.0.len().cmp(&other.0.len())
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let cnt = input
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, pair)| {
            let lists: Vec<List> = pair
                .split('\n')
                .map(|s| List::parse(s, &mut 0))
                .collect();
            if lists[0] <= lists[1] {
                Some(1 + idx)
            } else {
                None
            }
        })
        .sum();
    Ok(cnt)
}

pub fn part2(input: &str) -> crate::Result<usize> {
    let mut lists: Vec<List> = input
        .split("\n\n")
        .flat_map(|pair| {
            pair.split('\n')
                .map(|s| List::parse(s, &mut 0))
        })
        .collect();

    let div1 = List::parse("[[2]]", &mut 0);
    lists.push(div1.clone());
    let div2 = List::parse("[[6]]", &mut 0);
    lists.push(div2.clone());

    lists.sort();
    let product = lists
        .iter()
        .enumerate()
        .filter(|(_, l)| *l == &div1 || *l == &div2)
        .map(|(idx, _)| 1 + idx)
        .product();
    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(13, part1(input).unwrap());
    }
}
