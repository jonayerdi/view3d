pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::with_capacity(width * height);
        buffer.extend((0..width * height).map(|_| 0));
        Framebuffer {
            width,
            height,
            buffer,
        }
    }
    #[inline]
    pub fn slice(&self) -> &[u32] {
        &self.buffer
    }
    #[inline]
    pub fn slice_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }
}
