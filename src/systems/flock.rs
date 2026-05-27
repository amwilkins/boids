use std::{ops::Div, str};
use bevy_rand::prelude::*;
use rand_core::Rng;

use bevy::math::VectorSpace;

use crate::prelude::*;

pub fn flock(
    mut boids: Query<(&mut Boid, &mut Transform), With<Flock>>,
    time: Res<Time>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let centeroid: Vec2 =
        (boids.iter().map(|x| x.1.translation.xy()).sum::<Vec2>()).div(boids.count() as f32);

    //info!("Centeroid of boids: {}", centeroid);

    // Cohesion
    //
    // Separation
    //
    // Alignment

    // calculate new state
    for mut boid in boids.iter_mut().map(|x| x.0) {

        // currently just random acceleration
        boid.acceleration = Vec2::new(
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            (rng.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            );
        attract_center(&mut boid);

        let acc = boid.acceleration / 1000.0;
        boid.velocity += acc;
        boid.velocity = boid.velocity.clamp(
            -Vec2::splat(BOID_MAX_SPEED).div(2 as f32),
            Vec2::splat(BOID_MAX_SPEED).div(2 as f32),
            );
        let vel = boid.velocity;

        //turn towards velocity


        boid.position += vel;
        // velocity
        // acceleration
    }
    
    if time.elapsed_secs_f64().round() % 10.0 == 0.0 {
        for boid in &boids {
            info!("Boid info: {:?}", boid.0);
            info!("Boid transform: {:?}", boid.1);
            break;
        }
    }

    // apply changes
    for mut boid in boids.iter_mut() {
        update_translation(&boid.0, &mut boid.1, &time);
    }

    // let player_position = if let Ok(player_transform) = player.single() {
    //         player_transform.translation.truncate()
    //     } else {
    //         Vec2::ZERO
    //     };
    //
    //     for mut mob_pos in mobs.iter_mut() {
    //         let mut target = player_position - mob_pos.translation.truncate();//.normalize_or_zero();
    //         if target.x.abs() < 0.15 {
    //             target.x = 0.;
    //         }
    //         if target.y.abs() < 0.15 {
    //             target.y = 0.;
    //         }
    //         let move_delta = target.normalize_or_zero() * move_speed * time.delta_secs();
    //         mob_pos.translation += move_delta.extend(0.);
    //     }
}

// Calculate all the boid updates first,
// Then go through and apply them to the Transforms
// That way the calculations are from a static point,
// and maybe we can save some cpu cycles
// and prevent any weirdness

fn cohesion() {}

fn update_translation(boid: &Boid, transform: &mut Transform, time: &Res<Time>) {
    let heading = boid.position + boid.velocity;
    transform.translation = boid
        .position
        .extend(50.0)
        .clamp(Vec3::splat(0.0), Vec3::splat(MAP_SIZE as f32));
    transform.look_at(heading.extend(0.0), Dir3::Z);
}

// avoiding walls

fn attract_center(boid: &mut Boid){
        if boid.position.x < (MAP_SIZE as f32 / 2.0){
            boid.acceleration.x += 0.05
        } else {
            boid.acceleration.x -= 0.05
        }
        if boid.position.y < (MAP_SIZE as f32 / 2.0){
            boid.acceleration.y += 0.05
        } else {
            boid.acceleration.y -= 0.05
        }
        if boid.position.x < 2.0 {
            boid.acceleration.x += 1.0
        }
        if boid.position.x > (MAP_SIZE as f32 - 2.0){
            boid.acceleration.x -= 1.0
        }
        if boid.position.y < 2.0 {
            boid.acceleration.y += 1.0
        }
        if boid.position.y > (MAP_SIZE as f32 - 2.0){
            boid.acceleration.y -= 1.0
        }

}

