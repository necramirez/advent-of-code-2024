use std::fs;

fn main() {
    let data = fs::read_to_string("./src/bin/1a/input.txt").expect("Unable to read file");
    let lines = data.split("\n");

    let mut total_distance = 0;

    let mut left_list = Vec::<u32>::new();
    let mut right_list = Vec::<u32>::new();

    for line in lines {
        let mut ids = line.split_whitespace();
        let left_id = ids.next().unwrap_or_default().parse().unwrap_or_default();
        let right_id: u32 = ids.next().unwrap_or_default().parse().unwrap_or_default();
        left_list.push(left_id);
        right_list.push(right_id);
    }

    left_list.sort();
    right_list.sort();

    let zipped = left_list.iter().zip(right_list.iter());

    for (left, right) in zipped {
        total_distance += left.abs_diff(*right);
    }

    println!("{}", total_distance);
}
