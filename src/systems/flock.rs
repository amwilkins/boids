use bevy::color::palettes::css::*;
//use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
//use bevy::math::bounding::RayCast2d;
use bevy::math::ops::sqrt;
//use bevy_rand::prelude::*;
use rand::RngExt;
//use rand_core::Rng;

use crate::prelude::*;
use rand::seq::IteratorRandom;

pub fn flock(
    mut boid_query: Query<(&mut Boid, &mut Transform, Entity), With<Flock>>,
    mut gizmos: Gizmos,
    boid_settings: Res<BoidSettings>,
    //diagnostics: Res<DiagnosticsStore>,
    grid_query: Res<Grid>,
) {
    let grid = grid_query.as_ref();
    let boid_prime = boid_query.iter().nth(0).unwrap();
    gizmos.circle_2d(boid_prime.0.position, boid_settings.cohesion_range, WHITE);

    boid_query.par_iter_mut().for_each(|(mut boid, _, _)| {
        boid.acceleration = Vec2::ZERO;
        random_turn(&mut boid);
        update_from_neighbors(&mut boid, &boid_settings, &grid);
        update_basics(&mut boid, &boid_settings);
    });

    boid_query.iter_mut().for_each(|(boid, mut transform, _)| {
        // rotate and move
        transform.translation = boid.position.extend(50.0);
        let heading = transform.translation.xy() + boid.velocity;
        transform.look_at(heading.extend(0.0), Dir3::Z);
    });
}

fn update_basics(boid: &mut Boid, boid_settings: &Res<BoidSettings>) {
    let pos = boid.position;
    boid.acceleration += stay_in_bounds(pos);
    //boid.position = loop_map(pos);

    let mut new_velocity = boid.velocity + boid.acceleration;
    let current_speed = sqrt(new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y);

    if current_speed < boid_settings.min_speed {
        new_velocity = (new_velocity / current_speed) * boid_settings.min_speed;
    }

    new_velocity = new_velocity.normalize_or_zero() * boid_settings.max_speed;

    boid.velocity = new_velocity;
    boid.position += new_velocity;
}

fn update_from_neighbors(boid: &mut Boid, boid_settings: &Res<'_, BoidSettings>, grid: &Grid) {
    let mut rand = rand::rng();
    let all_neighbors: Vec<&Boid> = get_near_boids(grid, boid.position, boid_settings.count);
    let neighbors: Vec<&Boid> = all_neighbors
        .into_iter()
        .filter(|neighbor| {
            neighbor.position.distance(boid.position) < boid_settings.cohesion_range
                && neighbor.position != boid.position
        })
        .sample(&mut rand, boid_settings.other_boids_to_consider);
    let neighbor_positions = neighbors.iter().map(|x| x.position).collect();

    // keep seperate
    boid.acceleration += seperation(
        &boid.position,
        &neighbor_positions,
        &boid_settings.separation_range,
        &boid_settings.separation_coeff,
        &boid_settings.collision_coeff,
    );

    // keep together
    boid.acceleration += cohesion(
        &boid.position,
        &neighbor_positions,
        &boid_settings.cohesion_range,
        &boid_settings.cohesion_coeff,
    );

    // align
    let boid_clone = boid.clone();
    boid.velocity += alignment(
        &boid_clone,
        &neighbors,
        &boid_settings.alignment_range,
        &boid_settings.alignment_coeff,
    );
}

fn seperation(
    boid: &Vec2,
    other_boids: &Vec<Vec2>,
    seperation_range: &f32,
    seperation_coeff: &f32,
    _collision_coeff: &f32,
) -> Vec2 {
    let mut acceleration = Vec2::ZERO;

    other_boids.iter().for_each(|pos| {
        let distance = pos.distance(*boid);

        if distance < *seperation_range {
            acceleration += (boid - pos) / distance;
        };
    });
    acceleration = acceleration.normalize_or_zero() * seperation_coeff;
    acceleration
}

fn cohesion(
    boid: &Vec2,
    other_boids: &Vec<Vec2>,
    _cohesion_range: &f32,
    cohesion_coeff: &f32,
) -> Vec2 {
    let total: Vec2 = other_boids.iter().sum();

    let average = if other_boids.len() > 0 {
        total / (other_boids.len() as f32)
    } else {
        total
    };
    let acceleration = ((average - boid) / 2.0).normalize_or_zero() * cohesion_coeff;
    acceleration
}

