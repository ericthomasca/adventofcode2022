use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rounds: Vec<&str> = contents.split('\n').collect();

    for round in rounds {
        let player: Vec<&str> = round.split(" ").collect();
        println!("{:?}", player);
    }


    Ok(())
}