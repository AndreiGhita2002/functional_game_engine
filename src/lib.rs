use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::game::GameState;
use crate::render::asset::AssetStore;
use crate::render::GPUState;

pub mod game;
pub mod util;
pub mod render;
mod resources;


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(mut game_state: GameState) {
    // Window setup
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new()
        .with_title("hello world")
        .build(&event_loop).unwrap();

    let mut gpu_state = GPUState::new(window).await;

    let asset_store = AssetStore::new(&gpu_state.device);

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
                gpu_state.render(&game_state, asset_store.clone());
            },
            _ => ()
        };
    }).unwrap();
}
