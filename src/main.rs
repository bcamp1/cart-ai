extern crate sdl2;
extern crate rand;

mod cart;
mod neat;
mod text;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::mouse::{MouseState};
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use cart::Cart;
use text::Text;



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_size = (1280, 720);

    let window = video_subsystem.window(format!("Cart").as_str(), window_size.0, window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initiate Text
    let ttf_ctx = sdl2::ttf::init().expect("Failed to init SDL_TTF");
    let font = Text::new(&ttf_ctx, "./res/anon.ttf", 20, Color::RGB(255, 255, 255)).expect("Failed to create font");

    let mut cart = Cart::default();
    let mut high_score: u32 = 0;

    let track_thickness = 2.0;
    let track_color = Color::from((255, 255, 255));
    let track_y = 600.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    cart = Cart::default();
                    high_score = 0;
                },
                _ => {}
            }
        }

        // Keyboard input
        let key_state = KeyboardState::new(&event_pump);
        let right_pressed = key_state.is_scancode_pressed(Scancode::Right);
        let left_pressed = key_state.is_scancode_pressed(Scancode::Left);

        cart.update(right_pressed, left_pressed);

        if cart.dead {
            cart = Cart::default();
        }

        if cart.score > high_score {
            high_score = cart.score;
        }

        canvas.set_draw_color(Color::from((0, 0, 0)));
        canvas.clear();
        // Draw Track
        canvas.thick_line(0, track_y as i16, window_size.0 as i16, track_y as i16, track_thickness as u8, track_color);

        cart.draw(&mut canvas);
        font.draw_multi(&mut canvas, format!("SCORE: {}[]HIGH SCORE: {}", cart.score, high_score).as_str(), 5, 20, 20);
        canvas.present();
    }

}
