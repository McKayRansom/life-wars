use life_io::life::{Life, LifeRule};
use macroquad::{
    math::{self},
    ui::{
        self, hash,
        widgets::{self},
    },
    window,
};
pub struct PatternLibViewer {
    pub selected_pattern: Option<Life>,
}

const PATTEN_LIB_WIDTH: f32 = 250.;

impl PatternLibViewer {
    pub fn new() -> Self {
        Self {
            selected_pattern: None,
        }
    }

    pub fn draw(&mut self, ctx: &mut crate::context::Context, rule: &LifeRule) -> bool {
        let mut selected_pattern = false;

        widgets::Window::new(
            hash!(),
            math::vec2(window::screen_width() - PATTEN_LIB_WIDTH, 0.),
            math::vec2(
                PATTEN_LIB_WIDTH,
                window::screen_height() * 3. / 4.,
            ),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut ui::root_ui(), |ui| {
            ui.label(None, "Patterns");

            for pattern in ctx.pattern_lib.patterns.iter() {
                if rule != pattern.get_rule() {
                    continue;
                }
                if ui.button(None, pattern.get_name()) {
                    selected_pattern = true;
                    self.selected_pattern = Some(pattern.clone());
                }
            }
        });

        selected_pattern
    }
}
