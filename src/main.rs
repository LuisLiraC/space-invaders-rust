use std::error::Error;
use std::sync::mpsc;
use std::{io, thread};
use std::time::{Duration, Instant};

use crossterm::{terminal, ExecutableCommand, event};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use space_invaders::audio_manager::{AudioManager, SoundEffect};
use space_invaders::frame::{Drawable, new_frame};
use space_invaders::invaders::Invaders;
use space_invaders::player::Player;
use space_invaders::render::render;

fn main() -> Result<(), Box<dyn Error>> {

    let mut audio_manager = AudioManager::new();
    audio_manager.play(SoundEffect::Startup);
    let mut points = 0;

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = new_frame(points, false);
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame(points, player.turbo);

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio_manager.play(SoundEffect::Shoot);
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio_manager.play(SoundEffect::Lose);
                        break 'gameloop;
                    },
                    KeyCode::Char('h') => {
                        player.toggle_hyper_shot();
                    },
                    _ => {}
                }
            }
        }
        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio_manager.play(SoundEffect::Move);
        }
        if player.detect_hits(&mut invaders) {
            audio_manager.play(SoundEffect::Explode);
            points += 1;
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }
        let _ = render_tx.send(current_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or Lose
        if invaders.all_killed() {
            audio_manager.play(SoundEffect::Win);
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            audio_manager.play(SoundEffect::Lose);
            break 'gameloop;
        }
    }

    // Clean up section
    drop(render_tx);
    render_handler.join().unwrap();
    audio_manager.stop();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
