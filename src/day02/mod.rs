fn score(round: &str) -> i32 {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("invalid input"),
    }
}

pub fn part1(input: &str) -> crate::Result<i32> {
    Ok(input.lines().map(score).sum())
}

fn score_p2(round: &str) -> i32 {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!("invalid input"),
    }
}

pub fn part2(input: &str) -> crate::Result<i32> {
    Ok(input.lines().map(score_p2).sum())
}
