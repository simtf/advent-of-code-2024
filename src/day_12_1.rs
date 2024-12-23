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
    let dataset: String = fs::read_to_string("./resources/day_12.txt").unwrap();
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

    let mut count_price: usize = 0;
    for region in regions {
        let area = region.len();
        let perimeter = compute_perimeter(&region, &map);
        let price = area.mul(perimeter);
        count_price += price;
        if VERBOSE {
            println!(
                "Plant {:?}, area: {:?}, perimeter: {:?}, price: {:?}",
                region[0].plant, area, perimeter, price,
            );
        }
    }
    println!("Total price: {}", count_price);
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

fn compute_perimeter(region: &Vec<Spot>, map: &Vec<Vec<char>>) -> usize {
    let mut perimeter: usize = 0;
    for spot in region {
        let x = spot.position.x;
        let y = spot.position.y;
        if y == 0 || map[y - 1][x] != spot.plant {
            perimeter += 1;
        }
        if x == map[y].len() - 1 || map[y][x + 1] != spot.plant {
            perimeter += 1;
        }
        if y == map.len() - 1 || map[y + 1][x] != spot.plant {
            perimeter += 1;
        }
        if x == 0 || map[y][x - 1] != spot.plant {
            perimeter += 1;
        }
    }
    perimeter
}
