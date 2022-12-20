fn find<T: Copy + Eq>(slice: &[T], value: T) -> (usize, T) {
    slice
        .iter()
        .copied()
        .enumerate()
        .find(|(_, val)| *val == value)
        .unwrap()
}

fn mix(nums: &[i64], moved: &mut Vec<usize>) {
    for (id, val) in nums.iter().enumerate() {
        let old_idx = find(moved, id).0;
        assert_eq!(moved.remove(old_idx), id);

        let new_idx =
            (old_idx as i64 + *val).rem_euclid(moved.len() as i64) as usize;

        moved.insert(new_idx, id);
    }
}

fn coords(nums: &[i64], moved: &[usize]) -> i64 {
    let id0 = find(nums, 0).0;
    let mut idx = find(moved, id0).0;
    let mut sum = 0;
    for _ in 0..3 {
        idx = (idx + 1000) % nums.len();
        let id = moved[idx];
        sum += nums[id];
    }
    sum
}

pub fn part1(input: &str) -> crate::Result<i64> {
    let nums: Vec<i64> =
        input.lines().map(str::parse).collect::<Result<_, _>>()?;
    let mut moved: Vec<usize> = (0..nums.len()).collect();
    mix(&nums, &mut moved);
    Ok(coords(&nums, &moved))
}

pub fn part2(input: &str) -> crate::Result<i64> {
    const KEY: i64 = 811589153;
    let nums: Vec<i64> = input
        .lines()
        .map(|l| -> crate::Result<i64> { Ok(l.parse::<i64>()? * KEY) })
        .collect::<Result<_, _>>()?;
    let mut moved: Vec<usize> = (0..nums.len()).collect();
    for _ in 0..10 {
        mix(&nums, &mut moved);
    }
    Ok(coords(&nums, &moved))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "1
2
-3
3
-2
0
4";

        assert_eq!(part1(input).unwrap(), 3);
    }
}
