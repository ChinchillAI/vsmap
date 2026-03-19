use macroquad::prelude::*;

use vsmap_lib::{locations::{Location, Translocator}, map::Map, measurements::Vector};

struct MarkerStyle {
    rect_size: f32,
    padding: f32,
    text_size: f32,
    color: Color,
}

struct Viewer {
    map: Map,

    camera: Camera2D,
    target: Vec2,
    zoom: f32,
    last_mouse: Vec2,

    hovered: Option<(String, Location)>,
    selected: Option<(String, Location)>,
}

impl Default for Viewer {
    fn default() -> Self {
        let map: Map =
            toml::from_str(include_str!("../../../Map.toml")).expect("failed to load map");
        let target = if let Some(Some(start_location)) =
            map.locations.get("rabbit-lucky").map(|l| l.get_absolute())
        {
            vec2(start_location.x as f32, start_location.z as f32)
        } else {
            Vec2::ZERO
        };
        let zoom = 1.0f32;

        Self {
            map,
            camera: Self::camera_from_target(target, screen_width() / zoom, screen_height() / zoom),
            target,
            zoom,
            last_mouse: mouse_position().into(),
            hovered: None,
            selected: None,
        }
    }
}

impl Viewer {
    fn camera_from_target(target: Vec2, w: f32, h: f32) -> Camera2D {
        Camera2D::from_display_rect(Rect {
            x: target.x - (w / 2.0),
            y: target.y - (h / 2.0),
            w: w,
            h: h,
        })
    }

    fn handle_input(&mut self) {
        // Handle zoom
        let scroll_y = mouse_wheel().1;
        if scroll_y != 0.0 {
            self.zoom *= 1.1f32.powf(scroll_y.signum());
            self.zoom = self.zoom.clamp(0.0001, 100.0);
        }

        // Handle pan
        let mouse: Vec2 = mouse_position().into();
        if is_mouse_button_down(MouseButton::Left) {
            let current_world = self.camera.screen_to_world(mouse);
            let last_world = self.camera.screen_to_world(self.last_mouse);
            self.target -= current_world - last_world;
        }
        self.last_mouse = mouse;

        // Handle selection
        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected = if let Some(s) = &self.hovered {
                Some(s.clone())
            } else {
                None
            };
        }
    }

    fn draw_standard_marker(&self, id: &str, pos: Vector, style: MarkerStyle) {
        let x = pos.x as f32;
        let y = pos.z as f32;

        let world_rect_size = style.rect_size / self.zoom;
        let world_text_scale = style.text_size / 64.0 / self.zoom;
        let world_padding = style.padding / self.zoom;

        draw_rectangle(
            x - world_rect_size / 2.0,
            y - world_rect_size / 2.0,
            world_rect_size,
            world_rect_size,
            style.color,
        );

        draw_text_ex(
            id,
            x - world_rect_size / 2.0,
            y + world_rect_size / 2.0 + world_padding,
            TextParams {
                font_size: 64,
                font_scale: -world_text_scale, // Flip vertically
                font_scale_aspect: -1.0,       // Correct horizontal flip caused by vertical flip
                color: BLACK,
                ..Default::default()
            },
        );
    }

    fn draw_frame(&mut self) {
        clear_background(WHITE);

        self.camera = Self::camera_from_target(
            self.target,
            screen_width() / self.zoom,
            screen_height() / self.zoom,
        );
        set_camera(&self.camera);

        self.hovered = None;
        for (id, location) in &self.map.locations {
            if let Some(pos) = location.get_absolute() {
                let x = pos.x as f32;
                let y = pos.z as f32;
                match location {
                    Location::Ruin(r) => {
                        self.draw_standard_marker(
                            id,
                            pos,
                            MarkerStyle {
                                rect_size: 20.0,
                                padding: 5.0,
                                text_size: 16.0,
                                color: RED,
                            },
                        );
                    }
                    Location::Translocator(
                        Translocator { 
                            side: vsmap_lib::locations::TranslocatorSide::Enter,
                            name: _,
                            pos: _,
                            other_id
                        }
                    ) => {
                        let other = self.map.locations.get(other_id).unwrap();
                        let other_pos = other.get_absolute().unwrap();

                        draw_line(x, y, other_pos.x as f32, other_pos.z as f32, 5.0 / self.zoom, PURPLE);

                        self.draw_standard_marker(
                            "",
                            pos,
                            MarkerStyle {
                                rect_size: 20.0,
                                padding: 5.0,
                                text_size: 16.0,
                                color: PURPLE,
                            },
                        );

                        self.draw_standard_marker("", other_pos, MarkerStyle {
                                rect_size: 20.0,
                                padding: 5.0,
                                text_size: 16.0,
                                color: PURPLE,
                        });

                    }
                    Location::Translocator(
                        Translocator { 
                            side: vsmap_lib::locations::TranslocatorSide::Exit,
                            name: _,
                            pos: _,
                            other_id: _
                        }
                    ) => {

                    }
                    _ => {
                        self.draw_standard_marker(
                            id,
                            pos,
                            MarkerStyle {
                                rect_size: 20.0,
                                padding: 5.0,
                                text_size: 16.0,
                                color: BLUE,
                            },
                        );
                    }
                }

                // Updated hovered
                let world_rect_size = 20.0 / self.zoom;
                let current_world = self.camera.screen_to_world(self.last_mouse);
                if (current_world.x > x - world_rect_size / 2.0)
                    && (current_world.x < x + world_rect_size / 2.0)
                    && (current_world.y > y - world_rect_size / 2.0)
                    && (current_world.y < y + world_rect_size / 2.0)
                {
                    self.hovered = Some((id.clone(), location.clone()));
                }
            }
        }

        set_default_camera();
        draw_text(
            &format!(
                "Target: {:.0}, {:.0} | Zoom: {:.4}\nSelected: {:?} | Hovered: {:?}",
                self.target.x, self.target.y, self.zoom, self.selected, self.hovered
            ),
            10.0,
            20.0,
            20.0,
            BLACK,
        );
    }
}

#[macroquad::main("VSMap")]
async fn main() {
    let mut viewer = Viewer::default();

    loop {
        viewer.handle_input();
        viewer.draw_frame();
        next_frame().await
    }
}
