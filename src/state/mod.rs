use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::nalgebra;
use ggez::Context;
use ggez::graphics;
use rand;
use rand::distributions::{Uniform, Distribution};
use rand::thread_rng;

pub struct IterableSortVec {
    vec: Vec<u32>,
    rand: Uniform<usize>,
}

impl IterableSortVec {
    pub fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);

        for i in 0..(size as u32) {
            vec.push(i);
        }

        let rand = Uniform::new(0, vec.len());

        Self { vec, rand }
    }
}

impl ggez::event::EventHandler for IterableSortVec {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        self.vec.swap(self.rand.sample(&mut thread_rng()), self.rand.sample(&mut thread_rng()));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, (0, 0, 0).into());

        let (width, height) = graphics::drawable_size(ctx);
        let rect_width = width / self.vec.len() as f32;

        for (i, val) in self.vec.iter().enumerate() {
            let rect_height = (*val) as f32 * height / self.vec.len() as f32;
            let rect = Rect::new(
                i as f32 * rect_width,
                height - rect_height,
                rect_width,
                rect_height,
            );
            let drawable = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, (255, 0, 255).into())?;

            graphics::draw(ctx, &drawable, (nalgebra::Point2::new(0.0, 0.0), ))?;
        }

        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
    }
}
