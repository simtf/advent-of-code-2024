use std::fs;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spot {
    plant: char,
    position: Position,
}

const VERBOSE: bool = true;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_12_example.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\n").collect();

    let mut map: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];

    let mut regions: Vec<Vec<Spot>> = vec![];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            map[y][x] = lines[y].chars().nth(x).unwrap();
        }
    }

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let region: Vec<Spot> = compute_region(x, y, &map, &mut visited);
            if region.len() > 0 {
                regions.push(region);
            }
        }
    }

    let mut count_faces: usize = 0;
    for region in regions {
        let area = region.len();
        let faces = compute_faces(&region, &map);
        let price = area.mul(faces);
        count_faces += faces;
        if VERBOSE {
            println!(
                "Plant {:?}, area: {:?}, faces: {:?}, price: {:?}",
                region[0].plant, area, faces, price,
            );
        }
    }
    println!("Total price: {}", count_faces);
}

fn compute_region(
    x: usize,
    y: usize,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<Spot> {
    let mut region: Vec<Spot> = vec![];
    let mut min_y: usize = y;
    let mut max_y: usize = y;
    if min_y > 0 {
        min_y -= 1;
    }
    if max_y < map.len() - 1 {
        max_y += 1;
    }
    let mut min_x: usize = x;
    let mut max_x: usize = x;
    if min_x > 0 {
        min_x -= 1;
    }
    if max_x < map[0].len() - 1 {
        max_x += 1;
    }
    let plant: char = map[y][x];
    for cy in min_y..max_y + 1 {
        for cx in min_x..max_x + 1 {
            if !(cy != y && cx != x) && !visited[cy][cx] && map[cy][cx] == plant {
                visited[cy][cx] = true;
                region.push(Spot {
                    plant: plant,
                    position: Position { x: cx, y: cy },
                });
                compute_region(cx, cy, map, visited)
                    .iter()
                    .for_each(|r| region.push(r.clone()));
            }
        }
    }
    region
}

fn compute_faces(region: &Vec<Spot>, map: &Vec<Vec<char>>) -> usize {
    let mut faces: usize = 0;
    faces
}
