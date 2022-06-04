use crate::mesh::Mesh;
use crate::vec3::Vec3;

pub struct TowerTiling {
    meshes: Vec<Mesh>
}

impl TowerTiling {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new()
        }
    }

    pub fn add_tower(&mut self, base: &[Vec3], profile: &[(i32, i32)]) {
        let mut mesh = Mesh::new();

        // TODO: for glTF exporting, consider centering on
        // the centroid and using a matrix transform.
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

        if profile.len() > 0 {
            mesh.extrude_profile(top_face, profile);
        }

        self.meshes.push(mesh);
    }

    pub fn save(&self, fname_prefix: &str) {
        for (i, mesh) in self.meshes.iter().enumerate() {
            let fname = format!("{}_{}.obj", fname_prefix, i);
            mesh.save(&fname);
        }
    }
}