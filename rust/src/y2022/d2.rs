use std::fs;

struct Round {
    player_sign: u8,
    oponent_sign: u8
}

pub fn run() -> () {
    let file = fs::read_to_string("input.txt").unwrap(); 
    let result: i32 = file
        .split("\n")
        .map(|item| {
                let options: Vec<&str> = item.split(" ").collect();
                if item.len() < 2 { return 0 };

                let round = Round { 
                    player_sign: options[1].as_bytes()[0],
                    oponent_sign: options[0].as_bytes()[0]
                };
                return check_round_result(round);
        })
        .sum();

    println!("{:?}", result);
}

fn check_round_result(round: Round) -> i32 {
    let player_sign_value: u8 = round.player_sign - 87;
    let oponent_sign_value: u8 = round.oponent_sign - 64;

    let mut battle_result_value: u8 = 0;

    if player_sign_value == oponent_sign_value {
        battle_result_value = 3;
    } else if 
        (player_sign_value > oponent_sign_value && !(player_sign_value == 3 && oponent_sign_value == 1)) || 
        (player_sign_value == 1 && oponent_sign_value == 3){

        battle_result_value = 6;
    } 
    return (player_sign_value + battle_result_value) as i32;
} 
