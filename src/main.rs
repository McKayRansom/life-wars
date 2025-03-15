use life_io::life::{Cell, Life, LifeAlgoSelect, LifeRule};

use macroquad::{
    color::{self, colors},
    input::{get_char_pressed, is_mouse_button_down, mouse_position},
    miniquad::window::clipboard_get,
    shapes::{draw_rectangle, draw_rectangle_lines},
    time::get_time,
    window::{self, clear_background, next_frame, screen_height, screen_width},
};

pub struct ViewContext {
    grid_size: f32,
    grid_pos: (f32, f32),
    request_quit: bool,
    paused: bool,
    speed: f64,
    selected_faction: u8,
}

fn draw_life(life: &Life, ctx: &ViewContext) {
    for (x, y, cell) in life.iter() {
        let state = cell.get_state();
        if state > 0 {
            let mut color = match cell.get_faction() {
                0 => color::GREEN,
                1 => color::RED,
                2 => color::YELLOW,
                3 => color::BLUE,
                _ => color::WHITE,
            };
            if state == 2 {
                color.a = 0.75;
            } else if state == 3 {
                color.a = 0.5;
            }
            draw_rectangle(
                ctx.grid_pos.0 + x as f32 * ctx.grid_size,
                ctx.grid_pos.1 + y as f32 * ctx.grid_size,
                ctx.grid_size,
                ctx.grid_size,
                color,
            );
        }
    }
    let size = life.size();
    draw_rectangle_lines(
        ctx.grid_pos.0,
        ctx.grid_pos.1,
        size.0 as f32 * ctx.grid_size,
        size.1 as f32 * ctx.grid_size,
        2.,
        colors::WHITE,
    );
}

fn handle_input(life: &mut Life, ctx: &mut ViewContext) {
    let mouse_pos = mouse_position();
    let pos: (usize, usize) = (
        ((mouse_pos.0 - ctx.grid_pos.0) / ctx.grid_size) as usize,
        ((mouse_pos.1 - ctx.grid_pos.1) / ctx.grid_size) as usize,
    );

    if let Some(chr) = get_char_pressed() {
        match chr {
            'q' => ctx.request_quit = true,
            ' ' => ctx.paused = !ctx.paused,
            '1' => ctx.selected_faction = 0,
            '2' => ctx.selected_faction = 1,
            '3' => ctx.selected_faction = 2,
            '4' => ctx.selected_faction = 3,
            'g' => life.paste(&Life::new_life_from_rle(life_io::life::GOSPER_RLE), pos),
            'p' => {
                if let Some(string) = clipboard_get() {
                    println!("Pasting {string:?}");
                    life.paste(&Life::new_life_from_rle(string.as_str()), pos)
                } else {
                    println!("No clipboard!");
                }
            }
            _ => {}
        }
    }

    if is_mouse_button_down(macroquad::input::MouseButton::Left) {
        life.insert(pos, Cell::new(1, ctx.selected_faction));
    }
}

const BORDER_SIZE: f32 = 40.;

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
    let seed = 23317;

    // println!("Life viewer. Seed: {seed}");

    let font = macroquad::text::load_ttf_font("resources/Micro5-Regular.ttf").await.unwrap();

    let mut life = Life::new_rule(LifeAlgoSelect::Basic, (128, 128), LifeRule::GOL);
    life.randomize(seed, false);

    let mut last_update = get_time();

    let mut ctx: ViewContext = ViewContext {
        grid_size: 0.,
        grid_pos: (0., 0.),
        request_quit: false,
        paused: false,
        speed: 1. / 10.,
        selected_faction: 0,
    };

    loop {
        if !ctx.paused && get_time() - last_update > ctx.speed {
            last_update = get_time();
            life.update();
        }

        let size = life.size();
        ctx.grid_size = ((screen_width() - BORDER_SIZE) / size.0 as f32)
            .min((screen_height() - BORDER_SIZE) / size.1 as f32);
        ctx.grid_pos = (BORDER_SIZE, BORDER_SIZE);
        handle_input(&mut life, &mut ctx);

        // DRAW
        clear_background(colors::BLACK);
        draw_life(&life, &ctx);

        macroquad::text::draw_text_ex("Life Viewer", 10., 30., macroquad::text::TextParams {
            font: Some(&font),
            font_size: 40,
            // font_scale: 1.,
            // font_scale_aspect: todo!(),
            // rotation: todo!(),
            color: color::GREEN,
            ..Default::default()
        });

        next_frame().await;

        if ctx.request_quit {
            break;
        }
    }
}
