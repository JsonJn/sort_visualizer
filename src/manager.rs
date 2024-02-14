use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

pub trait State {
    fn new(width: u32, height: u32) -> Self;
    fn process_event(&mut self, event: Event);
    fn tick(&mut self, window_canvas: &mut WindowCanvas);
}

pub fn run<T: State>(width: u32, height: u32) {
    let mut state = T::new(width, height);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        // .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // canvas.window_mut().set_fullscreen(FullscreenType::True);

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            } else {
                state.process_event(event);
            }
        }

        canvas.set_draw_color(Color::WHITE);
        // let start = Instant::now();
        state.tick(&mut canvas);
        // let took = Instant::now().duration_since(start);
        // let nanos = took.as_nanos() as f64;
        // let theoretic_fps = 1_000_000_000.0f64 / nanos;
        // println!("frame time: {:?}ns", took.as_nanos());
        // println!("theoretic fps: {:?}", theoretic_fps);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
