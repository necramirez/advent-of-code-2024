use std::fs;

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

        let lefts = levels.iter();
        let rights = levels.iter().skip(1);

        let zipped = lefts.zip(rights);
        let diffs: Vec<i32> = zipped.map(|(left, right)| right - left).collect();

        let all_positive = diffs.iter().all(|n| *n > 0);
        let all_negative = diffs.iter().all(|n| *n < 0);
        let all_within_range = diffs.iter().all(|n| n.abs() >= 1 && n.abs() <= 3);

        if (all_positive || all_negative) && all_within_range {
            safe_level_count += 1;
        }
    }

    println!("{}", safe_level_count);
}
