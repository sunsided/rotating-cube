use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let rad = angle * PI / 180.0;
        let cosa = rad.cos();
        let sina = rad.sin();

        let old_y = self.y;
        self.y = self.y * cosa - self.z * sina;
        self.z = old_y * sina + self.z * cosa;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let rad = angle * PI / 180.0;
        let cosa = rad.cos();
        let sina = rad.sin();

        let old_x = self.x;
        self.x = self.z * sina + self.x * cosa;
        self.z = self.z * cosa - old_x * sina;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let rad = angle * PI / 180.0;
        let cosa = rad.cos();
        let sina = rad.sin();

        let old_x = self.x;
        self.x = self.x * cosa - self.y * sina;
        self.y = self.y * cosa + old_x * sina;
    }

    pub fn project(&self, width: f32, height: f32, fov: f32, view_dist: f32) -> Point3D {
        let factor = fov / (view_dist + self.z);
        Point3D {
            x: self.x * factor + width / 2.0,
            y: -self.y * factor + height / 2.0,
            z: 1.0,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl std::ops::Mul<f32> for Point3D {
    type Output = Point3D;

    fn mul(self, scale: f32) -> Point3D {
        Point3D {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl std::ops::Mul<Point3D> for f32 {
    type Output = Point3D;

    fn mul(self, vec: Point3D) -> Point3D {
        vec * self
    }
}

impl std::ops::Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
