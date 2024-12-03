advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn apply_safety_check<F>(safety_func: F, contents: &str) -> u32
where
    F: Fn(&Vec<u32>) -> bool,
{
    contents
        .lines()
        .map(|line| {
            parse(line)
        })
        .filter(|report| safety_func(report))
        .count() as u32
}

fn is_safe(report: &Vec<u32>) -> bool {
    let is_desc = report[0] > report[1];
    for i in 0..(report.len() - 1) {
        let a = report[i];
        let b = report[i + 1];
        if is_desc && a < b || !is_desc && a > b {
            return false;
        }

        let diff = a.abs_diff(b);
        if !(1..=3).contains(&diff){
            return false;
        }
    }
    return true;
}

fn is_safe_with_one_removed(report: &Vec<u32>) -> bool {
    (0..report.len()).any(|i| {
        let mut tmp = report.clone();
        tmp.remove(i);
        is_safe(&tmp)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(apply_safety_check(|report| is_safe(report), input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(apply_safety_check(|report| is_safe_with_one_removed(report), input))
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
