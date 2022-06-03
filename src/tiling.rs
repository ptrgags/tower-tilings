use serde::Deserialize;

use std::f64::consts::PI;

use crate::vec3::Vec3;

type TilingVector = (i32, i32, i32, i32);

fn make_twelfth_root_basis() -> [Vec3; 12] {
    let mut result = [(0.0, 0.0, 0.0); 12];
    for i in 0..12 {
        let angle = (i as f64) * PI / 6.0;
        result[i] = (angle.cos(), angle.sin(), 0.0);
    }
    result
}

const GRAPH_PAPER_BASIS: [Vec3; 12] = [
    (1.0, 0.0, 0.0),
    (1.0, 0.5, 0.0),
    (0.5, 1.0, 0.0),
    (0.0, 1.0, 0.0),
    (-0.5, 1.0, 0.0),
    (-1.0, 0.5, 0.0),
    (-1.0, 0.0, 0.0),
    (-1.0, -0.5, 0.0),
    (-0.5, -1.0, 0.0),
    (0.0, -1.0, 0.0),
    (0.5, -1.0, 0.0),
    (1.0, -0.5, 0.0),
];

#[derive(Deserialize)]
pub enum Basis {
    TwelfthRoot,
    GraphPaper
}

impl Basis {
    fn get_basis(&self) -> [Vec3; 12] {
        match self {
            Basis::TwelfthRoot => make_twelfth_root_basis(),
            Basis::GraphPaper => GRAPH_PAPER_BASIS
        }
    }
}

/* TODO: Not yet but soon.
#[derive(Deserialize)]
pub struct TilingFace {
    // Redundant, but helpful for debugging
    sides: usize,
    profile: Option<Vec<(i32, i32)>>
}
*/

#[derive(Deserialize)]
pub struct Seed {
    position: TilingVector,
    //anchored_faces: Vec<TilingFace>
}

#[derive(Deserialize)]
pub struct IntegerTiling {
    basis: Basis,
    translations: [TilingVector; 2],
    seeds: Vec<Seed>
}