use rand::Rng;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};

mod particle;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 400;

fn main() {
    let window = Window::new_centered("Particle System", (WIDTH, HEIGHT)).unwrap();

    window.run_loop(MyWindowHandler {
        p: Vec::new(),
        counter: 0,
        color: [1., 0., 0., 1.],
        mouse_pos: vec![0., 0.],
        start_pos: vec![(HEIGHT / 2) as f32, (WIDTH / 2) as f32],
    });
}

struct MyWindowHandler {
    p: Vec<particle::Particle>,
    counter: u32,
    color: [f32; 4],
    mouse_pos: Vec<f32>,
    start_pos: Vec<f32>,
}

impl WindowHandler for MyWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper, _info: WindowStartupInfo) {
        helper.set_resizable(false); // We don't handle resize yet.
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut rng = rand::thread_rng();
        if self.counter % 10 == 0 {
            self.p.push(particle::Particle::new(
                self.start_pos.clone(),
                vec![rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
                vec![0.0, 0.1],
                HEIGHT,
                WIDTH,
                self.color,
            ));
            self.counter = 0;
        }
        graphics.clear_screen(Color::BLACK);
        for part in &mut self.p {
            graphics.draw_circle(part.particle_pos(), part.size, part.particle_color());
            part.update();
        }
        self.counter += 1;
        self.p.retain(|x| !x.finished());

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vec2) {
        self.mouse_pos[0] = position.x;
        self.mouse_pos[1] = position.y;
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if button == MouseButton::Left {
            self.start_pos = self.mouse_pos.clone();
        }
        helper.request_redraw();
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<()>, unicode_codepoint: char) {
        let new_color: [f32; 4];

        if unicode_codepoint == 'r' {
            new_color = [1., 0., 0., 1.]
        } else if unicode_codepoint == 'g' {
            new_color = [0., 1., 0., 1.]
        } else if unicode_codepoint == 'b' {
            new_color = [0., 0., 1., 1.]
        } else {
            new_color = [0., 0., 0., 0.]
        }

        self.color = new_color;
        for i in &mut self.p {
            i.colour[0] = new_color[0]; // fix this to be RUSTY
            i.colour[1] = new_color[1];
            i.colour[2] = new_color[2];
        }

        helper.request_redraw();
    }
}
