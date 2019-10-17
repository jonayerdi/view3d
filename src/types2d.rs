use std::ops::{Add, Sub};

pub enum Dimension2D {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq> PartialEq for Vec2D<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl<T: Add<Output = T>> Add for Vec2D<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2D<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2D { x, y }
    }
}

#[derive(Clone)]
pub struct Triangle2d<T>(pub Vec2D<T>, pub Vec2D<T>, pub Vec2D<T>);

impl<T> Triangle2d<T> {
    pub fn sort_vectors_by<F>(&mut self, mut compare: F) -> &mut Self
    where
        F: FnMut(&Vec2D<T>, &Vec2D<T>) -> std::cmp::Ordering,
    {
        use std::cmp::Ordering;
        use std::mem::swap;
        // Comparisons
        let c01 = compare(&self.0, &self.1);
        let c12 = compare(&self.1, &self.2);
        let c02 = compare(&self.0, &self.2);
        // Handle every case
        if c01 == Ordering::Greater {
            if c12 == Ordering::Greater {
                swap(&mut self.0, &mut self.2);
            } else {
                swap(&mut self.0, &mut self.1);
                if c02 == Ordering::Greater {
                    swap(&mut self.1, &mut self.2);
                }
            }
        } else if c12 == Ordering::Greater {
            swap(&mut self.1, &mut self.2);
            if c02 == Ordering::Greater {
                swap(&mut self.0, &mut self.1);
            }
        }
        self
    }
}
