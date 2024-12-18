advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    //for capture in re.captures_iter(input) {
    //    let first = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
    //    let second = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
    //    total_sum += first * second;
    //}
    Some(re
        .captures_iter(input)
        .map(|capture| {
            let first = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let second = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            first * second
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let all = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    let mut add_up = true;

    for matches in all.captures_iter(input) {
        dbg!(&matches);
        let instruction = matches.get(0).unwrap().as_str();
        dbg!(instruction);
        match instruction {
            "do()" => add_up = true,
            "don't()" => add_up = false,
            _ => {
                if add_up {
                    let first = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let second = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    sum += first * second;
                }
            }
        }
    }

    Some(sum)
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
