use std::fmt::Display;


#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.red, self.green, self.blue)
    }
}
#[derive(Debug)]
struct Game {
    n: u32,
    sets: Vec<Set>
}

trait AdventOfCode {
    fn largest_rgb(&self) -> Set;
    fn valid_game_id(&self, rules: (u32,u32,u32)) -> u32;
    fn power(&self) -> u32;
}

impl AdventOfCode for Game {
    fn largest_rgb(&self) -> Set {
        let sets = &self.sets;
        let mut largest_set = Set { red: 0, green: 0, blue: 0 };
        for set in sets {
            if largest_set.red < set.red { largest_set.red = set.red }
            if largest_set.green < set.green { largest_set.green = set.green }
            if largest_set.blue < set.blue { largest_set.blue = set.blue }
        }
        largest_set
    }

    fn valid_game_id(&self, rules: (u32,u32,u32)) -> u32 {
        let (r,g,b) = rules;
        let largest_set = self.largest_rgb();
        if largest_set.red > r || largest_set.green > g && largest_set.blue > b {
            0
        } else {
            self.n
        }
    }

    fn power(&self) -> u32 {
        let largest_set = self.largest_rgb();
        largest_set.red * largest_set.green * largest_set.blue
    }
}


pub fn main(input: &str) -> String {
    let mut sum: u32 = 0;
    let mut i = 1;
    for line in input.lines() {
        let mut sets: Vec<Set> = vec![];
        let _sets = line.split(": ").nth(1).unwrap().split("; ").collect::<Vec<&str>>();
        for _set in _sets {
            let cubes = _set.split(", ").collect::<Vec<&str>>();

            let red_cube: Vec<&&str> = cubes.iter().filter(|cube| cube.contains("red")).collect();
            let red = match red_cube.get(0) {
                Some(r) => r.split(" ").nth(0).unwrap().to_string().parse::<u32>().unwrap(),
                None => 0
            };
            
            let green_cube: Vec<&&str> = cubes.iter().filter(|cube| cube.contains("green")).collect();
            let green = match green_cube.get(0) {
                Some(g) => g.split(" ").nth(0).unwrap().to_string().parse::<u32>().unwrap(),
                None => 0
            };

            let blue_cube: Vec<&&str> = cubes.iter().filter(|cube| cube.contains("blue")).collect();
            let blue = match blue_cube.get(0) {
                Some(b) => b.split(" ").nth(0).unwrap().to_string().parse::<u32>().unwrap(),
                None => 0
            };

            let set = Set { red, green, blue };
            sets.push(set);
        }
        let game = Game { n: i, sets};
        sum += game.power();
        i += 1;
    }
    sum.to_string()
}