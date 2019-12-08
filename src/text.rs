use sdl2::render::WindowCanvas;
use sdl2::ttf::*;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::surface;
use sdl2::rect::Rect;

pub struct Text<'a> {
    context: &'a Sdl2TtfContext,
    filename: &'a str,
    font: Font<'a, 'a>,
    color: Color,
    size: u16,
}

impl<'a> Text<'a> {
    // With color
    pub fn new(ctx: &'a Sdl2TtfContext, filename: &'a str, size: u16, color: Color) -> Result<Text<'a>, String> {
        let font_result = ctx.load_font(Path::new(filename), size);
        if font_result.is_err()  {
            return Err(format!("Failed to initialize font for {}", filename));
        }
        Ok(Text {
            context: ctx,
            filename: filename,
            font: font_result.unwrap(),
            color: color,
            size: size,
        })
    }

    pub fn render_surface(&self, text: &'a str) ->  Result<surface::Surface, FontError> {
        let partial = self.font.render(text);
        partial.solid(self.color)
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_font_size(&mut self, size: u16) {
        let new_font = self.context.load_font(Path::new(self.filename), size).expect("Failed to set font size");
        self.font = new_font;
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, text: &'a str, x: i32, y: i32) {
        let surface = self.render_surface(text).expect("Failed creating surface for font");
        let creator = canvas.texture_creator();
        let texture = creator.create_texture_from_surface(&surface).expect("Failed creating texture");
        let query = texture.query();
        canvas.copy(&texture, None, Rect::new(x, y, query.width, query.height)).expect("Failed copying font texture");
    }

    pub fn draw_multi(&self, canvas: &mut WindowCanvas, text: &'a str, line_spacing: i32, x: i32, y: i32) {
        let split = text.split("[]");
        let mut y_pos = y;
        for line in split {
            let surface = self.render_surface(line).expect("Failed creating surface for font");
            let creator = canvas.texture_creator();
            let texture = creator.create_texture_from_surface(&surface).expect("Failed creating texture");
            let query = texture.query();
            canvas.copy(&texture, None, Rect::new(x, y_pos, query.width, query.height)).expect("Failed copying font texture");
            y_pos += self.size as i32 + line_spacing;
        }
    }
}