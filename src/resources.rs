pub use crate::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug)]
pub struct Grid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl FromWorld for Grid {
    fn from_world(world: &mut World) -> Self {
        let boid_settings = world.resource::<BoidSettings>();

        Grid {
            cell_size: boid_settings.cohesion_range,
            cells: HashMap::with_capacity(boid_settings.count),
        } } }

impl Grid {
    pub fn insert(&mut self, entity: &Entity, pos: &Vec2) {
        let cell = (
            (pos.x / self.cell_size) as i32,
            (pos.y / self.cell_size) as i32,
        );
        self.cells.entry(cell).or_default().push(entity.clone());
    }

    pub fn get_neighboids(&self, pos: Vec2) -> Vec<Entity> {
        let (cx, cy) = (
            (pos.x / self.cell_size) as i32,
            (pos.y / self.cell_size) as i32,
        );
        let mut result = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell = (cx + dx, cy + dy);
                if let Some(ids) = self.cells.get(&cell) {
                    result.extend(ids);
                }
            }
        }
        result
    }

}

#[derive(Resource, Debug)]
pub struct BoidSettings {
    pub count: usize,
    pub cohesion_range: f32,
    pub alignment_range: f32,
    pub separation_range: f32,
    pub min_distance_between_boids: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub collision_coeff: f32,
    pub random_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 200,
            cohesion_range: 15.0,
            alignment_range: 15.0,
            separation_range: 4.0,
            min_distance_between_boids: 0.2,
            cohesion_coeff: 0.001,
            //cohesion_coeff: 0.0005,
            alignment_coeff: 0.8,
            separation_coeff: 0.003,
            collision_coeff: 40.0,
            random_coeff: 0.4,
            min_speed: 0.09,
            max_speed: 0.15,
        }
    }
}
