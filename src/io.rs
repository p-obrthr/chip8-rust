use raylib::prelude::*;

const SCALE: i32 = 10;

pub trait Io {
    fn render(&mut self, display: [[bool; 64]; 32]);
    fn check_update(&self) -> Option<Change>;
}

pub struct RaylibIo {
    rl: RaylibHandle,
    thread: RaylibThread,
}

#[derive(PartialEq)]
pub enum Change {
    Exit,
}

impl RaylibIo {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init()
            .size(64 * SCALE, 32 * SCALE)
            .title("Hello, World")
            .build();

        Self { rl, thread }
    }
}

impl Io for RaylibIo {
    fn render(&mut self, display: [[bool; 64]; 32]) {
        let mut d = self.rl.begin_drawing(&self.thread);

        for (j, col) in display.iter().enumerate() {
            for (i, pixel) in col.iter().enumerate() {
                if *pixel {
                    d.draw_rectangle(
                        i as i32 * SCALE,
                        j as i32 * SCALE,
                        SCALE,
                        SCALE,
                        Color::WHITE,
                    );
                }
            }
        }

        d.clear_background(Color::BLACK);
    }

    fn check_update(&self) -> Option<Change> {
        if self.rl.window_should_close() {
            return Some(Change::Exit);
        }

        None
    }
}
