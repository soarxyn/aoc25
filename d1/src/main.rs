use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("inputs/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i32 = 50;

    let mut p1_password = 0;
    let mut p2_password = 0;

    for line_res in reader.lines() {
        let line = line_res?;

        let dir: i32 = if line.starts_with('R') { 1 } else { -1 };
        let clicks: i32 = (line[1..]).parse().unwrap();

        for _ in 0..clicks {
            result = (result + dir).rem_euclid(100);

            if result == 0 {
                p2_password += 1;
            }
        }

        if result == 0 {
            p1_password += 1;
        }
    }

    println!("P1 Password is {}", p1_password);
    println!("P2 Password is {}", p2_password);

    Ok(())
}
