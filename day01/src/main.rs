use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("day01_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let elves: Vec<&str> = contents.split("\n\n").collect();
    let mut calorie_totals: Vec<i32> = Vec::new();

    for elf in elves {
        let items: Vec<&str> = elf.split('\n').collect();
        let mut total = 0;

        for item in items {
            let item: i32 = item.trim().parse().unwrap();
            total += item;
        }

        calorie_totals.push(total);
    }

    calorie_totals.sort_by(|a, b| b.cmp(a));
    let top_three_elves = &calorie_totals[..3];
    let top_elves_total: i32 = top_three_elves.iter().sum();
    
    println!("The elf carrying the most calories is carrying {:?} calories.", calorie_totals[0]);
    println!("The three elves carrying the most calories are carrying {:?} calories.", top_elves_total);

    Ok(())
}
