#[derive(Clone)]
pub struct Particle {
    pub pos: Vec<f64>,
    pub vel: Vec<f64>,
    pub acc: Vec<f64>,
    size: f64,
    pub lifetime: f32,
    pub colour: [f32; 4],
    max_speed: f64,
    max_acc: f64,
    height: f64,
    width: f64,
}

impl Particle {
    pub fn new(pos: Vec<f64>, vel: Vec<f64>, acc: Vec<f64>, height: u32, width: u32) -> Particle {
        return Particle {
            pos: pos,
            vel: vel,
            acc: acc,
            lifetime: 1.0,
            size: 10.,
            colour: [1.0, 0.0, 0.0, 1.0],
            max_speed: 10.0,
            max_acc: 0.5,
            height: height as f64,
            width: width as f64,
        };
    }

    pub fn update(&mut self) {
        self.pos[0] = self.pos[0] + self.vel[0];
        self.vel[0] = self.vel[0] + self.acc[0];
        self.pos[1] = self.pos[1] + self.vel[1];
        self.vel[1] = self.vel[1] + self.acc[1];
        self.check_limits();
        self.edges();
        self.lifetime = self.lifetime - 0.01;
        self.colour = [1.0, 0.0, 0.0, self.lifetime];
    }

    pub fn show(&self) -> [f64; 4] {
        return [self.pos[0], self.pos[1], self.size, self.size];
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
        if self.pos[1] >= self.height || self.pos[1] <= 0.0 {
            self.vel[1] = self.vel[1] * -1.0;
        }
        if self.pos[0] >= self.width || self.pos[0] <= 0.0 {
            self.vel[0] = self.vel[0] * -1.0;
        }
    }

    pub fn finished(self) -> bool {
        return self.lifetime <= 0.;
    }
}
