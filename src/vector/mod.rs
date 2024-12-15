use std::ops;

#[derive(Debug, Clone)]
pub struct Vector3 {
    e: [f64; 3],
}

impl Vector3 {
    pub fn new() -> Vector3 {
        Vector3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { e: [x, y, z] }
    }

    pub fn length(&self) -> f64 {
        (&self[0].powi(2) + &self[1].powi(2) + &self[2].powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn unit(&self) -> Vector3 {
        let len = self.length();
        Vector3 {
            e: [self[0] / len, self[1] / len, self[2] / len],
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            e: [self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]],
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
