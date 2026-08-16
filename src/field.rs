use std::collections::VecDeque;

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
    pub curve_length: u32,
    pub speed: f32,
    pub hide_ends: bool,
}

impl Field {
    fn outside(&self, x: f32, y: f32) -> bool {
        let m = 0.05 * self.width.max(self.height) as f32;
        x < -m || x > self.width as f32 + m || y < -m || y > self.height as f32 + m
    }

    /// Walk both ends of a curve onward through the flow field until they
    /// leave the canvas (plus a margin), so no curve endpoint is visible in
    /// the piece. Capped, since a vortex in the field can trap an end forever.
    fn extend_ends(&self, vertices: &mut VecDeque<Vertex>) {
        if !self.hide_ends {
            return;
        }
        let cap = (2.0 * (self.width + self.height) as f32 / self.step_size) as u32;
        let mut theta = vertices.back().unwrap().theta;
        for _ in 0..cap {
            let v = *vertices.back().unwrap();
            if self.outside(v.x, v.y) {
                break;
            }
            let x1 = v.x + self.step_size * v.theta.cos();
            let y1 = v.y + self.step_size * v.theta.sin();
            theta = (1.0 - self.speed) * theta
                + self.speed * noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            vertices.push_back(Vertex::new(x1, y1, theta));
        }
        let mut theta = vertices.front().unwrap().theta;
        for _ in 0..cap {
            let v = *vertices.front().unwrap();
            if self.outside(v.x, v.y) {
                break;
            }
            let x1 = v.x + self.step_size * (PI + v.theta).cos();
            let y1 = v.y + self.step_size * (PI + v.theta).sin();
            theta = (1.0 - self.speed) * theta
                + self.speed * noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            vertices.push_front(Vertex::new(x1, y1, theta));
        }
    }

    pub fn curve1(&self, x: f32, y: f32) -> Vec<Point> {
        let mut vertices: VecDeque<Vertex> = VecDeque::new();
        let mut theta = noise2d(&self.noise_function, &self.noise_opts, x, y) * PI;
        vertices.push_back(Vertex::new(x, y, theta));
        for _ in 0..self.curve_length {
            let v = *vertices.back().unwrap();
            let x1 = v.x + self.step_size * v.theta.cos();
            let y1 = v.y + self.step_size * v.theta.sin();
            theta = (1.0 - self.speed) * theta
                + self.speed * noise2d(&self.noise_function, &self.noise_opts, x1, y1) * PI;
            vertices.push_back(Vertex::new(x1, y1, theta));
        }
        self.extend_ends(&mut vertices);
        vertices.into_iter().map(|v| v.to_point()).collect()
    }

    pub fn curve2(&self, x: f32, y: f32) -> Vec<Point> {
        let mut vertices: VecDeque<Vertex> = VecDeque::new();
        let mut theta_back = noise2d(&self.noise_function, &self.noise_opts, x, y) * PI;
        let mut theta_front = theta_back;
        let v = Vertex::new(x, y, theta_back);
        vertices.push_back(v);
        let mut v_back: Vertex;
        let mut v_front: Vertex;
        let mut x_back1: f32;
        let mut y_back1: f32;
        let mut x_front1: f32;
        let mut y_front1: f32;
        let mut v1: Vertex;
        let mut v2: Vertex;
        for _ in 0..self.curve_length / 2 {
            v_back = *vertices.back().unwrap();
            v_front = *vertices.front().unwrap();
            x_back1 = v_back.x + self.step_size * v_back.theta.cos();
            y_back1 = v_back.y + self.step_size * v_back.theta.sin();
            x_front1 = v_front.x + self.step_size * (PI + v_front.theta).cos();
            y_front1 = v_front.y + self.step_size * (PI + v_front.theta).sin();
            theta_back = (1.0 - self.speed) * theta_back
                + self.speed
                    * noise2d(&self.noise_function, &self.noise_opts, x_back1, y_back1)
                    * PI;
            theta_front = (1.0 - self.speed) * theta_front
                + self.speed
                    * noise2d(&self.noise_function, &self.noise_opts, x_front1, y_front1)
                    * PI;
            v1 = Vertex::new(x_back1, y_back1, theta_back);
            v2 = Vertex::new(x_front1, y_front1, theta_front);
            vertices.push_back(v1);
            vertices.push_front(v2);
        }
        self.extend_ends(&mut vertices);
        vertices.into_iter().map(|v| v.to_point()).collect()
    }
}
