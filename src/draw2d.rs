use super::buffer::WindowBuffer;
use super::types2d::{Triangle, Vec2d};

#[allow(dead_code)]
impl WindowBuffer {
    // Bresenham's line algorithm
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line<T>(&mut self, p1: Vec2d<T>, p2: Vec2d<T>, color: u32)
    where
        Vec2d<T>: Into<Vec2d<usize>>,
    {
        let mut p1: Vec2d<usize> = p1.into();
        let mut p2: Vec2d<usize> = p2.into();
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

    pub fn draw_triangle<'a, T>(&mut self, triangle: &'a Triangle<T>, color: u32)
    where
        &'a Triangle<T>: Into<&'a Triangle<usize>>,
    {
        let triangle: &Triangle<usize> = triangle.into();
        self.draw_line(triangle.points[0], triangle.points[1], color);
        self.draw_line(triangle.points[1], triangle.points[2], color);
        self.draw_line(triangle.points[2], triangle.points[0], color);
    }

    pub fn fill_triangle<'a, T>(&mut self, triangle: &'a Triangle<T>, color: u32)
    where
        &'a Triangle<T>: Into<&'a Triangle<usize>>,
    {
        let triangle: &Triangle<usize> = triangle.into();
        // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    }

    pub fn fill_rect<T>(&mut self, point: Vec2d<T>, size: Vec2d<T>, color: u32)
    where
        Vec2d<T>: Into<Vec2d<usize>>,
    {
        let point: Vec2d<usize> = point.into();
        let size: Vec2d<usize> = size.into();
        let width = self.width;
        let slice = self.slice_mut();
        for y in point.y..point.y + size.y {
            let pixel_start = width * y + point.x;
            for pixel in slice[pixel_start..pixel_start + size.x].iter_mut() {
                *pixel = color;
            }
        }
    }
}
