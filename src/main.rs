use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::manager::Manager;
use crate::star::Star;

mod star;
mod manager;

const WIN_SIZE: (u32, u32) = (1280, 800);
const MAIN_LOOP_TIMEOUT_NANOS: u32 = 1_000_000_000u32 / 240;
const MAIN_LOOP_TIMEOUT_SECONDS: f64 = MAIN_LOOP_TIMEOUT_NANOS as f64 / 1_000_000_000.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Life game", WIN_SIZE.0, WIN_SIZE.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut manager = Manager::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    draw_circle(&mut canvas, 100, 100, 100);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        //canvas.present();

        manager.tick(MAIN_LOOP_TIMEOUT_SECONDS);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        manager.stars().iter().for_each(|star| draw_star(&mut canvas, star));
        canvas.present();

        ::std::thread::sleep(Duration::new(0, MAIN_LOOP_TIMEOUT_NANOS));
    }

    fn draw_star(canvas: &mut WindowCanvas, star: &Star) {
        let half_width: f64 = canvas.window().size().0 as f64 / 2.0;
        let half_height: f64 = canvas.window().size().1 as f64 / 2.0;

        let x = (half_width + half_width * star.x() / (star.z())) as u32;
        let y = (half_height + half_height * star.y() / (star.z())) as u32;
        if x >= canvas.window().size().0  || y >= canvas.window().size().1 || x <= 0 || y <= 0 {
            return;
        }
        //canvas.filled_circle(x, y, 1, Color::RGB(255, 255, 255)).unwrap();
        //canvas.filled_circle(10, 10, 1, Color::RGB(255, 255, 255)).unwrap();
        let c: u8 = (256.0*(1.0- star.z())) as u8;
        canvas.set_draw_color(Color::RGB(c, c, c));
        draw_circle(canvas, x as i32, y as i32, (2.0 * star.size()) as i32);
    }

    fn draw_circle(canvas: &mut WindowCanvas, x: i32, y: i32, radius: i32) {
        let mut offset_x: i32 = 0;
        let mut offset_y: i32 = radius;
        let mut d: i32 = radius - 1;
        let mut has_errors = false;
        //canvas.set_draw_color(Color::RGB(255, 255, 255));
        while offset_y >= offset_x {
            has_errors = canvas.draw_line(Point::new(x - offset_y, y + offset_x), Point::new(x + offset_y, y + offset_x)).is_err() || has_errors;
            has_errors = canvas.draw_line(Point::new(x - offset_x, y + offset_y), Point::new(x + offset_x, y + offset_y)).is_err() || has_errors;
            has_errors = canvas.draw_line(Point::new(x - offset_x, y - offset_y), Point::new(x + offset_x, y - offset_y)).is_err() || has_errors;
            has_errors = canvas.draw_line(Point::new(x - offset_y, y - offset_x), Point::new(x + offset_y, y - offset_x)).is_err() || has_errors;

            if has_errors {
                break;
            }

            if d >= 2*offset_x {
                d -= 2*offset_x + 1;
                offset_x += 1;
            } else if d < 2 * (radius - offset_y) {
                d += 2 * offset_y - 1;
                offset_y -= 1;
            } else {
                d += 2 * (offset_y - offset_x - 1);
                offset_y -= 1;
                offset_x += 1;
            }

        }
    }
}
