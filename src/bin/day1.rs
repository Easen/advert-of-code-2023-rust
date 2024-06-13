fn main() {
    let input = include_str!("../../inputs/1.txt").trim_end();
    println!("part1 = {}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|l| find_digit(l) * 10 + find_digit(&l.chars().rev().collect::<String>()))
        .sum()
}

fn find_digit(str: &str) -> u32 {
    match str.chars().find(|d| d.is_numeric()) {
        None => panic!("Cannot find digit in {}", str),
        Some(x) => x as u32 - 48,
    }
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

        assert_eq!(part1(&input), 142);
    }
}
