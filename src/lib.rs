mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}
#[wasm_bindgen]
pub struct Text {}
#[wasm_bindgen]
pub struct Chika {}

#[wasm_bindgen]
impl Chika {
    pub fn find_canvas(id: &str) -> HtmlCanvasElement {
        let window = web_sys::window().expect("Could not get window");
        let document = window.document().expect("Could not get document");
        let canvas = document
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        return canvas;
    }
    pub fn find_context(id: &str) -> CanvasRenderingContext2d {
        let canvas;
        if id.is_empty() {
            canvas = Self::find_canvas("chika");
        } else {
            canvas = Self::find_canvas(id);
        }
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        return context;
    }
    pub fn clean(id: &str) {
        let canvas = Self::find_canvas(id);
        let context = Self::find_context(id);
        context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into())
    }
    pub fn log(message: &str) {
        log(&format!("ChikaMessage: {}", message));
    }
    pub fn error(message: &str) {
        set_panic_hook();
        error(&format!("ChikaErrorMessage: {}", message));
    }
    pub fn alert(message: &str) {
        alert(&format!("{}", message));
    }
}

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str, text: &str, x: u32, y: u32, font: &str) {
        let canvas = Chika::find_context(id);
        if !font.is_empty() {
            canvas.set_font(font);
        }
        canvas.fill_text(text, x.into(), y.into()).unwrap();
    }
}
