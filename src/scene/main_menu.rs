// use super::credits::Credits;
// use super::settings::Settings;
use super::{EScene, Scene};
// use crate::audio::play_sfx;
// use crate::consts::*;
use crate::context::Context;
use crate::draw::{draw_life, ViewContext};
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
use macroquad::time;
// use macroquad::time::get_time;
use macroquad::ui::hash;
// use macroquad::ui::{hash, root_ui, widgets};
use macroquad::window::{screen_height, screen_width};

const MAIN_MENU_MAP: &str = "\
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
    // Continue,
    Start,
    // Levels,
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

    last_update: f64,
    life: Life,
}

impl MainMenu {
    pub async fn new(_ctx: &mut Context) -> Self {
        let mut main_menu = Self {
            menu: Menu::new(vec![
                // if Map::save_exists() {
                    // MenuItem::new(MenuOption::Continue, "Continue".to_string())
                // } else {
                    MenuItem::new(MenuOption::Start, "Start".to_string()),
                // },
                // MenuItem::new(MenuOption::Levels, "Levels".to_string()),
                // MenuItem::new(MenuOption::Freeplay, "Freeplay".to_string()),
                #[cfg(not(target_family = "wasm"))]
                MenuItem::new(MenuOption::Quit, "Quit".to_string()),
            ]),
            // popup: None,
            // settings_subscene: Settings::new(ctx, false),
            // credits_subscene: Credits::new(ctx),
            life: Life::new(life_io::life::LifeAlgoSelect::Cached, (128, 128)),
            last_update: 0.,
        };

        main_menu.life.paste(&Life::new_life_from_rle(MAIN_MENU_MAP), (64 - 8, 64 - 8));

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
                ctx.switch_scene_to = Some(EScene::Gameplay(Box::new(
                    super::GameOptions::New.create()
                        // .create()
                        // .expect("Error loading level"),
                )))
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
            },
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
        // self.map.metadata.grow_cities = false;

        let speed = 1./8.;
        if time::get_time() - self.last_update > speed {
            self.last_update = time::get_time();
            self.life.update();
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
        let mut view_ctx = ViewContext::new();
        view_ctx.resize_to_fit(self.life.size(), (screen_width(), screen_width()));
        // view_ctx.set_pos((-screen_width() / 2., -screen_height() / 2.));
        
        draw_life(&self.life, &view_ctx);

        let menu_height = 200.;

        let font_size: u16 = 120;

        let measure = measure_text("Life IO", Some(&ctx.font), font_size, 1.);

        let x = -measure.width / 2. + screen_width() / 2.;
        let y = -menu_height + screen_height() / 2.;

        let shadow_y = y + 5.;
        let shadow_x = x + 5.;

        draw_text_ex(
            "Life IO",
            shadow_x,
            shadow_y,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            "Life IO",
            x,
            y,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );

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
