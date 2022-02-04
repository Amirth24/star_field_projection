extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{MouseCursorEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent,  UpdateEvent};
use piston::window::WindowSettings;






pub mod app;







fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Star Field Simulation", [app::WIDTH as u32, app::HEIGHT as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = app::App::new(GlGraphics::new(opengl));


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(args) = e.mouse_cursor_args(){
            app.mouse_event(args);
        }
        
    }
}