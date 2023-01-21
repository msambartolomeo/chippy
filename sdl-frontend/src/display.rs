use std::error::Error;

use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::{pixels::Color, rect::Rect};

use chip::{Screen, DISPLAY_HEIGHT, DISPLAY_WIDTH};

const SCALE: u32 = 15;
const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const FOREGROUND_COLOR: Color = Color::RGB(255, 255, 255);

pub struct Display {
    canvas: WindowCanvas,
}

impl Display {
    pub fn init(sdl: &Sdl) -> Result<Display, Box<dyn Error>> {
        let video_subsystem = sdl.video()?;
        let window = video_subsystem
            .window(
                "chippy",
                SCALE * DISPLAY_WIDTH as u32,
                SCALE * DISPLAY_HEIGHT as u32,
            )
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        canvas.present();

        Ok(Display { canvas })
    }

    pub fn draw(&mut self, screen: &Screen) -> Result<(), String> {
        for (y, row) in screen.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let rect = Rect::new(
                    x as i32 * SCALE as i32,
                    y as i32 * SCALE as i32,
                    SCALE,
                    SCALE,
                );

                if *pixel {
                    self.canvas.set_draw_color(FOREGROUND_COLOR);
                } else {
                    self.canvas.set_draw_color(BACKGROUND_COLOR);
                }

                self.canvas.fill_rect(rect)?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
