use std::array;

pub type Mancala = [u8; 14];

pub trait MancalaExt: Clone {
    fn new() -> Self;
    fn swap_players(&mut self);
    fn take_action(&mut self, action: u8) -> Option<bool>;
    fn get_actions(&self) -> [bool; 6];

    fn seeds_in_goal(&self, player: bool) -> u8;
    fn seeds_in_pits(&self, player: bool) -> [u8; 6];
    fn is_game_over(&self) -> bool;

    fn heuristic_state_value(&self) -> i16 {
        self.seeds_in_goal(true) as i16 - self.seeds_in_goal(false) as i16
    }

    fn minimax(&self, depth: u8, player: bool) -> i16 {
        if depth == 0 || self.is_game_over() {
            return self.heuristic_state_value();
        }
        if player {
            let mut max = i16::MIN;
            let mask = self.get_actions();
            mask.iter()
                .enumerate()
                .filter(|(_, valid)| **valid)
                .for_each(|(action, _)| {
                    let mut child = (*self).clone();
                    let res = child
                        .take_action(action as u8)
                        .expect("invalid move preformed");
                    let eval =
                        MancalaExt::minimax(&child, depth - 1, if res { player } else { !player });
                    max = max.max(eval);
                });
            max
        } else {
            let mut min = i16::MAX;
            let mut enemy_view = self.clone();
            enemy_view.swap_players();
            let mask = enemy_view.get_actions();
            mask.iter()
                .enumerate()
                .filter(|(_, valid)| **valid)
                .for_each(|(action, _)| {
                    let mut child = enemy_view.clone();
                    let res = child
                        .take_action(action as u8)
                        .expect("invalid move preformed");
                    child.swap_players();
                    let eval =
                        MancalaExt::minimax(&child, depth - 1, if res { player } else { !player });
                    min = min.min(eval);
                });
            min
        }
    }

    fn best_move(&self) -> u8 {
        let mut max = i16::MIN;
        let mut best_move = 0;
        let mask = self.get_actions();
        mask.iter()
            .enumerate()
            .filter(|(_, valid)| **valid)
            .for_each(|(action, _)| {
                let mut child = (*self).clone();
                let res = child
                    .take_action(action as u8)
                    .expect("invalid move preformed");
                let eval = MancalaExt::minimax(&child, 10, res);
                if eval > max {
                    max = eval;
                    best_move = action as u8;
                }
            });
        best_move
    }
}

impl MancalaExt for Mancala {
    fn new() -> Self {
        [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0]
    }

    fn swap_players(&mut self) {
        *self = array::from_fn(|i| self[(i + 7) % 14]);
    }

    fn take_action(&mut self, action: u8) -> Option<bool> {
        if action > 5 {
            return None;
        }
        if !self.get_actions()[action as usize] {
            return None;
        }
        let mut seed = self[action as usize] as usize;
        self[action as usize] = 0;
        let mut end = action as usize + seed;
        if end > 12 {
            end += 1;
        }
        end %= 14;

        let mut i = action as usize;
        while seed > 0 {
            i += 1;
            i %= 14;
            if i == 13 {
                continue;
            }
            seed -= 1;
            self[i] += 1;
        }

        if self[end] == 1 && end < 6 {
            self[6] += self[12 - end] + 1;
            self[end] = 0;
            self[12 - end] = 0;
        }

        if self.seeds_in_pits(true) == [0; 6] {
            self[13] += self[7..13].iter_mut().fold(0, |mut acc, val| {
                acc += *val;
                *val = 0;
                acc
            });
        } else if self.seeds_in_pits(false) == [0; 6] {
            self[6] += self[0..6].iter_mut().fold(0, |mut acc, val| {
                acc += *val;
                *val = 0;
                acc
            });
        }

        if end == 6 {
            return Some(true);
        }

        Some(false)
    }

    fn get_actions(&self) -> [bool; 6] {
        array::from_fn(|i| self[i] != 0)
    }

    fn seeds_in_goal(&self, player: bool) -> u8 {
        self[if player { 6 } else { 13 }]
    }

    fn seeds_in_pits(&self, player: bool) -> [u8; 6] {
        if player {
            array::from_fn(|i| self[i])
        } else {
            array::from_fn(|i| self[i + 7])
        }
    }

    fn is_game_over(&self) -> bool {
        self.seeds_in_pits(true) == [0, 0, 0, 0, 0, 0]
            || self.seeds_in_pits(false) == [0, 0, 0, 0, 0, 0]
    }
}

pub fn to_string(mancala: &Mancala) -> String{
    let mut out = String::new();
    out += format!("    {:02}\n", mancala.seeds_in_goal(false)).as_str();
    (0..6).for_each(|i|{
        out += format!("{i} ↓{}  {}↑\n", mancala.seeds_in_pits(true)[i], mancala.seeds_in_pits(false)[5-i]).as_str();
    });
    out += format!("    {:02}", mancala.seeds_in_goal(true)).as_str();
    out
}
