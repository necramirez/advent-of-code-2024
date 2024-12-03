use std::fs;

fn get_intervals(vec: &Vec<i32>) -> Vec<i32> {
    let lefts = vec.iter();
    let rights = vec.iter().skip(1);

    let zipped = lefts.zip(rights);
    zipped.map(|(left, right)| right - left).collect()
}

fn is_all_good(vec: &Vec<i32>) -> bool {
    let all_positive = vec.iter().all(|n| *n > 0);
    let all_negative = vec.iter().all(|n| *n < 0);
    let all_within_range = vec.iter().all(|n| n.abs() >= 1 && n.abs() <= 3);

    (all_positive || all_negative) && all_within_range
}

fn main() {
    let data = fs::read_to_string("./src/bin/2a/input.txt").expect("Unable to read file");
    let lines = data.split("\n");

    let mut safe_level_count: u32 = 0;

    for line in lines {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or_default())
            .collect();
        if levels.len() == 0 {
            continue;
        }

        let diffs: Vec<i32> = get_intervals(&levels);
        if is_all_good(&diffs) {
            safe_level_count += 1;
            continue;
        }

        'inner: for i in 0..levels.len() {
            let mut cloned = levels.clone();
            cloned.drain(i..i + 1);

            let diffs = get_intervals(&cloned);
            if is_all_good(&diffs) {
                safe_level_count += 1;
                break 'inner;
            }
        }
    }

    println!("{}", safe_level_count);
}
