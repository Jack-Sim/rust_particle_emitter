use rand::Rng;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

mod particle;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

fn main() {
    let window = Window::new_centered("Particle System", (WIDTH, HEIGHT)).unwrap();
    window.run_loop(MyWindowHandler {
        p: Vec::new(),
        counter: 0,
    });
}

struct MyWindowHandler {
    p: Vec<particle::Particle>,
    counter: u32,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut rng = rand::thread_rng();
        if self.counter % 10 == 0 {
            self.p.push(particle::Particle::new(
                vec![(WIDTH / 2) as f32, (HEIGHT / 2) as f32],
                vec![rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
                vec![0.0, 0.1],
                HEIGHT,
                WIDTH,
            ));
            self.counter=0;
        }
        graphics.clear_screen(Color::from_rgb(0., 0., 0.));
        for i in &mut self.p {
            graphics.draw_circle(
                i.particle_pos(),
                i.size,
                Color::from_rgba(i.colour[0], i.colour[1], i.colour[2], i.colour[3]),
            );
            i.update();
        }
        self.counter += 1;
        self.p.retain(|x| !x.finished());

        helper.request_redraw();
    }
}
