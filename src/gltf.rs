use std::collections::HashMap;

use chrono::{Datelike, Utc};
use serde_json::json;

use crate::tiling::Material;
use crate::mesh::Mesh;
use crate::vec3::Vec3;

const GLTF_FLOAT: u32 = 5126;
const GLTF_UNSIGNED_INT: u32 = 5125;

pub struct BufferView {
    name: String,
    byte_offset: usize,
    byte_length: usize

    // no padding needed since everything is a vec3 or u32
}

impl BufferView {
    pub fn after_offset(&self) -> usize {
        self.byte_offset + self.byte_length
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            // In this case we're always using the embedded buffer
            "buffer": 0,
            "byteOffset": self.byte_offset,
            "byteLength": self.byte_length,
        })
    }
}

pub struct Accessor {
    name: String,
    buffer_view: usize,
    accessor_type: String,
    component_type: u32,
    count: usize,
    min: Option<[f64; 3]>,
    max: Option<[f64; 3]>
}

impl Accessor {
    pub fn to_json(&self) -> serde_json::Value {
        let mut result = json!({
            "name": self.name,
            "bufferView": self.buffer_view,
            "type": self.accessor_type,
            "componentType": self.component_type,
            "count": self.count,
        });

        if let (Some(min), Some(max)) = (self.min, self.max) {
            result["min"] = json!(min);
            result["max"] = json!(max);
        }

        result
    }
}

pub struct Primitive {
    material: usize,
    indices: usize,
    attributes: HashMap<String, usize>
}

impl Primitive {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "material": self.material,
            "attributes": self.attributes,
            "indices": self.indices
        })
    }
}

pub struct Instances {
    translation: usize,
}

impl Instances {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "attributes": {
                "TRANSLATION": self.translation
            }
        })
    }
}

pub struct Gltf {
    materials: Vec<Material>,
    instances: Instances,
    primitives: Vec<Primitive>,
    accessors: Vec<Accessor>,
    buffer_views: Vec<BufferView>,
    buffer_data: Vec<u8>
}

impl Gltf {
    pub fn new() -> Self {
        let instances = Instances {
            translation: 0
        };

        Self {
            materials: Vec::new(),
            instances,
            primitives: Vec::new(),
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            buffer_data: Vec::new()
        }
    }

    pub fn add_materials(&mut self, materials: Vec<Material>) {
        self.materials = materials;
    }

    

    fn add_buffer_view(&mut self, name: &str, data: Vec<u8>) -> usize {
        let index = self.buffer_views.len();

        // Compute the offset for this buffer view
        let byte_offset = if index == 0 {
            0
        } else {
            self.buffer_views[index - 1].after_offset()
        };

        let buffer_view = BufferView {
            name: String::from(name),
            byte_offset,
            byte_length: data.len()
        };

        self.buffer_views.push(buffer_view);
        // move the data into the buffer
        self.buffer_data.append(&mut data);

        index
    }

    fn add_accesor(&mut self, accessor: Accessor) -> usize {
        let index = self.accessors.len();
        self.accessors.push(accessor);

        index
    }

    fn get_min_max(vectors: &[Vec3]) -> ([f64; 3], [f64; 3]) {
        let mut x_min = f64::INFINITY;
        let mut y_min = f64::INFINITY;
        let mut z_min = f64::INFINITY;
        let mut x_max = -f64::INFINITY;
        let mut y_max = -f64::INFINITY;
        let mut z_max = -f64::INFINITY;

        for (x, y, z) in vectors.iter() {
            x_min = x_min.min(*x);
            y_min = y_min.min(*y);
            z_min = y_min.min(*z);

            x_max = x_max.max(*x);
            y_max = y_max.max(*y);
            z_max = y_max.max(*z);
        }

        (
            [x_min, y_min, z_min],
            [x_max, y_max, z_max]
        )
    }

    fn pack_vec3s(vectors: Vec<Vec3>) -> Vec<u8> {
        let mut result = Vec::new();
        
        for (x, y, z) in vectors {
            result.extend_from_slice(&x.to_le_bytes());
            result.extend_from_slice(&y.to_le_bytes());
            result.extend_from_slice(&z.to_le_bytes());
        }

        result
    }

