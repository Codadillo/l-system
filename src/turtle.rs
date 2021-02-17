pub trait BasicTurtle<I, F, P> {
    type Error;

    fn heading(&self) -> &F;
    fn rotate(&mut self, radians: F);

    fn position(&self) -> &P;
    fn travel(&mut self, distance: I);

    fn push(&mut self);
    fn pop(&mut self);

    fn flush(&mut self) -> Result<(), Self::Error>;
}

pub trait FancyTurtle<I, F, P, C>: BasicTurtle<I, F, P> {
    fn color(&self) -> &C;
    fn set_color(&mut self, color: C);
    fn scale_color(&mut self, scalar: C);

    fn line_width(&self) -> &F;
    fn set_line_width(&mut self, width: F);
    fn scale_line_width(&mut self, scalar: F);
}

#[cfg(feature = "web-sys")]
pub use web::*;

#[cfg(feature = "web-sys")]
mod web {
    use super::*;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;

    pub struct CanvasContextTurtle {
        angle: f64,
        pos: (f64, f64),
        color: (f64, f64, f64),
        stack: Vec<(f64, (f64, f64), f64, (f64, f64, f64))>,
        line_width: f64,
        context: CanvasRenderingContext2d,
    }

    impl From<CanvasRenderingContext2d> for CanvasContextTurtle {
        fn from(context: CanvasRenderingContext2d) -> Self {
            Self::new(context, (0., 0.), 1., (0., 0., 0.))
        }
    }

    impl CanvasContextTurtle {
        pub fn new(
            context: CanvasRenderingContext2d,
            pos: (f64, f64),
            line_width: f64,
            color: (f64, f64, f64),
        ) -> Self {
            context.move_to(pos.0, pos.1);
            context.set_stroke_style(&JsValue::from_str(&format!(
                "rgb({}, {}, {})",
                color.0, color.1, color.2
            )));
            context.set_line_width(line_width);

            Self {
                pos,
                color,
                context,
                line_width,
                angle: 0.,
                stack: vec![],
            }
        }
    }

    impl BasicTurtle<f64, f64, (f64, f64)> for CanvasContextTurtle {
        type Error = ();

        fn heading(&self) -> &f64 {
            &self.angle
        }
        fn rotate(&mut self, radians: f64) {
            self.angle -= radians
        }

        fn position(&self) -> &(f64, f64) {
            &self.pos
        }
        fn travel(&mut self, distance: f64) {
            self.context.line_to(
                self.pos.0 + self.angle.cos() * distance,
                self.pos.1 + self.angle.sin() * distance,
            );
        }

        fn push(&mut self) {
            self.stack
                .push((self.angle, self.pos, self.context.line_width(), self.color));
        }
        fn pop(&mut self) {
            if let Some((angle, pos, line_width, color)) = self.stack.pop() {
                self.set_color(color);
                self.set_line_width(line_width);

                self.angle = angle;
                self.pos = pos;
            }
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(self.context.stroke())
        }
    }

    impl FancyTurtle<f64, f64, (f64, f64), (f64, f64, f64)> for CanvasContextTurtle {
        fn color(&self) -> &(f64, f64, f64) {
            &self.color
        }
        fn set_color(&mut self, color: (f64, f64, f64)) {
            self.context.stroke();
            self.context.set_stroke_style(&JsValue::from_str(&format!(
                "rgb({}, {}, {})",
                color.0, color.1, color.2
            )));
            self.color = color
        }
        fn scale_color(&mut self, (r, g, b): (f64, f64, f64)) {
            self.set_color((self.color.0 * r, self.color.1 * g, self.color.2 * b));
        }

        fn line_width(&self) -> &f64 {
            &self.line_width
        }
        fn set_line_width(&mut self, width: f64) {
            self.context.stroke();

            self.context.set_line_width(width);
            self.line_width = width;
        }
        fn scale_line_width(&mut self, scalar: f64) {
            self.set_line_width(self.line_width * scalar)
        }
    }
}
