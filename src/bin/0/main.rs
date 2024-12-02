use std::collections::HashMap;

fn main() {
    let mut test_map = HashMap::<u32, u32>::new();
    test_map.insert(1, 1);
    let id: u32 = 1;
    let value = test_map.get(&id).unwrap();

    println!("{}", value);

    test_map.insert(1, value + 1);
    let value = test_map.get(&id).unwrap();
    println!("{}", value);
}
