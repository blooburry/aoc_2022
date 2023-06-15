use std::fs;
use std::io::Error;
use day_10::{Sprite, Operation};

fn main() {
    let path = std::env::args().skip(1).next().expect("pls provide path");
    let mut ops = parse_input(path).expect("couldn't parse input");

    ops.push(Operation::Noop);

    let mut sprite = Sprite::new(ops);

    println!("{:?}", &sprite);

    let mut crt_output = String::new();
    let mut crt_cursor: u32 = 0;

    while let Ok(_) = sprite.tick() {

        println!("cycle: {} sprite position: {} cursor position: {}", sprite.cycle, sprite.position, crt_cursor);

        crt_output.push(if (crt_cursor as i32 - sprite.position).abs() <= 1 {'#'} else {'.'});

        if (crt_cursor + 1) % 40 == 0 { crt_output.push('\n'); }

        println!("{crt_output}\n");

        crt_cursor = (crt_cursor + 1) % 40;
    };

    println!("{crt_output}");
}

fn parse_input(path: String) -> Result<Vec<Operation>, Error> {
    let mut res = vec!();
    for line in fs::read_to_string(path)?.replace("\r", "").trim_end_matches("\n").split("\n") {
        res.push(Operation::build(line)?);
    }

    Ok(res)
}

fn truncate_string(input: &str, max_length: usize) -> &str {
    if input.len() <= max_length {
        input
    } else {
        &input[..max_length]
    }
}