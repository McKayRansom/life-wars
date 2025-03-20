use context::Context;

use macroquad::{
    color::colors, texture::set_default_filter_mode, window::{self, clear_background, next_frame}
};

pub mod context;
pub mod scene;
pub mod viewer;
pub mod skin;
pub mod pattern_view;

fn window_conf() -> window::Conf {
    window::Conf {
        fullscreen: false,
        // high-dpi seems to change the zoom on webassembly??
        high_dpi: true,
        // icon: Some(Icon {
        //     small: include_bytes!("../icons/16x16.rgba").to_owned(),
        //     medium: include_bytes!("../icons/32x32.rgba").to_owned(),
        //     big: include_bytes!("../icons/64x64.rgba").to_owned(),
        // }),
        // platform: miniquad::conf::Platform {
        //     linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
        //     ..Default::default()
        // },
        window_height: 720,
        window_resizable: true,
        window_title: String::from("Life-IO"),
        window_width: 1280,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    set_default_filter_mode(window::miniquad::FilterMode::Nearest);

    let mut ctx = Context {
        ..Context::new().await
    };

    let mut current_scene: Box<dyn scene::Scene> = // match map::levels::TEST_LEVEL {
        // Some(level) => 
        // Box::new(
        //     scene::gameplay::Gameplay::new(
        //         &mut ctx,
        //         Box::new(scene::GameOptions::Continue.create()),
        //     )
        //     .await
        // );
    // None => 
                    // Box::new(scene::editor::Editor::new(&ctx));
    Box::new(scene::main_menu::MainMenu::new(&mut ctx).await);
    // };

    loop {
        current_scene.update(&mut ctx);

        clear_background(colors::DARKGRAY);

        current_scene.draw(&mut ctx);

        if ctx.request_quit {
            break;
        }

        if let Some(escene) = ctx.switch_scene_to.take() {
            current_scene = match escene {
                scene::EScene::MainMenu => {
                    Box::new(scene::main_menu::MainMenu::new(&mut ctx).await)
                },
                scene::EScene::Gameplay(map) => {
                    Box::new(scene::gameplay::Gameplay::new(&mut ctx, map).await)
                },
                scene::EScene::Editor => {
                    Box::new(scene::editor::Editor::new(&ctx))
                },
                scene::EScene::GameOptions => {
                    Box::new(scene::game_options::GameOptions::new())
                },
            };
        }

        next_frame().await;
    }
}
