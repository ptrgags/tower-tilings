mod mesh;
mod tiling;
mod tiling_mesh;
mod towers;
mod vec3;

use crate::mesh::Mesh;
use crate::vec3::Vec3;
use crate::tiling::IntegerTiling;
use crate::tiling_mesh::TilingMesh;

fn make_tower(base: &[Vec3], profile: &[(i32, i32)], fname: &str) {
    let mut mesh = Mesh::new();

    let vertices: Vec<usize> = base.iter()
        .map(|position| mesh.add_vertex(*position))
        .collect();

    let vertices_reversed: Vec<usize> = vertices.iter()
        .rev()
        .map(|x| *x)
        .collect();
    let _ = mesh.add_face(&vertices_reversed);
    let top_face = mesh.add_face(&vertices);

    // must be called before extrude()
    mesh.compute_face_normals();
    let top_face = mesh.extrude(top_face, 0.2);

    mesh.compute_face_normals();

    mesh.extrude_profile(top_face, profile);
    mesh.save(fname);
}

fn main() {
    let tiling_json = r#"{
        "basis": "TwelfthRoot",
        "translations": [
            [2, 2, 0, -1],
            [-1, 0, 2, 2]
        ],
        "seeds": [
            {
                "position": [0, 0, 0, 0]
            },
            {
                "position": [0, 1, 0, 0]
            },
            {
                "position": [1, 1, 0, 0]
            },
            {
                "position": [1, 2, 0, -1]
            },
            {
                "position": [1, 2, 1, -1]
            },
            {
                "position": [1, 1, 1, 0]
            },
            {
                "position": [1, 2, 1, 0]
            },
            {
                "position": [0, 2, 2, 0]
            },
            {
                "position": [0, 1, 2, 0]
            },
            {
                "position": [0, 1, 2, 1]
            },
            {
                "position": [-1, 1, 2, 0]
            },
            {
                "position": [-1, 1, 2, 1]
            },
            {
                "position": [-1, 0, 2, 1]
            },
            {
                "position": [-1, 1, 1, 0]
            }
        ]
    }"#;
    
    let tiling: IntegerTiling = serde_json::from_str(tiling_json).unwrap();
    let mut base_mesh = TilingMesh::new(tiling);
    base_mesh.compute_mesh();
    base_mesh.save("output/test-base.obj");
    let towers = base_mesh.make_towers();
    towers.save("output/test-towers");

    // Profile path for both towers
    let profile = vec![
        (1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (1, 0),
        (0, 4),
        (-4, 0),
        (0, 1),
        (2, 0),
        (0, 1),
        (1, 0),
        (1, -1),
        (0, 4),
        (-2, 0),
        (-1, -1),
        (0, 2)
    ];

    let triangle = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (0.5, 1.0, 0.0),
    ];
    make_tower(&triangle, &profile, "output/tri_tower.obj");

    let hexagon = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.5, 1.0, 0.0),
        (1.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
        (-0.5, 1.0, 0.0),
    ];
    make_tower(&hexagon, &profile, "output/hex_tower.obj");
}
