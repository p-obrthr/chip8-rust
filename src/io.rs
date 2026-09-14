use raylib::prelude::*;

pub trait Io {
    fn render(&mut self);
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
        let (rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        Self { rl, thread }
    }
}

impl Io for RaylibIo {
    fn render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }

    fn check_update(&self) -> Option<Change> {
        if self.rl.window_should_close() {
            return Some(Change::Exit);
        }

        None
    }
}
