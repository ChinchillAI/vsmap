use macroquad::prelude::*;

use vsmap_lib::map::Map;

#[macroquad::main("VSMap")]
async fn main() {
    let map: Map =
        toml::from_str(include_str!("../../../Map.toml")).expect("failed to load map");

    let mut target = Vec2::ZERO;
    let mut zoom = 1.0f32; 
    let mut last_mouse: Vec2 = mouse_position().into();
    let mut selected: &str;

    if let Some(Some(first_loc)) = map.locations.get("rabbit-lucky").map(|l| l.get_absolute()) {
        target = vec2(first_loc.x as f32, first_loc.z as f32);
    }

    loop {
        let mouse: Vec2 = mouse_position().into();
        let scroll_y = mouse_wheel().1;

        if scroll_y != 0.0 {
            zoom *= 1.1f32.powf(scroll_y.signum());
            zoom = zoom.clamp(0.0001, 100.0);
        }

        let view_w = screen_width() / zoom;
        let view_h = screen_height() / zoom;
        let camera = Camera2D::from_display_rect(Rect {
            x: target.x - (view_w / 2.0),
            y: target.y - (view_h / 2.0),
            w: view_w,
            h: view_h,
        });

        if is_mouse_button_down(MouseButton::Left) {
            let current_world = camera.screen_to_world(mouse);
            let last_world = camera.screen_to_world(last_mouse);
            target -= current_world - last_world;
        }
        last_mouse = mouse;

        clear_background(WHITE);
        set_camera(&camera);

        selected = "None";
        for (id, location) in &map.locations {
            if let Some(pos) = location.get_absolute() {
                let x = pos.x as f32;
                let y = pos.z as f32; // Z maps to Y (North is +Z, which goes UP)

                let screen_rect_size = 20.0;
                let screen_text_size = 16.0;
                let screen_padding = 5.0;

                let world_rect_size = screen_rect_size / zoom;
                let world_text_scale = screen_text_size / 64.0 / zoom;
                let world_padding = screen_padding / zoom;

                draw_rectangle(
                    x - world_rect_size / 2.0,
                    y - world_rect_size / 2.0,
                    world_rect_size,
                    world_rect_size,
                    GREEN,
                );

                draw_text_ex(
                    id,
                    x - world_rect_size / 2.0,
                    y + world_rect_size / 2.0 + world_padding, // +Y is UP, so we add padding to draw ABOVE the box
                    TextParams {
                        font_size: 64,
                        font_scale: -world_text_scale, // Flip vertically
                        font_scale_aspect: -1.0, // Correct horizontal flip caused by vertical flip
                        color: BLACK,
                        ..Default::default()
                    },
                );

                let current_world = camera.screen_to_world(mouse);
                if (current_world.x > x - world_rect_size / 2.0) && (current_world.x < x + world_rect_size / 2.0)
                    && (current_world.y > y - world_rect_size / 2.0) && (current_world.y < y + world_rect_size / 2.0) {
                    selected = id;
                }
            }
        }

        set_default_camera();
        draw_text(
            &format!(
                "Target: {:.0}, {:.0} | Zoom: {:.4} | Hovered: {}",
                target.x, target.y, zoom, selected
            ),
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        next_frame().await
    }
}
