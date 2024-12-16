
use std::f64::consts::PI;
use nalgebra as na;

// Constants
const AIR_DENSITY: f64 = 1.225; // kg/m³ at sea level, 15°C
const AIR_VISCOSITY: f64 = 1.81e-5; // kg/(m·s) at 15°C

#[derive(Debug, Clone)]
struct Vertex(na::Point3<f64>);

#[derive(Debug, Clone)]
struct Face {
    vertices: Vec<usize>,  // Indices into vertex array
    normal: na::Vector3<f64>,
    area: f64,
}

#[derive(Debug)]
struct IrregularPolyhedron {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    center_of_mass: na::Point3<f64>,
    orientation: na::Rotation3<f64>,
}

impl IrregularPolyhedron {
    fn new(vertices: Vec<[f64; 3]>, faces: Vec<Vec<usize>>) -> Self {
        let vertices: Vec<Vertex> = vertices
            .into_iter()
            .map(|v| Vertex(na::Point3::new(v[0], v[1], v[2])))
            .collect();

        let mut polyhedron_faces = Vec::new();
        for face_vertices in faces {
            let v0 = vertices[face_vertices[0]].0;
            let v1 = vertices[face_vertices[1]].0;
            let v2 = vertices[face_vertices[2]].0;

            // Calculate face normal and area
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2).normalize();
            
            // Calculate area using Newell's method for arbitrary polygons
            let mut area = 0.0;
            for i in 0..face_vertices.len() {
                let j = (i + 1) % face_vertices.len();
                let vi = vertices[face_vertices[i]].0;
                let vj = vertices[face_vertices[j]].0;
                area += (vj - vi).norm() * (vi + vj).norm() / 2.0;
            }

            polyhedron_faces.push(Face {
                vertices: face_vertices,
                normal: normal,
                area,
            });
        }

        // Calculate center of mass (assuming uniform density)
        let com = vertices.iter().fold(na::Point3::origin(), |acc, v| acc + v.0.coords)
            / vertices.len() as f64;

        IrregularPolyhedron {
            vertices,
            faces: polyhedron_faces,
            center_of_mass: com,
            orientation: na::Rotation3::identity(),
        }
    }

    fn rotate(&mut self, axis: &na::Vector3<f64>, angle: f64) {
        self.orientation *= na::Rotation3::from_axis_angle(&na::Unit::new_normalize(*axis), angle);
    }

    fn get_projected_area(&self, flow_direction: &na::Vector3<f64>) -> f64 {
        let flow_dir = self.orientation * flow_direction;
        
        // Calculate projected area by summing contributions from each face
        self.faces.iter().map(|face| {
            let cos_angle = face.normal.dot(&flow_dir).abs();
            face.area * cos_angle
        }).sum()
    }

    fn get_characteristic_length(&self) -> f64 {
        // Use maximum distance between any two vertices as characteristic length
        let mut max_distance = 0.0;
        for (i, v1) in self.vertices.iter().enumerate() {
            for v2 in self.vertices.iter().skip(i + 1) {
                let distance = (v2.0 - v1.0).norm();
                max_distance = max_distance.max(distance);
            }
        }
        max_distance
    }

    fn get_surface_area(&self) -> f64 {
        self.faces.iter().map(|face| face.area).sum()
    }
}

struct FlowConditions {
    velocity: na::Vector3<f64>,
    density: f64,
    viscosity: f64,
}

impl Default for FlowConditions {
    fn default() -> Self {
        FlowConditions {
            velocity: na::Vector3::new(10.0, 0.0, 0.0), // 10 m/s in x direction
            density: AIR_DENSITY,
            viscosity: AIR_VISCOSITY,
        }
    }
}

fn calculate_reynolds_number(length: f64, velocity: f64, conditions: &FlowConditions) -> f64 {
    (conditions.density * velocity * length) / conditions.viscosity
}

