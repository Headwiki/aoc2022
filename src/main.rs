use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut most_calories = 0;
    let mut second_most_calories = 0;
    let mut third_most_calories = 0;
    let mut accumulated_calories = 0;
    

    for line in reader.lines() {
        let moved_line = line?;
        if moved_line == "" {
            if accumulated_calories >= most_calories { 
                third_most_calories = second_most_calories;
                second_most_calories = most_calories;
                most_calories = accumulated_calories; 
            } else if accumulated_calories >= second_most_calories {
                third_most_calories = second_most_calories;
                second_most_calories = accumulated_calories;
            } else if accumulated_calories > third_most_calories {
                third_most_calories = accumulated_calories;
            }
            accumulated_calories = 0;
        } else {
            accumulated_calories = accumulated_calories + moved_line.parse::<u32>().unwrap();
        }
    }

    println!("{}", most_calories + second_most_calories + third_most_calories);

    Ok(())
}