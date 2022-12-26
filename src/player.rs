use std::time::Duration;
use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::invaders::Invaders;
use crate::shot::Shot;

const MAX_SHOTS: usize = 10;
const INITIAL_SHOTS: usize = 3;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    max_shots: usize,
    pub turbo: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 4,
            shots: Vec::new(),
            max_shots: 3,
            turbo: false,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < self.max_shots {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn toggle_hyper_shot(&mut self) {
        self.turbo = !self.turbo;
        self.max_shots = if self.turbo { MAX_SHOTS } else { INITIAL_SHOTS };
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    shot.explode();
                    hit = true;
                }
            }
        }
        hit
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = String::from("A");
        for shot in self.shots.iter() {
            shot.draw(frame)
        }
    }
}