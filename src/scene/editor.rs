
use life_io::{
    life::{Life, LifeOptions, LifeRule},
    pattern::Pattern,
    viewer::LifeViewer,
};
use macroquad::{
    color,
    input::{self, mouse_position},
    math::{self, RectOffset},
    shapes::draw_rectangle,
    ui::{
        self, Skin, hash, root_ui,
        widgets::{self},
    },
    window::{self},
};

use crate::{context::Context, pattern_view::PatternLibViewer};

const SIDE_BAR_WIDTH: f32 = 250.;

pub struct Editor {
    main_view: LifeViewer,
    clipboard: Option<Life>,
    edit_select: EditBar,
    mouse_down_pos: Option<(u16, u16)>,
    pattern_name: String,
    skin: Skin,
    pattern_view: PatternLibViewer,
}

#[derive(PartialEq, Eq, Default, Debug)]
pub enum EditBar {
    #[default]
    Fill,
    Clear,
    Copy,
    Paste,
    Pattern,
}

impl Editor {
    pub fn new(ctx: &Context) -> Self {
        let mut skin = root_ui().default_skin();

        let window_color = color::Color::new(0., 0., 0., 0.7);

        let window_style = root_ui()
            .style_builder()
            .color_inactive(window_color)
            .color_hovered(window_color)
            .color_selected(window_color)
            .color_clicked(window_color)
            .color(window_color)
            // .font_size(120)
            // .text_color(WHITE)
            .background_margin(RectOffset::new(4., 4., 2., 2.))
            .margin(RectOffset::new(4., 4., 2., 2.))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(4., 4., 2., 2.))
            .with_font(&ctx.font)
            .unwrap()
            .margin(RectOffset::new(4., 4., 2., 2.))
            .color_inactive(color::WHITE)
            .color_hovered(color::LIGHTGRAY)
            .color_clicked(color::GREEN)
            // .color_clicked(Color::from_rgba(187, 187, 187, 255))
            // .color_hovered(Color::from_rgba(170, 170, 170, 235))
            // .text_color(Color::from_rgba(0, 0, 0, 255))
            // .text_color(Color::from_rgba(180, 180, 100, 255))
            .text_color(color::BLACK)
            .text_color_hovered(color::BLACK)
            .text_color_clicked(color::BLACK)
            .font_size(32)
            .build();

        skin.button_style = button_style;
        skin.window_style = window_style;

