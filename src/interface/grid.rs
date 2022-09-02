use crate::engine::{WIDTH, Engine};
use sdl2::rect::Point;
use sdl2::{pixels::Color as SDL2Color, rect::Rect, render::Canvas, video::Window};

const CELL_SIZE: i32 = 128;

pub type CellRects = [Rect;WIDTH * WIDTH];

fn draw_cross(canvas: &mut Canvas<Window>, position: Point, width: i32, offset: Point) {
    let x = position.x;
    let y = position.y;

    canvas.draw_line(
        Point::new(x, y),
        Point::new(x + width / 2, y + width / 2)
    ).unwrap();

    canvas.draw_line(
        Point::new(x, y + width / 2),
        Point::new(x + width / 2, y)
    ).unwrap();
}

pub fn draw_grid_rects(canvas: &mut Canvas<Window>, engine: &Engine, offset: Point) {
    for (idx, cell) in engine.state.iter().enumerate() {
        let i = idx as usize;

        let x = (i % WIDTH) as i32;
        let y = (i / WIDTH) as i32;

        if engine.cursor_pos.x == x && engine.cursor_pos.y == y {
            canvas.set_draw_color(SDL2Color::RED);
        }
        else {
            canvas.set_draw_color(SDL2Color::WHITE);
        }


        canvas.draw_rect(Rect::new(
            x * CELL_SIZE + offset.x,
            y * CELL_SIZE + offset.y,
            CELL_SIZE as u32,
            CELL_SIZE as u32
        )).unwrap();

        match cell {
            Some(crate::engine::Player::X) => {
                // @TODO: draw X
                canvas.set_draw_color(SDL2Color::GREEN);

                draw_cross(
                    canvas,
                    Point::new(x * CELL_SIZE + 32 + offset.x, y * CELL_SIZE + 32 + offset.y),
                    CELL_SIZE,
                    offset
                );
            },
            Some(crate::engine::Player::O) => {
                // @TODO: draw O
                canvas.set_draw_color(SDL2Color::YELLOW);

                canvas.draw_rect(Rect::new(
                    x * CELL_SIZE + 32 + offset.x,
                    y * CELL_SIZE + 32 + offset.y,
                    CELL_SIZE as u32 - 64,
                    CELL_SIZE as u32 - 64
                )).unwrap();
            },
            None => {
                canvas.set_draw_color(SDL2Color::WHITE);
            }
        }
    }
}
