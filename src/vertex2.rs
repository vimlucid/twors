//! Represents a 2D point in space.

use std::ops::{Add, AddAssign, Mul, SubAssign};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vertex2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Mul for Vertex2<T>
where
    T: Mul<Output = T>,
{
    type Output = Vertex2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Add for Vertex2<T>
where
    T: AddAssign,
{
    type Output = Vertex2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result.x += rhs.x;
        result.y += rhs.y;
        result
    }
}

impl<T> AddAssign for Vertex2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for Vertex2<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vertex = Vertex2::new(1.0, 2.0);
        assert_eq!(vertex.x, 1.0);
        assert_eq!(vertex.y, 2.0)
    }

    #[test]
    fn mul() {
        let vertex = Vertex2::new(1.5, 2.0) * Vertex2::new(2.0, 3.0);
        assert_eq!(vertex.x, 3.0);
        assert_eq!(vertex.y, 6.0)
    }

    #[test]
    fn add_assign() {
        let mut vertex = Vertex2::new(1.0, 2.0);
        vertex += Vertex2::new(3.0, 4.0);
        assert_eq!(vertex.x, 4.0);
        assert_eq!(vertex.y, 6.0)
    }

    #[test]
    fn sub_assign() {
        let mut vertex = Vertex2::new(1.0, 2.0);
        vertex -= Vertex2::new(3.0, 1.0);
        assert_eq!(vertex.x, -2.0);
        assert_eq!(vertex.y, 1.0)
    }
}
