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

    // Enable backtrace for debugging
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }


    // Default window dimensions
    let wnd_width: u32 = 800;
    let wnd_height: u32 = 800;

    let sdl_context = sdl3::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("snake", wnd_width, wnd_height)
        .resizable().build()?;

    let mut canvas = wnd.into_canvas();

    // Create game state
    let mut game = game::Game::new(wnd_width, wnd_height)?;
    
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump()?;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                sdl3::event::Event::Quit {..} => {
                    running = false;
                }
                sdl3::event::Event::KeyDown {keycode, ..} => {
                    let key = keycode.expect("Keycode invalid for key down event!");
                    match key {
                        // If a player presses R, kill the snake and create a new game
                        sdl3::keyboard::Keycode::R => {
                            game.snake.die();
                            game = game::Game::new(wnd_width, wnd_height)?;
                        }
                        // If a player presses Q, quit the program
                        sdl3::keyboard::Keycode::Q => {
                            running = false;
                        }
                        // If a player presses P, pause the game
                        sdl3::keyboard::Keycode::P => {
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
