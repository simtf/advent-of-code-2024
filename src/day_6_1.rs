use std::{collections::HashSet, fs, io};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Spot {
    Empty,
    Obstacle,
    Out,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Guard {
    position: Position,
    direction: Direction,
    visited_positions: HashSet<Position>,
}

const VERBOSE: bool = false;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_6.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let size_x: usize = lines[0].len();
    let size_y: usize = lines.len();

    let mut guard: Guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::North,
        visited_positions: HashSet::new(),
    };

    let mut map: Vec<Vec<Spot>> = vec![vec![Spot::Empty; size_x]; size_y];
    for y in 0..size_y {
        for x in 0..size_x {
            let c: char = lines[y].chars().nth(x).unwrap();
            if c == '#' {
                map[y][x] = Spot::Obstacle;
            }
            if c == '^' {
                guard.position = Position { x, y };
                guard.visited_positions.insert(Position { x, y });
            }
        }
    }

    let mut next_spot: Spot = go_forward(&map, &mut guard);
    while next_spot != Spot::Out {
        next_spot = go_forward(&map, &mut guard);
    }
    println!(
        "The guard visited {} distinct positions.",
        guard.visited_positions.len()
    );
}

fn go_forward(map: &Vec<Vec<Spot>>, guard: &mut Guard) -> Spot {
    if VERBOSE {
        print(&map, &guard);
    }
    match guard.direction {
        Direction::North => {
            if guard.position.y <= 0 {
                return Spot::Out;
            } else if map[guard.position.y - 1][guard.position.x] == Spot::Obstacle {
                turn_90_degrees(guard);
                return go_forward(map, guard);
            } else {
                guard.position.y = guard.position.y - 1;
                guard.visited_positions.insert(Position {
                    x: guard.position.x,
                    y: guard.position.y,
                });
                return Spot::Empty;
            }
        }
        Direction::East => {
            if guard.position.x >= map[0].len() - 1 {
                return Spot::Out;
            } else if map[guard.position.y][guard.position.x + 1] == Spot::Obstacle {
                turn_90_degrees(guard);
                return go_forward(map, guard);
            } else {
                guard.position.x = guard.position.x + 1;
                guard.visited_positions.insert(Position {
                    x: guard.position.x,
                    y: guard.position.y,
                });
                return Spot::Empty;
            }
        }
        Direction::South => {
            if guard.position.y >= map.len() - 1 {
                return Spot::Out;
            } else if map[guard.position.y + 1][guard.position.x] == Spot::Obstacle {
                turn_90_degrees(guard);
                return go_forward(map, guard);
            } else {
                guard.position.y = guard.position.y + 1;
                guard.visited_positions.insert(Position {
                    x: guard.position.x,
                    y: guard.position.y,
                });
                return Spot::Empty;
            }
        }
        Direction::West => {
            if guard.position.x <= 0 {
                return Spot::Out;
            } else if map[guard.position.y][guard.position.x - 1] == Spot::Obstacle {
                turn_90_degrees(guard);
                return go_forward(map, guard);
            } else {
                guard.position.x = guard.position.x - 1;
                guard.visited_positions.insert(Position {
                    x: guard.position.x,
                    y: guard.position.y,
                });
                return Spot::Empty;
            }
        }
    }
}

fn turn_90_degrees(guard: &mut Guard) {
    match guard.direction {
        Direction::North => guard.direction = Direction::East,
        Direction::East => guard.direction = Direction::South,
        Direction::South => guard.direction = Direction::West,
        Direction::West => guard.direction = Direction::North,
    }
}

fn spot_code(spot: &Spot) -> char {
    return match spot {
        Spot::Empty => '.',
        Spot::Obstacle => '#',
        Spot::Out => '*',
    };
}

fn guard_code(guard: &Guard) -> char {
    return match guard.direction {
        Direction::North => '^',
        Direction::East => '>',
        Direction::South => 'v',
        Direction::West => '<',
    };
}

fn print(map: &Vec<Vec<Spot>>, guard: &Guard) {
    clearscreen::clear().unwrap();
    println!();
    println!(
        "Guard is at position ({}, {}).",
        guard.position.x, guard.position.y
    );
    println!(
        "Guard has visited {} distinct positions",
        guard.visited_positions.len()
    );
    println!("Guard is going to {:?}", guard.direction);
    println!();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if guard.position.x == x && guard.position.y == y {
                let c: char = guard_code(&guard);
                print!("{c} ");
            } else {
                let c: char = spot_code(&map[y][x]);
                print!("{c} ");
            }
        }
        println!();
    }
    println!();
    println!("Press any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
