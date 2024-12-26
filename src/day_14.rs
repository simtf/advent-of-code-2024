use std::{fs, io};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Robot {
    id: usize,
    position: Position,
    velocity: Velocity,
}

const VERBOSE: bool = false;

const TEST: bool = false;

const PATH: &str = if TEST {
    "./resources/day_14_example.txt"
} else {
    "./resources/day_14.txt"
};

const SIZE_X: usize = if TEST { 11 } else { 101 };
const SIZE_Y: usize = if TEST { 7 } else { 103 };

pub fn execute() {
    let dataset: String = fs::read_to_string(PATH).unwrap();
    let lines: Vec<&str> = dataset.split("\n").collect();

    let mut robots: Vec<Robot> = Vec::new();
    let mut id: usize = 0;
    for line in lines {
        let params: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let position: Vec<usize> = params[0]
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let velocity: Vec<isize> = params[1]
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        robots.push(Robot {
            id: id,
            position: Position {
                x: position[0],
                y: position[1],
            },
            velocity: Velocity {
                x: velocity[0],
                y: velocity[1],
            },
        });
        id += 1;
    }

    for i in 0..100 {
        for robot in &mut robots {
            next(robot);
        }
        if VERBOSE {
            clearscreen::clear().unwrap();
            robots.iter().for_each(|r| println!("{:?}", r));
            println!("Iteration: {}", i + 1);
            print(&robots);
            println!();
            println!("Press any key to continue...");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    let mut count_top_left: usize = 0;
    let mut count_top_right: usize = 0;
    let mut count_bottom_left: usize = 0;
    let mut count_bottom_right: usize = 0;
    for robot in &mut robots {
        if robot.position.x < SIZE_X / 2 && robot.position.y < SIZE_Y / 2 {
            count_top_left += 1;
        } else if robot.position.x > SIZE_X / 2 && robot.position.y < SIZE_Y / 2 {
            count_top_right += 1;
        } else if robot.position.x < SIZE_X / 2 && robot.position.y > SIZE_Y / 2 {
            count_bottom_left += 1;
        } else if robot.position.x > SIZE_X / 2 && robot.position.y > SIZE_Y / 2 {
            count_bottom_right += 1;
        }
    }

    println!("top-left: {}", count_top_left);
    println!("top-right: {}", count_top_right);
    println!("bottom-left: {}", count_bottom_left);
    println!("bottom-right: {}", count_bottom_right);
    println!(
        "Safety factor: {}",
        count_top_left * count_top_right * count_bottom_left * count_bottom_right
    );
}

fn next(robot: &mut Robot) {
    let next_x: usize = (robot.position.x + SIZE_X)
        .checked_add_signed(robot.velocity.x)
        .unwrap()
        % SIZE_X;
    let next_y: usize = (robot.position.y + SIZE_Y)
        .checked_add_signed(robot.velocity.y)
        .unwrap()
        % SIZE_Y;
    robot.position = Position {
        x: next_x,
        y: next_y,
    }
}

fn print(robots: &Vec<Robot>) {
    let mut map: Vec<Vec<usize>> = vec![vec![0; SIZE_X]; SIZE_Y];
    for robot in robots {
        map[robot.position.y][robot.position.x] += 1;
    }
    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            if map[y][x] == 0 {
                print!(". ");
            } else {
                print!("{} ", map[y][x]);
            }
        }
        println!();
    }
}
