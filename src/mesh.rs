use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use crate::vec3::Vec3;

pub struct Vertex {
    pub position: Vec3,
    pub half_edge: Option<usize>,
    pub deleted: bool
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            half_edge: None,
            deleted: false
        }
    }
}

pub struct HalfEdge {
    pub from_vertex: usize,
    pub previous: Option<usize>,
    pub next: Option<usize>,
    pub twin: Option<usize>,
    pub face: Option<usize>
}

impl HalfEdge {
    pub fn new(from_vertex: usize) -> Self {
        Self {
            from_vertex,
            previous: None,
            next: None,
            twin: None,
            face: None
        }
    }
}

pub struct Face {
    pub half_edge: usize,
    pub normal: Option<Vec3>,
}

impl Face {
    pub fn new(half_edge: usize) -> Self {
        Self {
            half_edge,
            normal: None
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub half_edges: Vec<HalfEdge>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: Vec::new()
        }
    }

    pub fn add_vertex(&mut self, position: Vec3) -> usize {
        let index = self.vertices.len();
        self.vertices.push(Vertex::new(position));
        index
    }

    pub fn add_face(&mut self, vertices: &[usize]) -> usize {
        assert!(vertices.len() >= 3, "Must have at least 3 vertices");

        let index = self.faces.len();
        let mut new_edges: Vec<usize> = Vec::new();

        let n = vertices.len();

        // Create edges for the sides of the faces, one per vertex of the face
        for i in 0..n {
            // Index of the edge to be created
            let half_edge_index = self.half_edges.len();
            new_edges.push(half_edge_index);
            
            // Create the half edge rooted at the current vertex
            let vert_index = vertices[i];
            let half_edge = HalfEdge::new(vert_index);
            self.half_edges.push(half_edge);
            
            // If this is the first half edge attached to the vertex,
            // point to this half edge
            if let None = self.vertices[vert_index].half_edge {
                self.vertices[i].half_edge = Some(half_edge_index);
            }
        }

        // Connect the edges into a loop
        for i in 0..n {
            let from = new_edges[i];
            let to = new_edges[(i + 1) % n];
            self.half_edges[from].next = Some(to);
            self.half_edges[to].previous = Some(from);
        }

        // Create the face
        let first_edge_index = new_edges[0];
        let face_index = self.faces.len();
        let face = Face::new(first_edge_index);
        self.faces.push(face);

        // All the edges should point to the face
        for i in 0..n {
            let index = new_edges[i];
            self.half_edges[index].face = Some(face_index)
        }

        index
    }

    pub fn compute_face_normals(&mut self) {
        for face in self.faces.iter_mut() {
            // Normal was already computed
            if let Some(_) = face.normal {
                continue;
            }

            // This assumes the face is a coplanar n-gon
            // so the normal of the triangle spanned by the first
            // 3 edges will give the normal
            let edge1 = &self.half_edges[face.half_edge];
            let edge2 = &self.half_edges[edge1.next.unwrap()];
            let edge3 = &self.half_edges[edge2.next.unwrap()];

            let (ax, ay, az) = &self.vertices[edge1.from_vertex].position;
            let (bx, by, bz) = &self.vertices[edge2.from_vertex].position;
            let (cx, cy, cz) = &self.vertices[edge3.from_vertex].position;

            let (bax, bay, baz) = (ax - bx, ay - by, az - bz);
            let (bcx, bcy, bcz) = (cx - bx, cy - by, cz - bz);

            // compute the cross product bc x ba
            // |  x   y   z  |
            // | bcx bcy bcz |
            // | bax bay baz |
            let nx = bcy * baz - bcz * bay;
            let ny = bcz * bax - bcx * baz;
            let nz = bcx * bay - bcy * bax;

            let length = (nx * nx + ny * ny + nz * nz).sqrt();
            face.normal = Some((nx / length, ny / length, nz / length));
        }
    }

    pub fn all_vertices(&self) -> std::slice::Iter<Vertex> {
        self.vertices.iter()
    }

    pub fn all_faces(&self) -> std::slice::Iter<Face> {
        self.faces.iter()
    }

    pub fn face_edge_iter(&self, face: usize) -> FaceEdgeIter {
        FaceEdgeIter::new(self, face)
    }

    pub fn get_face_positions(&self, face: usize) -> Vec<Vec3> {
        self.face_edge_iter(face)
            .map(|e| {
                let index = self.half_edges[e].from_vertex;
                self.vertices[index].position
            }).collect()
    }

