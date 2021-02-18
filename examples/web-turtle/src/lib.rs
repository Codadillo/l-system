use l_system::{
    turtle::{BasicTurtle, CanvasContextTurtle, FancyTurtle},
    LSystem, LSystemExecutor,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

const C_WIDTH: u32 = 800;
const C_HEIGHT: u32 = C_WIDTH * 3 / 4;

const TRAVEL: f64 = 10.;
const ROTATE: f64 = 0.5235987756;
const DCOLOR: (f64, f64, f64) = (81., 0., 0.);
const ICOLOR: (f64, f64, f64) = (77., 38., 91.);

#[wasm_bindgen(start)]
pub fn main() {
    let document = window().unwrap().document().unwrap();

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();
    canvas.set_width(C_WIDTH);
    canvas.set_height(C_HEIGHT);

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let mut system = LSystem::new("J.".into());
    system.register_rule("J".into(), || "JJ+[+H-H-H]-[-H+H+H]".into());
    system.register_rule("H".into(), || "JJ+[+H-H-H]-[-H+H+H]".into());

    let turtle = CanvasContextTurtle::new(
        context,
        -1.5707963268,
        (canvas.width() as f64 / 2., canvas.height() as f64),
        1.,
        (0., 0., 0.),
    );

    let mut executor = LSystemExecutor::new(turtle);
    executor.register_rule("J".into(), |state: &mut CanvasContextTurtle| {
        state.set_color(DCOLOR);
        state.travel(TRAVEL);
    });
    executor.register_rule("H".into(), |state: &mut CanvasContextTurtle| {
        state.increment_color(ICOLOR);
        state.travel(TRAVEL);
    });
    executor.register_rule("+".into(), |state: &mut CanvasContextTurtle| {
        state.rotate(ROTATE)
    });
    executor.register_rule("-".into(), |state: &mut CanvasContextTurtle| {
        state.rotate(-ROTATE)
    });
    executor.register_rule("[".into(), |state: &mut CanvasContextTurtle| state.push());
    executor.register_rule("]".into(), |state: &mut CanvasContextTurtle| state.pop());
    executor.register_rule(".".into(), |state: &mut CanvasContextTurtle| {
        state.flush().unwrap()
    });

    system.step_by(4);
    unsafe {
        console::debug_1(&JsValue::from_str(&system.axiom));
    }
    executor.execute(&system).unwrap();
}
