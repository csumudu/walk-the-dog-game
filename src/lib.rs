use rand::thread_rng;
use rand::Rng;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Size {
    width: f64,
    height: f64,
}

impl Size {
    fn new(width: f64) -> Size {
        Self {
            width,
            height: width,
        }
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    /**
     * Draw Squre
     */
    let p = Point { x: 100.0, y: 100.0 };
    let size = &Size {
        width: 200.0,
        height: 100.0,
    };

    let color = get_hex_color();
    let contex2 = context.clone();
    draw_squre(&contex2, &p, &size, &color);

    /**
     * Draw Image
     */
    wasm_bindgen_futures::spawn_local(async move {
        let (sender, reciver) = futures::channel::oneshot::channel::<()>();

        let image = HtmlImageElement::new().unwrap();

        let callback = Closure::once(|| {
            console::log_1(&JsValue::from_str("Image Loaded to the Canvas!"));
            sender.send(());
        });

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        callback.forget();

        image.set_src("images/boy_idle_1.png");

        reciver.await;
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);
    });

    Ok(())
}

fn get_hex_color() -> String {
    let mut rng = thread_rng();
    let color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );

    format!("#{:02x}{:02x}{:02x}", color.0, color.1, color.2)
}

fn draw_squre(context: &CanvasRenderingContext2d, cordinate: &Point, size: &Size, color: &str) {
    context.move_to(cordinate.x, cordinate.y);
    context.begin_path();

    context.line_to(cordinate.x, cordinate.y + size.height);
    context.line_to(cordinate.x + size.width, cordinate.y + size.height);
    context.line_to(cordinate.x + size.width, cordinate.y);
    context.line_to(cordinate.x, cordinate.y);

    context.close_path();
    context.stroke();
    context.set_fill_style(&color.into());
    context.fill();
}
