// seed-to-soil

use std::vec;

struct Map {
   // source_category: &'a str,
    source_range_start: Vec<u32>,
    // destination_category: &'a str,
    destination_range_start: Vec<u32>,
    range_length: Vec<u32>
}

pub fn main(input: &str) -> String {

    // let seeds = input.lines()
    //                         .nth(0)
    //                         .unwrap()
    //                         .split("seeds: ")
    //                         .nth(1)
    //                         .unwrap()
    //                         .split(" ")
    //                         .collect::<Vec<&str>>();
    
    let almanac = input.split("\n\n").collect::<Vec<&str>>();

    let (seeds_input, maps_input) = almanac.split_at(1);

    let binding = seeds_input.join("");
    let seeds = binding
                                .split("seeds: ")
                                .nth(1)
                                .unwrap()
                                .split(" ")
                                .collect::<Vec<&str>>();

    
    dbg!(maps_input);

    dbg!(seeds);

    dbg!(almanac.split_at(1));

    // for i in 0..maps.len() {
    //     let map = maps[i];
    //     dbg!(map);
    // }

    
    // for i in 0..input.lines().filter(|x| !x.contains("seeds: ")).collect::<Vec<&str>>().len() {
    //     let mut c = 0;
    //     let line = input.lines().nth(i).unwrap();

    //     dbg!(input.split("\n\n").collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>()[1]);
    //     if !line.is_empty() {
    //         let mut count_map_lines = 1;
    //         let mut vsrs = vec![];
    //         let mut vdrs = vec![];
    //         let mut vrl = vec![];

    //         loop {

    //             let next_line = input.lines().nth(i+count_map_lines).unwrap();

    //             if next_line.is_empty() {
    //                 break;
    //             } else {

    //                 let mut iter  = next_line.split(" "); 
                
    //                 let drs = iter.next().unwrap();
    //                 let srs = iter.next().unwrap();
    //                 let rl = iter.next().unwrap();
    
    //                 vdrs.push(drs);
    //                 vsrs.push(srs);
    //                 vrl.push(rl);
                    
    //                 c += 1;
    //                 count_map_lines += 1;

    //             }
    //         }


    //         dbg!(vsrs);
    //     }
    // }
    "todo!()".to_string()
}