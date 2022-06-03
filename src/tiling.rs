use serde::Deserialize;

use std::f64::consts::PI;

use crate::vec3::Vec3;

pub type TilingVector = (i32, i32, i32, i32);

fn make_twelfth_root_basis() -> [Vec3; 12] {
    let mut result = [(0.0, 0.0, 0.0); 12];
    for i in 0..12 {
        let angle = (i as f64) * PI / 6.0;
        result[i] = (angle.cos(), angle.sin(), 0.0);
    }
    result
}

const BASIS_COEFFICIENTS: [TilingVector; 12] = [
    (1, 0, 0, 0),
    (0, 1, 0, 0),
    (0, 0, 1, 0),
    (0, 0, 0, 1),
    (-1, 0, 1, 0),
    (0, -1, 0, 1),
    (-1, 0, 0, 0),
    (0, -1, 0, 0),
    (0, 0, -1, 0),
    (0, 0, 0, -1),
    (1, 0, -1, 0),
    (0, 1, 0, -1),
];

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
    pub fn get_basis(&self) -> [Vec3; 12] {
        match self {
            Basis::TwelfthRoot => make_twelfth_root_basis(),
            Basis::GraphPaper => GRAPH_PAPER_BASIS
        }
    }

    pub fn get_coefficients(&self) -> [TilingVector; 12] {
        BASIS_COEFFICIENTS
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
    pub position: TilingVector,
    //anchored_faces: Vec<TilingFace>
}

#[derive(Deserialize)]
pub struct IntegerTiling {
    pub basis: Basis,
    pub translations: [TilingVector; 2],
    pub seeds: Vec<Seed>
}