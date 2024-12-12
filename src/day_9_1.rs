use std::fs;

const VERBOSE: bool = false;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_9.txt").unwrap();
    let digits: Vec<usize> = dataset
        .chars()
        .map(|c: char| c.to_string().parse::<usize>().unwrap())
        .collect();

    let mut blocks: Vec<String> = vec![];
    for i in 0..digits.len() {
        for _ in 0..digits[i] {
            if i % 2 == 0 {
                blocks.push((i / 2).to_string());
            } else {
                blocks.push(String::from("."));
            }
        }
    }
    print_blocks(&blocks);

    let mut i: usize = 0;
    while i < blocks.len() {
        while blocks.last().unwrap() == "." {
            blocks.pop();
        }
        if blocks[i] == "." {
            blocks[i] = blocks.pop().unwrap();
            if VERBOSE {
                print_blocks(&blocks);
            }
        }
        i += 1;
    }
    print_blocks(&blocks);

    let mut checksum: usize = 0;
    for i in 1..blocks.len() {
        checksum += i * blocks[i].parse::<usize>().unwrap();
    }
    println!("Checksum: {checksum}");
}

fn print_blocks(blocks: &Vec<String>) {
    blocks.iter().for_each(|s| print!("{s}"));
    println!();
}
