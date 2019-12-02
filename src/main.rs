use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut nums: Vec<i32> = Vec::new();

    for line in reader.lines() {
        nums.push(line?.parse().unwrap());
    }

    let result: i32 = nums.iter()
        .map(|x| x / 3)
        .map(|x| x - 2)
        .fold(0, |acc, x| acc + x);

    println!("{}", result);

    Ok(())
}
