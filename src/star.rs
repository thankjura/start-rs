pub struct Star {
    x: f64,
    y: f64,
    z: f64,
    size: f64,
}

impl Star {
    pub fn new(x: f64, y: f64, z: f64, size: f64) -> Self {
        Self {
            x, y, z, size
        }
    }

    pub fn add_z(&mut self, z: f64) {
        self.z += z;
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}