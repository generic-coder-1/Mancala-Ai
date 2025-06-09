use std::io::{self, Write};

use board::{to_string, Mancala, MancalaExt};

mod board;
fn main() {
    let mut mancala = Mancala::new();
    let mut player = true;
    while !mancala.is_game_over() {
        println!("Mancala board:\n{}", to_string(&mancala));
        if player {
            print!("Your move:");
            let mut inp = get_inp();
            while inp
                .parse::<u8>()
                .ok()
                .and_then(|i| {
                    if *mancala.get_actions().get(i as usize).unwrap_or(&false) {
                        Some(())
                    } else {
                        None
                    }
                })
                .is_none()
            {
                print!("Input a valid move:");
                inp = get_inp();
            }
            player = mancala
                .take_action(inp.parse().expect("made sure it could parse"))
                .expect("move should be valid")
        } else {
            mancala.swap_players();
            let enemy_action = mancala.best_move();
            println!("Enemy move:{}", 5-enemy_action);
            player = !mancala
                .take_action(enemy_action)
                .expect("algorithm produced invalid move");
            mancala.swap_players();
        }
    }
    println!("Mancala board:\n{}", to_string(&mancala));
}

fn get_inp() -> String {
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("error reading user input");
    inp.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::board::{Mancala, MancalaExt};

    #[test]
    fn test_move() {
        let mut mancala = Mancala::new();
        let res = mancala.take_action(0);
        assert_eq!(mancala, [0, 5, 5, 5, 5, 4, 0, 4, 4, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(false));
    }

    #[test]
    fn test_extra_move() {
        let mut mancala = Mancala::new();
        let res = mancala.take_action(2);
        assert_eq!(mancala, [4, 4, 0, 5, 5, 5, 1, 4, 4, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(true));
    }

    #[test]
    fn test_capture() {
        let mut mancala = Mancala::new();
        let res = mancala.take_action(4);
        assert_eq!(mancala, [4, 4, 4, 4, 0, 5, 1, 5, 5, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(false));
        let res = mancala.take_action(0);
        assert_eq!(mancala, [0, 5, 5, 5, 0, 5, 7, 5, 0, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(false));
    }

    #[test]
    fn test_invalid_move() {
        let mut mancala = Mancala::new();
        let res = mancala.take_action(4);
        assert_eq!(mancala, [4, 4, 4, 4, 0, 5, 1, 5, 5, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(false));
        assert_eq!(mancala.get_actions(), [true, true, true, true, false, true]);
        let res = mancala.take_action(4);
        assert_eq!(mancala, [4, 4, 4, 4, 0, 5, 1, 5, 5, 4, 4, 4, 4, 0]);
        assert_eq!(res, None);
    }

    #[test]
    fn test_swap() {
        let mut mancala = Mancala::new();
        let res = mancala.take_action(0);
        assert_eq!(mancala, [0, 5, 5, 5, 5, 4, 0, 4, 4, 4, 4, 4, 4, 0]);
        assert_eq!(res, Some(false));
        mancala.swap_players();
        assert_eq!(mancala, [4, 4, 4, 4, 4, 4, 0, 0, 5, 5, 5, 5, 4, 0]);
    }

    #[test]
    fn test_sweep() {
        let mut mancala = [0,0,0,0,0,2, 0, 0,0,1,2,0,0, 3];
        let res = mancala.take_action(5);
        assert_eq!(mancala, [0,0,0,0,0,0, 1, 0,0,0,0,0,0, 7]);
        assert_eq!(res, Some(false));
    }
}
