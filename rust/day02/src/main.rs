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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1::main(input);
        assert_eq!(result, "8".to_string());
    }
    #[test]
    fn part2_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2::main(input);
        assert_eq!(result, "2286".to_string());
    }
}