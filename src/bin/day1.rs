use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/1.txt").trim_end();
    println!("part1 = {}", puzzle(input, false).iter().sum::<i32>());
    println!("part2 = {}", puzzle(input, true).iter().sum::<i32>());
}

fn puzzle(input: &str, part2: bool) -> Vec<i32> {
    let mut numbers: HashMap<&str, i32> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    if part2 == true {
        numbers.extend(HashMap::from([
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]));
    }

    return input
        .lines()
        .map(|l| {
            let keys = numbers.keys().copied().collect::<Vec<_>>();
            let found = find(l, keys).unwrap_or(vec![]);
            return found
                .into_iter()
                .map(|d| numbers.get(d).unwrap())
                .collect::<Vec<_>>();
        })
        .map(|matches: Vec<&i32>| *matches.first().unwrap() * 10 + *matches.last().unwrap())
        .collect();
}

fn find<'a>(input: &str, elements: Vec<&'a str>) -> Option<Vec<&'a str>> {
    let mut results = Vec::new();
    for n in 0..input.len() {
        let slice: &str = input.get(n..input.len()).unwrap();
        elements.iter().for_each(|element| {
            if slice.starts_with(element) {
                results.push(*element)
            }
        });
    }
    if results.len() == 0 {
        return None;
    }
    return Some(results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = puzzle(&input, false);
        assert_eq!(result, vec![12, 38, 15, 77]);
        assert_eq!(result.iter().sum::<i32>(), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = puzzle(&input, true);
        assert_eq!(result, vec![29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(result.iter().sum::<i32>(), 281);
    }
}
