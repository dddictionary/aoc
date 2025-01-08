use core::panic;
use std::collections::HashMap;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    input
        .lines()
        .map(|line| {
            //println!("line: {}", line);
            let mut split = line.split(":");
            if let (Some(test_values), Some(values), None) =
                (split.next(), split.next(), split.next())
            {
                let test_value = test_values.parse().unwrap();
                let nums = values
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                (test_value, nums)
            } else {
                panic!("Error parsing data")
            }
        })
        .collect()
}

fn is_possible(curr: &usize, target: &usize, values: &[usize]) -> bool {
    if values.is_empty() {
        return curr == target;
    }

    if curr > target {
        return false;
    }

    let (first, rest) = values.split_first().unwrap();

    is_possible(&(curr * first), target, rest)
        || is_possible(&(curr + first), target, rest)
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    Some(map.into_iter()
        .filter_map(|(key, val)| {
            if let Some(first) = val.first() {
                if is_possible(first, &key, &val) {
                    return Some(key);
                }
            }
            None
        })
        .sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
