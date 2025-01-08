use std::{collections::{HashMap, HashSet}, ops::{Add, Sub}};


advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    row: i64,
    col: i64,
}

impl Add for Antenna {
    type Output = Antenna;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row +  rhs.row,
            col: self.col + rhs.col,
        }
    }

}

impl Sub for Antenna {
    type Output = Antenna;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row -  rhs.row,
            col: self.col - rhs.col,
        }
    }

}

impl Antenna {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap()
        }
    }

    fn within_bounds(&self, height: i64, width: i64) -> bool {
        0 <= self.col && self.col <= width && 0 <= self.row && self.row <= height
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count().try_into().unwrap();
    let width = input.lines().next().unwrap().len().try_into().unwrap();

    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.char_indices() {
            if char == '.' {
                continue;
            }

            antennas.entry(char).or_default().push(Antenna::new(row, col));
        }
    }

    let mut antinodes: HashSet<Antenna> = HashSet::new();

    for positions in antennas.values() {
        for pair in positions.iter().com
    }
}

pub fn part_two(input: &str) -> Option<u32> {
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
