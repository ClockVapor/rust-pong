extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod pong;
mod game_object;

fn main() {
    pong::play();
} 
