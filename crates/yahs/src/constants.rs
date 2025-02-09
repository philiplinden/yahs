#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

use bevy::{prelude::*, reflect::Reflect};

pub const BOLTZMANN_CONSTANT: f32 = 1.380649e-23_f32; // [J/K]
pub const AVOGADRO_CONSTANT: f32 = 6.02214076e23_f32; // [1/mol]
pub const GAS_CONSTANT: f32 = BOLTZMANN_CONSTANT * AVOGADRO_CONSTANT; // [J/K-mol]

pub const STANDARD_G: f32 = 9.80665; // [m/s^2] standard gravitational acceleration
pub const EARTH_RADIUS_M: f32 = 6371007.2; // [m] mean radius of Earth

pub const PI: f32 = std::f32::consts::PI;
