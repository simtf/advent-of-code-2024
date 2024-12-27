use itertools::Itertools;
use std::process::exit;
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

    let dataset_map: Vec<String> = datasets[0]
        .split("\n")
        .map(|s| s.to_string())
        .map(|s| s.replace("#", "##"))
        .map(|s| s.replace("O", "[]"))
        .map(|s| s.replace(".", ".."))
        .map(|s| s.replace("@", "@."))
        .collect();
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

    let mut cpt = 0;
    let mut mov_histo = String::from("");
    for mov in moves.chars() {
        cpt += 1;
        mov_histo = format!("{mov_histo}{mov}");
        match mov {
            '>' => move_right(&mut robot, &mut map),
            'v' => move_down(&mut robot, &mut map),
            '<' => move_left(&mut robot, &mut map),
            '^' => move_up(&mut robot, &mut map),
            _ => continue,
        }
        if VERBOSE {
            clearscreen::clear().unwrap();
            println!("Iteration: {}", cpt);
            println!("Moves: {}", mov_histo);
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
            if map[y][x] == '[' {
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
                map[robot.y].remove(x);
                map[robot.y].insert(robot.x, '.');
                robot.x += 1;
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
                map[robot.y].remove(x);
                map[robot.y].insert(robot.x, '.');
                robot.x -= 1;
                break;
            }
            _ => continue,
        }
    }
}

fn move_down(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    if can_move_down_to(
        &Position {
            x: robot.x,
            y: robot.y + 1,
        },
        map,
    ) {
        move_box_down(
            &Position {
                x: robot.x,
                y: robot.y + 1,
            },
            map,
        );
        map[robot.y][robot.x] = '.';
        map[robot.y + 1][robot.x] = '@';
        robot.y += 1;
    }
}

fn move_up(robot: &mut Position, map: &mut Vec<Vec<char>>) {
    if can_move_up_to(
        &Position {
            x: robot.x,
            y: robot.y - 1,
        },
        map,
    ) {
        move_box_up(
            &Position {
                x: robot.x,
                y: robot.y - 1,
            },
            map,
        );
        map[robot.y][robot.x] = '.';
        map[robot.y - 1][robot.x] = '@';
        robot.y -= 1;
    }
}

fn move_box_down(position: &Position, map: &mut Vec<Vec<char>>) {
    match map[position.y][position.x] {
        '[' => {
            match map[position.y + 1][position.x] {
                '[' => {
                    move_box_down(
                        &Position {
                            x: position.x,
                            y: position.y + 1,
                        },
                        map,
                    );
                }
                ']' => {
                    move_box_down(
                        &Position {
                            x: position.x,
                            y: position.y + 1,
                        },
                        map,
                    );
                    if map[position.y + 1][position.x + 1] == '[' {
                        move_box_down(
                            &Position {
                                x: position.x + 1,
                                y: position.y + 1,
                            },
                            map,
                        );
                    }
                }
                '.' => {
                    if map[position.y + 1][position.x + 1] == '[' {
                        move_box_down(
                            &Position {
                                x: position.x + 1,
                                y: position.y + 1,
                            },
                            map,
                        );
                    }
                }
                _ => {}
            }
            if map[position.y + 1][position.x] == '.' && map[position.y + 1][position.x + 1] == '.'
            {
                map[position.y][position.x] = '.';
                map[position.y][position.x + 1] = '.';
                map[position.y + 1][position.x] = '[';
                map[position.y + 1][position.x + 1] = ']';
            }
        }
        ']' => {
            match map[position.y + 1][position.x] {
                '[' => {
                    move_box_down(
                        &Position {
                            x: position.x,
                            y: position.y + 1,
                        },
                        map,
                    );
                    if map[position.y + 1][position.x - 1] == ']' {
                        move_box_down(
                            &Position {
                                x: position.x - 1,
                                y: position.y + 1,
                            },
                            map,
                        );
                    }
                }
                ']' => {
                    move_box_down(
                        &Position {
                            x: position.x,
                            y: position.y + 1,
                        },
                        map,
                    );
                }
                '.' => {
                    if map[position.y + 1][position.x - 1] == ']' {
                        move_box_down(
                            &Position {
                                x: position.x - 1,
                                y: position.y + 1,
                            },
                            map,
                        );
                    }
                }
                _ => {}
            }
            if map[position.y + 1][position.x] == '.' && map[position.y + 1][position.x - 1] == '.'
            {
                map[position.y][position.x] = '.';
                map[position.y][position.x - 1] = '.';
                map[position.y + 1][position.x] = ']';
                map[position.y + 1][position.x - 1] = '[';
            }
        }
        _ => {}
    }
}

