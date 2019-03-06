extern crate sdl2;
extern crate stopwatch;

mod body;
mod system;
mod cam;
mod gui;
mod fuax_gfx;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;
use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use sdl2::rect::Point;
use stopwatch::Stopwatch;
use fuax_gfx::FauxGFX;

const GRAVITY_CONST: f32 = 0.0005;
const PI: f32 = 3.14159265;

fn main() {
    let mut cam = cam::Cam::new();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window_size = (1280, 720);

    let mut res_mult = 1.0;

    let window = video_subsystem.window(format!("Gravisim").as_str(), window_size.0, window_size.1)
        .position_centered()
        .allow_highdpi()
        .build()
        .unwrap();

    let mut show_hud = true;

    let draw_size = window.drawable_size();
    res_mult = draw_size.0 as f32 / window.size().0 as f32;

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut system = system::System::new();

    // Editor Speeds
    let density_speed = 0.1;

    // Editor
    let mut selected_size: f32 = 50.0;
    let mut selected_pos = (0f32, 0f32);
    let mut selected_vel = (0f32, 0f32);
    let mut selected_density = 1f32;
    let mut pos_selected = false;

    let mut mouse_x = 0f32;
    let mut mouse_y = 0f32;
    let mut raw_zoom = 0f32;

    let mut mouse_pressed = false;
    let mut pmouse_pressed = false;

    // GUI
    let ttf_ctx = sdl2::ttf::init().expect("Failed to init SDL_TTF");
    let font = gui::Text::new(&ttf_ctx, "./res/start.ttf", 12 * res_mult as u16, Color::RGB(255, 255, 255)).expect("Failed to create font");

    // FPS
    let mut fps_sw = Stopwatch::start_new();

    let mut total_time = 0f32;

    'running: loop {
        //FPS and Time Mult
        let mut elapsed_nanos = fps_sw.elapsed().subsec_nanos();
        if elapsed_nanos == 0 {
            elapsed_nanos = 1;
        }
        let fps = 1_000_000_000 / elapsed_nanos;
        let time_mult = (elapsed_nanos as f32) * 400.0 / 1_000_000_000f32;
        total_time += time_mult;
        fps_sw.restart();

        // Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    system.bodies = vec!();
                    cam.zoom = 1.0;
                    cam.x = 0.0;
                    cam.y = 0.0;
                    raw_zoom = 0.0;
                },
                Event::MouseWheel {y: y_pos, ..} => {
                    let delta_raw = 0.01 * time_mult * y_pos as f32;
                    raw_zoom += delta_raw;
                    let p_zoom = cam.zoom;
                    cam.zoom = 2f32.powf(raw_zoom);
                    let delta_zoom = cam.zoom - p_zoom;
                    let focus_point = cam.reverse_transform((mouse_x, mouse_y));
                    //cam.zoom += delta_zoom;
                    cam.x += delta_zoom * focus_point.0;
                    cam.y += delta_zoom * focus_point.1;
                },
                Event::KeyDown {keycode: Some(Keycode::H), ..} => {
                    show_hud = !show_hud;
                }
                _ => {}
            }
        }

        let key_state = KeyboardState::new(&event_pump);
        let mouse_state = MouseState::new(&event_pump);


        mouse_x = mouse_state.x() as f32 * res_mult;
        mouse_y = mouse_state.y() as f32 * res_mult;

        if !pos_selected {
            selected_pos = cam.reverse_transform((mouse_x, mouse_y));
        }

        pmouse_pressed = mouse_pressed;
        mouse_pressed = mouse_state.left();

        if mouse_pressed && !pmouse_pressed && !pos_selected {
            pos_selected = true;
            pmouse_pressed = true;
        }

        if pos_selected {
            if mouse_pressed && pmouse_pressed {
                let point1 = selected_pos;
                let point2 = cam.reverse_transform((mouse_x, mouse_y));
                selected_vel = ((point2.0 - point1.0) / 50.0, (point2.1 - point1.1) / 50.0);
            } else {
                pos_selected = false;
                system.add(selected_pos.0, selected_pos.1, selected_vel.0, selected_vel.1, selected_density, selected_size / cam.zoom);
            }
        }

        // Pan and zoom
        if key_state.is_scancode_pressed(Scancode::D) {
            cam.x += 1.0 / cam.zoom * time_mult;
        }
        if key_state.is_scancode_pressed(Scancode::A) {
            cam.x -= 1.0 / cam.zoom * time_mult;
        }
        if key_state.is_scancode_pressed(Scancode::W) {
            cam.y -= 1.0 / cam.zoom * time_mult;
        }
        if key_state.is_scancode_pressed(Scancode::S) {
            cam.y += 1.0 / cam.zoom * time_mult;
        }
        if key_state.is_scancode_pressed(Scancode::Z) {
            selected_size += 0.2 * time_mult;
            if selected_size < 1.0 {
                selected_size = 1.0;
            }
        }
        if key_state.is_scancode_pressed(Scancode::X) {
            selected_size -= 0.2 * time_mult;
            if selected_size < 1.0 {
                selected_size = 1.0;
            }
        }

        if key_state.is_scancode_pressed(Scancode::V) {
            selected_density += density_speed * time_mult;
            if selected_density < 1.0 {
                selected_density = 1.0;
            }
        }
        if key_state.is_scancode_pressed(Scancode::C) {
            selected_density -= density_speed * time_mult;
            if selected_density < 1.0 {
                selected_density = 1.0;
            }
        }

        system.update(&time_mult, &total_time);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let color_g = if selected_density > 255f32 {
            0 as u8
        } else {
            (255f32 - selected_density) as u8
        };

        let selected_transformed = cam.transform(selected_pos);
        
        canvas.filled_circle(Point::new(selected_transformed.0 as i32, selected_transformed.1 as i32), (selected_size) as i16, Color::RGBA(255, color_g, 255, 50)).expect("Failed to draw circle");
        
        if pos_selected {
            let point1 = selected_transformed;
            let point2 = (mouse_x, mouse_y);
            canvas.thick_line( Point::new(point1.0 as i32, point1.1 as i32), Point::new(point2.0 as i32, point2.1 as i32), 5, Color::RGBA(255, 255, 255, 50)).expect("Failed to draw cursor line");
        }
        
        system.render(&mut canvas, &cam);

        // Render Fonts
        if show_hud {
            font.draw_multiline(&mut canvas, format!("R: RESET\nH: TOGGLE HUD\nSCROLL: ZOOM\nZ/X: CHANGE SIZE\nC/V: CHANGE DENSITY").as_str(), 10 * res_mult as i32, 10 * res_mult as i32, false, 20 * res_mult as i32);
            font.draw(&mut canvas, format!("{} FPS", fps).as_str(), 10 * res_mult as i32, 10 * res_mult as i32, true);
        }
        canvas.present();
    }
}
