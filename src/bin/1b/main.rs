use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("./src/bin/1a/input.txt").expect("Unable to read file");
    let lines = data.split("\n");

    let mut similarity_score = 0;

    let mut left_list = Vec::<u32>::new();
    let mut right_counts = HashMap::<u32, u32>::new();

    for line in lines {
        let mut ids = line.split_whitespace();
        let left_id = ids.next().unwrap_or_default().parse().unwrap_or_default();
        let right_id: u32 = ids.next().unwrap_or_default().parse().unwrap_or_default();
        left_list.push(left_id);
        // WHY DID I HAVE THIS HERE IN THE FIRST PLACE?!
        // I forgot to remove it and I spent 30 minutes wondering why my counts stopped at 2
        // :HUGE-FACEPALM:
        // right_counts.insert(right_id, 1);
        match right_counts.get(&right_id) {
            Some(count) => {
                right_counts.insert(right_id, count + 1);
            }
            None => {
                right_counts.insert(right_id, 1);
            }
        }
    }

    for id in left_list {
        match right_counts.get(&id) {
            Some(count) => {
                similarity_score += id * count;
            }
            None => {
                similarity_score += 0;
            }
        }
    }

    println!("{}", similarity_score);
}
