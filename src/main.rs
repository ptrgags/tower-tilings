mod gltf;
mod mesh;
mod tiling;
mod tiling_mesh;
mod towers;
mod vec3;

use std::fs::File;
use std::io::BufReader;

use crate::tiling::IntegerTiling;
use crate::tiling_mesh::TilingMesh;

fn main() {
    let file = File::open("input/square-tiling.json").unwrap();
    let reader = BufReader::new(file);
    
    let tiling: IntegerTiling = serde_json::from_reader(reader).unwrap();
    let mut towers = TilingMesh::new(tiling);
    towers.compute_mesh();
    towers.save_base("output/test-base.obj");
    towers.make_towers();
    towers.save_towers("output/test-towers.glb", 3);
}
