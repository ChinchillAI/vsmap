use std::{fs::File, io::Write, path::PathBuf};

use vsmap_lib::map::MapRenderer;

pub struct SvgRenderer<W>
where
    W: Write,
{
    pub writer: W,
}

impl<W: Write> MapRenderer for SvgRenderer<W> {
    fn prepare(&mut self, min_x: i32, min_z: i32, width: i32, height: i32) {
        // 2. Write SVG Header
        writeln!(
            self.writer,
            r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            min_x, min_z, width, height
        )
        .expect("write failed");

        // Add CSS for beautiful, readable labels
        writeln!(
            self.writer,
            r#"  <style>
        .label {{ font: bold 12px sans-serif; fill: black; paint-order: stroke; stroke: white; stroke-width: 2.5px; stroke-linecap: round; stroke-linejoin: round; }}
        .point {{ fill: #ff4444; stroke: #333; stroke-width: 1px; }}
        .ruin {{ fill: #777777; }}
        .trader {{ fill: #44aa44; }}
      </style>"#
        ).expect("write failed");
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: &str) {
        writeln!(
            self.writer,
            r#" <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="blue" />"#,
            x1, y1, x2, y2
        )
        .expect("write failed");
    }

    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: &str) {
        // Draw the label (offset slightly to the bottom-right)
        writeln!(
            self.writer,
            r#"  <text class="label" x="{}" y="{}">{}</text>"#,
            x + 6,
            y + 4,
            text
        )
        .expect("write failed");
    }

    fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: &str) {
        // Draw the point
        writeln!(
            self.writer,
            r#"  <circle class="point {}" cx="{}" cy="{}" r="{}" />"#,
            color, x, y, radius
        )
        .expect("write failed");
    }

    fn finish(&mut self) {
        writeln!(self.writer, "</svg>").expect("write failed");
    }
}
