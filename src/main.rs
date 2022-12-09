use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut lines_with_most_calories = Vec::new();
    let mut accumulated_calories = 0;

    for line in reader.lines() {
        let line = line?;
        if line == "" {
            // If the vector has fewer than three elements, we can just push the
            // accumulated calories to the end of the vector.
            if lines_with_most_calories.len() < 3 {
                lines_with_most_calories.push(accumulated_calories);
            } else {
                // Otherwise, we need to insert the accumulated calories in the
                // correct position in the vector.
                for i in 0..3 {
                    if accumulated_calories > lines_with_most_calories[i] {
                        lines_with_most_calories.insert(i, accumulated_calories);
                        lines_with_most_calories.pop();
                        break;
                    }
                }
            }
            accumulated_calories = 0;
        } else {
            accumulated_calories = accumulated_calories + line.parse::<u32>().unwrap();
        }
    }

    let sum: u32 = lines_with_most_calories.iter().sum();
    println!("{}", sum);

    Ok(())
}