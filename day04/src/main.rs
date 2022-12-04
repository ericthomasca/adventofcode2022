use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("day04_input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let assignments: Vec<&str> = contents.lines().collect();
    let containing_assignment_pairs = part_1_count_redundant_pairs(&assignments);
    let overlapping_assignment_pairs = part_2_count_overlapping_pairs(&assignments);

    println!(
        "The number of assignment pairs where one range fully contains the other is {}",
        containing_assignment_pairs
    );

    println!(
        "The number of assignment pairs where the ranges overlap is {}",
        overlapping_assignment_pairs
    );

    Ok(())
}

fn part_1_count_redundant_pairs(assignments: &Vec<&str>) -> i32 {
    let mut redundant_assignment_pairs = 0;
    for assignment in assignments {
        let elves_str: Vec<&str> = assignment.trim().split(',').collect();
        let mut sections: Vec<i32> = Vec::new();
        for elf in elves_str {
            let sections_str: Vec<&str> = elf.split('-').collect();
            for section_str in &sections_str {
                let section: i32 = section_str.parse().expect("Invalid section");
                sections.push(section);
            }
        }

        let first_start = sections[0];
        let first_end = sections[1];
        let second_start = sections[2];
        let second_end = sections[3];

        if second_start >= first_start && first_end >= second_end
            || second_start <= first_start && first_end <= second_end
        {
            redundant_assignment_pairs += 1
        }
    }
    redundant_assignment_pairs
}

fn part_2_count_overlapping_pairs(assignments: &Vec<&str>) -> i32 {
    let mut overlapping_assignment_pairs = 0;
    for assignment in assignments {
        let elves_str: Vec<&str> = assignment.trim().split(',').collect();
        let mut sections: Vec<i32> = Vec::new();
        for elf in elves_str {
            let sections_str: Vec<&str> = elf.split('-').collect();
            for section_str in &sections_str {
                let section: i32 = section_str.parse().expect("Invalid section");
                sections.push(section);
            }
        }

        let first_start = sections[0];
        let first_end = sections[1];
        let second_start = sections[2];
        let second_end = sections[3];

        if (first_start..first_end + 1).contains(&second_start)
            || (first_start..first_end + 1).contains(&second_end)
            || (second_start..second_end + 1).contains(&first_start)
            || (second_start..second_end + 1).contains(&first_end)
        {
            overlapping_assignment_pairs += 1;
        }
    }
    overlapping_assignment_pairs
}
