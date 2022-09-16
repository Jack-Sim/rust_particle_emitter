extern crate piston_window;
use piston_window::*;
use rand::Rng;

mod particle;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

fn main() {
    let mut p: Vec<particle::Particle> = Vec::new();
    let mut counter = 0;
    let mut rng = rand::thread_rng();

    let mut window: PistonWindow = WindowSettings::new("Particle System", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    // The animation loop for the window
    while let Some(e) = window.next() {
        if counter % 10 == 0 {
            p.push(particle::Particle::new(
                vec![f64::from(WIDTH/2), f64::from(HEIGHT/2)],
                vec![rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
                vec![0.0, 0.1],
                HEIGHT,
                WIDTH,
            ));
        }
        window.draw_2d(&e, |c, g, _device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for i in &mut p {
                ellipse(i.colour, i.show(), c.transform, g);
                i.update();
            }
        });
        counter += 1;
        p.retain(|x| !x.clone().finished());
    }
}
