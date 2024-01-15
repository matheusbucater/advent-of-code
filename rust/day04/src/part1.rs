use std::vec;

trait AventOfCode {
    fn get_matches(&self) -> Vec<u32>;
    fn points(&self) -> u32;
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>
}

impl AventOfCode for Card {
    fn get_matches(&self) -> Vec<u32> {
        self.numbers_you_have.iter().filter(|x| self.winning_numbers.contains(x)).map(|x| x.to_owned()).collect::<Vec<u32>>()
    }
    fn points(&self) -> u32 {
        let matches = self.get_matches();
        let n = matches.len() as u32;
        let points = if n > 0 { 2u32.pow(n - 1) } else { 0 };
        points
    }
}

pub fn main(input: &str) -> String {
    let mut cards: Vec<Card> = vec![];
    for line in input.lines() {
        let numbers = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers_you_have = numbers[1].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let card = Card { winning_numbers, numbers_you_have };
        
        cards.push(card);
    }

    cards.iter().map(|card| card.points()).fold(0, |sum, x| sum + x).to_string()
}
