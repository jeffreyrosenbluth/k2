use wassily::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, theta: f32) -> Self {
        Vertex { x, y, theta }
    }

    pub fn to_point(self) -> Point {
        pt(self.x, self.y)
    }
}

pub struct Field {
    pub noise_function: Box<dyn NoiseFn<f64, 2>>,
    pub noise_opts: NoiseOpts,
    pub step_size: f32,
    pub width: u32,
    pub height: u32,
    pub max_length: u32,
}

impl Field {
    pub fn curve(&mut self, x: f32, y: f32) -> Vec<Point> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut theta = noise2d(&self.noise_function, &self.noise_opts, x, y) * PI;
        let v = Vertex::new(x, y, theta);
        vertices.push(v);
        let mut v: Vertex;
        let mut x1: f32;
        let mut y1: f32;
        let mut v1: Vertex;
        for _ in 0..self.max_length {
            v = *vertices.last().unwrap();
            x1 = v.x + self.step_size * v.theta.cos();
            y1 = v.y + self.step_size * v.theta.sin();
            theta = noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            v1 = Vertex::new(x1, y1, theta);
            vertices.push(v1);
        }
        vertices.into_iter().map(|v| v.to_point()).collect()
    }
}
