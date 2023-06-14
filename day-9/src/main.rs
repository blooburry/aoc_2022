use std::collections::HashSet;
use std::fs;
use std::io::Error;
use itertools::Itertools;
use scanf::sscanf;
use tuple::T2;
use day_9::Node;

fn main() {
    let (path, amount_of_followers) = std::env::args().skip(1).next_tuple().expect("provide path and amount pls");
    let amount_of_followers: usize = amount_of_followers.parse().expect("not a valid amount");

    let instructions = parse_input(path).expect("could not parse input");

    let mut cur = Node::new(T2(0, 0), None);

    for _ in 0..amount_of_followers {
        cur = Node::new(T2(0, 0), Some(Box::new(cur)));
    }

    let mut head = cur;
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    for instruction in instructions {
        println!("head position: {:?}", head.position);
        println!("tail position: {:?}", head.child.as_ref().unwrap().position);

        (*instruction)(&mut head, &mut visited);
    }
    println!("head position: {:?}", head.position);
    println!("tail position: {:?}", head.child.as_ref().unwrap().position);

    println!("visited {} places", visited.len());
}

fn parse_input(path: String) -> Result<Vec<Box<dyn Fn(&mut Node, &mut HashSet<(i32, i32)>)>>, Error> {

    let mut res: Vec<Box<dyn Fn(&mut Node, &mut HashSet<_>)>> = vec!();

    for line in fs::read_to_string(path)?.replace("\r", "").trim_end_matches("\n").split("\n") {
        let mut direction: String = String::new();
        let mut amount: u32 = 0;
        sscanf!(line, "{string} {u32}", direction, amount)?;
        let direction = direction.chars().next().unwrap();
        let direction = match direction {
            'U' => T2(1, 0),
            'D' => T2(-1, 0),
            'R' => T2(0, 1),
            'L' => T2(0, -1),
            _ => panic!(),
        };
        for _ in 0..amount {
            res.push(Box::new(move |n, visited|{
                n.move_node(direction, visited)
            }));
        }
    }

    Ok(res)
}