    fn add_position_accessor(&mut self, positions: Vec<Vec3>) -> usize {
        let count = positions.len();
        let (min, max) = Self::get_min_max(&positions);
        let buffer_view_data = Self::pack_vec3s(positions);
        let buffer_view = self.add_buffer_view("Indices", buffer_view_data);

        self.add_accesor(Accessor {
            name: String::from("Indices"),
            buffer_view: buffer_view,
            accessor_type: String::from("SCALAR"),
            component_type: GLTF_UNSIGNED_INT,
            count,
            min: Some(min),
            max: Some(max)
        })
    }

    fn add_normal_accessor(&mut self, normals: Vec<Vec3>) -> usize {
        let count = normals.len();
        let buffer_view_data = Self::pack_vec3s(normals);
        let buffer_view = self.add_buffer_view("Normals", buffer_view_data);

        self.add_accesor(Accessor {
            name: String::from("Normals"),
            buffer_view,
            accessor_type: String::from("VEC3"),
            component_type: GLTF_FLOAT,
            count,
            min: None,
            max: None
        })
    }

    fn add_indices_accessor(&mut self, indices: Vec<u32>) -> usize {
        let count = indices.len();
        let mut buffer_view_data: Vec<u8> = Vec::new();
        for index in indices {
            buffer_view_data.extend_from_slice(&index.to_le_bytes())
        }
        let buffer_view = self.add_buffer_view("Indices", buffer_view_data);

        self.add_accesor(Accessor {
            name: String::from("Indices"),
            buffer_view,
            accessor_type: String::from("SCALAR"),
            component_type: GLTF_UNSIGNED_INT,
            count,
            min: None,
            max: None
        })
    }

    pub fn add_instances(&mut self, translations: Vec<Vec3>) {
        let count = translations.len();
        let buffer_view_data = Self::pack_vec3s(translations);
        let buffer_view = self.add_buffer_view("Instance TRANSLATION", buffer_view_data);

        let accessor = self.add_accesor(Accessor {
            name: String::from("Instance TRANSLATION"),
            buffer_view,
            accessor_type: String::from("VEC3"),
            component_type: GLTF_FLOAT,
            count,
            min: None,
            max: None
        });

        self.instances = Instances {
            translation: accessor
        }
    }

    pub fn add_primitive(&mut self, mesh: &Mesh, material_id: usize) {
        let (positions, normals, indices) = mesh.triangulate();

        let position_accessor = self.add_position_accessor(positions);
        let normal_accessor = self.add_normal_accessor(normals);
        let indices_accessor = self.add_indices_accessor(indices);

        let mut attributes = HashMap::new();
        attributes.insert(String::from("POSITION"), position_accessor);
        attributes.insert(String::from("NORMAL"), normal_accessor);

        let primitive = Primitive {
            material: material_id,
            indices: indices_accessor,
            attributes
        };
        self.primitives.push(primitive);
    }

    pub fn save(&self, fname: &str) {
        todo!();
    }

    pub fn to_json(&self) -> serde_json::Value {
        let copyright = format!("© {} Peter Gagliardi", Utc::now().year());

        let material_json: Vec<serde_json::Value> = self.materials.iter()
            .map(|x| x.to_json())
            .collect();
        
        let primitive_json: Vec<serde_json::Value> = self.primitives.iter()
            .map(|x| x.to_json())
            .collect();
        
        let buffer_view_json: Vec<serde_json::Value> = self.buffer_views.iter()
            .map(|x| x.to_json())
            .collect();

        let accessor_json: Vec<serde_json::Value> = self.accessors.iter()
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
                        "EXT_mesh_gpu_instancing": self.instances.to_json()
                    }
                }
            ],
            "materials": material_json,
            "meshes": [
                {
                    "primitives": primitive_json
                }
            ],
            "accessors": accessor_json,
            "bufferViews": buffer_view_json,
            "buffers": [
                {
                    "byteLength": self.buffer_data.len()
                }
            ]
        })
    }
}