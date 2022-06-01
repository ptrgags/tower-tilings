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

    pub fn add_vertex(&mut self, position: (f64, f64, f64)) -> VertexHandle {
        let index = self.vertices.len();
        self.vertices.push(Vertex::new(position));

        VertexHandle {
            index
        }
    }

    pub fn add_face(&mut self, vertices: Vec<VertexHandle>) -> FaceHandle {
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
            let vert_index = vertices[i].index;
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

        FaceHandle {
            index
        }
    }

    pub fn extrude(&mut self, face: FaceHandle, _extrude_dist: f64) -> FaceHandle {
        println!("implement extrude()!");
        face
    }

    pub fn extrude_profile(&mut self, face: FaceHandle, _profile: Vec<(i32, i32)>) -> FaceHandle {
        println!("implement extrude_profile()!");
        face
    }

    pub fn save(&self, _fname: &str) {
        println!("implement save()!");
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

#[derive(Clone, Copy)]
pub struct VertexHandle {
    pub index: usize
}

#[derive(Clone, Copy)]
pub struct FaceHandle {
    pub index: usize
}