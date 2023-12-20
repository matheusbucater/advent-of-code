use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: u32,
    y: u32
}

pub fn main(input: &str) -> String {
    let mut y = 0;
    
    let mut nums: HashMap<u32, (Position, Position)> = HashMap::new();

    let mut spos: Position = Position { x: 0, y: 0};
    let mut epos: Position = Position { x: 0, y: 0};
    
    let mut pos: Position = Position { x: 0, y: 0};

    for line in input.lines() {
        let mut num = 0;
        let mut numx = 1;
        for i in 0..line.len() {
            let c = line.chars().rev().nth(i).unwrap();
            dbg!(c, numx, num);
            if c.is_numeric() {
                dbg!("a");
                if num != 0 {
                    spos.x = i as u32;
                    spos.y = y;
                }
                num += c as u32 * numx;
                numx *= 10;

            } else {
                if num != 0 {
                    epos.x = i as u32 - 1;
                    epos.y = y;
                    nums.insert(num, (spos, epos));
                    num = 0;
                    numx = 1;
                }
                if c != '.' {
                    pos.x = i as u32;
                    pos.y = y;
                }
            }
        }
        dbg!(&nums);
        y += 1;
    }
    "4361".to_string()
}