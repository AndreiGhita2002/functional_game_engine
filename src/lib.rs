use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::game::GameState;
use crate::render::asset::{AssetsToLoad, AssetStore};
use crate::render::{GPUState, Renderer};
use crate::render::sprite::SpriteRenderer;

pub mod game;
pub mod util;
pub mod render;
mod resources;


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(mut game_state: GameState, to_load: AssetsToLoad) {
    // Window setup
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new()
        .with_title("hello world")
        .build(&event_loop).unwrap();

    let mut gpu_state = GPUState::new(window).await;

    let asset_store = AssetStore::new(&gpu_state, Some(to_load));
    let mut sprite_renderer = SpriteRenderer::new(&gpu_state, asset_store.clone());

    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Stopping...");
                window_target.exit();
            },
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                game_state.sim_tick();
                gpu_state.window().request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                sprite_renderer.pre_render(&gpu_state, &game_state);
                gpu_state.render(&sprite_renderer);
            },
            _ => ()
        };
    }).unwrap();
}
