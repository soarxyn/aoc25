use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_pattern_p1(element: u64) -> bool {
    let digits = element
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    let split_idx = digits.len() / 2;

    if digits[..split_idx] == digits[split_idx..] {
        return true;
    }

    false
}

fn is_pattern_p2(element: u64) -> bool {
    let digits = element
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    for size in 1..(digits.len() / 2 + 1) {
        let mut chunks = digits.chunks(size);
        let first_chunk = chunks.next().unwrap();
        if chunks.all(|chunk| chunk == first_chunk) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let file = File::open("inputs/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    let ranges = line.split(",");
    let splits = ranges.map(|range| range.split("-"));

    let mut sum = 0;

    for split in splits {
        let [begin, end] = split
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();

        for el in begin..end + 1 {
            if is_pattern_p2(el) {
                sum += el;
            }
        }
    }

    println!("Sum is {}", sum);

    Ok(())
}
