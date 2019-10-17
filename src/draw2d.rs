use super::framebuffer::Framebuffer;
use super::types2d::{Triangle2d, Vec2D};

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

    /// Bresenham's line algorithm
    /// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line(&mut self, p1: Vec2D<usize>, p2: Vec2D<usize>, color: u32) {
        let mut p1 = p1;
        let mut p2 = p2;
        let mut delta_x = p2.x as isize - p1.x as isize;
        let mut delta_y = p2.y as isize - p1.y as isize;
        let width = self.width;
        let slice = self.slice_mut();
        let mut error = 0.0;
        if delta_x.abs() > delta_y.abs() {
            // Sort points so that p1.x < p2.x
            if delta_x < 0 {
                std::mem::swap(&mut p1, &mut p2);
                delta_x = -delta_x;
                delta_y = -delta_y;
            }
            // y_sign: 1 if slope goes upwards, or -1 if it goes downwards
            let y_sign: isize = if delta_y >= 0 { 1 } else { -1 };
            let delta_err = (delta_y as f64 / delta_x as f64).abs();
            let mut y = p1.y;
            // Iterate over x positions, paint one pixel on each
            for x in p1.x..=p2.x {
                slice[width * y + x] = color;
                error += delta_err;
                // Increment/decrement y position accordingly
                if error > 0.5 {
                    y = (y as isize + y_sign) as usize;
                    error -= 1.0;
                }
            }
        } else {
            // Sort points so that p1.y < p2.y
            if delta_y < 0 {
                std::mem::swap(&mut p1, &mut p2);
                delta_x = -delta_x;
                delta_y = -delta_y;
            } else if delta_y == 0 {
                // delta_x.abs() <= delta_y.abs()
                // therefore, delta_x == 0
                // hence, p1 == p2, nothing to draw
                return;
            }
            // x_sign: 1 if slope goes to the right, or -1 if it goes to the left
            let x_sign: isize = if delta_x >= 0 { 1 } else { -1 };
            let delta_err = (delta_x as f64 / delta_y as f64).abs();
            let mut x = p1.x;
            // Iterate over y positions, paint one pixel on each
            for y in p1.y..=p2.y {
                slice[width * y + x] = color;
                error += delta_err;
                // Increment/decrement x position accordingly
                if error > 0.5 {
                    x = (x as isize + x_sign) as usize;
                    error -= 1.0;
                }
            }
        }
    }

    pub fn draw_triangle<'a>(&mut self, triangle: &'a Triangle2d<usize>, color: u32) {
        let t = triangle;
        self.draw_line(t.0, t.1, color);
        self.draw_line(t.1, t.2, color);
        self.draw_line(t.2, t.0, color);
    }

    /// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn fill_triangle<'a>(&mut self, triangle: &'a Triangle2d<usize>, color: u32) {
        // Sort triangle vertices by y position
        // vertices with same y position sorted by x position
        let mut t = triangle.clone();
        t.sort_vectors_by(|v1, v2| v1.x.cmp(&v2.x))
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
            let t4 = Vec2D {
                x: (t.0.x as isize
                    + (((t.1.y - t.0.y) as f64 / (t.2.y - t.0.y) as f64) * (t.2.x - t.0.x) as f64)
                        as isize) as usize,
                y: t.1.y,
            };
            // The new point t4 could be to the left or to the right of t.1
            let (t_bottom, t_top) = if t4.x < t.1.x {
                (Triangle2d(t.0, t4, t.1), Triangle2d(t4, t.1, t.2))
            } else {
                (Triangle2d(t.0, t.1, t4), Triangle2d(t.1, t4, t.2))
            };
            self.fill_bottom_flat_triangle(&t_bottom, color);
            self.fill_top_flat_triangle(&t_top, color);
        }
    }

    /// Bresenham Algorithm
    /// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    ///
    /// Vertices must already be sorted by y and x
    ///
    /// t.0.y >= t.1.y
    ///
    /// t.1.y == t.2.y
    ///
    /// t.1.x <= t.2.x
    fn fill_bottom_flat_triangle(&mut self, t: &Triangle2d<usize>, color: u32) {}

    /// Bresenham Algorithm
    /// http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    ///
    /// Vertices must already be sorted by y and x:
    ///
    /// t.0.y == t.1.y
    ///
    /// t.0.x <= t.1.x
    ///
    /// t.1.y >= t.2.y
    fn fill_top_flat_triangle(&mut self, t: &Triangle2d<usize>, color: u32) {}
}
