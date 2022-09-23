use speedy2d::{dimen::Vec2, color::Color};
pub struct Particle {
    pub pos: Vec<f32>,
    pub vel: Vec<f32>,
    pub acc: Vec<f32>,
    pub size: f32,
    pub lifetime: f32,
    pub colour: [f32; 4],
    max_speed: f32,
    max_acc: f32,
    height: u32,
    width: u32,
}

impl Particle {
    pub fn new(pos: Vec<f32>, vel: Vec<f32>, acc: Vec<f32>, height: u32, width: u32, colour: [f32; 4]) -> Particle {
        Particle {
            pos,
            vel,
            acc,
            lifetime: 1.0,
            size: 5.,
            colour,
            max_speed: 10.0,
            max_acc: 0.5,
            height,
            width,
        }
    }

    pub fn update(&mut self) {
        self.pos[0] += self.vel[0];
        self.vel[0] += self.acc[0];
        self.pos[1] += self.vel[1];
        self.vel[1] += self.acc[1];
        self.check_limits();
        self.edges();
        self.lifetime -= 0.01;
        self.colour[3] = self.lifetime;
    }

    pub fn particle_pos(&self) -> Vec2 {
        Vec2::new(self.pos[0], self.pos[1])
    }

    pub fn particle_color(&self) -> Color {
        Color::from_rgba(self.colour[0], self.colour[1], self.colour[2], self.colour[3])
    }

    fn check_limits(&mut self) {
        if self.vel[0] > self.max_speed {
            self.vel[0] = self.max_speed;
        }
        if self.vel[1] > self.max_speed {
            self.vel[1] = self.max_speed;
        }
        if self.acc[0] > self.max_acc {
            self.acc[0] = self.max_acc;
        }
        if self.acc[1] > self.max_acc {
            self.acc[1] = self.max_acc;
        }
    }

    fn edges(&mut self) {
        if self.pos[1] >= self.height as f32 || self.pos[1] <= 0.0 {
            self.vel[1] *= -1.0;
        }
        if self.pos[0] >= self.width as f32 || self.pos[0] <= 0.0 {
            self.vel[0] *= -1.0;
        }
    }

    pub fn finished(&self) -> bool {
        self.lifetime <= 0.
    }
}
