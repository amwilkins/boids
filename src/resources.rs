pub use crate::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug)]
pub struct Grid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Boid>>,
}

impl FromWorld for Grid {
    fn from_world(world: &mut World) -> Self {
        let boid_settings = world.resource::<BoidSettings>();
        let cell_count = (MAP_SIZE / boid_settings.cohesion_range as u32).pow(2) as usize;

        Grid {
            cell_size: boid_settings.cohesion_range,
            cells: HashMap::with_capacity(cell_count),
        }
    }
}

impl Grid {
    pub fn insert(&mut self, boid: &Boid) {
        let cell = (
            (boid.position.x / self.cell_size) as i32,
            (boid.position.y / self.cell_size) as i32,
        );
        self.cells.entry(cell).or_default().push(boid.clone());
    }
}

#[derive(Resource, Debug)]
pub struct BoidSettings {
    pub count: usize,
    pub cohesion_range: f32,
    pub alignment_range: f32,
    pub separation_range: f32,
    //pub min_distance_between_boids: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub collision_coeff: f32,
    //pub random_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub other_boids_to_consider: usize,
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 1000,
            cohesion_range: 12.0,
            alignment_range: 10.0,
            separation_range: 1.5,
            //min_distance_between_boids: 0.3,
            cohesion_coeff: 0.0034,
            alignment_coeff: 0.035,
            separation_coeff: 0.057,
            collision_coeff: 40.0,
            //random_coeff: 0.5,
            min_speed: 0.2,
            max_speed: 0.2,
            other_boids_to_consider: 50,
        }
    }
}
