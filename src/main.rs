mod game;
mod time;
mod board;
mod tile;
mod snake;
mod goal;
mod score;

use std::error::Error;

//  TODO: Learn what traits are: for now take this return type as any type of error
fn main() -> Result<(), Box<dyn Error>> {

    let wnd_width: u32 = 1000;
    let wnd_height: u32 = 1000;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("snake", wnd_width, wnd_height).build()?;

    let mut canvas = wnd.into_canvas().build()?;

    // Create game state
    let mut game = game::Game::new(wnd_width, wnd_height)?;
    
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump()?;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                sdl2::event::Event::KeyDown {keycode, ..} => {
                    let key = keycode.ok_or(sdl2::Error::UnsupportedError)?;
                    match key {
                        // If a player presses R, kill the snake and create a new game
                        sdl2::keyboard::Keycode::R => {
                            game.snake.die();
                            game = game::Game::new(wnd_width, wnd_height)?;
                        }
                        // If a player presses Q, quit the program
                        sdl2::keyboard::Keycode::Q => {
                            running = false;
                        }
                        // If a player presses P, pause the game
                        sdl2::keyboard::Keycode::P => {
                            game.toggle_pause();
                        }
                        // Otherwise let game handle the keyboard input
                        _ => {
                            game.handle_key_press(key);
                        }
                    }
                }
                _ => {}
            }
        }

        game.clear_wnd(&mut canvas);

        game.draw_wnd(&mut canvas)?;

        game.update();
    }
    

    Ok(())
}
