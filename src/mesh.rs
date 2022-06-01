use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

pub struct Vertex {
    pub position: (f64, f64, f64),
    pub half_edge: Option<usize>,
}

impl Vertex {
    pub fn new(position: (f64, f64, f64)) -> Self {
        Self {
            position,
            half_edge: None
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
    pub normal: Option<(f64, f64, f64)>,
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

    pub fn add_vertex(&mut self, position: (f64, f64, f64)) -> usize {
        let index = self.vertices.len();
        self.vertices.push(Vertex::new(position));
        index
    }

    pub fn add_face(&mut self, vertices: Vec<usize>) -> usize {
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

    pub fn extrude(&mut self, face_index: usize, extrude_dist: f64) -> usize {
        let face = &mut self.faces[face_index];
        let (nx, ny, nz) = face.normal.unwrap();

        // TODO: remove the old face

        let old_vertices: Vec<usize> = self.face_edge_iter(face_index)
            .map(|edge| edge.from_vertex)
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
            self.add_face(vec![old1, old2, new2, new1]);
        }

        // create a face for the top;
        self.add_face(new_vertices)
    }

    pub fn extrude_profile(&mut self, face: usize, _profile: Vec<(i32, i32)>) -> usize {
        println!("implement extrude_profile()!");
        face
    }

    pub fn save(&self, fname: &str) {
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
                // OBJ is 1-indexed, hence the + 1
                .map(|edge| edge.from_vertex);
            let obj_indices = indices
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
    type Item = &'a HalfEdge;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.current_edge {
            let current = &self.mesh.half_edges[index];

            // Use the next pointer, but if we returned to the start,
            // mark the end of iteration.
            self.advance(current.next);
            Some(current)
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
        mesh.add_face(vec![a, b, c]);

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