mod color;
mod framebuffer;
mod triangle;
mod objLoader;

use color::Color;
use framebuffer::Framebuffer;
use triangle::draw_triangle;
use objLoader::load_obj;
use minifb::{Key, Window, WindowOptions};
use nalgebra::{Vector3, Rotation3};
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// 🔹 Normaliza el modelo a un rango [-0.5, 0.5] centrado
fn normalize_model(mesh: &mut objLoader::Mesh) {
    if mesh.vertices.is_empty() {
        return;
    }

    let mut min = mesh.vertices[0];
    let mut max = mesh.vertices[0];

    for v in &mesh.vertices {
        min.x = min.x.min(v.x);
        min.y = min.y.min(v.y);
        min.z = min.z.min(v.z);

        max.x = max.x.max(v.x);
        max.y = max.y.max(v.y);
        max.z = max.z.max(v.z);
    }

    let center = (max + min) / 2.0;
    let size = (max - min).norm();

    for v in &mut mesh.vertices {
        *v = (*v - center) / size; // centrar y escalar al rango [-0.5, 0.5]
    }
}

// 🔹 Proyección en perspectiva (3D → 2D)
fn project_vertex(v: Vector3<f32>, width: f32, height: f32) -> Vector3<f32> {
    let fov = 90.0_f32.to_radians();
    let aspect_ratio = width / height;
    let f = 1.0 / (fov / 2.0).tan();

    // Proyección simple
    let x = v.x * f / aspect_ratio / v.z;
    let y = v.y * f / v.z;

    Vector3::new(
        (x + 1.0) * width / 2.0,
        (1.0 - y) * height / 2.0,
        v.z,
    )
}

fn main() {
    let mut window = Window::new("🚀 Render Nave 3D", WIDTH, HEIGHT, WindowOptions::default())
        .expect("No se pudo crear la ventana");

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // 🔹 Cargar y normalizar el modelo
    let mut mesh = load_obj("model/nave2.obj");
    normalize_model(&mut mesh);

    // 🔹 Escalar el modelo para hacerlo más grande
    let scale = 2.0; 
    for v in &mut mesh.vertices {
        *v *= scale;
    }

    println!(
        "Modelo cargado: {} vértices, {} triángulos",
        mesh.vertices.len(),
        mesh.indices.len()
    );

    // 🔹 Control de tiempo para animación
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(Color::new(0, 0, 0));

        let elapsed = start_time.elapsed().as_secs_f32();

        // 🔹 Rotación 3D en Y (y un poco en X para perspectiva)
        let rotation = Rotation3::from_euler_angles(elapsed * 0.3, elapsed * 0.6, 0.0);

        // 🔹 Dibujar cada triángulo del modelo
        for face in &mesh.indices {
            let mut v0 = mesh.vertices[face[0]];
            let mut v1 = mesh.vertices[face[1]];
            let mut v2 = mesh.vertices[face[2]];

            // Rotar los vértices
            v0 = rotation * v0;
            v1 = rotation * v1;
            v2 = rotation * v2;

            // Mover hacia adelante (para que no esté detrás de la cámara)
            v0.z += 2.0;
            v1.z += 2.0;
            v2.z += 2.0;

            // Proyectar a coordenadas de pantalla
            let v0_screen = project_vertex(v0, WIDTH as f32, HEIGHT as f32);
            let v1_screen = project_vertex(v1, WIDTH as f32, HEIGHT as f32);
            let v2_screen = project_vertex(v2, WIDTH as f32, HEIGHT as f32);

            draw_triangle(
                &mut framebuffer,
                v0_screen,
                v1_screen,
                v2_screen,
                Color::new(87, 199, 199),
            );
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
