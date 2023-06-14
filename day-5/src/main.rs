use std::{fs, io};
use std::io::ErrorKind;
use std::ops::{Add, Div};
use itertools::Itertools;
use scanf::sscanf;

use day_5::CrateStack;

fn main() {
    let (mut stacks, instructions) = parse_input("rsrc/input_0.txt").expect("could not parse input");

    for instruction in instructions {
        instruction(&mut stacks).expect("Something went wrong while executing the instructions");
    }

    let mut crates_on_top = String::new();
    for stack in stacks {
        crates_on_top.push(*stack.get_last_crate().unwrap_or(&' '));
    }
    println!("{}", crates_on_top);
    println!("Hello, world!");
}

fn parse_input(path: &str) -> Result<(Vec<CrateStack>, Vec<Box<dyn Fn(&mut Vec<CrateStack>) -> Result<(), io::Error>>>), io::Error> {
    let lines = fs::read_to_string(path)?;
    let lines = lines.replace("\r", "");

    let (cargo, instructions): (&str, &str)
        = lines
        .trim_end_matches("\n")
        .split("\n\n")
        .next_tuple()
        .ok_or(io::Error::new(ErrorKind::InvalidInput, "No cargo or instructions"))?;

    fn parse_cargo(cargo: &str) -> Result<Vec<CrateStack>, io::Error> {
        let stacks_amount = cargo.split("\n").last()
            .ok_or(io::Error::new(ErrorKind::InvalidInput, "cargo empty"))?
            .chars().count().add(3).div(4);

        let mut stacks = Vec::with_capacity(stacks_amount);
        for _ in 1..=stacks_amount {stacks.push(CrateStack::new());}

        for line in cargo.split("\n"){
            for (pos, char) in line.chars().enumerate(){
                if char.is_uppercase() {stacks.get_mut((pos + 3) / 4 - 1)
                    .unwrap()
                    .build_stack(char)}
            }
        }

        Ok(stacks)
    }

    fn parse_instructions(instructions: &str) ->
    Result<Vec<Box<dyn Fn(&mut Vec<CrateStack>) -> Result<(), io::Error>>>, io::Error> {

        let mut res: Vec<Box<dyn Fn(&mut Vec<CrateStack>) -> Result<(), io::Error>>> = Vec::new();

        for line in instructions.split("\n") {
            let (mut amount, mut from_stack, mut to_stack): (u32, u32, u32) = (0, 0, 0);

            sscanf!(line, "move {u32} from {u32} to {u32}", amount, from_stack, to_stack)?;
            let (from_stack, to_stack) = (from_stack as usize - 1, to_stack as usize - 1);

            res.push(Box::new(move | stacks: &mut Vec<CrateStack> | {
                let mut crates_to_be_moved: Vec<char> = Vec::new();
                for _ in 0..amount {
                    let c = stacks.get_mut(from_stack)
                        .ok_or(io::Error::new(ErrorKind::InvalidInput, "cannot move crate from nonexistent stack"))?
                        .remove_crate()
                        .ok_or(io::Error::new(ErrorKind::InvalidInput, "stack empty"))?;
                    crates_to_be_moved.push(c);
                }
                while let Some(c) = crates_to_be_moved.pop() {
                    stacks.get_mut(to_stack)
                        .ok_or(io::Error::new(ErrorKind::InvalidInput, "cannot move crate to nonexistent stack"))?
                        .add_crate(c);
                }
                Ok(())
            }));
        }
        Ok(res)
    }

    Ok((parse_cargo(cargo)?, parse_instructions(instructions)?))
}