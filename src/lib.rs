use std::time::{Duration, Instant};
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

    // time keeping:
    let sim_tick_duration: Duration = Duration::from_secs_f32(1.0 / 30.0);
    let mut prev_time = Instant::now();

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
                let now = Instant::now();
                let delta = now - prev_time;
                if delta >= sim_tick_duration {
                    game_state.sim_tick(delta);
                    // game_state.print_comps::<Transform2D>("pos");
                    // game_state.print_comps::<Sprite>("sprite");
                    sprite_renderer.pre_render(&gpu_state, &game_state);
                    prev_time = now;
                }
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
                gpu_state.render(&sprite_renderer);
            },
            _ => ()
        };
    }).unwrap();
}
