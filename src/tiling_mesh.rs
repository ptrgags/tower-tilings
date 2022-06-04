use std::collections::HashMap;

use crate::tiling::{IntegerTiling, TilingVector};
use crate::mesh::Mesh;
use crate::towers::TowerTiling;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct CloudVertex {
    seed: TilingVector,
    outside: bool,
    // index in the mesh before garbage collection.
    index: usize
}

pub struct TilingMesh {
    mesh: Mesh,
    cloud: HashMap<TilingVector, CloudVertex>,
    tiling: IntegerTiling,
    basis_coefficients: [TilingVector; 12],
    // In the tiling, face descriptors are stored as
    // tiling.seeds[seed_index].faces?[face_index] if this exists.
    // this table is a map from global_face_index -> (seed_index, face_index)
    anchored_faces: Vec<(usize, usize)>
}

impl TilingMesh {
    pub fn new(tiling: IntegerTiling) -> Self {
        let basis_coefficients = tiling.basis.get_coefficients();
        Self {
            mesh: Mesh::new(),
            cloud: HashMap::new(),
            tiling,
            basis_coefficients,
            anchored_faces: Vec::new()
        }
    }

    pub fn compute_mesh(&mut self) {
        self.init_cloud();
        self.generate_faces();
    }

    pub fn save(&self, fname: &str) {
        self.mesh.save(fname);
    }

    fn init_cloud(&mut self) {
        let [
            (a1, b1, c1, d1), 
            (a2, b2, c2, d2)
        ] = self.tiling.translations;
        for seed in self.tiling.seeds.iter() {
            let (a, b, c, d) = seed.position;
            for i in -1..=1 {
                for j in -1..=1 {
                    let instance = (
                        a + i * a1 + j * a2,
                        b + i * b1 + j * b2,
                        c + i * c1 + j * c2,
                        d + i * d1 + j * d2,
                    );

                    // Insert every copy of the seed vertex. We'll delete
                    // the unused ones at the end.
                    let world_position = self.to_world(instance);
                    let index = self.mesh.add_vertex(world_position);
                    
                    let outside = i != 0 || j != 0;
                    let vertex = CloudVertex {
                        seed: seed.position,
                        outside,
                        index
                    };
                    self.cloud.insert(instance, vertex);
                }
            }
        }
    }

    fn generate_faces(&mut self) {
        let n = self.tiling.seeds.len();

        for i in 0..n {
            self.generate_seed_faces(i);
        }
    }

    fn generate_seed_faces(&mut self, seed: usize) {
        let star_directions = self.star_angles(seed, 10, 15);
        let n = star_directions.len();

        if n == 0 {
            return;
        }

        for i in 0..(n - 1) {
            let first_direction = star_directions[i];
            self.generate_face(seed, first_direction);
            self.anchored_faces.push((seed, i));
        }
    }

    fn star_angles(&self, seed: usize, start_angle: usize, end_angle: usize) -> Vec<usize> {
        let mut star_directions = Vec::new();
        let seed_position = self.tiling.seeds[seed].position;

        for k in start_angle..=end_angle {
            let index = k % 12;
            let adjacent = self.get_adjacent(seed_position, index);

            if self.cloud.contains_key(&adjacent) {
                star_directions.push(index);
            }
        }

        star_directions
    }

    fn generate_face(&mut self, seed: usize, first_direction: usize) {
        let start_position = self.tiling.seeds[seed].position;
        let mut current_position = start_position;
        let mut next_angle = first_direction;
        let mut face_vertices: Vec<usize> = Vec::new();

        loop {
            face_vertices.push(self.cloud[&current_position].index);

            current_position = self.get_adjacent(current_position, next_angle);            
            if current_position == start_position {
                break;
            }
            
            next_angle = (next_angle + 5) % 12;
            // TODO: Can this be merged with the code above?
            while !self.cloud.contains_key(&self.get_adjacent(current_position, next_angle)) {
                next_angle -= 1;
                next_angle %= 12;
            }
        }

        self.mesh.add_face(&face_vertices);   
    }

    fn to_world(&self, coefficients: TilingVector) -> Vec3 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let basis = self.tiling.basis.get_basis();

        // convert to an array so we can iterate over it. Also we need to do
        // math 
        let (c0, c1, c2, c3) = coefficients;
        let coeff_array = [c0, c1, c2, c3];
        for i in 0..4 {
            let (bx, by, bz) = basis[i];
            let coefficient = coeff_array[i] as f64;
            x += bx * coefficient;
            y += by * coefficient;
            z += bz * coefficient;
        }

        (x, y, z)
    }

    fn get_adjacent(&self, position: TilingVector, direction: usize) -> TilingVector {
        let (a, b, c, d) = position;
        let (da, db, dc, dd) = self.basis_coefficients[direction % 12];
        (
            a + da,
            b + db,
            c + dc,
            d + dd
        )
    }

    pub fn make_towers(&self) -> TowerTiling {
        // TODO: I shouldn't be able to do this.
        let n = self.mesh.faces.len();
        let mut towers = TowerTiling::new();
        for face in 0..n {
            let (seed, anchored_face) = self.anchored_faces[face];

            let empty = Vec::new();
            let mut profile = &empty;

            if let Some(faces) = &self.tiling.seeds[seed].faces {
                if let Some(profile_index) = faces[anchored_face].profile {
                    profile = &self.tiling.profiles[profile_index].offsets;
                }
            }

            let base = self.mesh.get_face_positions(face);
            towers.add_tower(&base, profile);
        }

        towers
    }
}