    pub fn extrude(&mut self, face_index: usize, extrude_dist: f64) -> usize {
        let face = &mut self.faces[face_index];
        let (nx, ny, nz) = face.normal.unwrap();

        let old_vertices: Vec<usize> = self.face_edge_iter(face_index)
            .map(|i| self.half_edges[i].from_vertex)
            .collect();
        let new_vertices: Vec<usize> = old_vertices.iter()
            .map(|i| {
                let (x, y, z) = self.vertices[*i].position;
                let extrude_position = (
                    x + extrude_dist * nx,
                    y + extrude_dist * ny,
                    z + extrude_dist * nz
                );
                self.add_vertex(extrude_position)
            })
            .collect();
        
        // Create new faces for the sides
        // new1 -- new2
        //  |       |
        // old1 -- old2
        let n = old_vertices.len();
        for i in 0..n {
            let old1 = old_vertices[i];
            let old2 = old_vertices[(i + 1) % n];
            let new1 = new_vertices[i];
            let new2 = new_vertices[(i + 1) % n];

            // TODO: This should check for existing edges
            self.add_face(&[old1, old2, new2, new1]);
        }

        // create a face for the top;
        self.add_face(&new_vertices)
    }

    fn compute_centroid(positions: &[Vec3]) -> Vec3 {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        let n = positions.len() as f64;
        for (x, y, z) in positions.iter() {
            cx += x;
            cy += y;
            cz += z;
        }

        (cx / n, cy / n, cz / n)
    }

    pub fn extrude_profile(&mut self, face: usize, profile: &[(i32, i32)]) -> usize {
        let (nx, ny, nz) = self.faces[face].normal.unwrap();
        
        // get the original vertices
        let old_vertices: Vec<usize> = self.face_edge_iter(face)
            .map(|e| self.half_edges[e].from_vertex)
            .collect();
        let old_positions: Vec<Vec3> = old_vertices.iter()
            .map(|v| self.vertices[*v].position)
            .collect();

        // compute the direction towards the center of the face, this
        // will be used for every extrusion
        let (cx, cy, cz) = Self::compute_centroid(&old_positions);
        let center_directions: Vec<Vec3> = old_positions.iter()
            .map(|(x, y, z)| (cx - x, cy - y, cz - z))
            .collect();

        // Using a custom coordiate system (center, normal)
        // where center is the direction from the current face vertex to the
        // centroid horizontally (center_directions above), and normal is
        // a height offset. The profile is given in relative offsets
        const CENTER_STEP: f64 = 1.0 / 8.0;
        const NORMAL_STEP: f64 = 0.1;
        let mut profile_c: i32 = 0;
        let mut profile_n: i32 = 0;
        let mut all_vertices: Vec<Vec<usize>> = vec![old_vertices];
        for (center_offset, normal_offset) in profile {
            profile_c += center_offset;
            profile_n += normal_offset;

            let dc = profile_c as f64 * CENTER_STEP;
            let dn = profile_n as f64 * NORMAL_STEP;

            let layer_vertices: Vec<usize> = old_positions.iter()
                .enumerate()
                .map(|(i, (x, y, z))| {
                    let (cx, cy, cz) = center_directions[i];
                    let position = (
                        x + dc * cx + dn * nx,
                        y + dc * cy + dn * ny,
                        z + dc * cz + dn * nz
                    );
                    self.add_vertex(position)
                })
                .collect();
            
                all_vertices.push(layer_vertices);
        }

        // Create the sides of the extruded portion
        for i in 0..(all_vertices.len() - 1) {
            let current_layer = &all_vertices[i];
            let next_layer = &all_vertices[i + 1];

            let n = current_layer.len();
            for j in 0..n {
                let current1 = current_layer[j];
                let current2 = current_layer[(j + 1) % n];
                let next1 = next_layer[j];
                let next2 = next_layer[(j + 1) % n];
                self.add_face(&[current1, current2, next2, next1]);
            }
        }

        // create a new face on the top

        let top = &all_vertices[all_vertices.len() - 1];
        self.add_face(top)
    }

    pub fn triangulate(&self) -> (Vec<Vec3>, Vec<Vec3>, Vec<u32>) {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        let mut vertex_count: usize = 0;
        for (i, face) in self.faces.iter().enumerate() {
            let normal = face.normal.unwrap();

            let ngon_positions: Vec<Vec3> = self.face_edge_iter(i)
                .map(|e| self.half_edges[e].from_vertex)
                .map(|v| self.vertices[v].position)
                .collect();

            let n: usize = ngon_positions.len();

            for j in 0..n {
                // Since the normals differ, we have to duplicate the positions
                // here
                positions.push(ngon_positions[j]);
                normals.push(normal);
            }

            for offset in Self::triangulate_ngon(n) {
                indices.push((vertex_count + offset) as u32);
            }
            vertex_count += n;
        }

        (positions, normals, indices)
    }

