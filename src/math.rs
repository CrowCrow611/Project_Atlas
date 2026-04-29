use raylib::prelude::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self::new(0., 0., 0.) }

    pub fn len(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        let l = self.len();
        if l < 1e-12 { Self::zero() } else { Self::new(self.x/l, self.y/l, self.z/l) }
    }

    pub fn to_rl(self) -> Vector3 {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl std::ops::Add for V3 {
    type Output = Self;
    fn add(self, o: Self) -> Self { V3::new(self.x+o.x, self.y+o.y, self.z+o.z) }
}

impl std::ops::Sub for V3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self { V3::new(self.x-o.x, self.y-o.y, self.z-o.z) }
}

impl std::ops::Mul<f64> for V3 {
    type Output = Self;
    fn mul(self, s: f64) -> Self { V3::new(self.x*s, self.y*s, self.z*s) }
}