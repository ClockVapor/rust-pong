use std::default::Default;
use std::f64;

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
            let diffs: [(f64, [f64; 2]); 4] = [
                ((self.pos[0] - self.size[0]/2.0 - 
                (other.pos[0] + other.size[0]/2.0)).abs(), [1.0, 0.0]),
                ((self.pos[0] + self.size[0]/2.0 - 
                (other.pos[0] - other.size[0]/2.0)).abs(), [-1.0, 0.0]),
                ((self.pos[1] - self.size[1]/2.0 - 
                (other.pos[1] + other.size[1]/2.0)).abs(), [0.0, -1.0]),
                ((self.pos[1] + self.size[1]/2.0 - 
                (other.pos[1] - other.size[1]/2.0)).abs(), [0.0, 1.0])
            ];

            let mut min_d = f64::MAX;
            let mut normal: [f64; 2] = [0.0, 0.0];
            for diff in diffs.iter() {
                let &(d, n) = diff;
                if d < min_d {
                    min_d = d;
                    normal = n;
                }
            }
            Some(normal)
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
