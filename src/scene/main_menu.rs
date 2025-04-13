// use super::credits::Credits;
// use super::settings::Settings;
use super::{EScene, Scene};
// use crate::audio::play_sfx;
// use crate::consts::*;
use crate::context::Context;
// use crate::map::draw::draw_map;
// use crate::map::{Map, DEFAULT_CITY_ID};
use crate::ui::{
    menu::{Menu, MenuItem},
    // popup::Popup,
};
use life_io::life::LifeOptions;
use life_io::{life::Life, viewer::LifeViewer};
// use crate::ui::skin::{MENU_FONT_SIZE, MENU_MARGIN};
// use crate::input::{action_pressed, Action};
// use crate::text::{self, draw_text};
use macroquad::color::{BLACK, WHITE};
// use macroquad::math::vec2;
use macroquad::text::{draw_text, draw_text_ex, measure_text};
use macroquad::ui::hash;
use macroquad::window::{screen_height, screen_width};


#[derive(Clone)]
enum MenuOption {
    Start,
    Editor,
    // Freeplay,
    // Settings,
    // Credits,
    #[cfg(not(target_family = "wasm"))]
    Quit,
}

pub struct MainMenu {
    menu: Menu<MenuOption>,
    // settings_subscene: Settings,
    // credits_subscene: Credits,
    // popup: Option<Popup>,
    background_life: LifeViewer,
}

impl MainMenu {
    pub async fn new(_ctx: &mut Context) -> Self {
        let mut main_menu = Self {
            menu: Menu::new(vec![
                MenuItem::new(MenuOption::Start, "Start".to_string()),
                MenuItem::new(MenuOption::Editor, "Editor".to_string()),
                // MenuItem::new(MenuOption::Levels, "Levels".to_string()),
                // MenuItem::new(MenuOption::Freeplay, "Freeplay".to_string()),
                #[cfg(not(target_family = "wasm"))]
                MenuItem::new(MenuOption::Quit, "Quit".to_string()),
            ]),
            // popup: None,
            // settings_subscene: Settings::new(ctx, false),
            // credits_subscene: Credits::new(ctx),
            background_life: LifeViewer::new(Box::new(Life::new_ex(
                (256, 150).into(),
                LifeOptions {
                    algo: life_io::life::LifeAlgoSelect::Basic,
                    rule: life_io::life::LifeRule::STAR_WARS,
                },
            ))),
        };

        main_menu.background_life.edit_life(|life| {
            life.randomize(1234, true);
            for _ in 0..50 {
                life.update();
            }
        });

        main_menu
    }

    fn menu_option_selected(&mut self, menu_option: MenuOption, ctx: &mut Context) {
        match menu_option {
            // MenuOption::Continue => match super::GameOptions::Continue.create() {
            //     Ok(map) => ctx.switch_scene_to = Some(EScene::Gameplay(Box::new(map))),
            //     Err(_) => self.popup = Some(Popup::new("Error loading save".into())),
            // },
            MenuOption::Start => {
                ctx.switch_scene_to = Some(EScene::GameOptions);
            }
            MenuOption::Editor => {
                ctx.switch_scene_to = Some(EScene::Editor);
            }
            // MenuOption::Levels => {
            //     ctx.switch_scene_to = Some(EScene::LevelSelect);
            // }
            // MenuOption::Freeplay => {
            //     ctx.switch_scene_to = Some(EScene::Gameplay(Box::new(
            //         super::GameOptions::New
            //             .create()
            //             .expect("Error generating map"),
            //     )))
            // }
            // MenuOption::Settings => {
            //     self.settings_subscene.active = true;
            // }
            // MenuOption::Credits => {
            //     self.credits_subscene.active = true;
            // }
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit => {
                ctx.request_quit = true;
            }
        }
    }
}

// TODO: Pit 2 AI players agains each other instead of this "screensaver" approach
fn randomize_edges(life: &mut Life) {
    let size = life.size();
    for x in 0..size.x {
        life.insert(
            (x, 0).into(),
            life_io::life::Cell::new(
                if macroquad::rand::rand() < u32::MAX / 10 {
                    1
                } else {
                    0
                },
                1,
            ),
        );
        life.insert(
            (x, size.y - 1).into(),
            life_io::life::Cell::new(
                if macroquad::rand::rand() < u32::MAX / 10 {
                    1
                } else {
                    0
                },
                0,
            ),
        );
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        // if self.settings_subscene.active {
        //     self.settings_subscene.update(ctx);
        //     return;
        // }

        // if self.credits_subscene.active {
        //     self.credits_subscene.update(ctx);
        //     return;
        // }

        if self.background_life.update(&mut ctx.view_context) {
            // fill with random
            self.background_life.edit_life(randomize_edges);

        }
    }
    fn draw(&mut self, ctx: &mut Context) {
        // if self.settings_subscene.active {
        //     self.settings_subscene.draw(ctx);
        //     return;
        // }

        // if self.credits_subscene.active {
        //     self.credits_subscene.draw(ctx);
        //     return;
        // }

        // ctx.tileset.reset_camera(self.map.grid.size_px());

        // zoom in for a better look
        self.background_life.fit_to_screen();
        self.background_life
            .change_zoom(0.8, (screen_width() / 2., screen_height() / 2.).into());
        // view_ctx.set_pos((-screen_width() / 2., -screen_height() / 2.));

        self.background_life.draw();

        let menu_height = 200.;

        let font_size: u16 = 120;

        let measure = measure_text("Life Wars", Some(&ctx.font), font_size, 1.);

        let x = -measure.width / 2. + screen_width() / 2.;
        let y = -menu_height + screen_height() / 2.;

        let shadow_y = y + 5.;
        let shadow_x = x + 5.;

        draw_text_ex(
            "Life Wars",
            shadow_x,
            shadow_y,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex("Life Wars", x, y, macroquad::text::TextParams {
            font: Some(&ctx.font),
            font_size,
            color: WHITE,
            ..Default::default()
        });

        if let Some(selected) = self.menu.draw(hash!()).cloned() {
            self.menu_option_selected(selected, ctx);
        }

        draw_text(
            // ctx,
            // format!("v{}", VERSION).as_str(),
            format!("v{}", "0.0").as_str(),
            40.,
            screen_height() - 40.,
            // text::Size::Small,
            20.,
            WHITE,
        );

        // if let Some(popup) = &self.popup {
        //     match popup.draw() {
        //         None => {}
        //         Some(crate::ui::popup::PopupResult::Ok) => self.popup = None,
        //         Some(crate::ui::popup::PopupResult::Cancel) => self.popup = None,
        //     }
        // }
    }
}