fn alignment(
    boid: &Boid,
    other_boids: &Vec<&Boid>,
    alignment_range: &f32,
    alignment_coeff: &f32,
) -> Vec2 {
    let mut velocity = Vec2::ZERO;
    let mut nearby_boids = 0;

    other_boids.iter().for_each(|other| {
        let distance = other.position.distance(boid.position);
        if distance < *alignment_range && distance > 0.0 {
            nearby_boids += 1;
            velocity += other.velocity;
        }
    });
    if nearby_boids > 0 {
        velocity /= nearby_boids as f32;
        velocity = (velocity - boid.velocity) * *alignment_coeff;
    }
    velocity
}

fn stay_in_bounds(pos: Vec2) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    let turn_factor: f32 = 0.5;
    let margin = 5.0;

    if pos.x < margin {
        acceleration.x += (margin - pos.x) / (pos.x - 0.0);
    } else if pos.x > (MAP_SIZE as f32 - margin) {
        acceleration.x = (MAP_SIZE as f32 - margin - pos.x) / (MAP_SIZE as f32 - pos.x);
    }

    if pos.y < margin {
        // let force_multiplier = (margin - pos.y) / margin;
        // acceleration.y += (margin - pos.y) * turn_factor * force_multiplier;
        acceleration.y += (margin - pos.y) / (pos.y - 0.0);
    } else if pos.y > (MAP_SIZE as f32 - margin) {
        let force_multiplier = (MAP_SIZE as f32 - margin - pos.y) / margin;
        acceleration.y -= (margin - pos.y) * turn_factor * force_multiplier;
    }

    acceleration = acceleration.normalize_or_zero() * turn_factor;
    acceleration
}

fn loop_map(pos: Vec2) -> Vec2 {
    let mut new_pos = pos;
    if pos.x < 0.0 {
        new_pos.x = MAP_SIZE as f32;
    }
    if pos.x > (MAP_SIZE as f32) {
        new_pos.x = 0.0;
    }
    if pos.y < 0.0 {
        new_pos.y = MAP_SIZE as f32;
    }
    if pos.y > (MAP_SIZE as f32) {
        new_pos.y = 0.0;
    }
    new_pos
}

// fn show_vision_range(gizmos: &mut Gizmos, pos: &Vec2, size: f32) {
//     gizmos.circle_2d(*pos, size, WHITE);

//create_raycast(&mut gizmos, &query.0.position, heading_angle);
// fn create_raycast(gizmos: &mut Gizmos, pos: &Vec2, heading: f32) {
//     let left_dir = Dir2::new(Vec2::from_angle(1.85 + heading)).unwrap();
//     let right_dir = Dir2::new(Vec2::from_angle(1.3 + heading)).unwrap();
//
//     let left_ray = Ray2d {
//         origin: *pos,
//         direction: left_dir,
//     };
//     let right_ray = Ray2d {
//         origin: *pos,
//         direction: right_dir,
//     };
//
//     let _left_raycast = RayCast2d::from_ray(left_ray, 5.0);
//     let _right_raycast = RayCast2d::from_ray(right_ray, 5.0);

// gizmos.line_2d(
//     *pos,
//     *pos + left_raycast.ray.direction * left_raycast.max,
//     WHITE,
// );
// gizmos.line_2d(
//     *pos,
//     *pos + right_raycast.ray.direction * right_raycast.max,
//     WHITE,
// );
// }

fn get_near_boids(grid: &Grid, pos: Vec2, _n: usize) -> Vec<&Boid> {
    let (cx, cy) = (
        (pos.x / grid.cell_size) as i32,
        (pos.y / grid.cell_size) as i32,
    );
    let mut result = Vec::<&Boid>::with_capacity(600);
    if let Some(boids) = grid.cells.get(&(cx, cy)) {
        result.extend(boids);
    }
    let mut other_boids = Vec::<&Boid>::with_capacity(600);

    // get surrounding cells
    for dx in -1..=1 {
        for dy in -1..=1 {
            let cell = (cx + dx, cy + dy);
            if cell != (cx, cy)
                && let Some(boids) = grid.cells.get(&cell)
            {
                other_boids.extend(boids);
            }
        }
    }
    result.extend(other_boids);
    result
}

fn random_turn(boid: &mut Boid) {
    let mut rand = rand::rng();
    // let old_velocity = boid.velocity.clone();

    let target = Vec2::from_angle(boid.velocity.to_angle() + rand.random_range(-0.08..=0.08));
    boid.acceleration += boid.acceleration.rotate(target);
    // //boid.velocity += boid.velocity.rotate(target);
}
