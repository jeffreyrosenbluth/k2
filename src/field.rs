use std::collections::{HashMap, VecDeque};

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
    /// Constant rotation added to every field angle; used to keep curves
    /// perpendicular to a rotated seed column.
    pub angle_offset: f32,
}

impl Field {
    #[inline]
    fn field_angle(&self, x: f32, y: f32) -> f32 {
        noise2d(&self.noise_function, &self.noise_opts, x, y) * PI + self.angle_offset
    }

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
                + self.speed * self.field_angle(x1, y1);
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
                + self.speed * self.field_angle(x1, y1);
            vertices.push_front(Vertex::new(x1, y1, theta));
        }
    }

    /// Jobard-Lefer evenly spaced streamlines: no two curves come closer
    /// than half of `sep`, and new curves are seeded `sep` away from the
    /// existing ones until the canvas is saturated. Returns each curve with
    /// its seed point. `curve_length` caps the steps per direction and the
    /// usual momentum smoothing (`speed`) applies.
    pub fn evenly_spaced(&self, sep: f32, two_sided: bool) -> Vec<(Point, Vec<Point>)> {
        let sep = sep.max(2.0);
        let d_test = 0.5 * sep;
        let w = self.width as f32;
        let h = self.height as f32;
        let margin = 0.05 * w.max(h);
        let cell = sep;
        let key = |p: Point| ((p.x / cell).floor() as i32, (p.y / cell).floor() as i32);
        let mut grid: HashMap<(i32, i32), Vec<Point>> = HashMap::new();
        let near = |grid: &HashMap<(i32, i32), Vec<Point>>, p: Point, r: f32| -> bool {
            let (i, j) = key(p);
            for di in -1..=1 {
                for dj in -1..=1 {
                    if let Some(cell_pts) = grid.get(&(i + di, j + dj)) {
                        if cell_pts.iter().any(|q| q.dist2(p) < r * r) {
                            return true;
                        }
                    }
                }
            }
            false
        };
        let in_bounds =
            |p: Point| p.x > -margin && p.x < w + margin && p.y > -margin && p.y < h + margin;
        let cap = self.curve_length.max(1) as usize;
        let angle_at =
            |p: Point| self.field_angle(p.x, p.y);
        let mut out: Vec<(Point, Vec<Point>)> = Vec::new();
        let mut queue: VecDeque<Point> = VecDeque::from([pt(w / 2.0, h / 2.0)]);
        while let Some(seed) = queue.pop_front() {
            if !in_bounds(seed) || near(&grid, seed, sep) {
                continue;
            }
            let mut pts: VecDeque<Point> = VecDeque::new();
            pts.push_back(seed);
            let theta0 = angle_at(seed);
            for heading in [0.0, PI] {
                if heading > 0.0 && !two_sided {
                    break;
                }
                let mut theta = theta0;
                let mut p = seed;
                for _ in 0..cap {
                    let q = pt(
                        p.x + self.step_size * (heading + theta).cos(),
                        p.y + self.step_size * (heading + theta).sin(),
                    );
                    if !in_bounds(q) || near(&grid, q, d_test) {
                        break;
                    }
                    theta = (1.0 - self.speed) * theta + self.speed * angle_at(q);
                    if heading == 0.0 {
                        pts.push_back(q);
                    } else {
                        pts.push_front(q);
                    }
                    p = q;
                }
            }
            // discard stubs shorter than one separation
            if (pts.len() as f32) * self.step_size < sep {
                continue;
            }
            let pts: Vec<Point> = pts.into();
            for p in &pts {
                grid.entry(key(*p)).or_default().push(*p);
            }
            // candidate seeds one separation off each side, every ~sep of arc
            let every = ((sep / self.step_size).ceil() as usize).max(1);
            for p in pts.iter().step_by(every) {
                let a = angle_at(*p);
                let (nx, ny) = (-a.sin(), a.cos());
                queue.push_back(pt(p.x + sep * nx, p.y + sep * ny));
                queue.push_back(pt(p.x - sep * nx, p.y - sep * ny));
            }
            out.push((seed, pts));
        }
        out
    }

    pub fn curve1(&self, x: f32, y: f32) -> Vec<Point> {
        let mut vertices: VecDeque<Vertex> = VecDeque::new();
        let mut theta = self.field_angle(x, y);
        vertices.push_back(Vertex::new(x, y, theta));
        for _ in 0..self.curve_length {
            let v = *vertices.back().unwrap();
            let x1 = v.x + self.step_size * v.theta.cos();
            let y1 = v.y + self.step_size * v.theta.sin();
            theta = (1.0 - self.speed) * theta
                + self.speed * self.field_angle(x1, y1);
            vertices.push_back(Vertex::new(x1, y1, theta));
        }
        self.extend_ends(&mut vertices);
        vertices.into_iter().map(|v| v.to_point()).collect()
    }

    pub fn curve2(&self, x: f32, y: f32) -> Vec<Point> {
        let mut vertices: VecDeque<Vertex> = VecDeque::new();
        let mut theta_back = self.field_angle(x, y);
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
                + self.speed * self.field_angle(x_back1, y_back1);
            theta_front = (1.0 - self.speed) * theta_front
                + self.speed * self.field_angle(x_front1, y_front1);
            v1 = Vertex::new(x_back1, y_back1, theta_back);
            v2 = Vertex::new(x_front1, y_front1, theta_front);
            vertices.push_back(v1);
            vertices.push_front(v2);
        }
        self.extend_ends(&mut vertices);
        vertices.into_iter().map(|v| v.to_point()).collect()
    }
}
