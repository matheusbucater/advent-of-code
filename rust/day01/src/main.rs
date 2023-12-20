use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> String{
    let nums = HashMap::from([
        ("2", 2),
        ("1", 1),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut search_line = line;
        let first = 'a: loop {
            for (key, val) in nums.iter() {
                if search_line.starts_with(key) {
                    break 'a val;
                }
            }
            search_line = &search_line[1..];
        };
        let last = 'a: loop {
            for (key, val) in nums.iter() {
                if search_line.ends_with(key) {
                    break 'a val;
                }
                if search_line.is_empty() {
                    break 'a first;
                }
            }
            search_line = &search_line[..search_line.len()-1]
        };
        let value = first * 10 + last;
        sum += value;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, "281".to_string());
    }
}