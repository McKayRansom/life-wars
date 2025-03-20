// use super::credits::Credits;
// use super::settings::Settings;
use super::{EScene, Scene};
// use crate::audio::play_sfx;
// use crate::consts::*;
use crate::context::Context;
use crate::viewer::LifeViewer;
// use crate::map::draw::draw_map;
// use crate::map::{Map, DEFAULT_CITY_ID};
use crate::scene::{
    menu::{Menu, MenuItem},
    // popup::Popup,
};
use life_io::life::Life;
// use crate::ui::skin::{MENU_FONT_SIZE, MENU_MARGIN};
// use crate::input::{action_pressed, Action};
// use crate::text::{self, draw_text};
use macroquad::color::{BLACK, WHITE};
// use macroquad::math::vec2;
use macroquad::text::{draw_text, draw_text_ex, measure_text};
use macroquad::ui::hash;
use macroquad::window::{screen_height, screen_width};

const _MAIN_MENU_MAP: &str = "\
#N 52514m.rle
#C https://conwaylife.com/wiki/52513M
#C https://www.conwaylife.com/patterns/52514m.rle
x = 16, y = 16, rule = B3/S23
bob2o2b5ob2o$2o3b3o2bo3b2o$2bobo7bo2bo$5bo2bob3o2bo$3o3bo2b2o3b2o$4b3o
bo3b3o$4bo2b3o5bo$3bob2obo5bo$o2b3o4b2ob3o$obobo2bo2bo$o6bo2b2o$b2obo
2bo4bo2bo$bo2b2o4bobob2o$2bo5b2o3bo$ob2ob3o4bo2bo$2o!
";

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
            background_life: LifeViewer::new(Box::new(Life::new_rule(
                life_io::life::LifeAlgoSelect::Basic,
                (512, 300),
                life_io::life::LifeRule::STAR_WARS,
            ))),
        };

        // main_menu.background_life.life.randomize(1234, true);
        //     .paste(&Life::new_rule(life_io::life::LifeAlgoSelect::Basic, (1)), (64 - 8, 64 - 8));

        // main_menu.map.get_city_mut(DEFAULT_CITY_ID).unwrap().name = "Alpha 0.1X - Roads".into();

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

impl Scene for MainMenu {
    fn update(&mut self, _ctx: &mut Context) {
        // if self.settings_subscene.active {
        //     self.settings_subscene.update(ctx);
        //     return;
        // }

        // if self.credits_subscene.active {
        //     self.credits_subscene.update(ctx);
        //     return;
        // }

        if self.background_life.update() {
            let size = self.background_life.life.size();
            for x in 0..size.0 {
                self.background_life.life.insert(
                    (x, 0),
                    life_io::life::Cell::new(
                        if macroquad::rand::rand() < u32::MAX / 20 {
                            1
                        } else {
                            0
                        },
                        1,
                    ),
                );
                self.background_life.life.insert(
                    (x, size.1 - 1),
                    life_io::life::Cell::new(
                        if macroquad::rand::rand() < u32::MAX / 20 {
                            1
                        } else {
                            0
                        },
                        0,
                    ),
                );
            }
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
        self.background_life.resize_to_fit(
            self.background_life.life.size(),
            (screen_width(), screen_height()),
        );
        self.background_life
            .change_zoom(0.8, (screen_width() / 2., screen_height() / 2.));
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
