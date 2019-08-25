use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> PartialEq for Vec2d<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl<T> Add for Vec2d<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2d<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Vec2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2d { x, y }
    }
}

#[derive(Clone)]
pub struct Triangle<T> {
    pub points: [Vec2d<T>; 3],
}

impl<T> Triangle<T> {
    pub fn new(points: [Vec2d<T>; 3]) -> Self {
        Triangle { points }
    }
}
