use std::vec;

trait AOC {
    fn add_instance(&mut self, n: u32);
}
trait AventOfCode {
    fn get_matches(&self) -> Vec<u32>;
    fn count_matches(&self) -> u32;
}

#[derive(Debug, Clone)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>
}

#[derive(Debug, Clone, Copy)]
struct MatchCard {
    card_number: u32,
    matches: u32,
    instances: u32,
}

impl AOC for MatchCard {
    fn add_instance(&mut self, n: u32) {
        self.instances = &self.instances + n;
    }
}

impl AventOfCode for Card {
    fn get_matches(&self) -> Vec<u32> {
        self.numbers_you_have.iter().filter(|x| self.winning_numbers.contains(x)).map(|x| x.to_owned()).collect::<Vec<u32>>()
    }
    fn count_matches(&self) -> u32 {
        self.get_matches().iter().count() as u32
    }
}

pub fn main(input: &str) -> String {
    let mut n_cards: Vec<MatchCard> =  vec![];
    let mut original_cards: Vec<Card> = vec![];
    
    let mut card_number = 1;
    for line in input.lines() {
        let numbers = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers_you_have = numbers[1].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let card = Card { card_number, winning_numbers, numbers_you_have };
        
        original_cards.push(card);
        card_number += 1;
    }

    for card in &original_cards {
        let card_number = card.card_number;
        let instances = 1;
        let matches = card.count_matches();
        n_cards.push(MatchCard { card_number, matches, instances });
    }

    for i in 0..n_cards.len() {
        let card = n_cards[i];
        if i+1 < n_cards.len() {
            for j in 1..=card.matches {
                n_cards[i+j as usize].add_instance(card.instances)
            }
        }
    }

    let sum = n_cards.iter().fold(0,|sum: u32, card: &MatchCard| sum + card.instances);
    
    sum.to_string()
}