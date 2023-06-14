use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind};
use day_8::Tree;

fn main() {
    let mut forest = parse_input("resources/input_1cd.txt").expect("could not parse input");

    // println!("the forest: {}", generate_grid_string(&forest.iter().flatten().collect()));

    let ref_forest = forest.clone();

    for (y, row) in forest.iter_mut().enumerate() {
        for (x, t) in row.iter_mut().enumerate() {

            // println!("Calculating the scenic score of tree at row {}, column {} with height {}", y, x, t.height());

            let trees_to_the_south: Vec<&Tree> = ref_forest.iter()
                .skip(y + 1)
                .map(|r| &r[x])
                .collect();
            let trees_to_the_north: Vec<&Tree> = ref_forest.iter()
                .take(y)
                .map(|r| &r[x])
                .rev()
                .collect();
            let trees_to_the_east: Vec<&Tree> = ref_forest.get(y).unwrap()
                .iter()
                .skip(x + 1)
                .collect();

            let trees_to_the_west: Vec<&Tree> = ref_forest.get(y).unwrap()
                .iter()
                .take(x)
                .rev()
                .collect();

            // println!("trees to the north: {:?}", trees_to_the_north);
            // println!("trees to the south: {:?}", trees_to_the_south);
            // println!("trees to the east: {:?}", trees_to_the_east);
            // println!("trees to the west: {:?}", trees_to_the_west);

            t.scenic_score = trees_visible_from_tree_in_row(trees_to_the_south, t)
                * trees_visible_from_tree_in_row(trees_to_the_north, t)
                * trees_visible_from_tree_in_row(trees_to_the_east, t)
                * trees_visible_from_tree_in_row(trees_to_the_west, t);
        }
    }

    println!("{}", generate_grid_string(&forest.iter().flatten().collect::<HashSet<_>>()));

    let t = forest.iter().flatten().map(|t| t.scenic_score).max().unwrap();

    println!("map of scenery: \n{}", generate_grid_string_scenery(&forest.iter().flatten().collect()));
    println!("best view is {}", t);
}

fn trees_visible_from_tree_in_row(row: Vec<&Tree>, tree: &Tree) -> u32 {

    let mut res = 0;

    for t in row {
        res += 1;
        if t.height() >= tree.height() {break;}
    }

    // println!("score in this direction: {}", res);

    res
}

fn parse_input(path: &str) -> Result<Vec<Vec<Tree>>, Error> {

    fs::read_to_string(path)?
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    c.to_digit(10)
                        .ok_or(Error::new(ErrorKind::InvalidData, "couldn't parse char"))
                        .map(|d| Tree::new((i as u32, j as u32), d))
                })
                .collect::<Result<Vec<Tree>, Error>>()
        })
        .collect()

}

fn generate_grid_string(tree_set: &HashSet<&Tree>) -> String {
    let max_row = tree_set.iter().map(|t| t.position().0).max().unwrap_or(0);
    let max_col = tree_set.iter().map(|t| t.position().1).max().unwrap_or(0);

    let mut grid: Vec<Vec<String>> = vec![vec!["_".to_owned(); (max_col + 1) as usize]; (max_row + 1) as usize];

    for t in tree_set {
        let (row, col) = t.position();
        let weight = t.height().to_string(); // Get the first character of the weight
        grid[row as usize][col as usize] = weight;
    }

    let mut grid_string = String::new();
    for row in &grid {
        for weight in row {
            grid_string.push_str(&format!("{:>3} ", weight));
        }
        grid_string.push('\n');
    }

    grid_string
}

fn generate_grid_string_scenery(tree_set: &HashSet<&Tree>) -> String {
    let max_row = tree_set.iter().map(|t| t.position().0).max().unwrap_or(0);
    let max_col = tree_set.iter().map(|t| t.position().1).max().unwrap_or(0);

    let mut grid: Vec<Vec<String>> = vec![vec!["_".to_owned(); (max_col + 1) as usize]; (max_row + 1) as usize];

    for t in tree_set {
        let (row, col) = t.position();
        let weight = t.scenic_score.to_string(); // Get the first character of the weight
        grid[row as usize][col as usize] = weight;
    }

    let mut grid_string = String::new();
    for row in &grid {
        for weight in row {
            grid_string.push_str(&format!("{:>3} ", weight));
        }
        grid_string.push('\n');
    }

    grid_string
}