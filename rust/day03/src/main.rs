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
        let answer = "4361".to_string();
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
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
