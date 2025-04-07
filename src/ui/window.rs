use life_io::viewer::ViewContext;
use macroquad::math::{Vec2, vec2};

#[derive(Default)]
pub enum HorizontalJusitfy {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Default)]
pub enum VerticalJustify {
    Top,
    #[default]
    Center,
    Bottom,
}

#[derive(Default)]
pub enum HorizontalAlign {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Default)]
pub enum VerticalAlign {
    Top,
    #[default]
    Center,
    Bottom,
}

#[derive(Default)]
pub struct Alignment {
    hor_justify: HorizontalJusitfy,
    hor_align: HorizontalAlign,
    vert_justify: VerticalJustify,
    vert_align: VerticalAlign,
}

pub trait UiContent {
    fn size(&self) -> Vec2;
    fn draw(&self, pos: Vec2);
}

#[derive(Default)]
pub struct Window {
    align: Alignment,
    content: Vec<Box<dyn UiContent>>,
    margin: Vec2,
}

impl Window {
    pub fn align(self, align: Alignment) -> Self {
        Self { align, ..self }
    }

    pub fn margin(self, margin: Vec2) -> Self {
        Self {margin, ..self }
    }

    pub fn draw(self, ctx: &mut ViewContext) {
        let size: Vec2 = self.content.iter().map(|content| content.size()).sum();
        let mut pos = vec2(
            match self.align.hor_justify {
                HorizontalJusitfy::Left => 0.,
                HorizontalJusitfy::Center => ctx.screen_size.x / 2. - size.x / 2.,
                HorizontalJusitfy::Right => ctx.screen_size.x - size.x,
            },
            match self.align.vert_justify {
                VerticalJustify::Top => 0.,
                VerticalJustify::Center => ctx.screen_size.y / 2. - size.y / 2.,
                VerticalJustify::Bottom => ctx.screen_size.y - size.y,
            },
        );

        for content in &self.content {
            let size = content.size();
            content.draw(pos + self.margin);
            pos += size;
        }
    }
}
