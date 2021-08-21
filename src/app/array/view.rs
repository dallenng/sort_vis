use nannou::color::Rgba;
use nannou::geom::Rect;
use nannou::Draw;

use crate::app::array::Array;

impl Array {
    pub fn view(&self, draw: &Draw, window_rect: Rect<f32>) {
        let w = window_rect.w() / self.len;
        let h_factor = window_rect.h() / self.len;
        let mut x = 0.0;
        for (&n, &access) in self.inner.iter().zip(&self.access) {
            let n = f32::from(n);

            let rect = Rect::from_w_h(w, h_factor * n)
                .bottom_left_of(window_rect)
                .shift_x(x);
            draw.rect().xy(rect.xy()).wh(rect.wh()).color(Rgba::new(
                (n / self.len).clamp(0.2, 1.0),
                0.2,
                0.9,
                1.0,
            ));
            draw.rect()
                .xy(rect.xy())
                .wh(rect.wh())
                .color(Rgba::new(access, access, access, 0.5));

            x += w;
        }
    }
}
