use life_io::life::Life;
use macroquad::color;

use crate::viewer::LifeViewer;

pub struct Editor {
    main_view: LifeViewer,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            main_view: LifeViewer::new(Box::new(Life::new(
                life_io::life::LifeAlgoSelect::Cached,
                (256, 256),
            ))),
        }
    }
}

const BORDER_SIZE: f32 = 40.;

impl super::Scene for Editor {
    fn update(&mut self, _ctx: &mut crate::context::Context) {
        self.main_view.update();
    }

    fn draw(&mut self, ctx: &mut crate::context::Context) {
        let size = self.main_view.life.size();
        self.main_view.resize_to_fit(
            size,
            (
                (macroquad::window::screen_width() - BORDER_SIZE * 2.),
                (macroquad::window::screen_height() - BORDER_SIZE * 2.),
            ),
        );
        self.main_view.set_pos((BORDER_SIZE, BORDER_SIZE));

        self.main_view.draw();

        macroquad::text::draw_text_ex(
            format!("Unnamed").as_str(),
            10.,
            20.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );

        let faction_text = format!(
            "Pop: {} Gen: {}",
            self.main_view.life.get_pop(0),
            self.main_view.life.get_generation()
        );

        let measure = macroquad::text::measure_text(faction_text.as_str(), Some(&ctx.font), 40, 1.);
        macroquad::text::draw_text_ex(
            faction_text.as_str(),
            macroquad::window::screen_width() - measure.width - 10.,
            macroquad::window::screen_height() - measure.height - 5.,
            macroquad::text::TextParams {
                font: Some(&ctx.font),
                font_size: 40,
                color: color::GREEN,
                ..Default::default()
            },
        );
    }
}
