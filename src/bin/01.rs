use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_ascii_whitespace();
        let num1: i32 = nums.next()?.parse().ok()?;
        let num2: i32 = nums.next()?.parse().ok()?;
        list1.push(num1);
        list2.push(num2);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let result: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut list1 = Vec::new();
    let mut list2 = HashMap::new();

    for line in input.lines() {
        let mut nums = line.split_ascii_whitespace();
        let num1: i32 = nums.next()?.parse().ok()?;
        let num2: i32 = nums.next()?.parse().ok()?;

        list1.push(num1);
        *list2.entry(num2).or_insert(0) += 1;
    }

    let result: i32 = list1
        .iter()
        .map(|k| list2.get(k).unwrap_or(&0) * k)
        .sum();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
