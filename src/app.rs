
use opengl_graphics::{GlGraphics};

use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;

use rand::Rng;
use std::f64;


fn map_range<T:std::ops::Add<Output = T>+ std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>+Copy>
( s: T, from_range: (T, T), to_range: (T, T)) -> T {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}



pub const WIDTH : i32 = 800;
pub const HEIGHT : i32 = 600;


struct Star{
    x: f64,
    y: f64,
    z: f64,
    pz: f64,
    speed: f64,
}

impl Star{
    fn new() -> Self{

            let z: f64 =  rand::thread_rng().gen_range(0..WIDTH) as f64;
        
        Self{
            x : rand::thread_rng().gen_range(-WIDTH/2..WIDTH/2).into(),
            y: rand::thread_rng().gen_range(-HEIGHT/2..HEIGHT/2).into(),
            z,
            pz:z,
            speed:0.5,
        }

    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs){


        let sx = map_range(self.x /self.z, (0.0,1.0), (0.0, args.window_size[0]));
        let sy = map_range(self.y /self.z, (0.0,1.0), (0.0, args.window_size[1]));

        let px= map_range(self.x/self.pz, (0.0,1.0), (0.0, args.window_size[0]));
        let py= map_range(self.y/self.pz, (0.0,1.0), (0.0, args.window_size[1]));
        let r = map_range(self.z, (0.0,args.window_size[1]), (4.0,0.0));
        // let circle = ellipse::circle(self.x , self.y  ;
        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.

            let transform = c.transform.trans(args.window_size[0]/2.0, args.window_size[1] /2.0);
            ellipse(color::WHITE, [sx,sy,r,r], transform, g);

            if self.speed > 14.0{

                line(graphics::color::WHITE, r/2.,[px,py,sx,sy],transform.trans(r/2., r/2.),g);
            }

        });
     
    }

    pub fn update(&mut self, acceleration: f64, dt: f64){
        self.pz = self.z;
        self.z -= self.speed *dt;

        if self.z < 1.0{
            self.z = rand::thread_rng().gen_range(0..WIDTH).into();
            self.x = rand::thread_rng().gen_range(-WIDTH/2..WIDTH/2).into();
            self.y = rand::thread_rng().gen_range(-HEIGHT/2..HEIGHT/2).into();
            self.pz = self.z;
        }       
        if self.speed > 0.0{

            self.speed += 0.5 * acceleration * dt;
        }else {
            self.speed = 0.5;
        }

    }
}


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.

    // Stars 
    stars: Vec<Star>,
    acceleration: f64,

}

impl App {


    pub fn new(gl: GlGraphics,) -> Self{

        let mut stars:Vec<Star> = vec![Star::new()];

        for _ in 0..=400{
            stars.push(Star::new())
        }
        Self{
            gl,
            stars,
            acceleration: 0.0,

        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        
        
        self.gl.draw(args.viewport(), |_, gl| {
            // Clear the screen.
            clear(graphics::color::BLACK, gl);

        });

        for star in &mut self.stars{
            star.render(&mut self.gl, args)
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        for star in &mut self.stars{
            star.update(self.acceleration ,args.dt);
        }
    }

    pub fn mouse_event(&mut self, args: [f64;2]){
        self.acceleration = map_range(args[1], (HEIGHT.into(),0.0), (-100.0,100.0));
    }
}