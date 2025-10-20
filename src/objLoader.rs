use nalgebra::Vector3;
use tobj;

pub struct Mesh {
    pub vertices: Vec<Vector3<f32>>,
    pub indices: Vec<[usize; 3]>,
}

pub fn load_obj(path: &str) -> Mesh {
    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("No se pudo cargar el archivo .obj");
    let mesh = &models[0].mesh;

    let vertices = mesh.positions.chunks(3)
        .map(|v| Vector3::new(v[0], v[1], v[2]))
        .collect::<Vec<_>>();

    let indices = mesh.indices.chunks(3)
        .map(|i| [i[0] as usize, i[1] as usize, i[2] as usize])
        .collect::<Vec<_>>();

    Mesh { vertices, indices }
}
