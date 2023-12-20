mod part1;
mod part2;

fn main() {
    let input1 = include_str!("./input1.txt");
    let input2 = include_str!("./input2.txt");

    let output1 = part1::main(input1);
    let output2 = part2::main(input2);

    dbg!(output1, output2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let answer = "35".to_string();
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1::main(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn part2_works() {
        let answer = "todo!()".to_string();
        let input = "
        ";
        let result = part2::main(input);
        assert_eq!(result, answer);
    }

}
