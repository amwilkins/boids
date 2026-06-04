use std::thread::Thread;

use bevy::color::palettes::css::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::math::bounding::RayCast2d;
use bevy::math::ops::sqrt;
use bevy_rand::prelude::*;
use rand_core::Rng;

use crate::prelude::*;
use rand::seq::IteratorRandom;

pub fn flock(
    mut boid_query: Query<(&mut Boid, &mut Transform, Entity), With<Flock>>,
    mut gizmos: Gizmos,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    boid_settings: Res<BoidSettings>,
    diagnostics: Res<DiagnosticsStore>,
    grid_query: Res<Grid>,
) {
    let grid = grid_query.as_ref();
    let boid_prime = boid_query.iter().nth(0).unwrap();
    gizmos.circle_2d(boid_prime.0.position, boid_settings.cohesion_range, WHITE);
    let mut rand = rand::rng();

    // sets different groups to run each frame
    let group = if let Some(frame) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
        (frame.value().unwrap() as u32 % 10) + 1 // + 1 to prevent div by zero later
    } else {
        rng.next_u32() % 10
    };


    for (mut boid, transform, entity) in boid_query.iter_mut() {
        // update position from last change, but don't look at the other boids
        if entity.index_u32() % group >= 6 { // this int is what % of boids update their heading
            boid.acceleration = boid.acceleration.rotate(
                Vec2::new(
                    (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
                    (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
                ) * boid_settings.random_coeff,
            );

            let old_acceleration = boid.acceleration.clone().normalize_or_zero();
            let pos = boid.position;
            boid.acceleration += stay_in_bounds(pos);
            boid.acceleration = (boid.acceleration + old_acceleration) / 25.0;
            boid.acceleration = boid.acceleration.clamp(Vec2::splat(-1.0), Vec2::splat(1.0));

            let vel = boid.velocity + boid.acceleration;
            boid.velocity = vel.clamp(
                Vec2::splat(-boid_settings.max_speed),
                Vec2::splat(boid_settings.max_speed),
            );
            boid.position += vel;
            continue;
        }

        let all_neighbors: Vec<&Boid> = get_near_boids(grid, boid.position, boid_settings.count);

        let neighbors: Vec<&Boid> = all_neighbors
            .iter()
            .map(|x| *x)
            .sample(&mut rand, boid_settings.other_boids_to_consider);
        let neighbor_positions = neighbors.iter().map(|x| x.position).collect();
        let old_acceleration = boid.acceleration.clone().normalize_or_zero();

        boid.acceleration = boid.acceleration.rotate(
            Vec2::new(
                (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
                (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            ) * boid_settings.random_coeff,
        );

        // keep seperate
        boid.acceleration += seperation(
            &transform.translation.xy(),
            &neighbor_positions,
            &boid_settings.separation_range,
            &boid_settings.separation_coeff,
            &boid_settings.min_distance_between_boids,
            &boid_settings.collision_coeff,
        );

        // keep together
        boid.acceleration += cohesion(
            &transform.translation.xy(),
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

        // stay in bounds
        let pos = boid.position;
        boid.acceleration += stay_in_bounds(pos);
        boid.acceleration = (boid.acceleration + old_acceleration) / 25.0;
        boid.acceleration = boid.acceleration.clamp(Vec2::splat(-1.0), Vec2::splat(1.0));

        let mut vel = boid.velocity + boid.acceleration;
        let max_speed = boid_settings.max_speed;
        let current_speed = sqrt(vel.x * vel.x + vel.y * vel.y);

        if current_speed < boid_settings.min_speed {
            vel.x = (vel.x / current_speed) * boid_settings.min_speed;
            vel.y = (vel.y / current_speed) * boid_settings.min_speed;
        }
        boid.velocity = vel.clamp(Vec2::splat(-max_speed), Vec2::splat(max_speed));
        let vel = boid.velocity;
        boid.position += vel;
    }

    // apply changes
    for mut boid in boid_query.iter_mut() {
        update_translation(&boid.0, &mut boid.1);
    }
}

fn update_translation(boid: &Boid, transform: &mut Transform) {
    // move
    transform.translation = boid.position.extend(50.0);
    // rotate
    let heading = transform.translation.xy() + boid.velocity;
    transform.look_at(heading.extend(0.0), Dir3::Z);
}

fn seperation(
    boid: &Vec2,
    other_boids: &Vec<Vec2>,
    seperation_range: &f32,
    seperation_coeff: &f32,
    min_distance_between_boids: &f32,
    collision_coeff: &f32,
) -> Vec2 {
    let mut acceleration = Vec2::ZERO;

    other_boids.iter().for_each(|pos| {
        let distance = pos.distance(*boid);

        // if almost crashing, strong seperation
        if distance < *min_distance_between_boids && distance > 0.0 {
            acceleration += (boid - pos) * collision_coeff

        // standard seperation
        } else if distance < *seperation_range {
            acceleration += (boid - pos) * seperation_coeff
        }
    });
    acceleration
}

fn cohesion(
    boid: &Vec2,
    other_boids: &Vec<Vec2>,
    cohesion_range: &f32,
    cohesion_coeff: &f32,
) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    let mut nearby_boids = 0;

    other_boids.iter().for_each(|pos| {
        if pos.distance(*boid) < *cohesion_range {
            nearby_boids += 1;
            acceleration += pos;
        }
    });

    if nearby_boids > 0 {
        acceleration /= nearby_boids as f32;
        acceleration = (acceleration - *boid) * *cohesion_coeff;
    }
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

// avoiding walls
fn stay_in_bounds(pos: Vec2) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    let turn_factor: f32 = 0.5;
    let margin = 5.0;

    // avoid walls
    if pos.x < margin {
        let force_multiplier = (margin - pos.x) / margin;
        acceleration.x += (margin - pos.x) * turn_factor * force_multiplier;
    } else if pos.x > (MAP_SIZE as f32 - margin) {
        let force_multiplier = (MAP_SIZE as f32 - margin - pos.x) / margin;
        acceleration.x -= (margin - pos.x) * turn_factor * force_multiplier;
    }

    if pos.y < margin {
        let force_multiplier = (margin - pos.y) / margin;
        acceleration.y += (margin - pos.y) * turn_factor * force_multiplier;
    } else if pos.y > (MAP_SIZE as f32 - margin) {
        let force_multiplier = (MAP_SIZE as f32 - margin - pos.y) / margin;
        acceleration.y -= (margin - pos.y) * turn_factor * force_multiplier;
    }

    acceleration
}

fn show_vision_range(gizmos: &mut Gizmos, pos: &Vec2, size: f32) {
    gizmos.circle_2d(*pos, size, WHITE);

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
}

fn get_near_boids(grid: &Grid, pos: Vec2, _n: usize) -> Vec<&Boid> {
    let (cx, cy) = (
        (pos.x / grid.cell_size) as i32,
        (pos.y / grid.cell_size) as i32,
    );
    let mut result = Vec::<&Boid>::with_capacity(100);
    if let Some(boids) = grid.cells.get(&(cx, cy)) {
        result.extend(boids);
    }
    let mut other_boids = Vec::<&Boid>::with_capacity(100);
    //let rng = &mut rand::rng();

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
    //result.extend(other_boids.sample(rng, 80));
    result.extend(other_boids);
    result
}
