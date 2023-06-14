use std::{fs, io};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::ErrorKind;

fn main() {
    let path = std::env::args().skip(1).next().expect("file path missing");
    let start_of_message = parse_input(path).expect("Could not parse input");
    println!("Message starts at index {}", start_of_message);
}

fn parse_input(path: String) -> Result<usize, io::Error> {
    let mut occurrences: HashMap<char, u32> = HashMap::new();
    let mut duplicates: u32 = 0;
    let chars =  fs::read_to_string(path)?.chars().enumerate().collect::<Vec<(usize, char)>>();

    for i in 0..chars.len() {
        // add a char to the map
        if let Some(v) = occurrences.get(&chars.get(i).unwrap().1){
            duplicates += 1;
            occurrences.insert(chars.get(i).unwrap().1, v + 1);
        } else {
            occurrences.insert(chars.get(i).unwrap().1, 1);
        }

        // remove a char from the map
        if let Some(j) = i.checked_sub(14) {
            if *occurrences.get(&chars.get(j).unwrap().1).unwrap() > 1 {
                duplicates -= 1;
                occurrences.entry(chars.get(j).unwrap().1).and_modify(|v| {*v -= 1});
            } else {
                occurrences.remove(&chars.get(j).unwrap().1);
            }

            if duplicates == 0 { return Ok(i + 1) }
        }

        println!("Iteration: {}, occurrences: {:?}", i, occurrences);
    }

    Err(io::Error::new(ErrorKind::InvalidData, "could not find start of message"))
}
