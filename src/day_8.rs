use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_8_example.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let size_x: usize = lines[0].len();
    let size_y: usize = lines.len();

    let mut positions_by_anthena: HashMap<String, Vec<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();

    let mut map: Vec<Vec<String>> = vec![vec![String::from(""); size_x]; size_y];
    for y in 0..size_y {
        for x in 0..size_x {
            let c: char = lines[y].chars().nth(x).unwrap();
            map[y][x] = c.to_string();
            if c != '.' {
                let s: String = c.to_string();
                let position: Position = Position { x, y };
                if positions_by_anthena.get(&s).is_none() {
                    positions_by_anthena.insert(s, vec![position]);
                } else {
                    let positions: &Vec<Position> = positions_by_anthena.get(&s).unwrap();
                    let new_positions: Vec<Position> = [&positions[..], &[position][..]].concat();
                    positions_by_anthena.insert(s, new_positions);
                }
            }
        }
    }

    for (_, positions) in &positions_by_anthena {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                detect_antinodes(
                    &positions[i],
                    &positions[j],
                    &mut antinodes,
                    &Position {
                        x: size_x - 1,
                        y: size_y - 1,
                    },
                );
            }
        }
    }

    print(&map, &antinodes);
    println!();
    // println!("{:?}", antinodes);
    println!("Found {} distinct antinodes.", antinodes.len());
}

fn detect_antinodes(a: &Position, b: &Position, antinodes: &mut HashSet<Position>, max: &Position) {
    let cpt: usize = antinodes.len();
    if a.x <= b.x {
        if a.y <= b.y {
            // a is top-left from b
            detect_top_left_antinodes(a, b, antinodes, max);
            detect_bottom_right_antinodes(b, a, antinodes, max);
        } else {
            // a is bottom-left from b
            detect_bottom_left_antinodes(a, b, antinodes, max);
            detect_top_right_antinodes(b, a, antinodes, max);
        }
    } else {
        if a.y <= b.y {
            // a is top-right from b
            detect_top_right_antinodes(a, b, antinodes, max);
            detect_bottom_left_antinodes(b, a, antinodes, max);
        } else {
            // a is bottom-right from b
            detect_bottom_right_antinodes(a, b, antinodes, max);
            detect_top_left_antinodes(b, a, antinodes, max);
        }
    }
    if cpt < antinodes.len() {
        antinodes.insert(a.clone());
        antinodes.insert(b.clone());
    }
}

fn detect_top_left_antinodes(
    a: &Position,
    b: &Position,
    antinodes: &mut HashSet<Position>,
    max: &Position,
) {
    let dist_x: usize = a.x.abs_diff(b.x);
    let dist_y: usize = a.y.abs_diff(b.y);
    if a.x >= dist_x && a.y >= dist_y {
        let antinode: Position = Position {
            x: a.x - dist_x,
            y: a.y - dist_y,
        };
        antinodes.insert(antinode.clone());
        detect_top_left_antinodes(&antinode, a, antinodes, max);
    }
}

fn detect_bottom_left_antinodes(
    a: &Position,
    b: &Position,
    antinodes: &mut HashSet<Position>,
    max: &Position,
) {
    let dist_x: usize = a.x.abs_diff(b.x);
    let dist_y: usize = a.y.abs_diff(b.y);
    if a.x >= dist_x && a.y + dist_y <= max.y {
        let antinode: Position = Position {
            x: a.x - dist_x,
            y: a.y + dist_y,
        };
        antinodes.insert(antinode.clone());
        detect_bottom_left_antinodes(&antinode, a, antinodes, max);
    }
}

fn detect_top_right_antinodes(
    a: &Position,
    b: &Position,
    antinodes: &mut HashSet<Position>,
    max: &Position,
) {
    let dist_x: usize = a.x.abs_diff(b.x);
    let dist_y: usize = a.y.abs_diff(b.y);
    if a.x + dist_x <= max.x && a.y >= dist_y {
        let antinode: Position = Position {
            x: a.x + dist_x,
            y: a.y - dist_y,
        };
        antinodes.insert(antinode.clone());
        detect_top_right_antinodes(&antinode, a, antinodes, max);
    }
}

fn detect_bottom_right_antinodes(
    a: &Position,
    b: &Position,
    antinodes: &mut HashSet<Position>,
    max: &Position,
) {
    let dist_x: usize = a.x.abs_diff(b.x);
    let dist_y: usize = a.y.abs_diff(b.y);
    if a.x + dist_x <= max.x && a.y + dist_y <= max.y {
        let antinode: Position = Position {
            x: a.x + dist_x,
            y: a.y + dist_y,
        };
        antinodes.insert(antinode.clone());
        detect_bottom_right_antinodes(&antinode, a, antinodes, max);
    }
}

fn print(map: &Vec<Vec<String>>, antinodes: &HashSet<Position>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if antinodes.contains(&Position { x, y }) {
                print!("# ");
            } else {
                print!("{} ", map[y][x]);
            }
        }
        println!();
    }
}
