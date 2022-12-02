use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rounds: Vec<&str> = contents.split('\n').collect();

    let mut player_1_total_score = 0;
    let mut player_2_total_score = 0;

    for round in rounds {
        let hands: Vec<&str> = round.split(' ').collect();
        let player_1_hand = hands[0];
        let player_2_hand = hands[1];

        let mut player_1_score = 0;
        let mut player_2_score = 0;

        if player_1_hand == "A" && player_2_hand == "X" {
            player_1_score += 1;
            player_2_score += 1;
            player_1_score += 3;
            player_2_score += 3;
        }
        else if player_1_hand == "A" && player_2_hand == "Y" {
            player_1_score += 1;
            player_2_score += 2;
            player_1_score += 0;
            player_2_score += 6;
        }
        else if player_1_hand == "A" && player_2_hand == "Z" {
            player_1_score += 1;
            player_2_score += 3;
            player_1_score += 6;
            player_2_score += 0;
        }
        else if player_1_hand == "B" && player_2_hand == "X" {
            player_1_score += 2;
            player_2_score += 1;
            player_1_score += 6;
            player_2_score += 0;
        }
        else if player_1_hand == "B" && player_2_hand == "Y" {
            player_1_score += 2;
            player_2_score += 2;
            player_1_score += 3;
            player_2_score += 3;
        }
        else if player_1_hand == "B" && player_2_hand == "Z" {
            player_1_score += 2;
            player_2_score += 3;
            player_1_score += 0;
            player_2_score += 6;
        }
        else if player_1_hand == "C" && player_2_hand == "X" {
            player_1_score += 3;
            player_2_score += 1;
            player_1_score += 6;
            player_2_score += 0;
        }
        else if player_1_hand == "C" && player_2_hand == "Y" {
            player_1_score += 3;
            player_2_score += 2;
            player_1_score += 6;
            player_2_score += 0;
        }
        else if player_1_hand == "C" && player_2_hand == "Z" {
            player_1_score += 3;
            player_2_score += 3;
            player_1_score += 3;
            player_2_score += 3;
        }
        else {
            println!("Problem mathing hand")
        }


        println!("{}", player_1_score);
        println!("{}", player_2_score);
    }

    Ok(())
}
