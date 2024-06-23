use crate::vec3::Vec3;

pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let (r, g, b) = (self.0.x, self.0.y, self.0.z);
        let r = (255.999 * r) as u8;
        let g = (255.999 * g) as u8;
        let b = (255.999 * b) as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}
