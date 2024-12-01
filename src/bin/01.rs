use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut v1: Vec<u32> = Vec::new();
    let mut v2: Vec<u32> = Vec::new();
    let mut distances = Vec::new();
    let lines = input.lines();

    for line in lines {
        let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        if numbers.len() == 2 {
            v1.push(numbers[0]);
            v2.push(numbers[1]);
        }
    }

    v1.sort();
    v2.sort();

    for (p1, p2) in v1.iter().zip(v2.iter()) {
        distances.push(p1.abs_diff(*p2));
    }
    //println!("v1: {:?}", v1);
    //println!("v2: {:?}", v2);
    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v1: Vec<u32> = Vec::new();
    let mut v2: Vec<u32> = Vec::new();
    let mut similarity = Vec::new();
    let lines = input.lines();

    for line in lines {
        let numbers: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        if numbers.len() == 2 {
            v1.push(numbers[0]);
            v2.push(numbers[1]);
        }
    }

    // create a hashmap where key: num, value: frequency of num in v2
    let map: HashMap<u32, u32> = v2.iter().fold(HashMap::new(), |mut acc, &value| {
        *acc.entry(value).or_insert(0) += 1;
        acc
    });

    for num in v1 {
        if let Some(freq) = map.get(&num) {
            similarity.push(freq * num);
        }
    }

    Some(similarity.iter().copied().sum())
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
