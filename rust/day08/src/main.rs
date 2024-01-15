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
        let answer1 = "2".to_string();
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
        ";
        let answer2 = "6".to_string();
        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
        ";
        let result1 = part1::main(input1);
        let result2 = part1::main(input2);
        assert_eq!(result1, answer1);
        assert_eq!(result2, answer2);
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
