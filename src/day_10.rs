use std::{collections::HashSet, fs, usize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Spot {
    position: Position,
    value: usize,
    nexts: Vec<Spot>,
}

const VERBOSE: bool = true;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_10.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let mut map: Vec<Vec<usize>> = vec![vec![0; lines[0].len()]; lines.len()];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            map[y][x] = lines[y]
                .chars()
                .nth(x)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
        }
    }

    let mut trail_heads: Vec<Spot> = vec![];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if map[y][x] == 0 {
                trail_heads.push(Spot {
                    position: Position { x, y },
                    value: 0,
                    nexts: vec![],
                });
            }
        }
    }

    trail_heads
        .iter_mut()
        .for_each(|spot: &mut Spot| compute_nexts(spot, &map));

    let mut count_distinct_scores: usize = 0;
    let mut count_scores: usize = 0;
    for trail_head in trail_heads {
        let mut distinct_final_spots: HashSet<Spot> = HashSet::new();
        let mut final_spots: Vec<Spot> = vec![];
        get_final_spots(&trail_head, &mut distinct_final_spots, &mut final_spots);
        count_distinct_scores += distinct_final_spots.len();
        count_scores += final_spots.len();
    }

    println!("Sum of distinct scores: {count_distinct_scores}");
    println!("Sum of scores: {count_scores}");
}

fn compute_nexts(spot: &mut Spot, map: &Vec<Vec<usize>>) {
    compute_next(spot, map);
    spot.nexts
        .iter_mut()
        .for_each(|spot| compute_nexts(spot, map));
}

fn compute_next(spot: &mut Spot, map: &Vec<Vec<usize>>) {
    let mut min_y: usize = spot.position.y;
    let mut max_y: usize = spot.position.y;
    if min_y > 0 {
        min_y -= 1;
    }
    if max_y < map.len() - 1 {
        max_y += 1;
    }
    let mut min_x: usize = spot.position.x;
    let mut max_x: usize = spot.position.x;
    if min_x > 0 {
        min_x -= 1;
    }
    if max_x < map[0].len() - 1 {
        max_x += 1;
    }
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if !(y != spot.position.y && x != spot.position.x) {
                if map[y][x] == spot.value + 1 {
                    let next_spot: Spot = Spot {
                        position: Position { x, y },
                        value: spot.value + 1,
                        nexts: vec![],
                    };
                    spot.nexts.push(next_spot);
                }
            }
        }
    }
}

fn get_final_spots(
    spot: &Spot,
    distinct_final_spots: &mut HashSet<Spot>,
    final_spots: &mut Vec<Spot>,
) {
    if spot.value == 9 {
        distinct_final_spots.insert(spot.clone());
        final_spots.push(spot.clone());
    }
    spot.nexts
        .iter()
        .for_each(|s: &Spot| get_final_spots(s, distinct_final_spots, final_spots));
}