    pub fn triangulate_ngon(n: usize) -> Vec<usize> {
        let mut result = Vec::new();

        let first = 0;
        for i in 0..(n - 2) {
            let second = i + 1;
            let third = i + 2;
            result.push(first);
            result.push(second);
            result.push(third);
        }

        result
    }

    pub fn save_obj(&self, fname: &str) {
        let file = File::create(fname).expect("could not open file");
        let mut file = LineWriter::new(file);

        for vertex in self.all_vertices() {
            let (x, y, z) = vertex.position;
            let vertex_line = format!("v {} {} {}\n", x, y, z);
            file.write_all(vertex_line.as_bytes())
                .expect("could not write vertex line");
        }

        for (i, _) in self.all_faces().enumerate() {
            let indices = self.face_edge_iter(i)
                .map(|i| self.half_edges[i].from_vertex);
            let obj_indices = indices
                // OBJ is 1-indexed
                .map(|x| format!("{}", x + 1))
                .collect::<Vec<String>>()
                .join(" ");
            let face_line = format!("f {}\n", obj_indices);
            file.write_all(face_line.as_bytes())
                .expect("Could not write face line");
        }
    }
}

/// Iterator that produces the edges around a face
pub struct FaceEdgeIter<'a> {
    mesh: &'a Mesh,
    first_edge: usize,
    current_edge: Option<usize>,
}

impl<'a> FaceEdgeIter<'a> {
    pub fn new(mesh: &'a Mesh, face: usize) -> Self {
        let first_edge = mesh.faces[face].half_edge;
        Self {
            mesh,
            first_edge,
            current_edge: Some(first_edge)
        }
    }

    fn advance(&mut self, next_edge: Option<usize>) {
        self.current_edge = match next_edge {
            Some(edge) => if edge == self.first_edge { 
                None 
            } else {
                Some(edge)
            },
            None => None
        };
    }
}

impl<'a> Iterator for FaceEdgeIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.current_edge {
            let current = &self.mesh.half_edges[index];

            // Use the next pointer, but if we returned to the start,
            // mark the end of iteration.
            self.advance(current.next);
            Some(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_triangle() {
        let mut mesh = Mesh::new();
        let pos_a = (0.0, 0.0, 0.0);
        let pos_b = (1.0, 0.0, 0.0);
        let pos_c = (0.0, 1.0, 0.0);
        let a = mesh.add_vertex(pos_a);
        let b = mesh.add_vertex(pos_b);
        let c = mesh.add_vertex(pos_c);
        mesh.add_face(&[a, b, c]);

        // Make sure the vertices were added properly
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.vertices[0].position, pos_a);
        assert_eq!(mesh.vertices[1].position, pos_b);
        assert_eq!(mesh.vertices[2].position, pos_c);

        assert_eq!(mesh.vertices[0].half_edge, Some(0));
        assert_eq!(mesh.vertices[1].half_edge, Some(1));
        assert_eq!(mesh.vertices[2].half_edge, Some(2));

        // Make sure the edges were added properly
        assert_eq!(mesh.half_edges.len(), 3);
        let ab = &mesh.half_edges[0];
        assert_eq!(ab.face, Some(0));
        assert_eq!(ab.from_vertex, 0);
        assert_eq!(ab.next, Some(1));
        assert_eq!(ab.previous, Some(2));
        assert_eq!(ab.twin, None);

        let bc = &mesh.half_edges[1];
        assert_eq!(bc.face, Some(0));
        assert_eq!(bc.from_vertex, 1);
        assert_eq!(bc.next, Some(2));
        assert_eq!(bc.previous, Some(0));
        assert_eq!(bc.twin, None);

        let ca = &mesh.half_edges[2];
        assert_eq!(ca.face, Some(0));
        assert_eq!(ca.from_vertex, 2);
        assert_eq!(ca.next, Some(0));
        assert_eq!(ca.previous, Some(1));
        assert_eq!(ca.twin, None);

        // We should have a single face. The normal has not
        // yet been computed, and it points to the first half edge
        // in the face we added
        assert_eq!(mesh.faces.len(), 1);
        assert_eq!(mesh.faces[0].half_edge, 0);
        assert!(mesh.faces[0].normal.is_none());
    }
}