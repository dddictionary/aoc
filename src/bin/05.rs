use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
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

    // println!("Orderings set: {:?}", orderings);

    let contains = |x: &usize, y: &usize| orderings.contains(&(*x, *y));

    Some(updates.lines().filter_map(|line| {
        let update = line
            .split(",")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<usize>>();
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