        Self {
            main_view: LifeViewer::new_fit_to_screen(Box::new(Life::new_ex(
                (256, 256),
                LifeOptions {
                    algo: life_io::life::LifeAlgoSelect::Cached,
                    rule: LifeRule::STAR_WARS, //from_str("B345/S4567").unwrap(),
                },
            ))),
            clipboard: None,
            edit_select: EditBar::Fill,
            mouse_down_pos: None,
            pattern_name: String::new(),
            skin,
            pattern_view: PatternLibViewer::new(),
        }
    }

    fn iter_area(min_pos: (u16, u16), max_pos: (u16, u16)) -> impl Iterator<Item = (u16, u16)> {
        (min_pos.1..max_pos.1).flat_map(move |y: u16| (min_pos.0..max_pos.0).map(move |x| (x, y)))
    }

    fn do_edit_action(&mut self, start_pos: (u16, u16), end_pos: (u16, u16)) {
        let min_pos = (start_pos.0.min(end_pos.0), start_pos.1.min(end_pos.1));
        let max_pos = (start_pos.0.max(end_pos.0), start_pos.1.max(end_pos.1));
        println!("Mouse down: {start_pos:?} to {end_pos:?}");
        match self.edit_select {
            EditBar::Fill => {
                for pos in Self::iter_area(min_pos, max_pos) {
                    self.main_view
                        .life
                        .insert(pos, life_io::life::Cell::new(1, 0))
                }
            }
            EditBar::Clear => {
                for pos in Self::iter_area(min_pos, max_pos) {
                    self.main_view
                        .life
                        .insert(pos, life_io::life::Cell::new(0, 0))
                }
            }
            EditBar::Copy => {
                if max_pos.0 == min_pos.0 || max_pos.1 == min_pos.1 {
                    return;
                }
                self.clipboard = Some(
                    self.main_view
                        .life
                        .copy(min_pos, (max_pos.0 - min_pos.0, max_pos.1 - min_pos.1)),
                );
            }
            EditBar::Paste => {
                if let Some(clipboard) = &self.clipboard {
                    self.main_view.life.paste(clipboard, start_pos, None);
                }
            }
            EditBar::Pattern => {
                if let Some(pattern) = &self.pattern_view.selected_pattern {
                    self.main_view.life.paste(&pattern.life, start_pos, None);
                }
            }
        }
    }

    fn handle_input(&mut self, _ctx: &mut Context) {
        if root_ui().is_mouse_over(input::mouse_position().into()) {
            return;
        }
        self.main_view.handle_input();

        let mouse_pos = input::mouse_position();
        if let Some(pos) = self.main_view.screen_to_life_pos(mouse_pos) {
            if input::is_mouse_button_pressed(input::MouseButton::Left) {
                self.mouse_down_pos = Some(pos);
            }
            if input::is_mouse_button_released(input::MouseButton::Left) {
                if let Some(mouse_down_pos) = self.mouse_down_pos {
                    let pos = (pos.0 + 1, pos.1 + 1);
                    self.do_edit_action(mouse_down_pos, pos);
                    self.mouse_down_pos = None;
                }
            }
        }
    }

    fn draw_clipboard(&mut self, ctx: &mut crate::context::Context) {
        widgets::Window::new(
            hash!(),
            math::vec2(
                window::screen_width() * 3. / 4.,
                window::screen_height() * 3. / 4.,
            ),
            math::vec2(
                window::screen_width() / 4.,
                window::screen_height() * 1. / 4.,
            ),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut ui::root_ui(), |ui| {
            if ui.button(None, "Save") {
                if let Some(clipboard) = &mut self.clipboard {
                    let mut pattern = Pattern::new_unclassified(clipboard.clone());
                    pattern.metadata.name = Some(self.pattern_name.clone());
                    ctx.pattern_lib.add_pattern(pattern);
                }
            }
            ui.input_text(hash!(), "Name", &mut self.pattern_name);
        });
    }

    fn draw_edit_bar(&mut self, _ctx: &crate::context::Context) {
        widgets::Window::new(
            hash!(),
            math::vec2(0., 0.),
            math::vec2(SIDE_BAR_WIDTH, window::screen_height()),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut ui::root_ui(), |ui| {
            // Group::new(hash!(), math::vec2(500., 100.))
            // .layout(ui::Layout::Horizontal)
            // .ui(ui, |ui| {
            if ui.button(None, "Fill") {
                self.edit_select = EditBar::Fill;
            }
            if ui.button(None, "Clear") {
                self.edit_select = EditBar::Clear;
            }
            if ui.button(None, "Copy") {
                self.edit_select = EditBar::Copy;
            }
            if ui.button(None, "Paste") {
                self.edit_select = EditBar::Paste;
            }
            // if ui.button(math::vec2(600., 0.), "Save") {
            //     self.edit_select = EditBar::Paste;
            // }
        });
    }

    fn draw_selected(&self) {
        if let Some(mouse_down_pos) = self.mouse_down_pos {
            let mouse_pos = mouse_position();
            if let Some(life_pos) = self.main_view.screen_to_life_pos(mouse_pos) {
                let life_pos = (life_pos.0 + 1, life_pos.1 + 1);
                let min_pos = (
                    life_pos.0.min(mouse_down_pos.0),
                    life_pos.1.min(mouse_down_pos.1),
                );
                let max_pos = (
                    life_pos.0.max(mouse_down_pos.0),
                    life_pos.1.max(mouse_down_pos.1),
                );

                let mouse_down_screen_pos = self.main_view.life_to_screen_pos(min_pos);
                draw_rectangle(
                    mouse_down_screen_pos.0,
                    mouse_down_screen_pos.1,
                    self.main_view.life_to_screen_scale(max_pos.0 - min_pos.0),
                    self.main_view.life_to_screen_scale(max_pos.1 - min_pos.1),
                    color::Color {
                        r: 1.,
                        g: 1.,
                        b: 1.,
                        a: 0.6,
                    },
                );
            }
        }
    }
}

impl super::Scene for Editor {
    fn update(&mut self, _ctx: &mut crate::context::Context) {
        self.main_view.update();
    }

    fn draw(&mut self, ctx: &mut crate::context::Context) {
        root_ui().push_skin(&self.skin);

        self.handle_input(ctx);

        self.main_view.draw();

        self.draw_selected();

        macroquad::text::draw_text_ex(
            "Editor",
            10. + SIDE_BAR_WIDTH,
            20.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );

        if self.pattern_view.draw(ctx, self.main_view.life.get_rule()) {
            self.edit_select = EditBar::Pattern;
        }
        self.draw_clipboard(ctx);
        self.draw_edit_bar(ctx);

        let faction_text = format!(
            "Pop: {} Gen: {}",
            self.main_view.life.get_pop(0),
            self.main_view.life.get_generation()
        );

        let measure = macroquad::text::measure_text(faction_text.as_str(), Some(&ctx.font), 40, 1.);
        macroquad::text::draw_text_ex(
            faction_text.as_str(),
            macroquad::window::screen_width() - measure.width - 10. - SIDE_BAR_WIDTH,
            macroquad::window::screen_height() - measure.height - 5.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );

        root_ui().pop_skin();
    }
}
