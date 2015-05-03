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

    pub fn collision_normal(&self, other: &GameObject) -> Option<[f64; 2]> {
        if self.intersects(other) {
            let dx = (self.pos[0] - other.pos[0]).abs();
            let dy = (self.pos[0] - other.pos[0]).abs();

            if dx <= dy {
                if self.pos[0] <= other.pos[0] { Some([-1.0, 0.0]) }
                else { Some([1.0, 0.0]) }
            } else {
                if self.pos[1] <= other.pos[1] { Some([0.0, -1.0]) }
                else { Some([0.0, 1.0]) }
            }
        } else {
            None
        }
    }
}

impl Default for GameObject {
    fn default() -> GameObject {
        GameObject { pos: [0.0, 0.0], vel: [0.0, 0.0], size: [1.0, 1.0] }
    }
}
