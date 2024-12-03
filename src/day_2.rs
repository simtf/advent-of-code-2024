use std::fs;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_2.txt").unwrap();
    let reports: Vec<&str> = dataset.split("\r\n").collect();
    let mut cpt: usize = 0;
    for report in reports {
        let levels: Vec<usize> = report
            .split(" ")
            .map(|level: &str| level.parse::<usize>().unwrap())
            .collect();
        if is_safe(&levels) {
            cpt += 1;
        }
    }
    println!("Count safe reports: {cpt}");
}

fn is_safe(levels: &Vec<usize>) -> bool {
    if increasing(&levels) || decreasing(&levels) {
        return true;
    }
    for i in 0..levels.len() {
        let mut levels: Vec<usize> = levels.clone();
        levels.remove(i);
        if increasing(&levels) || decreasing(&levels) {
            return true;
        }
    }
    return false;
}

fn increasing(levels: &Vec<usize>) -> bool {
    for i in 0..levels.len() - 1 {
        if levels[i + 1] <= levels[i] || levels[i + 1] - levels[i] > 3 {
            return false;
        }
    }
    return true;
}

fn decreasing(levels: &Vec<usize>) -> bool {
    for i in 0..levels.len() - 1 {
        if levels[i + 1] >= levels[i] || levels[i] - levels[i + 1] > 3 {
            return false;
        }
    }
    return true;
}
