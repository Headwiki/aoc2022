use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line?;
        let mut round = line.split_whitespace();

        total_score += parse_score(round.next().unwrap(), round.next().unwrap());
    }

    println!("{:?}", total_score);

    Ok(())
}
fn parse_score(opponent_action: &str, my_action: &str) -> u32 {
    match my_action {
        "X" => {
            1 + match opponent_action {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                &_ => todo!(),
            }
        },
        "Y" => {
            2 + match opponent_action {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                &_ => todo!(),
            }
        },
        "Z" => {
            3 + match opponent_action {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                &_ => todo!(),
            }
        },
        &_ => todo!(),
    }
}