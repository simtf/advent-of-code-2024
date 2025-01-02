use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spot {
    position: Position,
    direction: Direction,
    score: usize,
    parent: Option<Rc<Spot>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Case {
    code: char,
    score: usize,
}

const VERBOSE: bool = false;

const TEST: bool = false;

const PATH: &str = if TEST {
    "./resources/day_16_example.txt"
} else {
    "./resources/day_16.txt"
};

pub fn execute() {
    let dataset: String = fs::read_to_string(PATH).unwrap();
    let lines: Vec<&str> = dataset.split("\n").collect();

    let mut start_spot = Spot {
        position: Position { x: 0, y: 0 },
        direction: Direction::East,
        score: 0,
        parent: None,
    };

    let mut map: Vec<Vec<Case>> = vec![
        vec![
            Case {
                code: '.',
                score: usize::MAX - 1001,
            };
            lines[0].len()
        ];
        lines.len()
    ];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            map[y][x].code = lines[y].chars().nth(x).unwrap();
            if map[y][x].code == 'S' {
                start_spot.position = Position { x, y };
            }
        }
    }
    if VERBOSE {
        println!();
        print(&map);
    }

    let mut final_spots: Vec<Spot> = Vec::new();
    compute_nexts(&mut Rc::new(start_spot), &mut map, &mut final_spots);
    let min: usize = final_spots
        .iter()
        .map(|s| s.score)
        .min()
        .expect("No end spot found");

    println!();
    println!("Minimum score: {min}");

    let mut best_positions: HashSet<Position> = HashSet::new();
    for final_spot in final_spots {
        if final_spot.score == min {
            best_positions.insert(final_spot.position.clone());
            compute_best_positions(&final_spot.parent.unwrap(), &mut best_positions);
        }
    }
    if VERBOSE {
        println!();
        print_best_positions(&map, &best_positions);
    }

    println!();
    println!(
        "{} tiles are part of at least one of the best paths",
        best_positions.len()
    );
}

fn compute_nexts(spot: &mut Rc<Spot>, map: &mut Vec<Vec<Case>>, final_spots: &mut Vec<Spot>) {
    let mut nexts: Vec<Rc<Spot>> = Vec::new();
    compute_north(spot, map, final_spots).map(|next| nexts.push(Rc::new(next)));
    compute_east(spot, map, final_spots).map(|next| nexts.push(Rc::new(next)));
    compute_south(spot, map, final_spots).map(|next| nexts.push(Rc::new(next)));
    compute_west(spot, map, final_spots).map(|next| nexts.push(Rc::new(next)));
    for mut next in nexts {
        compute_nexts(&mut next, map, final_spots);
    }
}

fn compute_best_positions(spot: &Rc<Spot>, best_positions: &mut HashSet<Position>) {
    best_positions.insert(spot.position.clone());
    if spot.parent.is_some() {
        let parent = Rc::clone(&spot.parent.as_ref().unwrap());
        compute_best_positions(&parent, best_positions);
    }
}

fn compute_north(
    spot: &Rc<Spot>,
    map: &mut Vec<Vec<Case>>,
    final_spots: &mut Vec<Spot>,
) -> Option<Spot> {
    compute_next(
        spot.position.x,
        spot.position.y - 1,
        Direction::North,
        spot,
        map,
        final_spots,
    )
}

fn compute_east(
    spot: &Rc<Spot>,
    map: &mut Vec<Vec<Case>>,
    final_spots: &mut Vec<Spot>,
) -> Option<Spot> {
    compute_next(
        spot.position.x + 1,
        spot.position.y,
        Direction::East,
        spot,
        map,
        final_spots,
    )
}

fn compute_south(
    spot: &Rc<Spot>,
    map: &mut Vec<Vec<Case>>,
    final_spots: &mut Vec<Spot>,
) -> Option<Spot> {
    compute_next(
        spot.position.x,
        spot.position.y + 1,
        Direction::South,
        spot,
        map,
        final_spots,
    )
}

fn compute_west(
    spot: &Rc<Spot>,
    map: &mut Vec<Vec<Case>>,
    final_spots: &mut Vec<Spot>,
) -> Option<Spot> {
    compute_next(
        spot.position.x - 1,
        spot.position.y,
        Direction::West,
        spot,
        map,
        final_spots,
    )
}

fn compute_next(
    x: usize,
    y: usize,
    direction: Direction,
    spot: &Rc<Spot>,
    map: &mut Vec<Vec<Case>>,
    final_spots: &mut Vec<Spot>,
) -> Option<Spot> {
    if ['.', 'E'].contains(&map[y][x].code) {
        let next = Spot {
            position: Position { x, y },
            score: compute_score(&spot, &direction),
            direction,
            parent: Some(Rc::clone(spot)),
        };
        if map[y][x].code == 'E' {
            if VERBOSE {
                println!("Score: {:?}", next.score)
            }
            final_spots.push(next);
        } else if next.score <= map[y][x].score + 1001 {
            map[y][x].score = next.score;
            return Some(next);
        }
    }
    None
}

fn compute_score(spot: &Spot, direction: &Direction) -> usize {
    if spot.direction == *direction {
        spot.score + 1
    } else {
        spot.score + 1001
    }
}

fn print(map: &Vec<Vec<Case>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x].code);
        }
        println!();
    }
}

fn print_best_positions(map: &Vec<Vec<Case>>, best_positions: &HashSet<Position>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if best_positions.contains(&Position { x, y }) {
                print!("0");
            } else {
                print!("{}", map[y][x].code);
            }
        }
        println!();
    }
}
