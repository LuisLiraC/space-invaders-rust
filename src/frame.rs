use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<String>>;

pub fn new_frame(points: i32, turbo_enabled: bool) -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for col_num in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for row_num in 0..NUM_ROWS {
            if row_num == NUM_ROWS - 3 {
                col.push(String::from("-"));
            } else if row_num == NUM_ROWS - 2 {
                let score = format!("Score: {}", points);
                render_info(score, col_num, &mut col);
            } else if row_num == NUM_ROWS - 1 {
                let text = if turbo_enabled { "Enabled " } else { "Disabled" };
                let turbo = format!("Hyper Shot: {}", text);
                render_info(turbo, col_num, &mut col);
            } else {
                col.push(String::from(" "));
            }
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

fn render_info(text: String, col_num: usize, col: &mut Vec<String>) {
    match col_num {
        0 => col.push(text),
        _x => if _x >= text.len() {
            col.push(String::from(" "))
        }
    }
}