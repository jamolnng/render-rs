#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use anyhow::Result;
use winit::event_loop::EventLoop;

mod app;
mod camera;
mod shader;
mod state;
mod texture;
mod render;
mod utils;

pub fn run() -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Warn)
            .expect("Could not initialize console logger");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    let event_loop = EventLoop::<state::UserEvent>::with_user_event().build()?;
    let mut app = app::App::new(&event_loop);

    event_loop.run_app(&mut app)?;
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn wasm_main() {
    run().unwrap();
}
