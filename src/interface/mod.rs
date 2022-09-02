use crate::engine::{Engine, MoveKind, self};

use sdl2::{rect::Point, pixels::Color as SDL2Color, event::Event, rect::Rect, render::Canvas, video::Window, keyboard::Keycode};

use self::grid::{draw_grid_rects, CellRects};

mod grid;

const INIT_SIZE: (u32, u32) = (1024, 1024);
const BACKGROUND_COLOR: SDL2Color = SDL2Color::RGB(0x10, 0x10, 0x18);
const FOREGROUND_COLOR: SDL2Color = SDL2Color::RGB(0xff, 0xff, 0xff);

enum Input {
    Move(MoveKind),
    Place,
}

impl TryFrom<Keycode> for Input {
    type Error = ();

    fn try_from(key: Keycode) -> Result<Self, Self::Error> {
        Ok(match key {
            Keycode::Left  => Self::Move(MoveKind::Left),
            Keycode::Right => Self::Move(MoveKind::Right),
            Keycode::Up    => Self::Move(MoveKind::Up),
            Keycode::Down  => Self::Move(MoveKind::Down),
            Keycode::Space => Self::Place,
            _ => return Err(()),
        })
    }
}

pub fn run(mut engine: Engine) {
    let sdl = sdl2::init().expect("Failed to initialize SDL");

    let mut canvas = {
        let video = sdl.video().expect("Failed to initialize SDL video");

        let window = video
            .window("Tic-tac-toe", INIT_SIZE.0, INIT_SIZE.1)
            .position_centered()
            // disable for now to make the window floating
            // .resizable()
            .build()
            .expect("Failed to create SDL window");

        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("Failed to get SDL render canvas")
    };


    let mut events = sdl.event_pump().expect("Failed to get the SDL events");

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Ok(input) = Input::try_from(key) {
                        match input {
                            Input::Move(kind) => drop(engine.move_cursor(kind)),
                            Input::Place      => drop(engine.place()),
                        }
                    }
                }
                _ => {}

            }
        }

        draw(&mut canvas, &engine);
    }
}

fn draw(canvas: &mut Canvas<Window>, engine: &Engine) {
    canvas.clear();
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.fill_rect(Rect::new(0, 0, INIT_SIZE.0, INIT_SIZE.1)).unwrap();

    draw_grid_rects(canvas, engine, Point::new(100, 100));

    canvas.present();
}
