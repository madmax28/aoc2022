fn to_decimal_digit(digit: char) -> i64 {
    match digit {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("invalid digit"),
    }
}

fn to_decimal(snafu: &[char]) -> i64 {
    let mut res = 0;
    let mut val = 1;
    for c in snafu {
        res += val * to_decimal_digit(*c);
        val *= 5;
    }
    res
}

fn to_snafu_digit(digit: i64) -> char {
    match digit {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!("invalid digit"),
    }
}

pub fn part1(input: &str) -> crate::Result<String> {
    let nums: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().rev().collect())
        .collect();

    let mut sum = 0;
    for num in nums {
        sum += to_decimal(&num);
    }

    let mut snafu = Vec::new();
    while sum > 0 {
        let mut digit = sum.rem_euclid(5);
        sum /= 5;

        if digit > 2 {
            digit -= 5;
            sum += 1;
        }

        snafu.push(to_snafu_digit(digit));
    }
    Ok(snafu.into_iter().rev().collect())
}
