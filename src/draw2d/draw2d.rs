use super::line::LineIter;
use super::{Triangle2D, Vec2D};
use crate::framebuffer::Framebuffer;

#[allow(dead_code)]
impl Framebuffer {
    pub fn fill_rect(&mut self, point: Vec2D<usize>, size: Vec2D<usize>, color: u32) {
        let width = self.width;
        let slice = self.slice_mut();
        for y in point.y..point.y + size.y {
            let pixel_start = width * y + point.x;
            for pixel in slice[pixel_start..pixel_start + size.x].iter_mut() {
                *pixel = color;
            }
        }
    }

    pub fn draw_line(&mut self, p1: Vec2D<usize>, p2: Vec2D<usize>, color: u32) {
        let width = self.width;
        let slice = self.slice_mut();
        for p in LineIter::new(p1, p2) {
            slice[width * p.y + p.x] = color;
        }
    }

    pub fn draw_triangle<'a>(&mut self, triangle: &'a Triangle2D<usize>, color: u32) {
        let t = triangle;
        self.draw_line(t.0, t.1, color);
        self.draw_line(t.1, t.2, color);
        self.draw_line(t.2, t.0, color);
    }

    /// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn fill_triangle<'a>(&mut self, triangle: &'a Triangle2D<usize>, color: u32) {
        // Sort triangle vertices by y position
        // vertices with same y position sorted by x position
        let t = triangle
            .clone()
            .sort_vectors_by(|v1, v2| v1.x.cmp(&v2.x))
            .sort_vectors_by(|v1, v2| v1.y.cmp(&v2.y));
        // Check for top-flat triangle
        if t.0.y == t.1.y {
            self.fill_top_flat_triangle(&t, color);
        }
        // Check for bottom-flat triangle
        else if t.1.y == t.2.y {
            self.fill_bottom_flat_triangle(&t, color);
        }
        // General case, split triangle into top-flat and bottom-flat pair
        else {
            // Calculate new cutoff point t4, which is vertically aligned with t.1
            let t4 = Vec2D {
                x: (t.0.x as isize
                    + (((t.1.y - t.0.y) as f64 / (t.2.y - t.0.y) as f64) * (t.2.x - t.0.x) as f64)
                        as isize) as usize,
                y: t.1.y,
            };
            // The new point t4 could be to the left or to the right of t.1
            let (t_bottom, t_top) = if t4.x < t.1.x {
                (Triangle2D(t.0, t4, t.1), Triangle2D(t4, t.1, t.2))
            } else {
                (Triangle2D(t.0, t.1, t4), Triangle2D(t.1, t4, t.2))
            };
            self.fill_bottom_flat_triangle(&t_bottom, color);
            self.fill_top_flat_triangle(&t_top, color);
        }
    }

    /// Vertices must already be sorted by y and x
    ///
    /// t.0.y >= t.1.y
    ///
    /// t.1.y == t.2.y
    ///
    /// t.1.x <= t.2.x
    fn fill_bottom_flat_triangle(&mut self, t: &Triangle2D<usize>, color: u32) {
        // Fill from top to bottom
        self.fill_flat_triangle(LineIter::new(t.0, t.1), LineIter::new(t.0, t.2), color);
    }

    ///
    /// Vertices must already be sorted by y and x:
    ///
    /// t.0.y == t.1.y
    ///
    /// t.0.x <= t.1.x
    ///
    /// t.1.y >= t.2.y
    fn fill_top_flat_triangle(&mut self, t: &Triangle2D<usize>, color: u32) {
        // Fill from top to bottom
        self.fill_flat_triangle(LineIter::new(t.2, t.0), LineIter::new(t.2, t.1), color);
    }

    fn fill_flat_triangle(
        &mut self,
        mut left_line: LineIter<usize>,
        mut right_line: LineIter<usize>,
        color: u32,
    ) {
        let width = self.width;
        let slice = self.slice_mut();
        // First line points for both sides
        let mut left = left_line.next();
        let mut right = right_line.next();
        while let (Some(left_point), Some(right_point)) = (left, right) {
            // Draw line from left_point to right_point
            let y = left_point.y;
            let pixel_start = width * y + left_point.x;
            let distance = right_point.x - left_point.x;
            for pixel in slice[pixel_start..pixel_start + distance].iter_mut() {
                *pixel = color;
            }
            // Iterate until next left and right points (next y position)
            left = left_line.find(|&p| p.y != y);
            if left.is_some() {
                right = right_line.find(|&p| p.y != y);
            }
        }
    }
}
