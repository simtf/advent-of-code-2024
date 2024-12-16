use std::{fs, ops::Mul, usize};

const VERBOSE: bool = false;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_11.txt").unwrap();

    let cpt: usize = 25;
    compute_nexts(cpt, dataset);
}

fn compute_nexts(cpt: usize, data: String) {
    let next: String = compute_next(data);
    let s: Vec<&str> = next.split(" ").collect();
    if VERBOSE == true || cpt == 1 {
        println!("{cpt}\t{next}\ncount: {:?}", s.len());
    } else {
        println!("{cpt}");
    }
    if cpt > 1 {
        let next_cpt: usize = cpt - 1;
        compute_nexts(next_cpt, next);
    }
}

fn compute_next(data: String) -> String {
    let digits: Vec<&str> = data.split(" ").collect();
    let mut next: String = String::from("");
    for digit in digits {
        if digit == "0" {
            next += " 1";
        } else if digit.len() % 2 == 0 {
            let s: (&str, &str) = digit.split_at(digit.len() / 2);
            next += " ";
            next += s.0;
            next += " ";
            let mut s1: &str = s.1.trim_start_matches("0");
            if s1 == "" {
                s1 = "0";
            }
            next += s1;
        } else {
            next += " ";
            next += digit
                .parse::<usize>()
                .unwrap()
                .mul(2024)
                .to_string()
                .as_str();
        }
        if next.starts_with(" ") {
            next.remove(0);
        }
    }
    return next;
}
