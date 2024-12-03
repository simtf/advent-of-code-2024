use std::fs;

pub fn execute() {
    let dataset: String = fs::read_to_string("./resources/day_1.txt").unwrap();

    let mut left_nums: Vec<usize> = Vec::new();
    let mut right_nums: Vec<usize> = Vec::new();

    dataset.split("\r\n").for_each(|line: &str| {
        let nums: Vec<&str> = line.split("   ").collect();
        left_nums.push(nums[0].parse::<usize>().unwrap());
        right_nums.push(nums[1].parse::<usize>().unwrap());
    });

    left_nums.sort();
    right_nums.sort();

    let mut distances: Vec<usize> = Vec::new();
    for i in 0..left_nums.len() {
        distances.push(left_nums[i].abs_diff(right_nums[i]));
    }

    // println!("left numbers:  {:?}", left_nums);
    // println!("right numbers: {:?}", right_nums);
    // println!("distances:     {:?}", distances);
    println!("Total distance: {:?}", distances.iter().sum::<usize>());

    let mut occurences: Vec<usize> = Vec::new();
    for i in 0..left_nums.len() {
        let left_num = left_nums[i];
        let nb_occurences = right_nums
            .iter()
            .filter(|&&right_num| right_num == left_num)
            .count();
        occurences.push(left_num * nb_occurences);
    }

    // println!("occurences: {:?}", occurences);
    println!("Similarity score: {:?}", occurences.iter().sum::<usize>());
}
