use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Block {
    digit: String,
    len: usize,
}

const VERBOSE: bool = false;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_9.txt").unwrap();
    let digits: Vec<usize> = dataset
        .chars()
        .map(|c: char| c.to_string().parse::<usize>().unwrap())
        .collect();

    let mut blocks: Vec<Block> = vec![];
    for i in 0..digits.len() {
        if i % 2 == 0 {
            blocks.push(Block {
                digit: (i / 2).to_string(),
                len: digits[i],
            });
        } else {
            blocks.push(Block {
                digit: String::from("."),
                len: digits[i],
            });
        }
    }
    print_blocks(&blocks);

    let mut i: usize = 0;
    while i < blocks.len() {
        if blocks[i].digit == "." {
            let mut j: usize = blocks.len() - 1;
            while j > i {
                if blocks[j].digit != "." && blocks[j].len <= blocks[i].len {
                    let block: Block = blocks[j].clone();
                    blocks[j].digit = String::from(".");
                    blocks[i].len -= block.len;
                    blocks.insert(i, block);
                    if VERBOSE {
                        print_blocks(&blocks);
                    }
                    break;
                }
                j -= 1;
            }
        }
        i += 1;
    }
    print_blocks(&blocks);

    let mut digits: Vec<String> = vec![];
    for i in 0..blocks.len() {
        for _ in 0..blocks[i].len {
            digits.push(blocks[i].digit.clone());
        }
    }

    let mut checksum: usize = 0;
    for i in 1..digits.len() {
        if digits[i] != "." {
            checksum += i * digits[i].parse::<usize>().unwrap();
        }
    }
    println!("Checksum: {checksum}");
}

fn print_blocks(blocks: &Vec<Block>) {
    for i in 0..blocks.len() {
        for _ in 0..blocks[i].len {
            print!("{}", blocks[i].digit);
        }
    }
    println!();
}
