use itertools::Itertools;
use std::{fs, io};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

const VERBOSE: bool = false;

const TEST: bool = false;

const PATH: &str = if TEST {
    "./resources/day_15_example.txt"
} else {
    "./resources/day_15.txt"
};

pub fn execute() {
    let dataset: String = fs::read_to_string(PATH).unwrap();
    let datasets: Vec<String> = dataset.split("\n\n").map(|s| s.to_string()).collect();

    let dataset_map: Vec<String> = datasets[0].split("\n").map(|s| s.to_string()).collect();
    let moves: String = datasets[1].replace("\n", "");

    let mut robot = Position { x: 0, y: 0 };
    let mut map: Vec<Vec<char>> = vec![vec!['.'; dataset_map[0].len()]; dataset_map.len()];
    for y in 0..dataset_map.len() {
        for x in 0..dataset_map[0].len() {
            map[y][x] = dataset_map[y].chars().nth(x).unwrap();
            if map[y][x] == '@' {
                robot.x = x;
                robot.y = y;
            }
        }
    }

    for mov in moves.chars() {
        match mov {
            '>' => move_right(&mut robot, &mut map),
            'v' => move_down(&mut robot, &mut map),
            '<' => move_left(&mut robot, &mut map),
            '^' => move_up(&mut robot, &mut map),
            _ => continue,
        }
        if VERBOSE {
            clearscreen::clear().unwrap();
            println!("Move: {}", mov);
            println!();
            print(&map);
            println!();
            println!("Press any key to continue...");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    let mut sum: usize = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                sum = sum + 100 * y + x;
            }
        }
    }
    println!("The sum of all boxes' GPS coordinates is: {sum}");
}

fn move_right(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    for x in robot.x + 1..map[0].len() {
        match map[robot.y][x] {
            '#' => break,
            '.' => {
                map[robot.y][robot.x] = '.';
                if x - 1 == robot.x {
                    map[robot.y][x] = '@';
                } else {
                    map[robot.y][x] = 'O';
                    map[robot.y][robot.x + 1] = '@';
                }
                robot.x += 1;
                break;
            }
            _ => continue,
        }
    }
}

fn move_down(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    for y in robot.y + 1..map.len() {
        match map[y][robot.x] {
            '#' => break,
            '.' => {
                map[robot.y][robot.x] = '.';
                if y - 1 == robot.y {
                    map[y][robot.x] = '@';
                } else {
                    map[y][robot.x] = 'O';
                    map[robot.y + 1][robot.x] = '@';
                }
                robot.y += 1;
                break;
            }
            _ => continue,
        }
    }
}

fn move_left(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    for x in (0..robot.x).rev() {
        match map[robot.y][x] {
            '#' => break,
            '.' => {
                map[robot.y][robot.x] = '.';
                if x + 1 == robot.x {
                    map[robot.y][x] = '@';
                } else {
                    map[robot.y][x] = 'O';
                    map[robot.y][robot.x - 1] = '@';
                }
                robot.x -= 1;
                break;
            }
            _ => continue,
        }
    }
}

fn move_up(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    for y in (0..robot.y).rev() {
        match map[y][robot.x] {
            '#' => break,
            '.' => {
                map[robot.y][robot.x] = '.';
                if y + 1 == robot.y {
                    map[y][robot.x] = '@';
                } else {
                    map[y][robot.x] = 'O';
                    map[robot.y - 1][robot.x] = '@';
                }
                robot.y -= 1;
                break;
            }
            _ => continue,
        }
    }
}

fn print(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
