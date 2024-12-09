use std::fs;

const PART: usize = 2;
const VERBOSE: bool = true;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_7_example.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let mut total_sum: isize = 0;
    for line in lines {
        let line: Vec<&str> = line.split(": ").collect();
        let result: isize = line.get(0).unwrap().parse::<isize>().unwrap();
        let numbers: Vec<String> = line
            .get(1)
            .unwrap()
            .split(" ")
            .map(|n| n.parse::<String>().unwrap())
            .collect();

        let mut combinations: Vec<String> = vec![];
        combine(String::from(""), &mut combinations, numbers.len() - 1);

        for combination in combinations {
            let mut expression: Vec<String> = vec![];
            expression.push(String::from(&numbers[0]));
            for i in 1..numbers.len() {
                expression.push(String::from(combination.chars().nth(i - 1).unwrap()));
                expression.push(String::from(&numbers[i]));
            }
            if result == evaluate(&expression) {
                total_sum += result;
                if VERBOSE {
                    println!("{:?}={}", expression, result);
                }
                break;
            }
        }
    }
    println!("Total calibration result: {total_sum}");
}

fn combine(s: String, combinations: &mut Vec<String>, len: usize) {
    if s.len() == len {
        combinations.push(s);
    } else {
        combine(String::from(&s) + "+", combinations, len);
        combine(String::from(&s) + "*", combinations, len);
        if PART == 2 {
            combine(String::from(&s) + "|", combinations, len);
        }
    }
}

fn evaluate(expression: &Vec<String>) -> isize {
    let mut result: isize = to_isize(&expression[0]);
    for i in (1..expression.len() - 1).step_by(2) {
        let operator: &String = &expression[i];
        let number: isize = to_isize(&expression[i + 1]);
        if operator == "+" {
            result += number;
        } else if operator == "*" {
            result *= number;
        } else {
            result = to_isize(&format!("{}{}", result.to_string(), number.to_string()));
        }
    }
    return result;
}

fn to_isize(s: &String) -> isize {
    return s.parse::<isize>().unwrap();
}