fn calculate_drag_coefficients(reynolds: f64, shape_complexity: f64) -> (f64, f64) {
    // Pressure drag coefficient
    let cd_pressure = match reynolds {
        re if re < 1.0 => 24.0 / re,
        re if re < 1000.0 => 24.0 / re + 6.0 / (1.0 + re.sqrt()),
        re if re < 2e5 => 1.1 * shape_complexity,
        re if re < 7e5 => 0.8 * shape_complexity,
        _ => 0.6 * shape_complexity,
    };

    // Friction drag coefficient
    let cd_friction = match reynolds {
        re if re < 1e5 => 1.328 / re.sqrt(),
        _ => 0.074 / reynolds.powf(0.2),
    };

    (cd_pressure, cd_friction)
}

struct DragResult {
    pressure_drag: na::Vector3<f64>,
    friction_drag: na::Vector3<f64>,
    reynolds_number: f64,
    cd_pressure: f64,
    cd_friction: f64,
}

fn calculate_drag(polyhedron: &IrregularPolyhedron, conditions: &FlowConditions) -> DragResult {
    let velocity_magnitude = conditions.velocity.norm();
    let flow_direction = conditions.velocity.normalize();
    
    // Get characteristic length and Reynolds number
    let char_length = polyhedron.get_characteristic_length();
    let reynolds = calculate_reynolds_number(char_length, velocity_magnitude, conditions);
    
    // Calculate shape complexity factor (ratio of actual surface area to minimum possible surface area)
    let actual_surface_area = polyhedron.get_surface_area();
    let min_surface_area = 4.0 * PI * (char_length / 2.0).powi(2);  // sphere surface area
    let shape_complexity = actual_surface_area / min_surface_area;
    
    // Get drag coefficients
    let (cd_pressure, cd_friction) = calculate_drag_coefficients(reynolds, shape_complexity);
    
    // Calculate projected area
    let projected_area = polyhedron.get_projected_area(&flow_direction);
    
    // Calculate drag forces
    let dynamic_pressure = 0.5 * conditions.density * velocity_magnitude.powi(2);
    
    let pressure_drag = flow_direction * (cd_pressure * dynamic_pressure * projected_area);
    let friction_drag = flow_direction * (cd_friction * dynamic_pressure * actual_surface_area);
    
    DragResult {
        pressure_drag,
        friction_drag,
        reynolds_number: reynolds,
        cd_pressure,
        cd_friction,
    }
}

fn main() {
    // Example usage with an irregular tetrahedron
    let vertices = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.3, 1.2, 0.0],
        [0.5, 0.4, 1.5],
    ];
    
    let faces = vec![
        vec![0, 1, 2],
        vec![0, 1, 3],
        vec![1, 2, 3],
        vec![0, 2, 3],
    ];
    
    let mut polyhedron = IrregularPolyhedron::new(vertices, faces);
    
    // Rotate the polyhedron 30 degrees around the y-axis
    polyhedron.rotate(&na::Vector3::new(0.0, 1.0, 0.0), PI / 6.0);
    
    let conditions = FlowConditions::default();
    
    let result = calculate_drag(&polyhedron, &conditions);
    
    println!("Irregular Polyhedron Analysis:");
    println!("Characteristic Length: {:.3} m", polyhedron.get_characteristic_length());
    println!("Surface Area: {:.3} m²", polyhedron.get_surface_area());
    println!("Projected Area: {:.3} m²", 
             polyhedron.get_projected_area(&conditions.velocity.normalize()));
    
    println!("\nFlow Conditions:");
    println!("Velocity: [{:.1}, {:.1}, {:.1}] m/s", 
             conditions.velocity.x, conditions.velocity.y, conditions.velocity.z);
    
    println!("\nResults:");
    println!("Reynolds Number: {:.1e}", result.reynolds_number);
    println!("Pressure Drag Coefficient: {:.3}", result.cd_pressure);
    println!("Friction Drag Coefficient: {:.3}", result.cd_friction);
    println!("Pressure Drag: [{:.3}, {:.3}, {:.3}] N",
             result.pressure_drag.x, result.pressure_drag.y, result.pressure_drag.z);
    println!("Friction Drag: [{:.3}, {:.3}, {:.3}] N",
             result.friction_drag.x, result.friction_drag.y, result.friction_drag.z);
    println!("Total Drag: [{:.3}, {:.3}, {:.3}] N",
             result.pressure_drag.x + result.friction_drag.x,
             result.pressure_drag.y + result.friction_drag.y,
             result.pressure_drag.z + result.friction_drag.z);
}
