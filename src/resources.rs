
pub use crate::prelude::*;
// use rand::{
//     *,
// };
use std::collections::HashMap;
//use std::collections::BTreeMap;

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Debug)]
pub struct Grid {
    pub cell_size: f32,
    //pub cells: BTreeMap<(i32, i32), Vec<Boid>>,
    pub cells: HashMap<(i32, i32), Vec<Boid>>,
}

impl FromWorld for Grid {
    fn from_world(world: &mut World) -> Self {
        let boid_settings = world.resource::<BoidSettings>();

        Grid {
            cell_size: boid_settings.cohesion_range,
            //cells: BTreeMap::new(),
            cells: HashMap::with_capacity(boid_settings.count),
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

    pub fn get_neighboids(&self, pos: Vec2, _n: usize) -> Vec<&Boid> {
        let (cx, cy) = (
            (pos.x / self.cell_size) as i32,
            (pos.y / self.cell_size) as i32,
        );
        let mut result = Vec::<&Boid>::with_capacity(450);
        result.extend(self.cells.get(&(cx, cy)).unwrap());//.clone());

        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell = (cx + dx, cy + dy);
                if cell != (cx, cy)
                    && let Some(boids) = self.cells.get(&cell)
                {
                    result.extend(boids);
                }
            }
        }
        // //result = local_cell.append(near_cells.sample(&mut rand::rng(), n - local_cell.len()).cloned().collect());
        // near_cells.shuffle(&mut rand::rng());
        // near_cells.truncate(50);
        // result.append(&mut near_cells);
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
            count: 5000,
            cohesion_range: 10.0,
            alignment_range: 8.0,
            separation_range: 3.0,
            min_distance_between_boids: 0.2,
            cohesion_coeff: 0.002,
            //cohesion_coeff: 0.0005,
            alignment_coeff: 0.6,
            separation_coeff: 0.003,
            collision_coeff: 40.0,
            random_coeff: 0.4,
            min_speed: 0.12,
            max_speed: 0.16,
        }
    }
}
