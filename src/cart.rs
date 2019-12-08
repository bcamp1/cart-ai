use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use random_color::{Luminosity, RandomColor};
use rand::prelude::*;

const PI: f32 = 3.14159;

pub struct Cart {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub mass: f32,
    pub power: f32,
    pub pos: f32,
    pub vel: f32,
    pub acc: f32,
    pub track_friction: f32,

    pub pend_length: f32,
    pub pend_width: f32,
    pub pend_color: Color,
    pub ball_size: f32,
    pub pend_friction: f32,

    pub angle_pos: f32,
    pub angle_vel: f32,
    pub angle_acc: f32,

    pub gravity: f32,

    pub track_length: f32,
    pub window_width: f32,
    
    pub score: u32,
    pub dead: bool,
}

impl Cart {

    pub fn new(pend_length: f32, gravity: f32, mass: f32, power: f32, track_friction: f32, pend_friction: f32, track_length: f32, window_width: f32) -> Cart {
        let cart_color_array = RandomColor::new().luminosity(Luminosity::Bright).to_rgb_array();
        let cart_color = Color::from((cart_color_array[0] as u8, cart_color_array[1] as u8, cart_color_array[2] as u8));

        let pend_color_array = RandomColor::new().luminosity(Luminosity::Bright).to_rgb_array();
        let pend_color = Color::from((pend_color_array[0] as u8, pend_color_array[1] as u8, pend_color_array[2] as u8));

        let track_color_array = RandomColor::new().luminosity(Luminosity::Bright).to_rgb_array();
        let track_color = Color::from((track_color_array[0] as u8, track_color_array[1] as u8, track_color_array[2] as u8));

        // Pick Starting Angle
        let angle_pos = if rand::random() {
            0.001
        } else {
            -0.001
        };

        Cart {
            width: 30.0,
            height: 10.0,
            color: Color::from((255, 255, 255)),
            mass: mass,
            power: power,
            pos: 0.0,
            vel: 0.0,
            acc: 0.0,
            track_friction: track_friction,
            pend_length: pend_length,
            pend_width: 2.0,
            pend_color: Color::from((255, 50, 0)),
            ball_size: 4.0,
            pend_friction: pend_friction,
            angle_pos: angle_pos,
            angle_vel: 0.0,
            angle_acc: 0.0,
            gravity: gravity,
            track_length: track_length,
            window_width: window_width,
            score: 0,
            dead: false,
        }
    }

    pub fn default() -> Cart {
        let track_length = 1200.0;
        let pend_length = 500.0;
        let gravity = 10.0;
        let cart_mass = 700.0;
        let cart_power = 12.0;
        let track_friction = 10.0;
        let pend_friction = 1.0;

        Cart::new(
            pend_length,
            gravity/1000.0,
            cart_mass/1000.0,
            cart_power/1000.0,
            track_friction/1000.0,
            pend_friction/1000.0,
            track_length,
            1280.0,
        )
    }
      

    pub fn update(&mut self, right_pressed: bool, left_pressed: bool) {
        // Keyboard Controls
        self.acc = 0.0;

        if right_pressed {
            self.acc += self.power;
        }

        if left_pressed {
            self.acc -= self.power;
        }

        self.acc -= self.track_friction * self.vel; // Add Track Friction

        // Cart Kinematics
        self.vel += self.acc;
        self.pos += self.vel;

        if self.pos >= self.track_length / 2.0 {
            self.pos = self.track_length / 2.0;
            self.vel = 0.0;
            self.acc = 0.0;
        }

        if self.pos <= -self.track_length / 2.0 {
            self.pos = -self.track_length / 2.0;
            self.vel = 0.0;
            self.acc = 0.0;
        }

        // Update Pendulum
        self.angle_acc = (-self.mass * self.acc * self.angle_pos.cos() + self.gravity * self.angle_pos.sin()) / self.pend_length;
        self.angle_acc -= self.pend_friction * self.angle_vel;  

        self.angle_vel += self.angle_acc;
        self.angle_pos += self.angle_vel;

        // Update Score
        self.score += 1;

        if (self.angle_pos >= PI/2.0 || self.angle_pos <= -PI/2.0) {
            self.dead = true;
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        // Track
        let track_thickness = 2.0;
        let track_y = 600.0;
        let track_x1 = (self.window_width - self.track_length) / 2.0;
        let track_x2 = track_x1 + self.track_length;
        let track_center = track_x1 + self.track_length / 2.0;

        // Cart Body
        let cart_x = track_center + self.pos - self.width/2.0;
        let cart_y = track_y - track_thickness/2.0 - self.height;
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(cart_x as i32, cart_y as i32, self.width as u32, self.height as u32));

        // Pendulum Pole
        let x1 = cart_x + self.width / 2.0;
        let y1 = cart_y;
        let dx = self.pend_length * self.angle_pos.sin();
        let dy = -self.pend_length * self.angle_pos.cos();
        let x2 = x1 + dx;
        let y2 = y1 + dy;

        canvas.thick_line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, self.pend_width as u8, self.pend_color);

        // Pendulum Ends
        canvas.filled_circle(x2 as i16, y2 as i16, self.ball_size as i16, self.pend_color);
        canvas.filled_circle(x1 as i16, y1 as i16, self.pend_width as i16, self.color);

    }
}

