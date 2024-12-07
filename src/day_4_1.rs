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
    for y in 0..size_y {
        for x in 0..size_x {
            if get_char_at_or_default(x, y, &array) == &'X' {
                count += count_xmas_from(x, y, &array);
            }
        }
    }
    println!("XMAS occurs a total of {count} times.");
}

fn count_xmas_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    if get_north_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_north_east_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_east_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_south_east_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_south_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_south_west_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_west_from(x, y, array) == "MAS" {
        count += 1;
    }
    if get_north_west_from(x, y, array) == "MAS" {
        count += 1;
    }
    return count;
}

fn get_char_at(x: usize, y: usize, array: &Vec<Vec<char>>) -> Option<&char> {
    return array.get(y)?.get(x);
}

fn get_char_at_or_default(x: usize, y: usize, array: &Vec<Vec<char>>) -> &char {
    return get_char_at(x, y, array).unwrap_or(&'.');
}

fn get_north_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if y < 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x, y - 1, array),
        get_char_at_or_default(x, y - 2, array),
        get_char_at_or_default(x, y - 3, array)
    );
}

fn get_north_east_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x >= array[0].len() - 3 {
        return String::from("");
    }
    if y < 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x + 1, y - 1, array),
        get_char_at_or_default(x + 2, y - 2, array),
        get_char_at_or_default(x + 3, y - 3, array)
    );
}

fn get_east_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x >= array[0].len() - 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x + 1, y, array),
        get_char_at_or_default(x + 2, y, array),
        get_char_at_or_default(x + 3, y, array)
    );
}

fn get_south_east_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x >= array[0].len() - 3 {
        return String::from("");
    }
    if y >= array.len() - 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x + 1, y + 1, array),
        get_char_at_or_default(x + 2, y + 2, array),
        get_char_at_or_default(x + 3, y + 3, array)
    );
}

fn get_south_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if y >= array.len() - 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x, y + 1, array),
        get_char_at_or_default(x, y + 2, array),
        get_char_at_or_default(x, y + 3, array)
    );
}

fn get_south_west_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x < 3 {
        return String::from("");
    }
    if y >= array.len() - 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x - 1, y + 1, array),
        get_char_at_or_default(x - 2, y + 2, array),
        get_char_at_or_default(x - 3, y + 3, array)
    );
}

fn get_west_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x < 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x - 1, y, array),
        get_char_at_or_default(x - 2, y, array),
        get_char_at_or_default(x - 3, y, array)
    );
}

fn get_north_west_from(x: usize, y: usize, array: &Vec<Vec<char>>) -> String {
    if x < 3 {
        return String::from("");
    }
    if y < 3 {
        return String::from("");
    }
    return format!(
        "{}{}{}",
        get_char_at_or_default(x - 1, y - 1, array),
        get_char_at_or_default(x - 2, y - 2, array),
        get_char_at_or_default(x - 3, y - 3, array)
    );
}
