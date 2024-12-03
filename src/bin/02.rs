advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<u32> {
    input.split_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.lines().map(|line| parse(line)).collect::<Vec<Vec<u32>>>();
    println!("{:?}", &reports);
    let safe_count = reports.iter().fold(0, |acc, report| {
        let is_valid = report.windows(2).all(|pair| {
            let (first, second) = (pair[0], pair[1]);
            let diff = first.abs_diff(second);
            diff >= 1 && diff <= 3 && (first < second || first > second)
        });
        acc + if is_valid { 1 } else { 0 }
    });
    Some(safe_count)
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
