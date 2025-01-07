use std::collections::HashSet;

advent_of_code::solution!(6);

struct Grid {
    tiles: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = tiles.len();
        let width = tiles.first().unwrap().len();

        Self {
            tiles,
            height,
            width,
        }
    }

    fn get_guard_position(&self) -> (usize, usize) {
        for row in 0..self.width {
            for col in 0..self.height {
                if self.tiles[row][col] == '^' {
                    return (row, col);
                }
            }
        }
        panic!("No guard found in grid!")
    }

    fn get_next_position(
        &self,
        (curr_row, curr_col): (usize, usize),
        curr_direction: &mut Direction,
    ) -> Option<(usize, usize)> {
        let (next_row, next_col) = match curr_direction {
            Direction::Up => (curr_row.checked_sub(1)?, curr_col),
            Direction::Down => (curr_row + 1, curr_col),
            Direction::Left => (curr_row, curr_col.checked_sub(1)?),
            Direction::Right => (curr_row, curr_col + 1),
        };

        let char = self.tiles.get(next_row).and_then(|row| row.get(next_col))?;

        if char == &'#' {
            //if i see a obstacle, i can't move at all, just change the direction.
            curr_direction.turn_right();
            return Some((next_row, curr_col));
        }

        Some((next_row, next_col))
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // first part is always going up.
    // get a pointer to the guard.
    // get the bounds of the grid. do a loop{}
    // with a match checking to see if you encounter a #.
    // if you encounter the #, move to the right
    let grid = Grid::new(input);

    let (mut guard_row, mut guard_col) = grid.get_guard_position();

    let mut direction = Direction::Up;

    let mut visited = HashSet::new();

    visited.insert((guard_col, guard_col));

    while let Some((next_row, next_col)) = grid.get_next_position((guard_row, guard_col), &mut direction) {
        guard_row = next_row;
        guard_col = next_col;

        visited.insert((guard_row, guard_col));
    }

    Some(visited.len() as u32)

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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
