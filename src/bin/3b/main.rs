use std::fs;

fn main() {
    let data = fs::read_to_string("./src/bin/3a/input.txt").expect("Unable to read file");

    let mut sum_of_products: usize = 0;

    const MUL_TOKEN: &str = "mul(";
    const DO_TOKEN: &str = "do()";
    const DONT_TOKEN: &str = "don't()";

    const SMALLEST_MUL_WINDOW: usize = "mul(1,1)".len();
    const MUL_WINDOW: usize = MUL_TOKEN.len();
    let mut mul_w1: usize = 0;
    let mut mul_w2: usize = mul_w1 + MUL_WINDOW;

    const DEBUG: bool = false;
    let mut mul_count: usize = 0;

    let mut mul_enabled = true;
    let mut needle = MUL_TOKEN;
    // const DO_WINDOW: usize = "do()".len(); // not needed, same as MUL_WINDOW
    const DONT_WINDOW: usize = DONT_TOKEN.len();

    while mul_w1 < data.len() - SMALLEST_MUL_WINDOW {
        if mul_enabled {
            if DEBUG {
                println!(
                    "Checking for '{}'... {}",
                    DONT_TOKEN,
                    &data[mul_w1..mul_w1 + DONT_WINDOW]
                );
            }
            if mul_enabled && &data[mul_w1..mul_w1 + DONT_WINDOW] == DONT_TOKEN {
                if DEBUG {
                    println!("Disabling...");
                }
                mul_enabled = false;
                needle = DO_TOKEN;
                mul_w1 += DONT_WINDOW;
                mul_w2 = mul_w1 + MUL_WINDOW;
                continue;
            }
        }

        let slice = &data[mul_w1..mul_w2];

        if DEBUG {
            println!("Looking for '{}'... {}", needle, slice);
        }

        if slice != needle {
            mul_w1 += 1;
            mul_w2 = mul_w1 + MUL_WINDOW;
            continue;
        }

        if !mul_enabled {
            if DEBUG {
                println!("Enabling...");
            }
            mul_enabled = true;
            needle = MUL_TOKEN;
            mul_w1 += MUL_WINDOW;
            mul_w2 = mul_w1 + MUL_WINDOW;
            continue;
        }

        let digit_w1: usize = mul_w2;
        let mut digit_w2: usize = digit_w1;
        while &data[digit_w2..digit_w2 + 1] != ","
            && digit_w2 < data.len() - 1
            && digit_w2 - digit_w1 < 3
        {
            if DEBUG {
                println!("    Now for digit1... {}", &data[digit_w1..digit_w2 + 1]);
            }
            digit_w2 += 1;
        }
        let digit1: usize = data[digit_w1..digit_w2]
            .parse::<usize>()
            .unwrap_or_default();

        if DEBUG {
            println!("    Now for ','... {}", &data[digit_w2..digit_w2 + 1]);
        }
        if &data[digit_w2..digit_w2 + 1] != "," {
            mul_w1 = digit_w2;
            mul_w2 = mul_w1 + MUL_WINDOW;
            continue;
        }

        let digit_w1: usize = digit_w2 + 1;
        let mut digit_w2: usize = digit_w1;
        while &data[digit_w2..digit_w2 + 1] != ")"
            && digit_w2 < data.len() - 1
            && digit_w2 - digit_w1 < 3
        {
            if DEBUG {
                println!("    Now for digit2... {}", &data[digit_w1..digit_w2 + 1]);
            }
            digit_w2 += 1;
        }
        let digit2: usize = data[digit_w1..digit_w2]
            .parse::<usize>()
            .unwrap_or_default();

        if DEBUG {
            println!("    Now for ')'... {}", &data[digit_w2..digit_w2 + 1]);
        }
        if &data[digit_w2..digit_w2 + 1] != ")" {
            mul_w1 = digit_w2;
            mul_w2 = mul_w1 + MUL_WINDOW;
            continue;
        }

        sum_of_products += digit1 * digit2;
        mul_count += 1;

        // if DEBUG && mul_count > 10 {
        //     break;
        // }

        mul_w1 = digit_w2 + 1;
        mul_w2 = mul_w1 + MUL_WINDOW;
    }

    if DEBUG {
        println!("Found {}", mul_count);
    }
    println!("{}", sum_of_products);
}
