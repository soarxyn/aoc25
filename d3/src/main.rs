use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn best_n(array: &Vec<u32>, n: usize) -> u64 {
    let mut starting_idx = 0;
    let mut num = 0u64;
    let mut selected = 0;

    while selected < n {
        let mut current_max = 0;

        for i in starting_idx..array.len() {
            let current_entry = array[i];

            if current_entry > current_max && (array.len() - i) >= (n - selected) {
                current_max = current_entry;
                starting_idx = i + 1;
            }
        }

        num *= 10;
        num += current_max as u64;
        selected += 1;
    }

    num
}

fn main() -> io::Result<()> {
    let file = File::open("inputs/input.txt")?;
    let reader = BufReader::new(file);

    let n_p1 = 2;
    let n_p2 = 12;

    let mut joltage_sum_p1 = 0;
    let mut joltage_sum_p2 = 0;

    for line_res in reader.lines() {
        let line = line_res?;

        let joltages = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        let line_number_p1 = best_n(&joltages, n_p1);
        joltage_sum_p1 += line_number_p1;

        let line_number_p2 = best_n(&joltages, n_p2);
        joltage_sum_p2 += line_number_p2;
    }

    println!(
        "Total joltage is - P1: {}, P2: {}",
        joltage_sum_p1, joltage_sum_p2
    );

    Ok(())
}
