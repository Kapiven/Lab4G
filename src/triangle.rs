use crate::{color::Color, framebuffer::Framebuffer};
use nalgebra::Vector3;

pub fn draw_triangle(frame: &mut Framebuffer, v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, color: Color) {
    // Algoritmo de triángulo con bounding box
    let min_x = v0.x.min(v1.x.min(v2.x)).max(0.0) as i32;
    let max_x = v0.x.max(v1.x.max(v2.x)).min(frame.width as f32 - 1.0) as i32;
    let min_y = v0.y.min(v1.y.min(v2.y)).max(0.0) as i32;
    let max_y = v0.y.max(v1.y.max(v2.y)).min(frame.height as f32 - 1.0) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            if inside_triangle(p, v0, v1, v2) {
                frame.draw_pixel(x, y, color);
            }
        }
    }
}

fn edge(a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

fn inside_triangle(p: Vector3<f32>, a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) -> bool {
    let w0 = edge(b, c, p);
    let w1 = edge(c, a, p);
    let w2 = edge(a, b, p);
    (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0)
}
