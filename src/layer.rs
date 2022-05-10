/*
Project: bevy-parallax
File: layer.rs
Version: 0.1.2
Revisions:
    tmills9208: Added support for y scrolling, along with updating relevant structs to adjust the y speed appropriately. Default for y-speed will be 0 to hopefully prevent breaking changes
*/

use bevy::prelude::*;
use serde::Deserialize;

/// Layer initialization data
#[derive(Debug, Deserialize)]
pub struct LayerData {
    /// Relative speed of layer to the camera movement
    pub speed: f32,
    pub speed_y: f32,
    /// Path to layer texture file
    pub path: String,
    /// Size of a tile of the texture
    pub tile_size: Vec2,
    /// Columns in the texture file
    pub cols: usize,
    /// Rows in the texture file
    pub rows: usize,
    /// Scale of the texture
    pub scale: f32,
    /// Z position of the layer
    pub z: f32,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,
}

impl Default for LayerData {
    fn default() -> Self {
        Self {
            speed: 1.0,
            speed_y: 0.0,
            path: "".to_string(),
            tile_size: Vec2::ZERO,
            cols: 1,
            rows: 1,
            scale: 1.0,
            z: 0.0,
            transition_factor: 1.2,
        }
    }
}

/// Core component for parallax layer
#[derive(Component)]
pub struct LayerComponent {
    /// Relative speed of layer to the camera movement
    pub speed: f32,
    pub speed_y: f32,
    /// Number of textures in the layer
    pub texture_count: f32,
    /// Number used to determine when textures are moved to opposite side of camera
    pub transition_factor: f32,
}

/// Core component for layer texture
#[derive(Component)]
pub struct LayerTextureComponent {
    /// Width of the texture
    pub width: f32,
}
