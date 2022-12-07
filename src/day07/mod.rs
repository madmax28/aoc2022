use std::collections::HashMap;

#[derive(Debug)]
enum Entry {
    Dir(Vec<usize>),
    File(i32),
}

impl Entry {
    fn dir(&mut self) -> Option<&mut Vec<usize>> {
        match self {
            Entry::Dir(dir) => Some(dir),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Fs<'a> {
    entries: Vec<Entry>,
    lookup: HashMap<Vec<&'a str>, usize>,
}

impl<'a> Fs<'a> {
    fn add_with<F>(&mut self, path: &Vec<&'a str>, add: F)
    where
        F: Fn() -> Entry,
    {
        if !self.lookup.contains_key(path) {
            self.entries.push(add());
            let child = self.entries.len() - 1;
            self.lookup.insert(path.clone(), child);

            let parent = self.lookup[&path[..path.len() - 1]];
            self.entries[parent].dir().unwrap().push(child);
        }
    }

    fn from_shell_output(output: &'a str) -> crate::Result<Self> {
        let mut fs = Fs {
            entries: vec![Entry::Dir(Vec::new())],
            lookup: HashMap::new(),
        };
        fs.lookup.insert(vec!["/"], 0);

        let mut path = vec!["/"];
        for line in output.lines() {
            match &line[..4] {
                "$ cd" => match &line[5..] {
                    "/" => _ = path.split_off(1),
                    ".." => _ = path.pop(),
                    dir => {
                        path.push(dir);
                        fs.add_with(&path, || Entry::Dir(Vec::new()));
                    }
                },
                "$ ls" => (),
                "dir " => (),
                // file
                _ => {
                    let size = line
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<i32>()?;
                    let start = line
                        .chars()
                        .take_while(|c| !c.is_ascii_alphabetic())
                        .count();

                    path.push(&line[start..]);
                    fs.add_with(&path, || Entry::File(size));
                    path.pop();
                }
            }
        }

        Ok(fs)
    }

    fn p1(&self, node: usize) -> (i32, i32) {
        match &self.entries[node] {
            Entry::Dir(entries) => {
                let (mut size, mut p1) = (0, 0);
                for child in entries {
                    let (entry_size, entry_p1) = self.p1(*child);
                    size += entry_size;
                    p1 += entry_p1;
                }

                if size <= 100000 {
                    p1 += size;
                }

                (size, p1)
            }
            Entry::File(size) => (*size, 0),
        }
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    let fs = Fs::from_shell_output(input)?;
    let (_, p1) = fs.p1(0);
    Ok(p1)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    const SPACE: i32 = 70000000;
    const REQUIRED: i32 = 30000000;

    let fs = Fs::from_shell_output(input)?;
    let (used, _) = fs.p1(0);
    let free = SPACE - used;
    let target = REQUIRED - free;

    let mut best = used;
    for (idx, node) in fs.entries.iter().enumerate() {
        if let Entry::Dir(_) = node {
            let (size, _) = fs.p1(idx);
            if size >= target && size < best {
                best = size;
            }
        }
    }
    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let fs = Fs::from_shell_output(input).unwrap();
        let path = vec!["/", "a", "e"];
        let node = fs.lookup[&path];
        let (size_e, p1_e) = fs.p1(node);
        assert_eq!(584, size_e);
        assert_eq!(584, p1_e);

        assert_eq!(95437, part1(input).unwrap());
    }
}
