use std::collections::HashMap;

use chrono::{Datelike, Utc};
use serde_json::json;

use crate::tiling::Material;
use crate::vec3::Vec3;

pub struct Instances {
    pub translations: Vec<Vec3>
}

impl Instances {
    pub fn new() -> Self {
        Self {
            translations: Vec::new()
        }
    }
}

pub struct BufferView {
}

pub struct Accessor {
    buffer_view: usize,
    accessor_type: String,
    component_type: u32,
}

pub struct Primitive {
    material: usize,
    indices: Accessor,
    attributes: HashMap<String, usize>
}

pub struct Gltf {
    materials: Vec<Material>,
    instances: Instances,
    primitives: Vec<Primitive>,
    accessors: Vec<Accessor>
}

impl Gltf {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            instances: Instances::new(),
            primitives: Vec::new(),
            accessors: Vec::new()
        }
    }

    pub fn add_materials(&mut self, materials: Vec<Material>) {
        self.materials = materials;
    }

    pub fn to_json(&self) -> serde_json::Value {
        let copyright = format!("© {} Peter Gagliardi", Utc::now().year());

        let material_json: Vec<serde_json::Value> = self.materials.iter()
            .map(|x| x.to_json())
            .collect();

        json!({
            "asset": {
                "version": 2.0,
                "copyright": copyright,
                "generator": "Tower tiling generator from https://github.com/ptrgags/tower-tilings"
            },
            "scene": 0,
            "scenes": [
                {
                    "nodes": [0]
                }
            ],
            "nodes": [
                {
                    "mesh": 0,
                    "name": "Tower Tiling",
                    "extensions": {
                        "EXT_mesh_gpu_instancing": {
                            "attributes": {
                                "TRANSLATION": 0 // TODO: where to get the index?
                            }
                        }
                    }
                }
            ],
            "materials": material_json,
            "meshes": [
                {
                    "primitives": [
                        // TODO: Loop over all primitives
                        {
                            "material": 0,
                            // TODO: how to manage these references?
                            "attributes": {
                                "POSITION": 0,
                                "NORMAL": 1,
                            },
                            "indices": 2
                        }
                    ]
                }
            ],
            "accessors": [
                // TODO: loop over accessors and generate this
                {
                    "type": "VEC3",
                    "componentType": 5126
                }
            ],
            "bufferViews": [
                // TODO: Generate this from all the accessors
                {
                    "buffer": 0,
                    "byteOffset": 0,
                    "byteLength": 0
                }
            ],
            "buffers": [
                {
                    "byteLength": 0 // TODO: Compute from all the buffer views
                }
            ]
        })
    }
}