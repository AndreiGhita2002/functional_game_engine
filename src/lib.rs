use std::ops::Deref;
use std::time::{Duration, Instant};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::asset::AssetStore;
use crate::game::GameState;
use crate::render::{GPUState, Renderer};
use crate::render::sprite_render::SpriteRenderer;
use crate::util::res::Res;

pub mod game;
pub mod util;
pub mod render;
pub mod asset;

type SetupFn = fn(&mut GameState, Res<AssetStore>);

/// Contains lists of user defined actions
/// This is the main struct the user interacts with
pub struct Application {
    // user defined functions:
    /// setup function is run before the window is created
    setup_fn: fn(&mut GameState, Res<AssetStore>),
}

impl Application {
    pub fn new() -> Self {
        Application {
            setup_fn: |_g, _a| {},
        }
    }

    // pub fn with_setup(mut self, setup_fn: Box<SetupFn>) -> Self {
    pub fn with_setup(mut self, setup_fn: SetupFn) -> Self {
        self.setup_fn = setup_fn;
        self
    }

    pub fn run(self) {
        pollster::block_on(run(self));
    }
}

pub async fn run(app: Application) {
    // Window setup
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new()
        .with_title("hello world")
        .build(&event_loop).unwrap();

    let (g, surface) = GPUState::new(window).await;
    let gpu_state = Res::new(g);
    let mut game_state = GameState::new();
    // let asset_store = AssetStore::new(&gpu_state, saved_assets);
    let mut asset_store = AssetStore::new(gpu_state.clone());

    (app.setup_fn)(&mut game_state, asset_store.clone());

    let mut sprite_renderer = SpriteRenderer::new(gpu_state.clone(), asset_store.clone());
    // let mut model_renderer = ModelRenderer::new(gpu_state.clone(), asset_store.clone()); //todo implement

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

                    // updating the buffers
                    let gpu = gpu_state.read().unwrap();
                    asset_store.update_from_game(&game_state, gpu.deref());

                    sprite_renderer.pre_render(&game_state);
                    // model_renderer.pre_render(&game_state);
                    prev_time = now;
                }
                let gpu = gpu_state.read().unwrap();
                gpu.window().request_redraw();
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
                let mut gpu = gpu_state.read().unwrap();
                gpu.render(&sprite_renderer, &surface);
                // gpu_state.render(&model_renderer);
            },
            _ => ()
        };
    }).unwrap();
}
