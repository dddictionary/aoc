use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (HashSet<(usize, usize)>, &str) {
    let (orderings, updates) = input.split_once("\n\n").unwrap();
    // println!("Orderings: {orderings}");
    // println!("Updates: {updates}");
    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            let (Some(x), Some(y), None) = (split.next(), split.next(), split.next()) else {
                panic!("Rules are not correct.");
            };

            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();

    (orderings, updates)
}

pub fn parse_updates(line: &str) -> Vec<usize> {
    line
            .split(",")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<usize>>()
}

pub fn part_one(input: &str) -> Option<u32> {

    let (orderings, updates) = parse_input(input);

    // println!("Orderings set: {:?}", orderings);

    let contains = |x: &usize, y: &usize| orderings.contains(&(*x, *y));

    Some(updates.lines().filter_map(|line| {
        let update = parse_updates(line);
        // println!("Update: {:?}", update);
        if update.is_sorted_by(contains).then_some(std::cmp::Ordering::Less).is_some() {
            let len = update.len();
            if len > 1 {
                Some(update[len / 2])
            } else {
                None
            }
        } else {
            None
        }
    })
    .sum::<usize>() as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let (orderings, updates) = parse_input(input);
    let contains = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            std::cmp::Ordering::Less
        } else if orderings.contains(&(y, x)){
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    };

    Some(updates.lines().filter_map(|line| {
        let mut update = parse_updates(line);
        if update.is_sorted_by(|a, b| contains(a,b) != std::cmp::Ordering::Greater) {
            Some(0 as u32)
        } else {
            update.sort_by(contains);
            Some(update[update.len()/2] as u32)
        }
    })
    .sum())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
