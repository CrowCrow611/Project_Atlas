use crate::math::V3;
use raylib::prelude::*;

const G:    f64 = 0.5;
const SOFT: f64 = 0.4;

#[allow(dead_code)]
pub struct Body {
    pub name:   &'static str,
    pub pos:    V3,
    pub vel:    V3,
    pub acc:    V3,
    pub mass:   f64,
    pub radius: f64,
    pub color:  Color,
}

pub struct Trail {
    pub pts: Vec<Vector3>,
}

impl Trail {
    pub fn new() -> Self { Self { pts: Vec::new() } }

    pub fn push(&mut self, p: Vector3) {
        self.pts.push(p);
        if self.pts.len() > 800 { self.pts.remove(0); }
    }

    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>, col: Color) {
        let n = self.pts.len();
        for i in 1..n {
            let t = i as f32 / n as f32;
            let a = (t * t * 150.) as u8;
            d.draw_line_3D(self.pts[i-1], self.pts[i], Color::new(col.r, col.g, col.b, a));
        }
    }
}

pub struct Sim {
    pub bodies: Vec<Body>,
    pub trails: Vec<Trail>,
    pub disc_rot: f32,
    pub wg_rot: f32,
    pub paused: bool,
    pub timescale: f64,
    pub time: f64,
}

fn compute_acc(bodies: &mut Vec<Body>) {
    let snap: Vec<(V3, f64)> = bodies.iter().map(|b| (b.pos, b.mass)).collect();
    for i in 0..bodies.len() {
        let mut a = V3::zero();
        for j in 0..snap.len() {
            if i == j { continue; }
            let r  = snap[j].0 - snap[i].0;
            let r2 = r.len()*r.len() + SOFT*SOFT;
            a = a + r.norm() * (G * snap[j].1 / r2);
        }
        bodies[i].acc = a;
    }
}

fn verlet(bodies: &mut Vec<Body>, dt: f64) {
    for b in bodies.iter_mut() { b.vel = b.vel + b.acc * (dt * 0.5); }
    for b in bodies.iter_mut() { b.pos = b.pos + b.vel * dt; }
    compute_acc(bodies);
    for b in bodies.iter_mut() { b.vel = b.vel + b.acc * (dt * 0.5); }
}

impl Sim {
    pub fn new() -> Self {
        let bh_m = 1.0e6_f64;
        let wg_m = 1.5e4_f64;
        let wg_r = 150.0_f64;
        let wg_v = (G * bh_m / wg_r).sqrt();
        let wp   = V3::new(wg_r, 0., 0.);
        let wv   = V3::new(0., wg_v, 0.);

        let mut bodies: Vec<Body> = vec![
            Body {
                name: "Urgo Prime",
                pos: V3::zero(), vel: V3::zero(), acc: V3::zero(),
                mass: bh_m, radius: 14.,
                color: Color::WHITE,
            },
            Body {
                name: "Urgo Aquas",
                pos: wp, vel: wv, acc: V3::zero(),
                mass: wg_m, radius: 14.,
                color: Color::new(30, 80, 200, 255),
            },
        ];

        for (r, name, color, radius, mass) in [
            (22., "Sentima", Color::new( 75, 170,  80, 255), 2.2,  4.),
            (36., "Venta",   Color::new(205, 120,  48, 255), 3.0,  7.),
            (56., "Kael",    Color::new(128,  82, 168, 255), 3.8, 10.),
        ] {
            let v = (G * wg_m / r).sqrt();
            bodies.push(Body {
                name,
                pos: V3::new(wp.x + r, wp.y, wp.z),
                vel: V3::new(wv.x, wv.y + v, wv.z),
                acc: V3::zero(),
                mass, radius, color,
            });
        }

        compute_acc(&mut bodies);
        let trails = (0..bodies.len()).map(|_| Trail::new()).collect();

        Sim {
            bodies, trails,
            disc_rot: 0., wg_rot: 0.,
            paused: false, timescale: 1.,
            time: 0.,
        }
    }

    pub fn tick(&mut self, raw_dt: f64) {
        if self.paused { return; }
        let dt   = raw_dt * self.timescale;
        self.time += dt;
        let subs = if self.timescale > 16. { 16 } else if self.timescale > 4. { 8 } else { 4 };
        for _ in 0..subs { verlet(&mut self.bodies, dt / subs as f64); }

        let tau = std::f32::consts::TAU;
        self.disc_rot = (self.disc_rot + raw_dt as f32 * self.timescale as f32 * 22.) % tau;
        self.wg_rot   = (self.wg_rot   + raw_dt as f32 * self.timescale as f32 *  9.) % tau;

        for i in 1..self.bodies.len() {
            let p = self.bodies[i].pos.to_rl();
            self.trails[i].push(p);
        }
    }
}