use std::default::Default;

pub struct GameObject {
    pub pos: [f64; 2],
    pub vel: [f64; 2],
    pub size: [f64; 2],
}

impl GameObject {
    pub fn intersects(&self, other: &GameObject) -> bool {
        self.pos[0] - self.size[0]/2.0 < other.pos[0] + other.size[0]/2.0 &&
        self.pos[0] + self.size[0]/2.0 > other.pos[0] - other.size[0]/2.0 &&
        self.pos[1] - self.size[1]/2.0 < other.pos[1] + other.size[1]/2.0 &&
        self.pos[1] + self.size[1]/2.0 > other.pos[1] - other.size[1]/2.0
    }
}

impl Default for GameObject {
    fn default() -> GameObject {
        GameObject { pos: [0.0, 0.0], vel: [0.0, 0.0], size: [1.0, 1.0] }
    }
}
