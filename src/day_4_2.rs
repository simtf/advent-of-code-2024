use std::{fs, usize};

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_4.txt").unwrap();
    let lines: Vec<&str> = dataset.split("\r\n").collect();

    let size_x: usize = lines[0].len();
    let size_y: usize = lines.len();

    let mut array: Vec<Vec<char>> = vec![vec!['.'; size_x]; size_y];
    for y in 0..size_y {
        for x in 0..size_x {
            array[y][x] = lines[y].chars().nth(x).unwrap();
        }
    }

    let mut count: usize = 0;
    for y in 1..size_y - 1 {
        for x in 1..size_x - 1 {
            if get_char_at_or_default(x, y, &array) == &'A' {
                count += count_xmas_from(x, y, &array);
            }
        }
    }
    println!("XMAS occurs a total of {count} times.");
}

fn count_xmas_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> usize {
    if (get_char_at_or_default(x - 1, y - 1, array) == &'M'
        && get_char_at_or_default(x + 1, y + 1, array) == &'S'
        || get_char_at_or_default(x - 1, y - 1, array) == &'S'
            && get_char_at_or_default(x + 1, y + 1, array) == &'M')
        && (get_char_at_or_default(x - 1, y + 1, array) == &'M'
            && get_char_at_or_default(x + 1, y - 1, array) == &'S'
            || get_char_at_or_default(x - 1, y + 1, array) == &'S'
                && get_char_at_or_default(x + 1, y - 1, array) == &'M')
    {
        return 1;
    } else {
        return 0;
    }
}

fn get_char_at(x: usize, y: usize, array: &Vec<Vec<char>>) -> Option<&char> {
    return array.get(y)?.get(x);
}

fn get_char_at_or_default(x: usize, y: usize, array: &Vec<Vec<char>>) -> &char {
    return get_char_at(x, y, array).unwrap_or(&'.');
}
