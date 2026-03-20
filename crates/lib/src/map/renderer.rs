use glam::IVec2;

use crate::{
    locations::{Location, TranslocatorSide},
    map::Map,
};

pub trait MapRenderer {
    fn prepare(&mut self, mix_x: i32, min_z: i32, width: i32, height: i32);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: &str);
    fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: &str);
    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: &str);
    fn finish(&mut self);
}

pub fn render<R: MapRenderer>(map: &Map, renderer: &mut R) {
    let coords: Vec<IVec2> = map
        .locations
        .values()
        .filter_map(|loc| loc.get_absolute())
        .collect();

    if coords.is_empty() {
        println!("No absolute coordinates found to render!");
        return;
    }

    let min_x = coords.iter().map(|v| v.x).min().unwrap() - 50;
    let max_x = coords.iter().map(|v| v.x).max().unwrap() + 50;
    let min_z = coords.iter().map(|v| -v.y).min().unwrap() - 50;
    let max_z = coords.iter().map(|v| -v.y).max().unwrap() + 50;
    let width = max_x - min_x;
    let height = max_z - min_z;

    renderer.prepare(min_x, min_z, width, height);

    // 3. Plot each location
    for (_id, location) in &map.locations {
        if let Some(pos) = location.get_absolute() {
            let (color_class, name) = match location {
                Location::Ruin(ruin) => (
                    "ruin",
                    ruin.clone().name.unwrap_or_else(|| "unnamed".to_string()),
                ),
                Location::Trader(trader) => ("trader", trader.name.clone()),
                Location::Translocator(translocator) => ("ruin", translocator.name.clone()),
                Location::Mine(mine) => ("ruin", mine.name.clone().unwrap()),
            };

            if let Location::Translocator(enter) = location {
                match enter.side {
                    TranslocatorSide::Enter => {
                        if let Some(Location::Translocator(exit)) =
                            map.locations.get(&enter.other_id)
                        {
                            let Some(p1) = enter.pos.absolute else {
                                continue;
                            };
                            let Some(p2) = exit.pos.absolute else {
                                continue;
                            };

                            renderer.draw_line(p1.x, -p1.y, p2.x, -p2.y, "blue");
                        }
                    }
                    _ => {}
                }
            }

            renderer.draw_circle(pos.x, -pos.y, 4, "red");

            renderer.draw_text(pos.x + 6, -pos.y + 4, &name, "black");
        }
    }

    renderer.finish();
}
