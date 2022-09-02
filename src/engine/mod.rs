#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player { X, O }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winner { X, O, Tie }

use std::ops::Add;

use sdl2::rect::{Point};

pub enum MoveKind { Left, Right, Up, Down }

pub const WIDTH: usize = 3;

pub type GameState = [Option<Player>;WIDTH * WIDTH];

pub struct Engine {
    pub state: GameState,
    turn: Player,
    pub cursor_pos: Point,
    pub width: usize,
}

impl Engine {
    pub fn new(first_turn: Player) -> Self {
        Self {
            state: Default::default(),
            turn: first_turn,
            cursor_pos: Point::new(1, 1),
            width: WIDTH,
        }
    }

    pub fn update_state_for_testing(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn place(&mut self) -> bool {
        let x = self.cursor_pos.x;
        let y = self.cursor_pos.y;

        let idx = self.coord_to_idx(x as usize, y as usize);

        if self.state[idx].is_some() {
            return false;
        }

        self.state[idx] = Some(self.turn);

        self.switch_turn();

        let winner = self.check_winner();

        if winner.is_some() {
            println!("The winner is: {:?}", winner.unwrap());
        }

        true
    }

    fn coord_to_idx(&self, x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    fn check_winner(&self) -> Option<Winner> {
        let s = self.state;
        let mut winner: Option<Player> = None;

        // Check horizontally
        for y in 0..3 {
            if
                s[y * WIDTH].is_some() &&
                s[y * WIDTH] == s[(y * WIDTH) + 1] &&
                s[(y * WIDTH) +1] == s[(y * WIDTH) + 2]
            {
                winner = Some(s[y * WIDTH].unwrap());
            }
        }

        // Check vertically
        for x in 0..3 {
            if
                s[x].is_some() &&
                s[x] == s[x + (WIDTH * 1)] &&
                s[x + (WIDTH * 1)] == s[x + (WIDTH * 2)]
            {
                winner = Some(s[x].unwrap());
            }
        }

        // Check diagonally
        if s[0].is_some() && s[0] == s[4] && s[4] == s[8] {
            winner = Some(s[0].unwrap());
        }
        if s[2].is_some() && s[2] == s[4] && s[4] == s[6] {
            winner = Some(s[2].unwrap());
        }

        // Check for tie
        if winner.is_none() && s.into_iter().all(|x| x.is_some()) {
            return Some(Winner::Tie);
        }

        match winner {
            Some(Player::X) => Some(Winner::X),
            Some(Player::O) => Some(Winner::O),
            _ => None,
        }
    }

    fn switch_turn(&mut self) {
        self.turn = if matches!(self.turn, Player::X) { Player::O } else { Player::X };
    }

    fn within_bounds(&self, point: Point) -> bool {
        return
            point.x >= 0 && point.x < WIDTH as i32 &&
            point.y >= 0 && point.y < WIDTH as i32
    }

    pub fn move_cursor(&mut self, dir: MoveKind) -> bool {
        let v = match dir {
            MoveKind::Up    => Point::new( 0, -1),
            MoveKind::Down  => Point::new( 0,  1),
            MoveKind::Left  => Point::new(-1,  0),
            MoveKind::Right => Point::new( 1,  0),
        };

        if self.within_bounds(self.cursor_pos.add(v)) {
            self.cursor_pos = self.cursor_pos.add(v);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_winner_horizontally() {
        let mut engine = Engine::new(Player::X);

        engine.update_state_for_testing([
            None, None, None,
            None, None, None,
            None, None, None,
        ]);

        assert!(engine.check_winner().is_none());

        engine.update_state_for_testing([
            None,            None,            None,
            Some(Player::X), Some(Player::X), Some(Player::X),
            None,            None,            None,
        ]);

        assert!(engine.check_winner() == Some(Winner::X));

        engine.update_state_for_testing([
            None,            None,            None,
            Some(Player::X), Some(Player::O), Some(Player::X),
            None,            None,            None,
        ]);

        assert!(engine.check_winner().is_none());
    }

    #[test]
    fn check_winner_vertically() {
        let mut engine = Engine::new(Player::X);

        engine.update_state_for_testing([
            None,            Some(Player::O), None,
            None,            Some(Player::O), None,
            None,            Some(Player::O), None,
        ]);

        assert!(engine.check_winner() == Some(Winner::O));
    }

    #[test]
    fn check_winner_diagonally() {
        let mut engine = Engine::new(Player::X);

        engine.update_state_for_testing([
            Some(Player::O), None,            None,
            None,            Some(Player::O), None,
            None,            None,            Some(Player::O),
        ]);

        assert!(engine.check_winner() == Some(Winner::O));

        engine.update_state_for_testing([
            None,            None,            Some(Player::O),
            None,            Some(Player::O), None,
            Some(Player::O), None,            None,
        ]);

        assert!(engine.check_winner() == Some(Winner::O));
    }

    #[test]
    fn check_winner_tie() {
        let mut engine = Engine::new(Player::X);

        engine.update_state_for_testing([
            Some(Player::X), Some(Player::O), Some(Player::X),
            Some(Player::X), Some(Player::O), Some(Player::O),
            Some(Player::O), Some(Player::X), Some(Player::O),
        ]);

        assert!(engine.check_winner() == Some(Winner::Tie));
    }
}
