use bevy::color::palettes::css::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::math::bounding::RayCast2d;
use bevy::math::ops::sqrt;
use bevy_rand::prelude::*;
use rand_core::Rng;

use crate::prelude::*;

pub fn flock(
    mut boid_query: Query<(&mut Boid, &mut Transform, Entity), With<Flock>>,
    mut gizmos: Gizmos,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    boid_settings: Res<BoidSettings>,
    diagnostics: Res<DiagnosticsStore>,
    grid: ResMut<Grid>,
) {
    let boid_prime = boid_query.iter().nth(0).unwrap();
    gizmos.circle_2d(boid_prime.0.position, boid_settings.cohesion_range, WHITE);

    // // Logging
    if let Some(frame) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_COUNT) {
        let frame = frame.value().unwrap();
        if frame % (60.0 * 2.0) == 0.0 {
            let others = grid.get_neighboids(boid_prime.0.position, 100);
            let cell = (
                (boid_prime.0.position.x / grid.cell_size) as i32,
                (boid_prime.0.position.y / grid.cell_size) as i32,
            );
            info!("Boid Prime cell: {:?}", cell);
            info!("Neighboids: {:?}", others.len());
        }
    }

    // calculate new state
    for mut query in boid_query.iter_mut() {
        let mut neighbors = grid.get_neighboids(query.0.position, 100);
        //neighbors.truncate(100);

        let neighbor_positions = neighbors.iter().map(|x| x.position).collect();

        let transform = query.1;
        let old_acceleration = query.0.acceleration.clone().normalize_or_zero();

        // start with a small random acceleration
        // actually make this random rotate left/right a small amount
        query.0.acceleration += Vec2::new(
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
        ) * boid_settings.random_coeff;

        // keep seperate
        query.0.acceleration += seperation(
            &transform.translation.xy(),
            &neighbor_positions,
            &boid_settings.separation_range,
            &boid_settings.separation_coeff,
            &boid_settings.min_distance_between_boids,
            &boid_settings.collision_coeff,
        );

        // keep together
        query.0.acceleration += cohesion(
            &transform.translation.xy(),
            &neighbor_positions,
            &boid_settings.cohesion_range,
            &boid_settings.cohesion_coeff,
        );

        // align
        let boid_clone = query.0.clone();
        query.0.velocity += alignment(
            &boid_clone,
            &neighbors,
            &boid_settings.alignment_range,
            &boid_settings.alignment_coeff,
        );

        query.0.acceleration += stay_in_bounds(&boid_clone);
        query.0.acceleration = query
            .0
            .acceleration
            .clamp(Vec2::splat(-1.0), Vec2::splat(1.0));
        query.0.acceleration = (query.0.acceleration + old_acceleration) / 25.0;

        let mut vel = query.0.velocity + query.0.acceleration;
        let max_speed = boid_settings.max_speed;
        let current_speed = sqrt(vel.x * vel.x + vel.y * vel.y);

        if current_speed < boid_settings.min_speed {
            vel.x = (vel.x / current_speed) * boid_settings.min_speed;
            vel.y = (vel.y / current_speed) * boid_settings.min_speed;
        }
        query.0.velocity = vel.clamp(Vec2::splat(-max_speed), Vec2::splat(max_speed));
        query.0.position += vel;
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
    let mut acceleration = Vec2::new(0.0, 0.0);

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
    other_boids: &Vec<Boid>,
    alignment_range: &f32,
    alignment_coeff: &f32,
) -> Vec2 {
    let mut velocity = Vec2::new(0.0, 0.0);
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
fn stay_in_bounds(boid: &Boid) -> Vec2 {
    let mut acceleration = Vec2::ZERO;
    let turn_factor: f32 = 0.5;
    let margin = 5.0;

    // avoid walls
    if boid.position.x < margin {
        let force_multiplier = (margin - boid.position.x) / margin;
        acceleration.x += (margin - boid.position.x) * turn_factor * force_multiplier;
    } else if boid.position.x > (MAP_SIZE as f32 - margin) {
        let force_multiplier = (MAP_SIZE as f32 - margin - boid.position.x) / margin;
        acceleration.x -= (margin - boid.position.x) * turn_factor * force_multiplier;
    }

    if boid.position.y < margin {
        let force_multiplier = (margin - boid.position.y) / margin;
        acceleration.y += (margin - boid.position.y) * turn_factor * force_multiplier;
    } else if boid.position.y > (MAP_SIZE as f32 - margin) {
        let force_multiplier = (MAP_SIZE as f32 - margin - boid.position.y) / margin;
        acceleration.y -= (margin - boid.position.y) * turn_factor * force_multiplier;
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
