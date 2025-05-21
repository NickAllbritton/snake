mod game;
mod time;
mod board;
mod tile;
mod snake;
mod goal;
mod score;


fn main() -> Result<(), String> {

    let wnd_width: u32 = 1000;
    let wnd_height: u32 = 1000;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("snake", wnd_width, wnd_height)
        .build()
        .unwrap();

    let mut canvas = wnd.into_canvas()
        .build()
        .unwrap();

    // Create game state
    let mut game = game::Game::new(wnd_width, wnd_height);
    
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump()?;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                sdl2::event::Event::KeyDown {keycode, ..} => {
                    match keycode.unwrap() {
                        // If a player presses R, kill the snake and create a new game
                        sdl2::keyboard::Keycode::R => {
                            game.snake.die();
                            game = game::Game::new(wnd_width, wnd_height);
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
                            game.handle_key_press(keycode.unwrap());
                        }
                    }
                }
                _ => {}
            }
        }

        game.clear_wnd(&mut canvas);

        game.draw_wnd(&mut canvas);

        game.update();
    }
    

    Ok(())
}
