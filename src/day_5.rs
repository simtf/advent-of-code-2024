use std::fs;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_5.txt").unwrap();
    let blocks: Vec<&str> = dataset.split("\r\n\r\n").collect();
    let rules: Vec<&str> = blocks.get(0).unwrap().split("\r\n").collect();
    let updates: Vec<&str> = blocks.get(1).unwrap().split("\r\n").collect::<Vec<&str>>();

    let mut count_ordered: usize = 0;
    let mut count_not_ordered: usize = 0;
    for update in updates {
        let mut pages: Vec<&str> = update.split(",").collect();
        if are_correctly_ordered(&pages, &rules) {
            count_ordered += get_middle_page_number(&pages);
        } else {
            sort(&mut pages, &rules);
            count_not_ordered += get_middle_page_number(&pages);
        }
    }
    println!("Sum of correctly ordered middle page number is {count_ordered}.");
    println!("Sum of incorrectly ordered middle page number is {count_not_ordered}.");
}

fn are_correctly_ordered(pages: &Vec<&str>, rules: &Vec<&str>) -> bool {
    for i in 0..pages.len() - 1 {
        let rule: String = format!("{}|{}", pages[i], pages[i + 1]);
        let rule: &str = rule.as_str();
        if !rules.contains(&rule) {
            return false;
        }
    }
    return true;
}

fn sort(pages: &mut Vec<&str>, rules: &Vec<&str>) {
    while !are_correctly_ordered(pages, rules) {
        for i in 0..pages.len() - 1 {
            let rule: String = format!("{}|{}", pages[i], pages[i + 1]);
            let rule: &str = rule.as_str();
            if !rules.contains(&rule) {
                pages.swap(i, i + 1);
            }
        }
    }
}

fn get_middle_page_number(pages: &Vec<&str>) -> usize {
    return pages[pages.len() / 2].parse::<usize>().unwrap();
}
