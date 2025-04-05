use macroquad::{
    color::{self, Color}, math::RectOffset, text::Font, texture::set_default_filter_mode, ui::{root_ui, Skin}
};

pub const MENU_FONT_SIZE: u16 = 48;
// pub const MENU_MARGIN: f32 = 16.;
// pub const MENU_OUTER_MARGIN: f32 = 16.;

pub const MENU_MARGIN: f32 = 8.;
pub const MENU_OUTER_MARGIN: f32 = 4.;

pub const BUTTON_INNER_MARGIN: (f32, f32) = (16., 2.);
pub const BUTTON_OUTER_MARGIN: (f32, f32) = (16., 8.);
pub const BUTTON_MARGIN: (f32, f32) = (
    BUTTON_INNER_MARGIN.0 + BUTTON_OUTER_MARGIN.0,
    BUTTON_INNER_MARGIN.1 + BUTTON_OUTER_MARGIN.1,
);

pub async fn init() -> Font {
    let mut font = macroquad::text::load_ttf_font("resources/Micro5-Regular.ttf")
        .await
        .unwrap();

    font.set_filter(macroquad::texture::FilterMode::Nearest);

    set_default_filter_mode(macroquad::texture::FilterMode::Nearest);

    let skin2 = {
        let label_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(color::WHITE)
            .font_size(MENU_FONT_SIZE)
            .build();

        // let window_color = Color::from_hex(0x585858);
        // let window_color = Color::new(0., 0., 0., 0.);
        // let window_color = Color::new(0., 0.,0., 0.3);
        let window_color = Color::new(0., 0., 0., 0.7);

        let window_style = root_ui()
            .style_builder()
            .color_inactive(window_color)
            .color_hovered(window_color)
            .color_selected(window_color)
            .color_clicked(window_color)
            .color(window_color)
            // .font_size(120)
            // .text_color(WHITE)
            .background_margin(RectOffset::new(
                MENU_OUTER_MARGIN,
                MENU_OUTER_MARGIN,
                MENU_OUTER_MARGIN,
                MENU_OUTER_MARGIN,
            ))
            .margin(RectOffset::new(
                MENU_MARGIN,
                MENU_MARGIN,
                MENU_MARGIN,
                MENU_MARGIN,
            ))
            .build();

        let button_style = root_ui()
            .style_builder()
            // .background(
            //     Image::from_file_with_format(
            //         include_bytes!("../../resources/button_background_2.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            .background_margin(RectOffset::new(
                BUTTON_OUTER_MARGIN.0,
                BUTTON_OUTER_MARGIN.0,
                BUTTON_OUTER_MARGIN.1,
                BUTTON_OUTER_MARGIN.1,
            ))
            // .background_hovered(
            //     Image::from_file_with_format(
            //         include_bytes!("../../resources/button_hovered_background_2.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            // .background_clicked(
            //     Image::from_file_with_format(
            //         include_bytes!("../../resources/button_clicked_background_2.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            .with_font(&font)
            .unwrap()
            .margin(RectOffset::new(
                BUTTON_INNER_MARGIN.0,
                BUTTON_INNER_MARGIN.0,
                BUTTON_INNER_MARGIN.1,
                BUTTON_INNER_MARGIN.1,
            ))
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
            .font_size(MENU_FONT_SIZE)
            .build();

        // let checkbox_style = root_ui()
        //     .style_builder()
        //     .background(
        //         Image::from_file_with_format(
        //             include_bytes!("../examples/ui_assets/checkbox_background.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     .background_hovered(
        //         Image::from_file_with_format(
        //             include_bytes!("../examples/ui_assets/checkbox_hovered_background.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     .background_clicked(
        //         Image::from_file_with_format(
        //             include_bytes!("../examples/ui_assets/checkbox_clicked_background.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     .build();

        let editbox_style = root_ui()
            .style_builder()
            // .background(
            //     Image::from_file_with_format(
            //         include_bytes!("../examples/ui_assets/editbox_background.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            // .background_margin(RectOffset::new(2., 2., 2., 2.))
            .with_font(&font)
            .unwrap()
            .text_color(color::WHITE) // THIS ONE ACTUALLY MATTERS??
            .text_color_clicked(color::YELLOW)
            .text_color_hovered(color::ORANGE)
            .color_selected(color::PINK)
            .color(color::BLACK)
            .color_hovered(color::BLACK)
            .color_selected_hovered(color::BLACK)
            .color_clicked(color::BLANK)
            .color_selected(color::BLACK)
            .color_inactive(color::BLACK)
            .margin(RectOffset::new(0., 0., 0., 0.))
            // .
            .font_size(50)
            .build();

        // let combobox_style = root_ui()
        //     .style_builder()
        //     .background(
        //         Image::from_file_with_format(
        //             include_bytes!("../examples/ui_assets/combobox_background.png"),
        //             None,
        //         )
        //         .unwrap(),
        //     )
        //     .background_margin(RectOffset::new(4., 25., 6., 6.))
        //     .with_font(&font)
        //     .unwrap()
        //     .text_color(Color::from_rgba(120, 120, 120, 255))
        //     .color(Color::from_rgba(210, 210, 210, 255))
        //     .font_size(25)
        //     .build();

        let margin = 16.;

        Skin {
            window_style,
            button_style,
            label_style,
            // checkbox_style,
            editbox_style,
            // combobox_style,
            margin,

            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin2);

    font
}
