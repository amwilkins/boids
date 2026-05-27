use bevy::color::palettes::css::*;
use bevy_rand::prelude::*;
use rand_core::Rng;
use std::ops::Div;

use bevy::math::{bounding::RayCast2d, VectorSpace};

use crate::prelude::*;

pub fn flock(
    mut boids: Query<(&mut Boid, &mut Transform), With<Flock>>,
    // readonly_boids: Query<&mut Boid, With<Flock>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let centeroid: Vec2 =
        (boids.iter().map(|x| x.1.translation.xy()).sum::<Vec2>()).div(boids.count() as f32);

    let boid_positions: Vec<Vec2> = boids.iter_mut().map(|x| x.1.translation.xy()).collect();
    let boid_velocities: Vec<Vec2> = boids.iter_mut().map(|x| x.0.velocity).collect();

    // calculate new state
    for query in boids.iter_mut() {
        let mut boid = query.0;
        let transform = query.1;
        let pos = boid.position.clone();

        // currently just random acceleration
        boid.acceleration = Vec2::new(
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
        );
        stay_in_bounds(&mut boid);

        // // center of boids
        // boid.acceleration += (centeroid - pos) * 0.01;
        
        // keep seperate
        boid.acceleration += seperation(&transform.translation.xy(), &boid_positions);

        // keep seperate
        boid.acceleration += cohesion(&transform.translation.xy(), &boid_positions);

        // align
        let mut vel = boid.velocity;
        boid.acceleration += alignment(&vel, &boid_positions, &boid_velocities);

        let acc = boid.acceleration / 1000.0;
        boid.velocity += acc;
        // boid.velocity = boid.velocity.clamp(
        //     //-Vec2::splat(BOID_MAX_SPEED).div(2 as f32),
        //     -Vec2::splat(0.5),
        //     Vec2::splat(0.5),
        //     //Vec2::splat(BOID_MAX_SPEED).div(2 as f32),
        // );

        // boid.velocity.x = boid.velocity.x.clamp(-BOID_MAX_SPEED, BOID_MAX_SPEED);
        // boid.velocity.y = boid.velocity.y.clamp(-BOID_MAX_SPEED, BOID_MAX_SPEED);

        vel = boid.velocity.normalize_or_zero() / 10.0;
        boid.position += vel;
        boid.position = boid
            .position
            .clamp(Vec2::splat(0.0), Vec2::splat(MAP_SIZE as f32));

        let heading_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        create_raycast(&mut gizmos, &boid.position, heading_angle);
    }

    // if time.elapsed_secs_f64().round() % 10.0 == 0.0 {
    //     for boid in &boids {
    //         info!("Boid info: {:?}", boid.0);
    //         info!("Boid transform: {:?}", boid.1);
    //         info!("Boid rotation X: {:?}");
    //         break;
    //     }
    // }

    // apply changes
    for mut boid in boids.iter_mut() {
        update_translation(&boid.0, &mut boid.1, &time);
    }
}

fn update_translation(boid: &Boid, transform: &mut Transform, time: &Res<Time>) {
    let heading = boid.position + boid.velocity;
    transform.translation = boid.position.extend(50.0);
    transform.look_at(heading.extend(0.0), Dir3::Z);
}

fn seperation(boid: &Vec2, other_boids: &Vec<Vec2>) -> Vec2 {
    let mut acceleration = Vec2::new(0.0, 0.0);

    other_boids.iter().for_each(|pos| {
        if pos.distance_squared(*boid) < 6.0 {
            acceleration += boid - pos
        }
    });

    acceleration
}

fn cohesion(boid: &Vec2, other_boids: &Vec<Vec2>) -> Vec2 {
    let mut acceleration = Vec2::new(0.0, 0.0);

    other_boids.iter().for_each(|pos| {
        if pos.distance_squared(*boid) < 36.0 {
            acceleration -= boid - pos
        }
    });

    acceleration
}


fn alignment(boid: &Vec2, other_pos: &Vec<Vec2>, other_vec: &Vec<Vec2>) -> Vec2 {
    let mut acceleration = Vec2::new(0.0, 0.0);
    let other_boids: Vec<(&Vec2, &Vec2)> = other_pos.iter().zip(other_vec.iter()).collect();

    other_boids.iter().for_each(|(pos, vec)| {
        if pos.distance_squared(*boid) < 64.0 {
            acceleration += *vec
        }
    });

    acceleration
}

// avoiding walls
fn stay_in_bounds(boid: &mut Boid) {
    // if boid.position.x < (MAP_SIZE as f32 / 2.0) {
    //     boid.acceleration.x += 0.05
    // } else {
    //     boid.acceleration.x -= 0.05
    // }
    // if boid.position.y < (MAP_SIZE as f32 / 2.0) {
    //     boid.acceleration.y += 0.05
    // } else {
    //     boid.acceleration.y -= 0.05
    // }

    if boid.position.x < 4.0 {
        boid.acceleration.x += 1.0
    }
    if boid.position.x > (MAP_SIZE as f32 - 4.0) {
        boid.acceleration.x -= 1.0
    }
    if boid.position.y < 4.0 {
        boid.acceleration.y += 1.0
    }
    if boid.position.y > (MAP_SIZE as f32 - 4.0) {
        boid.acceleration.y -= 1.0
    }
}

fn create_raycast(gizmos: &mut Gizmos, pos: &Vec2, heading: f32) {
    let left_dir = Dir2::new(Vec2::from_angle(1.85 + heading)).unwrap();
    let right_dir = Dir2::new(Vec2::from_angle(1.3 + heading)).unwrap();

    let left_ray = Ray2d {
        origin: *pos,
        direction: left_dir,
    };
    let right_ray = Ray2d {
        origin: *pos,
        direction: right_dir,
    };

    let left_raycast = RayCast2d::from_ray(left_ray, 5.0);
    let right_raycast = RayCast2d::from_ray(right_ray, 5.0);

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
