/// Bresenham Algorithm
/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

use crate::util::approx::{ApproxInto,ApproxFrom};
use super::{Dimension2D, Vec2D};
use std::cmp::PartialOrd;
use std::ops::Add;
use std::convert::{TryFrom,TryInto};

pub struct LineIter<T> {
    index_max: T,
    delta_error: f64,
    index_dimension: Dimension2D,
    value_increment: isize,
    index: T,
    value: T,
    error: f64,
}

impl<T> LineIter<T>
where
    T: Copy + ApproxInto<f64>,
{
    pub fn new(p1: Vec2D<T>, p2: Vec2D<T>) -> LineIter<T> {
        let mut p1 = p1;
        let mut p2 = p2;
        let mut delta_x: f64 = p2.x.approx() - p1.x.approx();
        let mut delta_y: f64 = p2.y.approx() - p1.y.approx();
        let (index_dimension, delta_large, delta_error) = if delta_x.abs() > delta_y.abs() {
            (Dimension2D::X, delta_x, (delta_y / delta_x).abs())
        } else {
            (Dimension2D::Y, delta_y, (delta_x / delta_y).abs())
        };
        if delta_large < 0.0 {
            // Sort points so that p1.[index_dimension] < p2.[index_dimension]
            std::mem::swap(&mut p1, &mut p2);
            delta_x = -delta_x;
            delta_y = -delta_y;
        }
        let (index, index_max, value, value_increment) = match index_dimension {
            Dimension2D::X => (p1.x, p2.x, p1.y, if delta_y >= 0.0 { 1 } else { -1 }),
            Dimension2D::Y => (p1.y, p2.y, p1.x, if delta_x >= 0.0 { 1 } else { -1 }),
        };
        LineIter {
            index_max,
            delta_error,
            index_dimension,
            value_increment,
            index,
            value,
            error: 0.0,
        }
    }
}

impl<T> Iterator for LineIter<T>
where
    T: Copy + PartialOrd + Add<Output=T> + TryInto<isize> + TryFrom<isize> + ApproxFrom<usize>,
{
    type Item = Vec2D<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // Iteration ended
        if self.index > self.index_max {
            return None;
        }
        // Initialize the current point
        let current = match self.index_dimension {
            Dimension2D::X => Some(Vec2D::new(self.index, self.value).into()),
            Dimension2D::Y => Some(Vec2D::new(self.value, self.index).into()),
        };
        // Increment index
        self.index = self.index + T::approx_from(1);
        if self.index <= self.index_max {
            // Calculate error and value for the next point
            self.error += self.delta_error;
            // Increment/decrement y value accordingly
            if self.error > 0.5 {
                self.value = T::try_from(self.value.try_into().unwrap_or_else(|_| panic!()) + self.value_increment).unwrap_or_else(|_| panic!());
                self.error -= 1.0;
            }
        }
        // Return the current point
        current
    }
}
