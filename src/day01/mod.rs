pub fn part1(input: &str) -> crate::Result<i32> {
    let max = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<i32>().expect("invalid input"))
                .sum()
        })
        .max()
        .expect("invalid input");
    Ok(max)
}

pub fn part2(input: &str) -> crate::Result<i32> {
    let mut elves = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<i32>().expect("invalid input"))
                .sum()
        })
        .collect::<Vec<i32>>();
    elves.sort();
    Ok(elves.iter().rev().take(3).sum())
}
