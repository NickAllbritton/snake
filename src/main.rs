mod view;
mod model;

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

    // Create game_state
    let mut game = model::game::Game::new(wnd_width, wnd_height);
    
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                _ => {}
            }
        }

        game.clear_wnd(&mut canvas);

        game.update();

        game.draw_wnd(&mut canvas);
    }
    

    Ok(())
}
