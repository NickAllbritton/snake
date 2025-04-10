mod view;

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

    // TODO: Why is the board off center from the window???
    // Create board_view
    let mut board = view::board::Board::new((wnd_width/20 - 5).try_into().unwrap(),
                                            (wnd_height/20 - 5).try_into().unwrap(),
                                            wnd_width*9/10, wnd_height*9/10);
    // Create game_state
    
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


        // Black background color
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); // Paint the background color
        
        board.render(&mut canvas);

        canvas.present();
    }
    

    Ok(())
}