fn move_box_up(position: &Position, map: &mut Vec<Vec<char>>) {
    match map[position.y][position.x] {
        '[' => {
            match map[position.y - 1][position.x] {
                '[' => {
                    move_box_up(
                        &Position {
                            x: position.x,
                            y: position.y - 1,
                        },
                        map,
                    );
                }
                ']' => {
                    move_box_up(
                        &Position {
                            x: position.x,
                            y: position.y - 1,
                        },
                        map,
                    );
                    if map[position.y - 1][position.x + 1] == '[' {
                        move_box_up(
                            &Position {
                                x: position.x + 1,
                                y: position.y - 1,
                            },
                            map,
                        );
                    }
                }
                '.' => {
                    if map[position.y - 1][position.x + 1] == '[' {
                        move_box_up(
                            &Position {
                                x: position.x + 1,
                                y: position.y - 1,
                            },
                            map,
                        );
                    }
                }
                _ => {}
            }
            if map[position.y - 1][position.x] == '.' && map[position.y - 1][position.x + 1] == '.'
            {
                map[position.y][position.x] = '.';
                map[position.y][position.x + 1] = '.';
                map[position.y - 1][position.x] = '[';
                map[position.y - 1][position.x + 1] = ']';
            }
        }
        ']' => {
            match map[position.y - 1][position.x] {
                '[' => {
                    move_box_up(
                        &Position {
                            x: position.x,
                            y: position.y - 1,
                        },
                        map,
                    );
                    if map[position.y - 1][position.x - 1] == ']' {
                        move_box_up(
                            &Position {
                                x: position.x - 1,
                                y: position.y - 1,
                            },
                            map,
                        );
                    }
                }
                ']' => {
                    move_box_up(
                        &Position {
                            x: position.x,
                            y: position.y - 1,
                        },
                        map,
                    );
                }
                '.' => {
                    if map[position.y - 1][position.x - 1] == ']' {
                        move_box_up(
                            &Position {
                                x: position.x - 1,
                                y: position.y - 1,
                            },
                            map,
                        );
                    }
                }
                _ => {}
            }
            if map[position.y - 1][position.x] == '.' && map[position.y - 1][position.x - 1] == '.'
            {
                map[position.y][position.x] = '.';
                map[position.y][position.x - 1] = '.';
                map[position.y - 1][position.x] = ']';
                map[position.y - 1][position.x - 1] = '[';
            }
        }
        _ => {}
    }
}

fn can_move_down_to(position: &Position, map: &Vec<Vec<char>>) -> bool {
    match map[position.y][position.x] {
        '#' => false,
        '.' => true,
        '[' => {
            can_move_down_to(
                &Position {
                    x: position.x,
                    y: position.y + 1,
                },
                map,
            ) && can_move_down_to(
                &Position {
                    x: position.x + 1,
                    y: position.y + 1,
                },
                map,
            )
        }
        ']' => {
            can_move_down_to(
                &Position {
                    x: position.x,
                    y: position.y + 1,
                },
                map,
            ) && can_move_down_to(
                &Position {
                    x: position.x - 1,
                    y: position.y + 1,
                },
                map,
            )
        }
        _ => false,
    }
}

fn can_move_up_to(position: &Position, map: &Vec<Vec<char>>) -> bool {
    match map[position.y][position.x] {
        '#' => false,
        '.' => true,
        '[' => {
            can_move_up_to(
                &Position {
                    x: position.x,
                    y: position.y - 1,
                },
                map,
            ) && can_move_up_to(
                &Position {
                    x: position.x + 1,
                    y: position.y - 1,
                },
                map,
            )
        }
        ']' => {
            can_move_up_to(
                &Position {
                    x: position.x,
                    y: position.y - 1,
                },
                map,
            ) && can_move_up_to(
                &Position {
                    x: position.x - 1,
                    y: position.y - 1,
                },
                map,
            )
        }
        _ => false,
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
