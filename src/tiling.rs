use std::f64::consts::PI;

use serde::Deserialize;
use serde_json::json;

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

#[derive(Deserialize)]
pub struct Profile {
    pub name: Option<String>,
    pub offsets: Vec<(i32, i32)>
}

/// Simplified material that can be converted to a glTF
/// PBR material
#[derive(Deserialize, Clone)]
pub struct Material {
    pub base_color: Vec3,
    pub metallic: f64,
    pub roughness: f64
}

impl Material {
    pub fn to_json(&self) -> serde_json::Value {
        let (r, g, b) = self.base_color;

        json!({
            "pbrMetallicRoughness": {
                "baseColorFactor": [r, g, b, 1.0],
                "metallicFactor": self.metallic,
                "roughnessFactor": self.roughness
            }
        })
    }
}


#[derive(Deserialize)]
pub struct TilingFace {
    // Redundant, but helpful for debugging
    pub sides: usize,
    pub profile: Option<usize>,
    pub material: usize
}

#[derive(Deserialize)]
pub struct Seed {
    pub position: TilingVector,
    pub faces: Option<Vec<TilingFace>>
}

#[derive(Deserialize)]
pub struct IntegerTiling {
    pub basis: Basis,
    pub translations: [TilingVector; 2],
    pub seeds: Vec<Seed>,
    pub profiles: Vec<Profile>,
    pub materials: Vec<Material>,
}