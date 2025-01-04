advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let target = "XMAS";
    let target_rev = "SAMX";

    let matches_target = |s: &str| s == target || s == target_rev;

    let horizontal_count = lines
        .iter()
        .flat_map(|&line| line.as_bytes().windows(4))
        .filter(|window| matches_target(std::str::from_utf8(window).unwrap()))
        .count();

    let vertical_count = (0..width)
        .flat_map(|col| {
            (0..height - 3).map({
                let value = lines.clone();
                move |row| {
                    (0..4)
                        .map(|i| value[row + i].chars().nth(col).unwrap())
                        .collect::<String>()
                }
            })
        })
        .filter(|vertical_str| matches_target(vertical_str))
        .count();

    let diagonal_right_count = (0..height - 3)
        .flat_map(|row| {
            (0..width - 3).map({
                let value = lines.clone();
                move |col| {
                    (0..4)
                        .map(|i| value[row + i].chars().nth(col + i).unwrap())
                        .collect::<String>()
                }
            })
        })
        .filter(|diagonal_string| matches_target(diagonal_string))
        .count();

    let diagonal_left_count = (0..height - 3)
    .flat_map(|row| {
        (3..width).map({
            let value = lines.clone();
            move |col| {
                (0..4)
                    .map(|i| value[row + i].chars().nth(col - i).unwrap())
                    .collect::<String>()
            }
        })
    })
    .filter(|diagonal_string| matches_target(diagonal_string))
    .count();

    Some((horizontal_count + vertical_count + diagonal_left_count + diagonal_right_count) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    let target = "MAS";
    let target_rev = "SAM";

    let matches_target = |d1: &str, d2: &str| (d1 == target || d1 == target_rev) && (d2 == target || d2 == target_rev);

    let mut count = 0;

    for row in 0..(height - 2) {
        for col in 0..(width - 2) {
            let mut diagonal1 = String::new();
            let mut diagonal2 = String::new();

            for i in 0..3 {
                let char = lines[row + i].chars().nth(col + i).unwrap();

                diagonal1.push(char);

                let char = lines[row + i].chars().nth(col + 2 - i).unwrap();
                diagonal2.push(char);


            }

            if matches_target(&diagonal1, &diagonal2) {
                count += 1
            }

        }

    }
    Some(count)
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
