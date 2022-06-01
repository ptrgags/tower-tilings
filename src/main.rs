mod mesh;

use crate::mesh::Mesh;

fn main() {
    let mut mesh = Mesh::new();

    let a = mesh.add_vertex((0.0, 0.0, 0.0));
    let b = mesh.add_vertex((1.0, 0.0, 0.0));
    let c = mesh.add_vertex((0.5, 1.0, 0.0));

    let _ = mesh.add_face(vec![c, b, a]);
    let top_face = mesh.add_face(vec![a, b, c]);

    // must be called before extrude()
    mesh.compute_face_normals();
    let top_face = mesh.extrude(top_face, 0.5);

    let profile = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, 1),
    ];

    mesh.extrude_profile(top_face, profile);
    mesh.save("tower.obj");
}
