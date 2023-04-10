pub trait RenderTarget<C, E> {
    fn put_pixel(&mut self, pos: (u32, u32), colour: C) -> Result<(), E>;
    fn size(&self) -> (u32, u32);
}

#[cfg(feature="embedded-graphics-rendertarget")]
mod embedded_graphics_rendertarget{
    use std::iter;
    use embedded_graphics::draw_target::DrawTarget;
    use embedded_graphics::geometry::Point;
    use embedded_graphics::Pixel;
    use crate::RenderTarget;

    impl<C: embedded_graphics::pixelcolor::PixelColor, E, T: DrawTarget<Color=C, Error=E>> RenderTarget<C, E> for T {
        fn put_pixel(&mut self, pos: (u32, u32), colour: C) -> Result<(), E> {
            self.draw_iter(iter::once(Pixel(Point::new(pos.0 as i32, pos.1 as i32), colour)))
        }

        fn size(&self) -> (u32, u32) {
            let dims = self.bounding_box().size;
            (dims.width, dims.height)
        }
    }
}
