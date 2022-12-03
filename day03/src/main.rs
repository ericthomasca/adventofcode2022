use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("day03_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let elves: Vec<&str> = contents.split('\n').collect();

    let item_priority_total_part_1 = part_1_priority_sum(&elves);
    let item_priority_total_part_2 = part_2_priority_sum(&elves);

    println!(
        "Total priority of items common to both compartments for all sacks is {}",
        item_priority_total_part_1
    );
    println!(
        "Total priority of the badges for all badge groups is {}",
        item_priority_total_part_2
    );

    Ok(())
}

fn part_1_priority_sum(elves: &Vec<&str>) -> usize {
    let item_reference = create_item_reference();
    let mut item_priority_total = 0;
    for elf in elves {
        let first_compartment = &elf[..(elf.len() / 2)];
        let second_compartment = &elf[(elf.len() / 2)..];

        let first_compartment_items: Vec<char> = first_compartment.chars().collect();
        let second_compartment_items: Vec<char> = second_compartment.chars().collect();

        let mut items_in_both = Vec::new();
        for first_item in &first_compartment_items {
            for second_item in &second_compartment_items {
                if first_item == second_item {
                    items_in_both.push(first_item);
                }
            }
        }
        items_in_both.sort();
        items_in_both.dedup();

        let common_item = *items_in_both[0];
        let common_item_priority = item_reference
            .iter()
            .position(|&r| r == common_item)
            .unwrap()
            + 1;
        item_priority_total += common_item_priority;
    }
    item_priority_total
}

fn part_2_priority_sum(elves: &[&str]) -> usize {
    let item_reference = create_item_reference();
    let mut item_priority_total = 0;
    let badge_group_chunks = elves.chunks(3);

    for badge_group in badge_group_chunks {
        let first_elf = badge_group[0];
        let second_elf = badge_group[1];
        let third_elf = badge_group[2];

        let first_elf_items: Vec<char> = first_elf.chars().collect();
        let second_elf_items: Vec<char> = second_elf.chars().collect();
        let third_elf_items: Vec<char> = third_elf.chars().collect();

        let mut items_in_all = Vec::new();
        for first_badge in &first_elf_items {
            for second_badge in &second_elf_items {
                for third_badge in &third_elf_items {
                    if first_badge == second_badge && second_badge == third_badge {
                        items_in_all.push(first_badge)
                    }
                }
            }
        }
        items_in_all.sort();
        items_in_all.dedup();
        let common_item = *items_in_all[0];
        let common_item_priority = item_reference
            .iter()
            .position(|&r| r == common_item)
            .unwrap()
            + 1;
        item_priority_total += common_item_priority;
    }
    item_priority_total
}

fn create_item_reference() -> Vec<char> {
    let mut item_reference: Vec<char> = Vec::new();
    let binding = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let items = binding.chars();
    for item in items {
        item_reference.push(item);
    }
    item_reference
}
