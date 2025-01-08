advent_of_code::solution!(7);



fn is_possible(curr: &usize, target: &usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        return curr == target;
    }

    if curr > target {
        return false;
    }

    let (first, rest) = rest.split_first().unwrap();

    is_possible(&(curr * first), target, rest) || is_possible(&(curr + first), target, rest)
}

fn is_possible_concat(curr: &usize, target: &usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        return curr == target;
    }

    if curr > target {
        return false;
    }

    let (first, rest) = rest.split_first().unwrap();

    is_possible_concat(&(curr * first), target, rest)
    || is_possible_concat(&(curr + first), target, rest)
    || is_possible_concat(&concat(*curr,* first), target, rest)

}

fn concat(a: usize, b: usize) -> usize {
    // Calculate the number of digits in b
    let num_digits = (b as f64).log10().floor() as u32 + 1;

    // Combine the numbers using math
    a * 10usize.pow(num_digits) + b
}


// fn check(input: &str, filter: Fn)
fn check<F>(filter: F, input: &str) -> usize
where
    F: Fn(&usize, &usize, &[usize]) -> bool {
        input
        .lines()
        .filter_map(|line| {
            //println!("line: {}", line);
            let mut split = line.split(":");
            if let (Some(test_values), Some(values), None) =
                (split.next(), split.next(), split.next())
            {
                let test_value: usize = test_values.parse().unwrap();
                let nums = values
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>();

                let (first, rest) = nums.split_first().unwrap();
                if filter(first, &test_value, rest) {
                    return Some(test_value);
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
    }


pub fn part_one(input: &str) -> Option<usize> {
    Some(check(is_possible, input))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(check(is_possible_concat, input))
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
