use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut file = File::open("day02_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let rounds: Vec<&str> = contents.split('\n').collect();
    
    
    let my_total_score = strategy_part_1(rounds);
    
    println!("My total score is {}", my_total_score);

    Ok(())

}

fn strategy_part_1(rounds: Vec<&str>) -> i32 {
    let mut my_total_score = 0;
    for round in rounds {
        let hands: Vec<&str> = round.split(' ').collect();
        let opponent_hand = hands[0];
        let my_hand = hands[1];
        let mut my_score = 0;
        let opponent_rock = "A";
        let opponent_paper = "B";
        let opponent_scissors = "C";
        let my_rock = "X";
        let my_paper = "Y";
        let my_scissors = "Z";

        if opponent_hand == opponent_rock && my_hand == my_rock {
            my_score += 4;
        }
        else if opponent_hand == opponent_rock && my_hand == my_paper {
            my_score += 8;
        }
        else if opponent_hand == opponent_rock && my_hand == my_scissors {
            my_score += 3;
        }
        else if opponent_hand == opponent_paper && my_hand == my_rock {
            my_score += 1;
        }
        else if opponent_hand == opponent_paper && my_hand == my_paper {
            my_score += 5;
        }
        else if opponent_hand == opponent_paper && my_hand == my_scissors {
            my_score += 9;
        }
        else if opponent_hand == opponent_scissors && my_hand == my_rock {
            my_score += 7;
        }
        else if opponent_hand == opponent_scissors && my_hand == my_paper {
            my_score += 2;
        }
        else if opponent_hand == opponent_scissors && my_hand == my_scissors {
            my_score += 6;
        }
        else {
            println!("Problem mathing hand")
        }

        my_total_score += my_score;
    }
    my_total_score
}
