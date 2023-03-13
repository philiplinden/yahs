use toml::Value;

use super::materials::Material;

pub fn read_as_f32(config: &Value, key: &str) -> f32 {
    config[key].as_float().unwrap() as f32
}

pub fn read_as_material(config: &Value, key: &str) -> Material {
    Material::new(config[key].as_str().unwrap())
}

pub fn read_as_bool(config: &Value, key: &str) -> bool {
    config[key].as_bool().unwrap() as bool
}
